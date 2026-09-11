use super::Radarr;
use crate::error::Result;
use crate::helpers::{force_save, str_val};
use crate::interface::ApiClientExt;
use crate::is_default;
use crate::req::{Request, path_join};
use crate::shared::{BulkIndexer, FieldInput, FieldOutput, Protocol};
use serde::{Deserialize, Serialize};

const BP_INDEXER: &str = "v3/indexer";

/// IndexerInput is the input for a new or updated indexer.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndexerInput {
    /// Whether automatic searches use this indexer.
    #[serde(default)]
    pub enable_automatic_search: bool,
    /// Whether interactive searches use this indexer.
    #[serde(default)]
    pub enable_interactive_search: bool,
    /// Whether RSS sync uses this indexer.
    #[serde(default)]
    pub enable_rss: bool,
    /// Download client used for releases from this indexer.
    #[serde(default)]
    pub download_client_id: i64,
    /// Indexer priority.
    #[serde(default)]
    pub priority: i64,
    /// Indexer ID. Leave zero when creating.
    #[serde(default, skip_serializing_if = "is_default")]
    pub id: i64,
    /// Configuration contract name.
    #[serde(default)]
    pub config_contract: String,
    /// Implementation type of the indexer.
    #[serde(default)]
    pub implementation: String,
    /// Indexer name.
    #[serde(default)]
    pub name: String,
    /// Protocol the indexer uses.
    #[serde(default)]
    pub protocol: Protocol,
    /// Tags applied to this indexer.
    #[serde(default)]
    pub tags: Vec<i32>,
    /// Implementation-specific settings.
    #[serde(default)]
    pub fields: Vec<FieldInput>,
}

/// IndexerOutput is the output from the indexer methods.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndexerOutput {
    /// Whether automatic searches use this indexer.
    #[serde(default)]
    pub enable_automatic_search: bool,
    /// Whether interactive searches use this indexer.
    #[serde(default)]
    pub enable_interactive_search: bool,
    /// Whether RSS sync uses this indexer.
    #[serde(default)]
    pub enable_rss: bool,
    /// Whether the indexer supports RSS.
    #[serde(default)]
    pub supports_rss: bool,
    /// Whether the indexer supports search.
    #[serde(default)]
    pub supports_search: bool,
    /// Download client used for releases from this indexer.
    #[serde(default)]
    pub download_client_id: i64,
    /// Indexer priority.
    #[serde(default)]
    pub priority: i64,
    /// Indexer ID.
    #[serde(default, skip_serializing_if = "is_default")]
    pub id: i64,
    /// Configuration contract name.
    #[serde(default)]
    pub config_contract: String,
    /// Implementation type of the indexer.
    #[serde(default)]
    pub implementation: String,
    /// Display name of the implementation.
    #[serde(default)]
    pub implementation_name: String,
    /// Link to more information about the indexer.
    #[serde(default)]
    pub info_link: String,
    /// Indexer name.
    #[serde(default)]
    pub name: String,
    /// Protocol the indexer uses.
    #[serde(default)]
    pub protocol: Protocol,
    /// Tags applied to this indexer.
    #[serde(default)]
    pub tags: Vec<i32>,
    /// Implementation-specific settings.
    #[serde(default)]
    pub fields: Vec<FieldOutput>,
}

impl Radarr {
    /// Returns all configured indexers.
    pub async fn get_indexers(&self) -> Result<Vec<IndexerOutput>> {
        self.api.get_into(Request::new(BP_INDEXER)).await
    }

    /// Returns a single indexer.
    pub async fn get_indexer(&self, indexer_id: i64) -> Result<IndexerOutput> {
        self.api
            .get_into(Request::new(path_join(&[BP_INDEXER, &str_val(indexer_id)])))
            .await
    }

    /// Tests an indexer.
    pub async fn test_indexer(&self, indexer: &IndexerInput) -> Result<()> {
        let req = Request::new(path_join(&[BP_INDEXER, "test"])).with_json(indexer)?;
        self.api.post_any(req).await
    }

    /// Creates an indexer without testing it.
    pub async fn add_indexer(&self, indexer: &IndexerInput) -> Result<IndexerOutput> {
        let mut indexer = indexer.clone();
        indexer.id = 0;

        let req = Request::new(BP_INDEXER)
            .with_json(&indexer)?
            .with_query(force_save(true));
        self.api.post_into(req).await
    }

    /// Updates an indexer.
    pub async fn update_indexer(
        &self,
        indexer: &IndexerInput,
        force: bool,
    ) -> Result<IndexerOutput> {
        let req = Request::new(path_join(&[BP_INDEXER, &str_val(indexer.id)]))
            .with_json(indexer)?
            .with_query(force_save(force));
        self.api.put_into(req).await
    }

    /// Removes a single indexer.
    pub async fn delete_indexer(&self, indexer_id: i64) -> Result<()> {
        self.api
            .delete_any(Request::new(path_join(&[
                BP_INDEXER,
                &str_val(indexer_id),
            ])))
            .await
    }

    /// Bulk updates indexers.
    pub async fn update_indexers(&self, indexer: &BulkIndexer) -> Result<Vec<IndexerOutput>> {
        let req = Request::new(path_join(&[BP_INDEXER, "bulk"])).with_json(indexer)?;
        self.api.put_into(req).await
    }
}
