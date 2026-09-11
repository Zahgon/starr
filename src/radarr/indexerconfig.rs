use super::Radarr;
use crate::error::Result;
use crate::helpers::str_val;
use crate::interface::ApiClientExt;
use crate::req::{Request, path_join};
use serde::{Deserialize, Serialize};

const BP_INDEXER_CONFIG: &str = "v3/config/indexer";

/// IndexerConfig represents the `/config/indexer` endpoint.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndexerConfig {
    /// Hardcoded subtitle tags that are allowed anyway.
    #[serde(default)]
    pub whitelisted_hardcoded_subs: String,
    /// Config ID.
    #[serde(default)]
    pub id: i64,
    /// Largest release size accepted, in bytes.
    #[serde(default)]
    pub maximum_size: i64,
    /// Minimum age of a usenet release, in minutes.
    #[serde(default)]
    pub minimum_age: i64,
    /// Usenet retention, in days.
    #[serde(default)]
    pub retention: i64,
    /// Minutes between RSS syncs.
    #[serde(default)]
    pub rss_sync_interval: i64,
    /// Days to wait after availability before searching.
    #[serde(default)]
    pub availability_delay: i32,
    /// Whether indexer flags are preferred over other releases.
    #[serde(default)]
    pub prefer_indexer_flags: bool,
    /// Whether releases with hardcoded subtitles are allowed.
    #[serde(default)]
    pub allow_hardcoded_subs: bool,
}

impl Radarr {
    /// Returns the indexer config.
    pub async fn get_indexer_config(&self) -> Result<IndexerConfig> {
        self.api.get_into(Request::new(BP_INDEXER_CONFIG)).await
    }

    /// Updates the single indexer config.
    pub async fn update_indexer_config(
        &self,
        indexer_config: &IndexerConfig,
    ) -> Result<IndexerConfig> {
        let req = Request::new(path_join(&[BP_INDEXER_CONFIG, &str_val(indexer_config.id)]))
            .with_json(indexer_config)?;
        self.api.put_into(req).await
    }
}
