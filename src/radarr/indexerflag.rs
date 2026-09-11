use super::Radarr;
use crate::error::Result;
use crate::interface::ApiClientExt;
use crate::is_default;
use crate::req::Request;
use serde::{Deserialize, Serialize};

const BP_INDEXER_FLAG: &str = "v3/indexerflag";

/// IndexerFlag is the `/api/v3/indexerflag` resource.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndexerFlag {
    /// Flag ID.
    #[serde(default)]
    pub id: i32,
    /// Flag name.
    #[serde(default, skip_serializing_if = "is_default")]
    pub name: String,
    /// Lowercase flag name.
    #[serde(default, skip_serializing_if = "is_default")]
    pub name_lower: String,
}

impl Radarr {
    /// Returns all indexer flags.
    pub async fn get_indexer_flags(&self) -> Result<Vec<IndexerFlag>> {
        self.api.get_into(Request::new(BP_INDEXER_FLAG)).await
    }
}
