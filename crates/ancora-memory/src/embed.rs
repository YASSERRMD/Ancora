/// A dense floating-point embedding vector.
pub type Embedding = Vec<f32>;

/// Converts text into an embedding vector.
pub trait EmbeddingProvider: Send + Sync {
    fn embed(&self, text: &str) -> Embedding;
}

/// Test provider that maps each unique text to a one-hot-like vector based on its hash.
#[cfg(test)]
pub struct HashEmbeddingProvider {
    dims: usize,
}

#[cfg(test)]
impl HashEmbeddingProvider {
    pub fn new(dims: usize) -> Self {
        Self { dims }
    }
}

#[cfg(test)]
impl EmbeddingProvider for HashEmbeddingProvider {
    fn embed(&self, text: &str) -> Embedding {
        let mut v = vec![0.0f32; self.dims];
        let idx = text.bytes().fold(0usize, |acc, b| (acc * 31 + b as usize) % self.dims);
        v[idx] = 1.0;
        v
    }
}
