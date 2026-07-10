use std::sync::Arc;

use ancora_memory::mem_store::MemStore;
use ancora_memory::vector_store::{
    CollectionSpec, Distance, Payload, PayloadValue, Point, PointId, QueryRequest, ScoredPoint,
    VectorStore,
};

/// Build the `VectorStore` a runtime queries documents against. Config bytes
/// are the same JSON blob `ancora_runtime_new_with_config` already accepts,
/// read for an additional top-level `"memory"` key:
/// `{"memory":{"pgvector_url":"postgres://..."}}`. Missing, empty,
/// unrecognized, or unreachable config falls back to an in-memory store
/// (`MemStore`), mirroring `ModelBackend::from_config_bytes`'s
/// never-fail-on-malformed-input behavior -- a bad or temporarily-down
/// Postgres URL should not make the whole runtime unusable.
pub(crate) fn memory_store_from_config_bytes(bytes: &[u8]) -> Arc<dyn VectorStore> {
    if bytes.is_empty() {
        return Arc::new(MemStore::new());
    }
    let Ok(config) = serde_json::from_slice::<serde_json::Value>(bytes) else {
        return Arc::new(MemStore::new());
    };
    let Some(memory) = config.get("memory") else {
        return Arc::new(MemStore::new());
    };
    let Some(url) = memory.get("pgvector_url").and_then(|v| v.as_str()) else {
        return Arc::new(MemStore::new());
    };
    match ancora_memory::backends::pgvector_store::PgVectorStore::connect(url) {
        Ok(store) => Arc::new(store),
        Err(_) => Arc::new(MemStore::new()),
    }
}

// ---- wire decode: CollectionSpec ------------------------------------------

/// Decode a `CollectionSpec` from JSON bytes:
/// `{"name":"docs","dimensions":768,"distance":"cosine"}`. `distance`
/// defaults to `"cosine"` when absent or unrecognized. Returns `None` if the
/// bytes aren't valid JSON or are missing `name`/`dimensions`.
pub(crate) fn decode_collection_spec(bytes: &[u8]) -> Option<CollectionSpec> {
    let value: serde_json::Value = serde_json::from_slice(bytes).ok()?;
    let name = value.get("name")?.as_str()?.to_owned();
    let dimensions = value.get("dimensions")?.as_u64()? as usize;
    let distance = match value.get("distance").and_then(|v| v.as_str()) {
        Some("dot") => Distance::Dot,
        Some("l2") => Distance::L2,
        _ => Distance::Cosine,
    };
    Some(CollectionSpec::new(name, dimensions, distance))
}

// ---- wire decode: points (upsert) -----------------------------------------

fn json_value_to_payload_value(value: &serde_json::Value) -> PayloadValue {
    match value {
        serde_json::Value::String(s) => PayloadValue::String(s.clone()),
        serde_json::Value::Number(n) => match n.as_i64() {
            Some(i) => PayloadValue::Integer(i),
            None => PayloadValue::Float(n.as_f64().unwrap_or(0.0)),
        },
        serde_json::Value::Bool(b) => PayloadValue::Bool(*b),
        _ => PayloadValue::Null,
    }
}

fn decode_payload(value: Option<&serde_json::Value>) -> Payload {
    let mut payload = Payload::new();
    let Some(obj) = value.and_then(|v| v.as_object()) else {
        return payload;
    };
    for (k, v) in obj {
        payload.insert(k.clone(), json_value_to_payload_value(v));
    }
    payload
}

/// Decode points to upsert from a JSON array:
/// `[{"id":1,"vector":[0.1,0.2],"payload":{"text":"..."}}]`. `id` must be a
/// non-negative integer -- the FFI wire format only supports numeric point
/// ids, matching the pgvector backend's `BIGINT PRIMARY KEY` requirement, so
/// the same request works unmodified against either backend. Skips (rather
/// than fails on) any array element missing `id`/`vector`, so one malformed
/// point doesn't lose the rest of a batch.
pub(crate) fn decode_points(bytes: &[u8]) -> Option<Vec<Point>> {
    let value: serde_json::Value = serde_json::from_slice(bytes).ok()?;
    let arr = value.as_array()?;
    let points = arr
        .iter()
        .filter_map(|item| {
            let id = item.get("id")?.as_u64()?;
            let vector: Vec<f32> = item
                .get("vector")?
                .as_array()?
                .iter()
                .map(|v| v.as_f64().unwrap_or(0.0) as f32)
                .collect();
            let mut point = Point::new(PointId::Num(id), vector);
            point.payload = decode_payload(item.get("payload"));
            Some(point)
        })
        .collect();
    Some(points)
}

// ---- wire decode: query request -------------------------------------------

/// Decode a `QueryRequest` from JSON bytes:
/// `{"vector":[0.1,0.2],"top_k":5,"score_threshold":0.0}`. `top_k` defaults
/// to 10, `score_threshold` is optional.
pub(crate) fn decode_query_request(bytes: &[u8]) -> Option<QueryRequest> {
    let value: serde_json::Value = serde_json::from_slice(bytes).ok()?;
    let vector: Vec<f32> = value
        .get("vector")?
        .as_array()?
        .iter()
        .map(|v| v.as_f64().unwrap_or(0.0) as f32)
        .collect();
    let top_k = value.get("top_k").and_then(|v| v.as_u64()).unwrap_or(10) as usize;
    let mut req = QueryRequest::new(vector, top_k);
    if let Some(t) = value.get("score_threshold").and_then(|v| v.as_f64()) {
        req = req.with_score_threshold(t as f32);
    }
    Some(req)
}

// ---- wire decode: point ids (delete) --------------------------------------

/// Decode point ids to delete from a JSON array of non-negative integers:
/// `[1,2,3]`.
pub(crate) fn decode_point_ids(bytes: &[u8]) -> Option<Vec<PointId>> {
    let value: serde_json::Value = serde_json::from_slice(bytes).ok()?;
    let arr = value.as_array()?;
    arr.iter().map(|v| v.as_u64().map(PointId::Num)).collect()
}

// ---- wire encode: scored points (query response) ---------------------------

fn payload_value_to_json(value: &PayloadValue) -> serde_json::Value {
    match value {
        PayloadValue::String(s) => serde_json::Value::String(s.clone()),
        PayloadValue::Integer(n) => serde_json::Value::Number((*n).into()),
        PayloadValue::Float(f) => serde_json::Number::from_f64(*f)
            .map(serde_json::Value::Number)
            .unwrap_or(serde_json::Value::Null),
        PayloadValue::Bool(b) => serde_json::Value::Bool(*b),
        PayloadValue::Null => serde_json::Value::Null,
    }
}

fn point_id_to_json(id: &PointId) -> serde_json::Value {
    match id {
        PointId::Num(n) => serde_json::Value::Number((*n).into()),
        PointId::Uuid(s) => serde_json::Value::String(s.clone()),
    }
}

/// Encode query results as a JSON array:
/// `[{"id":1,"score":0.93,"payload":{"text":"..."}}]`.
pub(crate) fn encode_scored_points(points: &[ScoredPoint]) -> String {
    let arr: Vec<serde_json::Value> = points
        .iter()
        .map(|p| {
            let payload: serde_json::Map<String, serde_json::Value> = p
                .payload
                .iter()
                .map(|(k, v)| (k.clone(), payload_value_to_json(v)))
                .collect();
            serde_json::json!({
                "id": point_id_to_json(&p.id),
                "score": p.score,
                "payload": payload,
            })
        })
        .collect();
    serde_json::Value::Array(arr).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_config_bytes_select_in_memory_store() {
        let store = memory_store_from_config_bytes(b"");
        // In-memory store: creating and describing a collection round-trips.
        store
            .create_collection(CollectionSpec::new("t", 2, Distance::Cosine))
            .unwrap();
        assert_eq!(store.describe_collection("t").unwrap().dimensions, 2);
    }

    #[test]
    fn config_without_memory_key_selects_in_memory_store() {
        let store = memory_store_from_config_bytes(b"{}");
        store
            .create_collection(CollectionSpec::new("t", 2, Distance::Cosine))
            .unwrap();
    }

    #[test]
    fn unreachable_pgvector_url_falls_back_to_in_memory_store() {
        let bytes = br#"{"memory":{"pgvector_url":"postgres://nobody@127.0.0.1:1/nope"}}"#;
        let store = memory_store_from_config_bytes(bytes);
        // Falls back to MemStore rather than failing/panicking.
        store
            .create_collection(CollectionSpec::new("t", 2, Distance::Cosine))
            .unwrap();
    }

    #[test]
    fn decode_collection_spec_reads_name_dimensions_distance() {
        let spec =
            decode_collection_spec(br#"{"name":"docs","dimensions":3,"distance":"dot"}"#).unwrap();
        assert_eq!(spec.name, "docs");
        assert_eq!(spec.dimensions, 3);
        assert!(matches!(spec.distance, Distance::Dot));
    }

    #[test]
    fn decode_collection_spec_defaults_distance_to_cosine() {
        let spec = decode_collection_spec(br#"{"name":"docs","dimensions":3}"#).unwrap();
        assert!(matches!(spec.distance, Distance::Cosine));
    }

    #[test]
    fn decode_collection_spec_missing_fields_returns_none() {
        assert!(decode_collection_spec(br#"{"name":"docs"}"#).is_none());
        assert!(decode_collection_spec(b"not json").is_none());
    }

    #[test]
    fn decode_points_reads_id_vector_and_payload() {
        let points =
            decode_points(br#"[{"id":1,"vector":[0.1,0.2],"payload":{"text":"hi"}}]"#).unwrap();
        assert_eq!(points.len(), 1);
        assert_eq!(points[0].id, PointId::Num(1));
        assert_eq!(points[0].vector, vec![0.1, 0.2]);
        assert_eq!(
            points[0].payload.get("text"),
            Some(&PayloadValue::String("hi".to_owned()))
        );
    }

    #[test]
    fn decode_points_skips_malformed_entries() {
        let points = decode_points(br#"[{"id":1,"vector":[0.1]},{"vector":[0.2]}]"#).unwrap();
        assert_eq!(points.len(), 1);
    }

    #[test]
    fn decode_query_request_reads_vector_top_k_threshold() {
        let req = decode_query_request(br#"{"vector":[0.1,0.2],"top_k":5,"score_threshold":0.5}"#)
            .unwrap();
        assert_eq!(req.vector, vec![0.1, 0.2]);
        assert_eq!(req.top_k, 5);
        assert_eq!(req.score_threshold, Some(0.5));
    }

    #[test]
    fn decode_query_request_defaults_top_k_to_ten() {
        let req = decode_query_request(br#"{"vector":[0.1]}"#).unwrap();
        assert_eq!(req.top_k, 10);
    }

    #[test]
    fn decode_point_ids_reads_numeric_array() {
        let ids = decode_point_ids(b"[1,2,3]").unwrap();
        assert_eq!(ids, vec![PointId::Num(1), PointId::Num(2), PointId::Num(3)]);
    }

    #[test]
    fn encode_scored_points_round_trips_through_json() {
        let points = vec![ScoredPoint {
            id: PointId::Num(7),
            score: 0.42,
            payload: {
                let mut p = Payload::new();
                p.insert("text".to_owned(), PayloadValue::String("hello".to_owned()));
                p
            },
        }];
        let json = encode_scored_points(&points);
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed[0]["id"], 7);
        assert_eq!(parsed[0]["payload"]["text"], "hello");
    }
}
