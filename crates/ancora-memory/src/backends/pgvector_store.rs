//! Live, `tokio-postgres`-backed `VectorStore` implementation for pgvector.
//!
//! Requires the `pgvector` feature. Bridges the crate's synchronous
//! `VectorStore` trait to the async `tokio-postgres` client via an owned
//! multi-thread tokio runtime, so callers keep calling plain `&self` methods
//! with no `.await` anywhere in their own code.
//!
//! DDL and DML SQL reuse the pure generator functions in `backends::pgvector`
//! directly. Binding the embedding and payload parameters needs two extra
//! pieces verified empirically against a live Postgres + pgvector instance:
//! `tokio-postgres` resolves each bound parameter's Postgres type from the
//! query's own context during PREPARE (e.g. the INSERT target column, or the
//! `<=>` distance operator's operand type) and then checks that type against
//! what the Rust value's `ToSql` impl declares itself compatible with,
//! client-side, before ever sending anything. A plain `String` fails that
//! check for both the pgvector `vector` column and a `JSONB` column, so the
//! embedding is bound as `pgvector::Vector` (this crate's `postgres`
//! feature provides the matching `ToSql`/`FromSql`) and the payload as
//! `serde_json::Value` (via `tokio-postgres`'s `with-serde_json-1` feature)
//! rather than as encoded strings, even though `backends::pgvector`'s SQL
//! text itself needs no explicit `::vector`/`::jsonb` cast at all.

use std::collections::HashMap;
use std::sync::Mutex;

use pgvector::Vector as PgVector;
use tokio_postgres::NoTls;

use crate::backends::pgvector::{
    create_hnsw_index_sql, create_ivfflat_index_sql, delete_by_filter_sql, delete_by_ids_sql,
    drop_table_sql, filter_to_sql, sanitize_identifier, upsert_sql, FilterParam,
};
use crate::vector_store::{
    CollectionInfo, CollectionSpec, Distance, Filter, HnswConfig, HybridQueryRequest, IndexConfig,
    IvfFlatConfig, Payload, PayloadValue, Point, PointId, QueryRequest, ScoredPoint, VectorStore,
    VectorStoreError,
};

/// A `VectorStore` backed by a real Postgres connection with the `pgvector`
/// extension. Point ids must be numeric (`PointId::Num`): the underlying
/// table uses `id BIGINT PRIMARY KEY`, matching every DDL/DML helper in
/// `backends::pgvector`, so a `PointId::Uuid` is rejected rather than
/// silently truncated or hashed.
pub struct PgVectorStore {
    client: tokio_postgres::Client,
    runtime: tokio::runtime::Runtime,
    distances: Mutex<HashMap<String, Distance>>,
}

impl PgVectorStore {
    /// Connect to `url` (a standard `postgres://` connection string) and
    /// ensure the `vector` extension is available. Does not create any
    /// collection tables; call `create_collection` for that.
    pub fn connect(url: &str) -> Result<Self, VectorStoreError> {
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .worker_threads(2)
            .enable_all()
            .build()
            .map_err(|e| VectorStoreError::Io(e.to_string()))?;

        let (client, connection) = runtime
            .block_on(tokio_postgres::connect(url, NoTls))
            .map_err(|e| VectorStoreError::Io(e.to_string()))?;

        // tokio_postgres::connect returns a Client plus a Connection future
        // that must be polled to actually drive I/O; spawn it in the
        // background for the lifetime of the runtime.
        runtime.spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("pgvector connection error: {e}");
            }
        });

        runtime
            .block_on(client.batch_execute("CREATE EXTENSION IF NOT EXISTS vector;"))
            .map_err(|e| VectorStoreError::Io(e.to_string()))?;

        Ok(Self {
            client,
            runtime,
            distances: Mutex::new(HashMap::new()),
        })
    }
}

fn table_name(name: &str) -> Result<&str, VectorStoreError> {
    sanitize_identifier(name).map_err(VectorStoreError::InvalidFilter)
}

fn pg_err(e: tokio_postgres::Error) -> VectorStoreError {
    VectorStoreError::Io(e.to_string())
}

fn point_id_to_i64(id: &PointId) -> Result<i64, VectorStoreError> {
    match id {
        PointId::Num(n) => Ok(*n as i64),
        PointId::Uuid(s) => Err(VectorStoreError::InvalidFilter(format!(
            "pgvector backend requires numeric point ids (BIGINT primary key); got uuid `{s}`"
        ))),
    }
}

fn payload_to_json(payload: &Payload) -> serde_json::Value {
    let mut obj = serde_json::Map::new();
    for (k, v) in payload {
        let jv = match v {
            PayloadValue::String(s) => serde_json::Value::String(s.clone()),
            PayloadValue::Integer(n) => serde_json::Value::Number((*n).into()),
            PayloadValue::Float(f) => serde_json::Number::from_f64(*f)
                .map(serde_json::Value::Number)
                .unwrap_or(serde_json::Value::Null),
            PayloadValue::Bool(b) => serde_json::Value::Bool(*b),
            PayloadValue::Null => serde_json::Value::Null,
        };
        obj.insert(k.clone(), jv);
    }
    serde_json::Value::Object(obj)
}

fn json_to_payload(value: &serde_json::Value) -> Payload {
    let mut payload = Payload::new();
    let Some(obj) = value.as_object() else {
        return payload;
    };
    for (k, v) in obj {
        let pv = match v {
            serde_json::Value::String(s) => PayloadValue::String(s.clone()),
            serde_json::Value::Number(n) => match n.as_i64() {
                Some(i) => PayloadValue::Integer(i),
                None => PayloadValue::Float(n.as_f64().unwrap_or(0.0)),
            },
            serde_json::Value::Bool(b) => PayloadValue::Bool(*b),
            _ => PayloadValue::Null,
        };
        payload.insert(k.clone(), pv);
    }
    payload
}

fn cosine_query_sql(table: &str, limit: usize, offset: usize) -> String {
    format!(
        "SELECT id, payload, 1 - (embedding <=> $1) AS score \
         FROM {table} \
         ORDER BY embedding <=> $1 \
         LIMIT {limit} OFFSET {offset};"
    )
}

fn cosine_query_with_filter_sql(table: &str, limit: usize, filter_sql: &str) -> String {
    format!(
        "SELECT id, payload, 1 - (embedding <=> $1) AS score \
         FROM {table} \
         WHERE {filter_sql} \
         ORDER BY embedding <=> $1 \
         LIMIT {limit};"
    )
}

fn hybrid_query_sql(table: &str, limit: usize, alpha: f32) -> String {
    let alpha = alpha.clamp(0.0, 1.0);
    let beta = 1.0 - alpha;
    format!(
        "SELECT id, payload, \
         ({alpha} * (1 - (embedding <=> $1)) + {beta} * ts_rank(to_tsvector(payload->>'text'), plainto_tsquery($2))) AS score \
         FROM {table} \
         ORDER BY score DESC \
         LIMIT {limit};"
    )
}

/// Bind a `FilterParam` to a `tokio_postgres` query as a boxed `ToSql`.
fn bind_filter_param(p: &FilterParam) -> Box<dyn tokio_postgres::types::ToSql + Sync + Send> {
    match p {
        FilterParam::Text(s) => Box::new(s.clone()),
        FilterParam::Int(n) => Box::new(*n),
        FilterParam::Float(f) => Box::new(*f),
        FilterParam::Bool(b) => Box::new(*b),
    }
}

impl VectorStore for PgVectorStore {
    fn create_collection(&self, spec: CollectionSpec) -> Result<(), VectorStoreError> {
        let table = table_name(&spec.name)?;
        self.runtime.block_on(async {
            let create_sql = crate::backends::pgvector::create_table_sql(table, spec.dimensions);
            self.client
                .batch_execute(&create_sql)
                .await
                .map_err(pg_err)?;
            match &spec.index {
                IndexConfig::Hnsw(HnswConfig { m, ef_construct }) => {
                    self.client
                        .batch_execute(&create_hnsw_index_sql(table, *m, *ef_construct))
                        .await
                        .map_err(pg_err)?;
                }
                IndexConfig::IvfFlat(IvfFlatConfig { lists }) => {
                    self.client
                        .batch_execute(&create_ivfflat_index_sql(table, *lists))
                        .await
                        .map_err(pg_err)?;
                }
                IndexConfig::Flat => {}
            }
            Ok::<(), VectorStoreError>(())
        })?;
        self.distances
            .lock()
            .unwrap()
            .insert(spec.name.clone(), spec.distance);
        Ok(())
    }

    fn drop_collection(&self, name: &str) -> Result<(), VectorStoreError> {
        let table = table_name(name)?;
        self.runtime
            .block_on(self.client.batch_execute(&drop_table_sql(table)))
            .map_err(pg_err)?;
        self.distances.lock().unwrap().remove(name);
        Ok(())
    }

    fn describe_collection(&self, name: &str) -> Result<CollectionInfo, VectorStoreError> {
        let table = table_name(name)?;
        let (count, dims) = self.runtime.block_on(async {
            let count_row = self
                .client
                .query_one(&crate::backends::pgvector::count_sql(table), &[])
                .await
                .map_err(pg_err)?;
            let count: i64 = count_row.get(0);
            let dims: i32 = match self
                .client
                .query_opt(&crate::backends::pgvector::dimension_query_sql(table), &[])
                .await
                .map_err(pg_err)?
            {
                Some(row) => row.get(0),
                None => 0,
            };
            Ok::<_, VectorStoreError>((count, dims))
        })?;
        let distance = self
            .distances
            .lock()
            .unwrap()
            .get(name)
            .copied()
            .unwrap_or(Distance::Cosine);
        Ok(CollectionInfo {
            name: name.to_owned(),
            dimensions: dims.max(0) as usize,
            point_count: count.max(0) as u64,
            distance,
        })
    }

    fn upsert(&self, collection: &str, points: Vec<Point>) -> Result<(), VectorStoreError> {
        let table = table_name(collection)?;
        let sql = upsert_sql(table);
        self.runtime.block_on(async {
            for point in &points {
                let id = point_id_to_i64(&point.id)?;
                let vector = PgVector::from(point.vector.clone());
                let payload_json = payload_to_json(&point.payload);
                self.client
                    .execute(&sql, &[&id, &vector, &payload_json])
                    .await
                    .map_err(pg_err)?;
            }
            Ok(())
        })
    }

    fn query(
        &self,
        collection: &str,
        req: QueryRequest,
    ) -> Result<Vec<ScoredPoint>, VectorStoreError> {
        let table = table_name(collection)?;
        let vector = PgVector::from(req.vector.clone());
        self.runtime.block_on(async {
            let rows = if let Some(filter) = &req.filter {
                let (filter_sql, params) = filter_to_sql(filter, 1);
                let sql = cosine_query_with_filter_sql(table, req.top_k, &filter_sql);
                let mut bound: Vec<Box<dyn tokio_postgres::types::ToSql + Sync + Send>> =
                    vec![Box::new(vector.clone())];
                bound.extend(params.iter().map(bind_filter_param));
                let refs: Vec<&(dyn tokio_postgres::types::ToSql + Sync)> =
                    bound.iter().map(|b| b.as_ref() as _).collect();
                self.client.query(&sql, &refs).await.map_err(pg_err)?
            } else {
                let sql = cosine_query_sql(table, req.top_k, req.offset);
                self.client.query(&sql, &[&vector]).await.map_err(pg_err)?
            };
            rows_to_scored_points(&rows, req.score_threshold)
        })
    }

    fn hybrid_query(
        &self,
        collection: &str,
        req: HybridQueryRequest,
    ) -> Result<Vec<ScoredPoint>, VectorStoreError> {
        let table = table_name(collection)?;
        let vector = PgVector::from(req.dense_vector.clone());
        let sql = hybrid_query_sql(table, req.top_k, req.alpha);
        let rows = self
            .runtime
            .block_on(self.client.query(&sql, &[&vector, &req.keyword]))
            .map_err(pg_err)?;
        rows_to_scored_points(&rows, req.score_threshold)
    }

    fn delete(&self, collection: &str, ids: Vec<PointId>) -> Result<(), VectorStoreError> {
        let table = table_name(collection)?;
        let numeric_ids = ids
            .iter()
            .map(point_id_to_i64)
            .collect::<Result<Vec<i64>, _>>()?;
        if numeric_ids.is_empty() {
            return Ok(());
        }
        let sql = delete_by_ids_sql(table, numeric_ids.len());
        let refs: Vec<&(dyn tokio_postgres::types::ToSql + Sync)> = numeric_ids
            .iter()
            .map(|n| n as &(dyn tokio_postgres::types::ToSql + Sync))
            .collect();
        self.runtime
            .block_on(self.client.execute(&sql, &refs))
            .map_err(pg_err)?;
        Ok(())
    }

    fn delete_by_filter(&self, collection: &str, filter: Filter) -> Result<u64, VectorStoreError> {
        let table = table_name(collection)?;
        let (sql, params) = delete_by_filter_sql(table, &filter);
        let bound: Vec<Box<dyn tokio_postgres::types::ToSql + Sync + Send>> =
            params.iter().map(bind_filter_param).collect();
        let refs: Vec<&(dyn tokio_postgres::types::ToSql + Sync)> =
            bound.iter().map(|b| b.as_ref() as _).collect();
        let rows = self
            .runtime
            .block_on(self.client.query(&sql, &refs))
            .map_err(pg_err)?;
        Ok(rows.len() as u64)
    }
}

fn rows_to_scored_points(
    rows: &[tokio_postgres::Row],
    threshold: Option<f32>,
) -> Result<Vec<ScoredPoint>, VectorStoreError> {
    let mut out = Vec::with_capacity(rows.len());
    for row in rows {
        let id: i64 = row.get(0);
        let payload_json: serde_json::Value = row.get(1);
        let score: f64 = row.get(2);
        let score = score as f32;
        if let Some(t) = threshold {
            if score < t {
                continue;
            }
        }
        out.push(ScoredPoint {
            id: PointId::Num(id as u64),
            score,
            payload: json_to_payload(&payload_json),
        });
    }
    Ok(out)
}
