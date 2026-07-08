use crate::indicator::Indicator;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FeedFormat {
    Stix,
    Taxii,
    Csv,
    Json,
    Internal,
}

#[derive(Debug, Clone)]
pub struct ThreatFeed {
    pub id: String,
    pub tenant_id: String,
    pub name: String,
    pub format: FeedFormat,
    pub source_url: String,
    pub last_updated_tick: u64,
    pub enabled: bool,
    pub metadata: HashMap<String, String>,
}

impl ThreatFeed {
    pub fn new(
        id: impl Into<String>,
        tenant_id: impl Into<String>,
        name: impl Into<String>,
        format: FeedFormat,
        source_url: impl Into<String>,
        tick: u64,
    ) -> Self {
        Self {
            id: id.into(),
            tenant_id: tenant_id.into(),
            name: name.into(),
            format,
            source_url: source_url.into(),
            last_updated_tick: tick,
            enabled: true,
            metadata: HashMap::new(),
        }
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }
    pub fn update_tick(&mut self, tick: u64) {
        self.last_updated_tick = tick;
    }
}

pub struct FeedStore {
    feeds: HashMap<String, ThreatFeed>,
    indicators: HashMap<String, Vec<String>>,
}

impl FeedStore {
    pub fn new() -> Self {
        Self {
            feeds: HashMap::new(),
            indicators: HashMap::new(),
        }
    }

    pub fn register_feed(&mut self, feed: ThreatFeed) {
        self.feeds.insert(feed.id.clone(), feed);
    }

    pub fn get_feed(&self, id: &str) -> Option<&ThreatFeed> {
        self.feeds.get(id)
    }

    pub fn add_indicator_to_feed(
        &mut self,
        feed_id: impl Into<String>,
        indicator_id: impl Into<String>,
    ) {
        self.indicators
            .entry(feed_id.into())
            .or_default()
            .push(indicator_id.into());
    }

    pub fn indicators_for_feed(&self, feed_id: &str) -> &[String] {
        self.indicators
            .get(feed_id)
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }

    pub fn feed_count(&self) -> usize {
        self.feeds.len()
    }

    pub fn enabled_feeds(&self) -> Vec<&ThreatFeed> {
        self.feeds.values().filter(|f| f.enabled).collect()
    }

    pub fn get_feed_mut(&mut self, id: &str) -> Option<&mut ThreatFeed> {
        self.feeds.get_mut(id)
    }

    pub fn for_tenant(&self, tenant_id: &str) -> Vec<&ThreatFeed> {
        self.feeds
            .values()
            .filter(|f| f.tenant_id == tenant_id)
            .collect()
    }
}
