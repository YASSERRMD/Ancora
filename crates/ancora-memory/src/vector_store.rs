use std::collections::HashMap;

// ---- error ---------------------------------------------------------------

/// Errors that can be returned by a VectorStore operation.
#[derive(Debug, Clone, PartialEq)]
pub enum VectorStoreError {
    /// Collection does not exist.
    NotFound(String),
    /// Collection already exists.
    AlreadyExists(String),
    /// Vector dimension mismatch.
    DimensionMismatch { expected: usize, got: usize },
    /// Filter expression is invalid or unsupported.
    InvalidFilter(String),
    /// Backend-specific I/O error.
    Io(String),
}

impl std::fmt::Display for VectorStoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotFound(n) => write!(f, "collection not found: {n}"),
            Self::AlreadyExists(n) => write!(f, "collection already exists: {n}"),
            Self::DimensionMismatch { expected, got } =>
                write!(f, "dimension mismatch: expected {expected}, got {got}"),
            Self::InvalidFilter(msg) => write!(f, "invalid filter: {msg}"),
            Self::Io(msg) => write!(f, "I/O error: {msg}"),
        }
    }
}

impl std::error::Error for VectorStoreError {}

// ---- distance metric ------------------------------------------------------

/// Vector distance metric used for similarity search.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Distance {
    Cosine,
    Dot,
    L2,
}

impl Distance {
    /// Compute the similarity score between two vectors using this metric.
    /// Higher is always more similar, regardless of the underlying geometry.
    pub fn score(&self, a: &[f32], b: &[f32]) -> f32 {
        match self {
            Distance::Cosine => cosine_similarity(a, b),
            Distance::Dot => dot_product(a, b),
            Distance::L2 => l2_similarity(a, b),
        }
    }
}

/// Cosine similarity: dot(a, b) / (|a| * |b|). Returns 1.0 for identical vectors.
pub fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    let dot: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    let mag_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let mag_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();
    if mag_a == 0.0 || mag_b == 0.0 { 0.0 } else { dot / (mag_a * mag_b) }
}

/// Dot product (inner product) of two vectors.
pub fn dot_product(a: &[f32], b: &[f32]) -> f32 {
    a.iter().zip(b.iter()).map(|(x, y)| x * y).sum()
}

/// L2 (Euclidean) distance converted to a similarity score: 1 / (1 + dist).
pub fn l2_similarity(a: &[f32], b: &[f32]) -> f32 {
    let dist: f32 = a.iter().zip(b.iter()).map(|(x, y)| (x - y).powi(2)).sum::<f32>().sqrt();
    1.0 / (1.0 + dist)
}

// ---- payload value --------------------------------------------------------

/// A value stored in a point's payload (metadata).
#[derive(Debug, Clone, PartialEq)]
pub enum PayloadValue {
    String(String),
    Integer(i64),
    Float(f64),
    Bool(bool),
    Null,
}

impl From<&str> for PayloadValue {
    fn from(s: &str) -> Self { Self::String(s.to_owned()) }
}
impl From<String> for PayloadValue {
    fn from(s: String) -> Self { Self::String(s) }
}
impl From<i64> for PayloadValue {
    fn from(n: i64) -> Self { Self::Integer(n) }
}
impl From<f64> for PayloadValue {
    fn from(f: f64) -> Self { Self::Float(f) }
}
impl From<bool> for PayloadValue {
    fn from(b: bool) -> Self { Self::Bool(b) }
}

/// Metadata map attached to every point.
pub type Payload = HashMap<String, PayloadValue>;

// ---- payload schema -------------------------------------------------------

/// Field type annotation for a payload key.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FieldType { Keyword, Integer, Float, Bool }

/// Schema hint for a collection's payload fields.
/// Backends may use this to build indexes for filter acceleration.
#[derive(Debug, Clone, Default)]
pub struct PayloadSchema {
    pub fields: HashMap<String, FieldType>,
}

impl PayloadSchema {
    pub fn new() -> Self { Self::default() }
    pub fn field(mut self, name: impl Into<String>, kind: FieldType) -> Self {
        self.fields.insert(name.into(), kind);
        self
    }

    /// Validate that a payload's typed fields match the schema.
    ///
    /// Returns `Err` with a description of the first mismatch found.
    pub fn validate(&self, payload: &Payload) -> Result<(), String> {
        for (key, expected_type) in &self.fields {
            if let Some(val) = payload.get(key) {
                let ok = match (expected_type, val) {
                    (FieldType::Keyword, PayloadValue::String(_)) => true,
                    (FieldType::Integer, PayloadValue::Integer(_)) => true,
                    (FieldType::Float, PayloadValue::Float(_)) => true,
                    (FieldType::Bool, PayloadValue::Bool(_)) => true,
                    _ => false,
                };
                if !ok {
                    return Err(format!("field `{key}` has wrong type for schema"));
                }
            }
        }
        Ok(())
    }
}

// ---- point id / point / scored point --------------------------------------

/// Identifier for a vector point. Backends may use u64 or UUID strings.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PointId {
    Num(u64),
    Uuid(String),
}

impl From<u64> for PointId {
    fn from(n: u64) -> Self { Self::Num(n) }
}
impl From<&str> for PointId {
    fn from(s: &str) -> Self { Self::Uuid(s.to_owned()) }
}

/// A vector point to be stored or retrieved.
#[derive(Debug, Clone)]
pub struct Point {
    pub id: PointId,
    pub vector: Vec<f32>,
    pub payload: Payload,
    /// Optional named vectors for multi-vector collections.
    pub named_vectors: HashMap<String, Vec<f32>>,
}

impl Point {
    pub fn new(id: impl Into<PointId>, vector: Vec<f32>) -> Self {
        Self { id: id.into(), vector, payload: Payload::new(), named_vectors: HashMap::new() }
    }

    pub fn with_payload(mut self, key: impl Into<String>, val: impl Into<PayloadValue>) -> Self {
        self.payload.insert(key.into(), val.into()); self
    }

    pub fn with_named_vector(mut self, name: impl Into<String>, vec: Vec<f32>) -> Self {
        self.named_vectors.insert(name.into(), vec); self
    }
}

/// A point returned by a search, annotated with its similarity score.
#[derive(Debug, Clone)]
pub struct ScoredPoint {
    pub id: PointId,
    pub score: f32,
    pub payload: Payload,
}

// ---- filter expression ----------------------------------------------------

/// A metadata filter that narrows the points visible to a query.
#[derive(Debug, Clone)]
pub enum Filter {
    /// Match points where a payload key equals a value.
    Eq(String, PayloadValue),
    /// Match points where a payload key does NOT equal a value.
    Ne(String, PayloadValue),
    /// Match points where a numeric payload key is greater than a threshold.
    Gt(String, PayloadValue),
    /// Match points where a numeric payload key is less than a threshold.
    Lt(String, PayloadValue),
    /// Logical AND of two filters.
    And(Box<Filter>, Box<Filter>),
    /// Logical OR of two filters.
    Or(Box<Filter>, Box<Filter>),
}

impl Filter {
    pub fn and(self, other: Filter) -> Filter {
        Filter::And(Box::new(self), Box::new(other))
    }
    pub fn or(self, other: Filter) -> Filter {
        Filter::Or(Box::new(self), Box::new(other))
    }
}

/// Evaluate a filter expression against a payload.
///
/// Returns `true` if the payload satisfies the filter. Used by the in-memory
/// reference implementation; backends with native filter support may bypass this.
pub fn filter_matches(payload: &Payload, filter: &Filter) -> bool {
    match filter {
        Filter::Eq(key, val) => payload.get(key).map(|v| v == val).unwrap_or(false),
        Filter::Ne(key, val) => payload.get(key).map(|v| v != val).unwrap_or(true),
        Filter::Gt(key, val) => match (payload.get(key), val) {
            (Some(PayloadValue::Integer(a)), PayloadValue::Integer(b)) => a > b,
            (Some(PayloadValue::Float(a)), PayloadValue::Float(b)) => a > b,
            _ => false,
        },
        Filter::Lt(key, val) => match (payload.get(key), val) {
            (Some(PayloadValue::Integer(a)), PayloadValue::Integer(b)) => a < b,
            (Some(PayloadValue::Float(a)), PayloadValue::Float(b)) => a < b,
            _ => false,
        },
        Filter::And(a, b) => filter_matches(payload, a) && filter_matches(payload, b),
        Filter::Or(a, b) => filter_matches(payload, a) || filter_matches(payload, b),
    }
}

// ---- collection lifecycle -------------------------------------------------

/// HNSW index configuration (used by backends that support it).
#[derive(Debug, Clone)]
pub struct HnswConfig {
    /// Number of bidirectional links per node. Higher = more accuracy, more memory.
    pub m: u16,
    /// Size of the dynamic candidate list during construction.
    pub ef_construct: u16,
}

impl Default for HnswConfig {
    fn default() -> Self { Self { m: 16, ef_construct: 100 } }
}

/// IVF-Flat index configuration for backends that support it (e.g., pgvector).
#[derive(Debug, Clone)]
pub struct IvfFlatConfig {
    pub lists: u32,
}

impl Default for IvfFlatConfig {
    fn default() -> Self { Self { lists: 100 } }
}

/// Index algorithm options.
#[derive(Debug, Clone)]
pub enum IndexConfig {
    Hnsw(HnswConfig),
    IvfFlat(IvfFlatConfig),
    Flat,
}

/// Specification for creating a new vector collection.
#[derive(Debug, Clone)]
pub struct CollectionSpec {
    pub name: String,
    pub dimensions: usize,
    pub distance: Distance,
    pub payload_schema: PayloadSchema,
    pub index: IndexConfig,
}

impl CollectionSpec {
    pub fn new(name: impl Into<String>, dimensions: usize, distance: Distance) -> Self {
        Self {
            name: name.into(),
            dimensions,
            distance,
            payload_schema: PayloadSchema::new(),
            index: IndexConfig::Hnsw(HnswConfig::default()),
        }
    }
    pub fn with_schema(mut self, schema: PayloadSchema) -> Self {
        self.payload_schema = schema; self
    }
    pub fn with_index(mut self, index: IndexConfig) -> Self {
        self.index = index; self
    }
}

/// Description of an existing collection returned by `describe_collection`.
#[derive(Debug, Clone)]
pub struct CollectionInfo {
    pub name: String,
    pub dimensions: usize,
    pub point_count: u64,
    pub distance: Distance,
}

// ---- query request --------------------------------------------------------

/// Named-vector collection configuration for multi-vector points.
///
/// Some backends (e.g., Qdrant) allow a single point to carry multiple named
/// embedding vectors -- one per modality (text, image, audio).
#[derive(Debug, Clone, Default)]
pub struct NamedVectorConfig {
    /// Map from vector name to its dimensionality and distance metric.
    pub vectors: HashMap<String, (usize, Distance)>,
}

impl NamedVectorConfig {
    pub fn new() -> Self { Self::default() }
    pub fn add(mut self, name: impl Into<String>, dims: usize, distance: Distance) -> Self {
        self.vectors.insert(name.into(), (dims, distance));
        self
    }
}

/// A dense-vector similarity search request.
#[derive(Debug, Clone)]
pub struct QueryRequest {
    pub vector: Vec<f32>,
    pub top_k: usize,
    pub filter: Option<Filter>,
    pub score_threshold: Option<f32>,
    pub with_payload: bool,
    pub offset: usize,
    /// Optional name for multi-vector collections; `None` uses the primary vector.
    pub vector_name: Option<String>,
}

impl QueryRequest {
    pub fn new(vector: Vec<f32>, top_k: usize) -> Self {
        Self { vector, top_k, filter: None, score_threshold: None, with_payload: true, offset: 0, vector_name: None }
    }
    pub fn with_filter(mut self, f: Filter) -> Self { self.filter = Some(f); self }
    pub fn with_score_threshold(mut self, t: f32) -> Self { self.score_threshold = Some(t); self }
    pub fn with_offset(mut self, o: usize) -> Self { self.offset = o; self }
    pub fn with_vector_name(mut self, name: impl Into<String>) -> Self {
        self.vector_name = Some(name.into()); self
    }
}

/// A hybrid search request combining dense vector similarity and keyword matching.
#[derive(Debug, Clone)]
pub struct HybridQueryRequest {
    pub dense_vector: Vec<f32>,
    /// Keyword query string for BM25 / full-text scoring.
    pub keyword: String,
    pub top_k: usize,
    /// Blend factor: 0.0 = pure keyword, 1.0 = pure vector.
    pub alpha: f32,
    pub filter: Option<Filter>,
    pub score_threshold: Option<f32>,
}

impl HybridQueryRequest {
    pub fn new(dense_vector: Vec<f32>, keyword: impl Into<String>, top_k: usize) -> Self {
        Self { dense_vector, keyword: keyword.into(), top_k, alpha: 0.5, filter: None, score_threshold: None }
    }
    pub fn with_alpha(mut self, alpha: f32) -> Self { self.alpha = alpha; self }
    pub fn with_filter(mut self, f: Filter) -> Self { self.filter = Some(f); self }
}

/// Blend a dense vector score and a keyword (BM25) score using the given alpha.
///
/// `alpha = 1.0` returns pure vector score; `alpha = 0.0` returns pure keyword score.
/// Both inputs are assumed to be in [0.0, 1.0]; the result is clamped.
pub fn hybrid_score(vector_score: f32, keyword_score: f32, alpha: f32) -> f32 {
    let blended = alpha * vector_score + (1.0 - alpha) * keyword_score;
    blended.clamp(0.0, 1.0)
}

/// Simple BM25-inspired keyword score: 1.0 if all query words appear in text, else 0.0.
///
/// Production backends use proper BM25; this reference version is term-presence only.
pub fn keyword_score_naive(text: &str, query: &str) -> f32 {
    let text_lower = text.to_ascii_lowercase();
    let all_match = query.split_whitespace().all(|w| text_lower.contains(&w.to_ascii_lowercase()));
    if all_match { 1.0 } else { 0.0 }
}

// ---- pagination -----------------------------------------------------------

/// A paginated result page from a vector query.
#[derive(Debug, Clone)]
pub struct Page<T> {
    pub items: Vec<T>,
    /// Total number of items available (may be an estimate for large collections).
    pub total: Option<u64>,
    /// The offset passed to produce this page.
    pub offset: usize,
}

impl<T> Page<T> {
    pub fn new(items: Vec<T>, offset: usize) -> Self {
        Self { total: None, offset, items }
    }
    pub fn with_total(mut self, total: u64) -> Self {
        self.total = Some(total); self
    }
    pub fn has_next(&self) -> bool {
        match self.total {
            Some(t) => (self.offset + self.items.len()) < t as usize,
            None => !self.items.is_empty(),
        }
    }
}

// ---- batch config ---------------------------------------------------------

/// Configuration for batch upsert operations.
#[derive(Debug, Clone)]
pub struct BatchConfig {
    /// Maximum number of points per internal batch.
    pub batch_size: usize,
    /// Number of times to retry a failed batch before propagating the error.
    pub retries: u8,
}

impl Default for BatchConfig {
    fn default() -> Self { Self { batch_size: 100, retries: 3 } }
}

/// Split `points` into chunks of `batch_size` for batch upsert.
pub fn make_batches(points: Vec<Point>, batch_size: usize) -> Vec<Vec<Point>> {
    if batch_size == 0 {
        return vec![points];
    }
    points.chunks(batch_size).map(|c| c.to_vec()).collect()
}

// ---- score threshold ------------------------------------------------------

/// Apply a score threshold, keeping only results above the cutoff.
///
/// Used by the in-memory reference impl and any backend that does threshold
/// filtering in Rust rather than at the store level.
pub fn apply_score_threshold(results: Vec<ScoredPoint>, threshold: f32) -> Vec<ScoredPoint> {
    results.into_iter().filter(|p| p.score >= threshold).collect()
}

// ---- the trait ------------------------------------------------------------

// ---- hybrid score tests --------------------------------------------------

#[cfg(test)]
mod hybrid_tests {
    use super::*;

    #[test]
    fn hybrid_score_pure_vector() {
        let s = hybrid_score(0.8, 0.2, 1.0);
        assert!((s - 0.8).abs() < 1e-6);
    }

    #[test]
    fn hybrid_score_pure_keyword() {
        let s = hybrid_score(0.8, 0.2, 0.0);
        assert!((s - 0.2).abs() < 1e-6);
    }

    #[test]
    fn hybrid_score_blended() {
        let s = hybrid_score(1.0, 0.0, 0.5);
        assert!((s - 0.5).abs() < 1e-6);
    }

    #[test]
    fn keyword_score_naive_all_words_present() {
        assert!((keyword_score_naive("apple banana cherry", "apple cherry") - 1.0).abs() < 1e-6);
    }

    #[test]
    fn keyword_score_naive_missing_word_is_zero() {
        assert!((keyword_score_naive("apple banana", "cherry")).abs() < 1e-6);
    }

    #[test]
    fn apply_score_threshold_removes_low() {
        let pts = vec![
            ScoredPoint { id: PointId::Num(1), score: 0.9, payload: Payload::new() },
            ScoredPoint { id: PointId::Num(2), score: 0.3, payload: Payload::new() },
        ];
        let filtered = apply_score_threshold(pts, 0.5);
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].id, PointId::Num(1));
    }
}

// ---- pagination tests ----------------------------------------------------

#[cfg(test)]
mod pagination_tests {
    use super::*;

    #[test]
    fn page_has_next_when_items_lt_total() {
        let page: Page<i32> = Page::new(vec![1, 2], 0).with_total(5);
        assert!(page.has_next());
    }

    #[test]
    fn page_no_next_when_at_end() {
        let page: Page<i32> = Page::new(vec![4, 5], 3).with_total(5);
        assert!(!page.has_next());
    }

    #[test]
    fn page_has_next_without_total_when_nonempty() {
        let page: Page<i32> = Page::new(vec![1], 0);
        assert!(page.has_next());
    }

    #[test]
    fn page_no_next_without_total_when_empty() {
        let page: Page<i32> = Page::new(vec![], 0);
        assert!(!page.has_next());
    }
}

// ---- distance metric tests -----------------------------------------------

#[cfg(test)]
mod distance_tests {
    use super::*;

    #[test]
    fn cosine_identical_vectors_is_one() {
        let v = vec![1.0, 0.0, 0.0];
        assert!((cosine_similarity(&v, &v) - 1.0).abs() < 1e-6);
    }

    #[test]
    fn cosine_orthogonal_is_zero() {
        let a = vec![1.0, 0.0];
        let b = vec![0.0, 1.0];
        assert!((cosine_similarity(&a, &b)).abs() < 1e-6);
    }

    #[test]
    fn dot_product_basic() {
        let a = vec![2.0, 3.0];
        let b = vec![4.0, 5.0];
        assert!((dot_product(&a, &b) - 23.0).abs() < 1e-6);
    }

    #[test]
    fn l2_identical_is_one() {
        let v = vec![1.0, 2.0, 3.0];
        assert!((l2_similarity(&v, &v) - 1.0).abs() < 1e-6);
    }

    #[test]
    fn distance_score_method_dispatches() {
        let a = vec![1.0, 0.0];
        let b = vec![1.0, 0.0];
        assert!((Distance::Cosine.score(&a, &b) - 1.0).abs() < 1e-6);
        assert!((Distance::Dot.score(&a, &b) - 1.0).abs() < 1e-6);
        assert!((Distance::L2.score(&a, &b) - 1.0).abs() < 1e-6);
    }
}

// ---- batch helper tests --------------------------------------------------

#[cfg(test)]
mod batch_tests {
    use super::*;

    #[test]
    fn make_batches_splits_correctly() {
        let pts: Vec<Point> = (0u64..5).map(|i| Point::new(i, vec![i as f32])).collect();
        let batches = make_batches(pts, 2);
        assert_eq!(batches.len(), 3); // [0,1], [2,3], [4]
        assert_eq!(batches[0].len(), 2);
        assert_eq!(batches[2].len(), 1);
    }

    #[test]
    fn make_batches_zero_size_returns_all() {
        let pts: Vec<Point> = (0u64..5).map(|i| Point::new(i, vec![i as f32])).collect();
        let batches = make_batches(pts, 0);
        assert_eq!(batches.len(), 1);
        assert_eq!(batches[0].len(), 5);
    }

    #[test]
    fn batch_config_defaults() {
        let cfg = BatchConfig::default();
        assert_eq!(cfg.batch_size, 100);
        assert_eq!(cfg.retries, 3);
    }
}

// ---- unit tests for trait utilities --------------------------------------

#[cfg(test)]
mod filter_tests {
    use super::*;

    fn make_payload(key: &str, val: impl Into<PayloadValue>) -> Payload {
        let mut p = Payload::new();
        p.insert(key.to_owned(), val.into());
        p
    }

    #[test]
    fn filter_eq_matches_string() {
        let p = make_payload("tag", "hello");
        assert!(filter_matches(&p, &Filter::Eq("tag".to_owned(), PayloadValue::String("hello".to_owned()))));
    }

    #[test]
    fn filter_eq_rejects_wrong_value() {
        let p = make_payload("tag", "hello");
        assert!(!filter_matches(&p, &Filter::Eq("tag".to_owned(), PayloadValue::String("world".to_owned()))));
    }

    #[test]
    fn filter_gt_integer() {
        let p = make_payload("n", 5i64);
        assert!(filter_matches(&p, &Filter::Gt("n".to_owned(), PayloadValue::Integer(3))));
        assert!(!filter_matches(&p, &Filter::Gt("n".to_owned(), PayloadValue::Integer(5))));
    }

    #[test]
    fn filter_lt_integer() {
        let p = make_payload("n", 3i64);
        assert!(filter_matches(&p, &Filter::Lt("n".to_owned(), PayloadValue::Integer(5))));
        assert!(!filter_matches(&p, &Filter::Lt("n".to_owned(), PayloadValue::Integer(3))));
    }

    #[test]
    fn filter_and_requires_both() {
        let mut p = Payload::new();
        p.insert("a".to_owned(), PayloadValue::String("x".to_owned()));
        p.insert("b".to_owned(), PayloadValue::Integer(1));
        let f = Filter::Eq("a".to_owned(), PayloadValue::String("x".to_owned()))
            .and(Filter::Eq("b".to_owned(), PayloadValue::Integer(1)));
        assert!(filter_matches(&p, &f));
    }

    #[test]
    fn filter_or_requires_one() {
        let p = make_payload("a", "yes");
        let f = Filter::Eq("a".to_owned(), PayloadValue::String("yes".to_owned()))
            .or(Filter::Eq("b".to_owned(), PayloadValue::Integer(99)));
        assert!(filter_matches(&p, &f));
    }
}

/// The unified interface that every vector-store backend must implement.
pub trait VectorStore: Send + Sync {
    fn create_collection(&self, spec: CollectionSpec) -> Result<(), VectorStoreError>;
    fn drop_collection(&self, name: &str) -> Result<(), VectorStoreError>;
    fn describe_collection(&self, name: &str) -> Result<CollectionInfo, VectorStoreError>;

    fn upsert(&self, collection: &str, points: Vec<Point>) -> Result<(), VectorStoreError>;
    fn batch_upsert(&self, collection: &str, batches: Vec<Vec<Point>>) -> Result<(), VectorStoreError> {
        for batch in batches {
            self.upsert(collection, batch)?;
        }
        Ok(())
    }

    fn query(&self, collection: &str, req: QueryRequest) -> Result<Vec<ScoredPoint>, VectorStoreError>;
    fn hybrid_query(&self, collection: &str, req: HybridQueryRequest) -> Result<Vec<ScoredPoint>, VectorStoreError>;

    fn delete(&self, collection: &str, ids: Vec<PointId>) -> Result<(), VectorStoreError>;
    fn delete_by_filter(&self, collection: &str, filter: Filter) -> Result<u64, VectorStoreError>;
}
