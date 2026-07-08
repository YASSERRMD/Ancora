/// Model prefetch and caching subsystem.
///
/// Manages a prefetch queue and an in-memory cache metadata store.
/// Actual I/O (reading model files) is intentionally not performed here;
/// this module handles scheduling, cache state tracking, and eviction policy.
use std::collections::{HashMap, VecDeque};
use std::time::Instant;

/// State of a model in the prefetch/cache system.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CacheState {
    /// Not in cache.
    Absent,
    /// Queued for prefetch.
    Queued,
    /// Currently being loaded (prefetch in progress).
    Loading,
    /// Fully in cache and ready.
    Cached,
    /// Evicted from cache (was cached, freed to reclaim RAM).
    Evicted,
}

impl std::fmt::Display for CacheState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            CacheState::Absent => "absent",
            CacheState::Queued => "queued",
            CacheState::Loading => "loading",
            CacheState::Cached => "cached",
            CacheState::Evicted => "evicted",
        };
        write!(f, "{}", s)
    }
}

/// Cache entry metadata.
#[derive(Debug, Clone)]
pub struct CacheEntry {
    pub model_id: String,
    pub state: CacheState,
    pub ram_bytes: u64,
    /// Number of times this model has been accessed from cache.
    pub access_count: u64,
    /// Last access time.
    pub last_accessed: Option<Instant>,
    /// Priority (higher = less likely to evict).
    pub priority: u8,
}

impl CacheEntry {
    fn new(model_id: impl Into<String>, ram_bytes: u64) -> Self {
        CacheEntry {
            model_id: model_id.into(),
            state: CacheState::Absent,
            ram_bytes,
            access_count: 0,
            last_accessed: None,
            priority: 128,
        }
    }
}

/// Eviction policy for the cache.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EvictionPolicy {
    /// Least Recently Used.
    Lru,
    /// Least Frequently Used.
    Lfu,
    /// Lowest priority first (by priority field).
    LowestPriority,
}

/// Prefetch cache manager.
pub struct PrefetchCache {
    /// Model cache entries indexed by model ID.
    entries: HashMap<String, CacheEntry>,
    /// Prefetch queue (FIFO).
    queue: VecDeque<String>,
    /// Total RAM budget in bytes.
    ram_budget: u64,
    /// Eviction policy.
    eviction_policy: EvictionPolicy,
}

impl PrefetchCache {
    /// Create a new cache with the given RAM budget.
    pub fn new(ram_budget_bytes: u64, eviction_policy: EvictionPolicy) -> Self {
        PrefetchCache {
            entries: HashMap::new(),
            queue: VecDeque::new(),
            ram_budget: ram_budget_bytes,
            eviction_policy,
        }
    }

    /// Total RAM budget.
    pub fn ram_budget(&self) -> u64 {
        self.ram_budget
    }

    /// Used RAM by cached models.
    pub fn used_ram(&self) -> u64 {
        self.entries
            .values()
            .filter(|e| e.state == CacheState::Cached)
            .map(|e| e.ram_bytes)
            .sum()
    }

    /// Available RAM.
    pub fn available_ram(&self) -> u64 {
        self.ram_budget.saturating_sub(self.used_ram())
    }

    /// Enqueue a model for prefetch.
    ///
    /// No-op if the model is already cached or queued.
    pub fn enqueue(&mut self, model_id: &str, ram_bytes: u64) {
        let entry = self
            .entries
            .entry(model_id.to_string())
            .or_insert_with(|| CacheEntry::new(model_id, ram_bytes));

        match entry.state {
            CacheState::Cached | CacheState::Queued | CacheState::Loading => return,
            _ => {}
        }

        entry.state = CacheState::Queued;
        self.queue.push_back(model_id.to_string());
    }

    /// Process the next item in the prefetch queue.
    ///
    /// Transitions the model from Queued -> Loading -> Cached.
    /// Returns the model ID that was "loaded" or None if queue empty / no RAM.
    pub fn process_next(&mut self) -> Option<String> {
        while let Some(id) = self.queue.pop_front() {
            if let Some(entry) = self.entries.get_mut(&id) {
                if entry.state != CacheState::Queued {
                    continue;
                }
                let needed = entry.ram_bytes;
                // Evict if needed.
                while self.available_ram() < needed {
                    if !self.evict_one() {
                        return None; // Cannot free enough RAM.
                    }
                }
                if let Some(entry) = self.entries.get_mut(&id) {
                    entry.state = CacheState::Cached;
                    entry.last_accessed = Some(Instant::now());
                    return Some(id);
                }
            }
        }
        None
    }

    /// Mark a cached model as accessed.
    pub fn touch(&mut self, model_id: &str) {
        if let Some(entry) = self.entries.get_mut(model_id) {
            entry.access_count += 1;
            entry.last_accessed = Some(Instant::now());
        }
    }

    /// Evict one cached model according to the eviction policy.
    fn evict_one(&mut self) -> bool {
        let victim = match self.eviction_policy {
            EvictionPolicy::Lru => self
                .entries
                .iter()
                .filter(|(_, e)| e.state == CacheState::Cached)
                .max_by_key(|(_, e)| {
                    e.last_accessed
                        .map(|i| i.elapsed())
                        .unwrap_or(std::time::Duration::MAX)
                })
                .map(|(id, _)| id.clone()),
            EvictionPolicy::Lfu => self
                .entries
                .iter()
                .filter(|(_, e)| e.state == CacheState::Cached)
                .min_by_key(|(_, e)| e.access_count)
                .map(|(id, _)| id.clone()),
            EvictionPolicy::LowestPriority => self
                .entries
                .iter()
                .filter(|(_, e)| e.state == CacheState::Cached)
                .min_by_key(|(_, e)| e.priority)
                .map(|(id, _)| id.clone()),
        };

        if let Some(id) = victim {
            if let Some(entry) = self.entries.get_mut(&id) {
                entry.state = CacheState::Evicted;
                return true;
            }
        }
        false
    }

    /// Manually evict a specific model from the cache.
    pub fn evict(&mut self, model_id: &str) -> bool {
        if let Some(entry) = self.entries.get_mut(model_id) {
            if entry.state == CacheState::Cached {
                entry.state = CacheState::Evicted;
                return true;
            }
        }
        false
    }

    /// Get the cache state for a model.
    pub fn state(&self, model_id: &str) -> CacheState {
        self.entries
            .get(model_id)
            .map(|e| e.state.clone())
            .unwrap_or(CacheState::Absent)
    }

    /// Set priority for a model (higher = evict last).
    pub fn set_priority(&mut self, model_id: &str, priority: u8) {
        if let Some(entry) = self.entries.get_mut(model_id) {
            entry.priority = priority;
        }
    }

    /// List all cached model IDs.
    pub fn cached_ids(&self) -> Vec<&str> {
        self.entries
            .iter()
            .filter(|(_, e)| e.state == CacheState::Cached)
            .map(|(id, _)| id.as_str())
            .collect()
    }

    /// Number of models in cache.
    pub fn cached_count(&self) -> usize {
        self.entries
            .values()
            .filter(|e| e.state == CacheState::Cached)
            .count()
    }

    /// Queue depth.
    pub fn queue_depth(&self) -> usize {
        self.queue.len()
    }
}
