use super::Prowlarr;
use crate::error::Result;
use crate::helpers::str_val;
use crate::interface::ApiClientExt;
use crate::is_default;
use crate::req::Request;
use crate::shared::Protocol;
use crate::values::Values;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

const BP_SEARCH: &str = "v1/search";

/// Search is the output from the Prowlarr search endpoint.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Search {
    /// Unique release identifier used to grab the item.
    #[serde(default)]
    pub guid: String,
    /// Release age in days.
    #[serde(default)]
    pub age: i64,
    /// Release age in hours.
    #[serde(default)]
    pub age_hours: f64,
    /// Release age in minutes.
    #[serde(default)]
    pub age_minutes: f64,
    /// Release size in bytes.
    #[serde(default)]
    pub size: i64,
    /// Number of files in the release.
    #[serde(default)]
    pub files: i32,
    /// Number of times the release was grabbed.
    #[serde(default)]
    pub grabs: i32,
    /// ID of the indexer that returned this release.
    #[serde(default)]
    pub indexer_id: i64,
    /// Name of the indexer that returned this release.
    #[serde(default)]
    pub indexer: String,
    /// Release title.
    #[serde(default)]
    pub title: String,
    /// Title used for sorting.
    #[serde(default)]
    pub sort_title: String,
    /// IMDb ID, when the indexer provides one.
    #[serde(default)]
    pub imdb_id: i64,
    /// TMDb ID, when the indexer provides one.
    #[serde(default)]
    pub tmdb_id: i64,
    /// TVDb ID, when the indexer provides one.
    #[serde(default)]
    pub tvdb_id: i64,
    /// TVMaze ID, when the indexer provides one.
    #[serde(default)]
    pub tv_maze_id: i64,
    /// When the release was published.
    #[serde(default)]
    pub publish_date: Option<DateTime<Utc>>,
    /// Link to the release comments.
    #[serde(default)]
    pub comment_url: String,
    /// Link used to download the release.
    #[serde(default)]
    pub download_url: String,
    /// Link to more information about the release.
    #[serde(default)]
    pub info_url: String,
    /// Indexer flags set on the release.
    #[serde(default, skip_serializing_if = "is_default")]
    pub indexer_flags: Vec<String>,
    /// Categories the release belongs to.
    #[serde(default)]
    pub categories: Vec<Category>,
    /// Protocol used to download the release.
    #[serde(default)]
    pub protocol: Protocol,
    /// Release file name.
    #[serde(default)]
    pub file_name: String,
    /// Torrent info hash.
    #[serde(default)]
    pub info_hash: String,
    /// Number of seeders.
    #[serde(default)]
    pub seeders: i32,
    /// Number of leechers.
    #[serde(default)]
    pub leechers: i32,
}

/// Category is part of the [`Search`] output.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Category {
    /// Category ID.
    #[serde(default)]
    pub id: i64,
    /// Category name.
    #[serde(default)]
    pub name: String,
    /// Nested sub categories.
    #[serde(default)]
    pub sub_categories: Vec<Category>,
}

/// SearchInput is the input to the search endpoint.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchInput {
    /// Query is required. Fill it in.
    #[serde(default)]
    pub query: String,
    /// Defaults to `search` if left empty.
    #[serde(default, rename = "type")]
    pub search_type: String,
    /// Restrict the search to these indexers.
    #[serde(default)]
    pub indexer_ids: Vec<i64>,
    /// Restrict the search to these categories.
    #[serde(default)]
    pub categories: Vec<i64>,
    /// Defaults to 100 if left empty or less than 1.
    #[serde(default)]
    pub limit: i32,
    /// Skip this many records.
    #[serde(default)]
    pub offset: i32,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct GrabRequest<'a> {
    guid: &'a str,
    indexer_id: i64,
}

impl Prowlarr {
    /// Searches the Prowlarr indexers for media and content.
    ///
    /// Must provide a query in the [`SearchInput`].
    pub async fn search(&self, search: &SearchInput) -> Result<Vec<Search>> {
        const DEFAULT_SEARCH_LIMIT: i32 = 100;

        let search_type = if search.search_type.is_empty() {
            "search"
        } else {
            &search.search_type
        };

        let limit = if search.limit < 1 {
            DEFAULT_SEARCH_LIMIT
        } else {
            search.limit
        };

        let mut query = Values::new();
        query.set("query", search.query.clone());
        query.set("type", search_type);
        query.set("limit", str_val(limit));
        query.set("offset", str_val(search.offset));

        for val in &search.categories {
            query.add("categories", str_val(*val));
        }

        for val in &search.indexer_ids {
            query.add("indexerIds", str_val(*val));
        }

        self.api
            .get_into(Request::new(BP_SEARCH).with_query(query))
            .await
    }

    /// Attempts to download a searched item by GUID.
    pub async fn grab(&self, guid: &str, indexer_id: i64) -> Result<Search> {
        self.grab_search(&Search {
            guid: guid.to_string(),
            indexer_id,
            ..Default::default()
        })
        .await
    }

    /// Attempts to download an item returned from a search.
    ///
    /// Pass the item from the [`search`](Prowlarr::search) output.
    pub async fn grab_search(&self, search: &Search) -> Result<Search> {
        let grab = GrabRequest {
            guid: &search.guid,
            indexer_id: search.indexer_id,
        };

        self.api
            .post_into(Request::new(BP_SEARCH).with_json(&grab)?)
            .await
    }
}
