use super::Lidarr;
use crate::error::Result;
use crate::helpers::str_val;
use crate::interface::ApiClientExt;
use crate::req::{Request, path_join};
use serde::{Deserialize, Serialize};

const BP_INDEXER_CONFIG: &str = "v1/config/indexer";

/// IndexerConfig represents the `/config/indexer` endpoint.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndexerConfig {
    /// Config ID.
    #[serde(default)]
    pub id: i64,
    /// Maximum release size in megabytes.
    #[serde(default)]
    pub maximum_size: i64,
    /// Minimum release age in minutes.
    #[serde(default)]
    pub minimum_age: i64,
    /// How long to keep releases, in days.
    #[serde(default)]
    pub retention: i64,
    /// How often RSS feeds are synced, in minutes.
    #[serde(default)]
    pub rss_sync_interval: i64,
}

impl Lidarr {
    /// Returns the indexer config.
    pub async fn get_indexer_config(&self) -> Result<IndexerConfig> {
        self.api.get_into(Request::new(BP_INDEXER_CONFIG)).await
    }

    /// Updates the single indexer config.
    pub async fn update_indexer_config(&self, config: &IndexerConfig) -> Result<IndexerConfig> {
        let req =
            Request::new(path_join(&[BP_INDEXER_CONFIG, &str_val(config.id)])).with_json(config)?;
        self.api.put_into(req).await
    }
}
