use super::Prowlarr;
use crate::error::Result;
use crate::helpers::{force_save, str_val};
use crate::interface::ApiClientExt;
use crate::is_default;
use crate::req::{Request, path_join};
use crate::shared::{ApplyTags, FieldInput, FieldOutput, Protocol};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

const BP_INDEXER: &str = "v1/indexer";

/// IndexerInput is the input for a new or updated indexer.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndexerInput {
    /// Whether the indexer is enabled.
    #[serde(default)]
    pub enable: bool,
    /// Whether grabs are redirected instead of proxied.
    #[serde(default)]
    pub redirect: bool,
    /// Indexer priority.
    #[serde(default)]
    pub priority: i64,
    /// Indexer ID. Leave zero when creating.
    #[serde(default, skip_serializing_if = "is_default")]
    pub id: i64,
    /// App profile applied to this indexer.
    #[serde(default)]
    pub app_profile_id: i64,
    /// Configuration contract name.
    #[serde(default)]
    pub config_contract: String,
    /// Implementation type of the indexer.
    #[serde(default)]
    pub implementation: String,
    /// Indexer name.
    #[serde(default)]
    pub name: String,
    /// Protocol the indexer serves.
    #[serde(default)]
    pub protocol: Protocol,
    /// Tags applied to this indexer.
    #[serde(default, skip_serializing_if = "is_default")]
    pub tags: Vec<i32>,
    /// Implementation-specific settings.
    #[serde(default)]
    pub fields: Vec<FieldInput>,
}

/// IndexerOutput is the output from the indexer methods.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndexerOutput {
    /// Whether the indexer is enabled.
    #[serde(default)]
    pub enable: bool,
    /// Whether grabs are redirected instead of proxied.
    #[serde(default)]
    pub redirect: bool,
    /// Whether the indexer supports RSS.
    #[serde(default)]
    pub supports_rss: bool,
    /// Whether the indexer supports search.
    #[serde(default)]
    pub supports_search: bool,
    /// Whether the indexer supports redirect.
    #[serde(default)]
    pub supports_redirect: bool,
    /// App profile applied to this indexer.
    #[serde(default)]
    pub app_profile_id: i64,
    /// Indexer ID.
    #[serde(default, skip_serializing_if = "is_default")]
    pub id: i64,
    /// Indexer priority.
    #[serde(default)]
    pub priority: i64,
    /// Name used for sorting.
    #[serde(default)]
    pub sort_name: String,
    /// Indexer name.
    #[serde(default)]
    pub name: String,
    /// Protocol the indexer serves.
    #[serde(default)]
    pub protocol: Protocol,
    /// Privacy level of the indexer.
    #[serde(default)]
    pub privacy: String,
    /// Name of the indexer definition.
    #[serde(default)]
    pub definition_name: String,
    /// Indexer description.
    #[serde(default)]
    pub description: String,
    /// Indexer language.
    #[serde(default)]
    pub language: String,
    /// Character encoding used by the indexer.
    #[serde(default, skip_serializing_if = "is_default")]
    pub encoding: String,
    /// Display name of the implementation.
    #[serde(default)]
    pub implementation_name: String,
    /// Implementation type of the indexer.
    #[serde(default)]
    pub implementation: String,
    /// Configuration contract name.
    #[serde(default)]
    pub config_contract: String,
    /// Link to more information about the indexer.
    #[serde(default)]
    pub info_link: String,
    /// When the indexer was added.
    #[serde(default)]
    pub added: Option<DateTime<Utc>>,
    /// Search capabilities reported by the indexer.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<Capabilities>,
    /// Tags applied to this indexer.
    #[serde(default)]
    pub tags: Vec<i32>,
    /// Current indexer URLs.
    #[serde(default)]
    pub indexer_urls: Vec<String>,
    /// Previously used indexer URLs.
    #[serde(default)]
    pub legacy_urls: Vec<String>,
    /// Implementation-specific settings.
    #[serde(default)]
    pub fields: Vec<FieldOutput>,
}

/// Capabilities is part of [`IndexerOutput`].
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Capabilities {
    /// Whether the indexer supports raw search queries.
    #[serde(default)]
    pub supports_raw_search: bool,
    /// Maximum number of results per query.
    #[serde(default)]
    pub limits_max: i64,
    /// Default number of results per query.
    #[serde(default)]
    pub limits_default: i64,
    /// Supported generic search parameters.
    #[serde(default)]
    pub search_params: Vec<String>,
    /// Supported TV search parameters.
    #[serde(default)]
    pub tv_search_params: Vec<String>,
    /// Supported movie search parameters.
    #[serde(default)]
    pub movie_search_params: Vec<String>,
    /// Supported music search parameters.
    #[serde(default)]
    pub music_search_params: Vec<String>,
    /// Supported book search parameters.
    #[serde(default)]
    pub book_search_params: Vec<String>,
    /// Categories the indexer serves.
    #[serde(default)]
    pub categories: Vec<Categories>,
}

/// Categories is part of [`Capabilities`].
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Categories {
    /// Category ID.
    #[serde(default)]
    pub id: i64,
    /// Category name.
    #[serde(default)]
    pub name: String,
    /// Nested sub categories.
    #[serde(default)]
    pub sub_categories: Vec<Categories>,
}

/// IndexerDefaultCategory is a category from
/// [`get_indexer_categories`](Prowlarr::get_indexer_categories).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndexerDefaultCategory {
    /// Category ID.
    #[serde(default)]
    pub id: i64,
    /// Category name.
    #[serde(default, skip_serializing_if = "is_default")]
    pub name: String,
    /// Category description.
    #[serde(default, skip_serializing_if = "is_default")]
    pub description: String,
    /// Nested sub categories.
    #[serde(default, skip_serializing_if = "is_default")]
    pub sub_categories: Vec<IndexerDefaultCategory>,
}

/// BulkIndexer is the input to [`update_indexers`](Prowlarr::update_indexers).
///
/// Use [`ptr`](crate::helpers::ptr) to build the optional values.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BulkIndexer {
    /// Indexer IDs to update.
    #[serde(default)]
    pub ids: Vec<i64>,
    /// Tags to apply.
    #[serde(default, skip_serializing_if = "is_default")]
    pub tags: Vec<i32>,
    /// How the tags are applied.
    #[serde(default, skip_serializing_if = "is_default")]
    pub apply_tags: ApplyTags,
    /// Whether the indexers are enabled.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enable: Option<bool>,
    /// App profile to apply.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub app_profile_id: Option<i64>,
    /// Priority to apply.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority: Option<i64>,
    /// Minimum seeders to apply.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minimum_seeders: Option<i32>,
    /// Seed ratio to apply.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub seed_ratio: Option<i32>,
    /// Seed time to apply.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub seed_time: Option<i32>,
    /// Pack seed time to apply.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pack_seed_time: Option<i32>,
}

impl Prowlarr {
    /// Returns all configured indexers.
    pub async fn get_indexers(&self) -> Result<Vec<IndexerOutput>> {
        self.api.get_into(Request::new(BP_INDEXER)).await
    }

    /// Tests an indexer.
    pub async fn test_indexer(&self, indexer: &IndexerInput) -> Result<()> {
        let req = Request::new(path_join(&[BP_INDEXER, "test"])).with_json(indexer)?;
        self.api.post_any(req).await
    }

    /// Returns a single indexer.
    pub async fn get_indexer(&self, indexer_id: i64) -> Result<IndexerOutput> {
        self.api
            .get_into(Request::new(path_join(&[BP_INDEXER, &str_val(indexer_id)])))
            .await
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

    /// Updates the indexer.
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
            .delete_any(Request::new(path_join(&[BP_INDEXER, &str_val(indexer_id)])))
            .await
    }

    /// Bulk updates indexers.
    pub async fn update_indexers(&self, indexer: &BulkIndexer) -> Result<IndexerOutput> {
        let req = Request::new(path_join(&[BP_INDEXER, "bulk"])).with_json(indexer)?;
        self.api.put_into(req).await
    }

    /// Bulk deletes indexers.
    pub async fn delete_indexers(&self, bulk: &BulkIndexer) -> Result<()> {
        let req = Request::new(path_join(&[BP_INDEXER, "bulk"])).with_json(bulk)?;
        self.api.delete_any(req).await
    }

    /// Returns the default indexer category tree.
    pub async fn get_indexer_categories(&self) -> Result<Vec<IndexerDefaultCategory>> {
        self.api
            .get_into(Request::new(path_join(&[BP_INDEXER, "categories"])))
            .await
    }
}
