use super::{CustomFormatOutput, Movie, Radarr};
use crate::error::Result;
use crate::helpers::str_val;
use crate::interface::ApiClientExt;
use crate::paginate::{PageReq, adjust_per_page, set_per_page};
use crate::req::{Request, path_join};
use crate::shared::{Protocol, Quality, Value};
use crate::values::Values;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

const BP_BLOCKLIST: &str = "v3/blocklist";

/// BlockList represents the `/api/v3/blocklist` endpoint.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockList {
    /// Page this response represents.
    #[serde(default)]
    pub page: i32,
    /// Number of records per page.
    #[serde(default)]
    pub page_size: i32,
    /// Key the records are sorted by.
    #[serde(default)]
    pub sort_key: String,
    /// Direction the records are sorted in.
    #[serde(default)]
    pub sort_direction: String,
    /// Total records available.
    #[serde(default)]
    pub total_records: i32,
    /// The block list items.
    #[serde(default)]
    pub records: Vec<BlockListRecord>,
}

/// BlockListRecord represents a single block list item.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockListRecord {
    /// Movie the release belongs to.
    #[serde(default)]
    pub movie: Option<Movie>,
    /// Quality of the blocked release.
    #[serde(default)]
    pub quality: Option<Quality>,
    /// Languages found in the release.
    #[serde(default)]
    pub languages: Vec<Value>,
    /// Custom formats matched by the release.
    #[serde(default)]
    pub custom_formats: Vec<CustomFormatOutput>,
    /// Movie ID.
    #[serde(default)]
    pub movie_id: i64,
    /// Block list item ID.
    #[serde(default)]
    pub id: i64,
    /// When the item was blocked.
    #[serde(default)]
    pub date: Option<DateTime<Utc>>,
    /// Title of the blocked release.
    #[serde(default)]
    pub source_title: String,
    /// Protocol the release was grabbed with.
    #[serde(default)]
    pub protocol: Protocol,
    /// Indexer the release came from.
    #[serde(default)]
    pub indexer: String,
    /// Why the release was blocked.
    #[serde(default)]
    pub message: String,
}

impl Radarr {
    /// Returns the count of block list items requested.
    ///
    /// If you need control over the page, use [`Radarr::get_block_list_page`].
    pub async fn get_block_list(&self, records: i32) -> Result<BlockList> {
        let mut list = BlockList::default();
        let mut per_page = set_per_page(records, 0);

        let mut page = 1;
        loop {
            let curr = self
                .get_block_list_page(Some(&PageReq::new().page_size(per_page).page(page)))
                .await?;

            let curr_len = curr.records.len();
            list.records.extend(curr.records);

            let collected = list.records.len() as i32;
            if collected >= curr.total_records
                || (collected >= records && records != 0)
                || curr_len == 0
            {
                list.page_size = curr.total_records;
                list.total_records = curr.total_records;
                list.sort_direction = curr.sort_direction;
                list.sort_key = curr.sort_key;

                break;
            }

            per_page = adjust_per_page(records, curr.total_records, collected, per_page);
            page += 1;
        }

        Ok(list)
    }

    /// Returns block list items based on filters.
    pub async fn get_block_list_page(&self, params: Option<&PageReq>) -> Result<BlockList> {
        let mut params = params.cloned().unwrap_or_default();
        params.check_set("sortKey", "date");

        self.api
            .get_into(Request::new(BP_BLOCKLIST).with_query(params.params()))
            .await
    }

    /// Removes a single block list item.
    pub async fn delete_block_list(&self, list_id: i64) -> Result<()> {
        self.api
            .delete_any(Request::new(path_join(&[BP_BLOCKLIST, &str_val(list_id)])))
            .await
    }

    /// Removes multiple block list items.
    pub async fn delete_block_lists(&self, ids: &[i64]) -> Result<()> {
        #[derive(Serialize)]
        struct Input<'a> {
            ids: &'a [i64],
        }

        let req = Request::new(path_join(&[BP_BLOCKLIST, "bulk"])).with_json(&Input { ids })?;
        self.api.delete_any(req).await
    }

    /// Returns blocklist entries for a movie.
    pub async fn get_blocklist_by_movie_id(&self, movie_id: i64) -> Result<Vec<BlockListRecord>> {
        let mut params = Values::new();
        params.set("movieId", str_val(movie_id));

        self.api
            .get_into(Request::new(path_join(&[BP_BLOCKLIST, "movie"])).with_query(params))
            .await
    }
}
