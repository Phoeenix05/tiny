//!

use std::sync::Arc;

// NOTE: only change these two types when changing the cache key or data types.
// doing so will update them everywhere.
type CacheKey = String;
type CacheInnerData = String;

type CacheData = Arc<CacheInnerData>;
type MiniMokaCache = mini_moka::unsync::Cache<CacheKey, CacheData>;
type MiniMokaCacheBuilder = mini_moka::unsync::CacheBuilder<CacheKey, CacheData, MiniMokaCache>;

/// TTL, aka. Time To Live
#[derive(Default)]
pub enum TTL {
    /// TTL set to 15 minutes.
    Short,
    /// TTL set to 30 minutes.
    Medium,
    /// TTL set to 1 hour.
    #[default]
    Long,
    /// set TTL to any length in seconds.
    Custom(u64),
}

impl Into<u64> for TTL {
    fn into(self) -> u64 {
        match self {
            TTL::Short => 900,
            TTL::Medium => 1800,
            TTL::Long => 3600,
            TTL::Custom(v) => v,
        }
    }
}

/// TTL, aka. Time To Idle
#[derive(Default)]
pub enum TTI {
    /// TTI set to 5 minutes.
    #[default]
    Short,
    /// TTI set to 15 minutes.
    Medium,
    /// TTI set to 30 minutes.
    Long,
    /// set TTI to any length in seconds.
    Custom(u64),
}

impl Into<u64> for TTI {
    fn into(self) -> u64 {
        match self {
            TTI::Short => 300,
            TTI::Medium => 900,
            TTI::Long => 1800,
            TTI::Custom(v) => v,
        }
    }
}

pub struct Cache {
    cache: MiniMokaCache,
}

impl Cache {
    pub fn builder() -> CacheBuilder {
        CacheBuilder::default()
    }

    pub fn insert(&mut self, key: CacheKey, data: CacheInnerData) {
        self.cache.insert(key, Arc::new(data));
    }

    pub fn get(&mut self, key: &CacheKey) -> Result<CacheInnerData, ()> {
        if let Some(cached_data) = self.cache.get(key) {
            let inner_data = Arc::into_inner(cached_data.clone()).unwrap();
            Ok(inner_data)
        } else {
            Err(())
        }
    }

    pub fn contains(&mut self, key: &CacheKey) -> bool {
        self.cache.contains_key(key)
    }
}

/// builder for our response cache "handler"
#[derive(Default)]
pub struct CacheBuilder {
    cache_builder: MiniMokaCacheBuilder,
    capacity: Option<u64>,
    ttl: TTL,
    tti: TTI,
}

impl CacheBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_capacity(mut self, cap: u64) -> Self {
        self.capacity = Some(cap);
        self
    }

    pub fn with_ttl(mut self, ttl: TTL) -> Self {
        self.ttl = ttl;
        self
    }

    pub fn with_tti(mut self, tti: TTI) -> Self {
        self.tti = tti;
        self
    }

    pub fn build(self) -> Cache {
        let cache = self
            .cache_builder
            .max_capacity(self.capacity.unwrap_or(512))
            .time_to_live(std::time::Duration::from_secs(self.ttl.into()))
            .time_to_idle(std::time::Duration::from_secs(self.tti.into()))
            .build();

        Cache { cache }
    }
}
