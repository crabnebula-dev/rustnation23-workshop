use std::time::{Duration, Instant};
use cached::CanExpire;
use serde::Serialize;
use rss::Channel;

#[derive(Debug, Serialize, Clone)]
pub struct ChannelWithExpiry {
    #[serde(skip)]
    pub insert_time: std::time::Instant,
    #[serde(flatten)]
    pub inner: Channel,
}

impl From<Channel> for ChannelWithExpiry {
    fn from(inner: Channel) -> Self {
        Self {
            insert_time: Instant::now(),
            inner,
        }
    }
}

impl CanExpire for ChannelWithExpiry {
    fn is_expired(&self) -> bool {
        let ttl_mins = self
            .inner
            .ttl()
            .map(|ttl| u64::from_str_radix(ttl, 10).unwrap())
            .unwrap_or(60);

        let ttl = Duration::from_secs(ttl_mins * 60);

        self.insert_time.elapsed() >= ttl
    }
}