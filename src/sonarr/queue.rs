use super::Sonarr;
use crate::error::{Error, Result};
use crate::helpers::str_val;
use crate::interface::ApiClientExt;
use crate::is_default;
use crate::paginate::{PageReq, adjust_per_page, set_per_page};
use crate::req::{Request, path_join, set_api_path};
use crate::shared::{
    Protocol, Quality, QueueDeleteOpts, StatusMessage, Value, queue_delete_values,
};
use crate::values::Values;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

const BP_QUEUE: &str = "v3/queue";

/// Queue is the `/api/v3/queue` endpoint.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Queue {
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
    /// The queue items.
    #[serde(default)]
    pub records: Vec<QueueRecord>,
}

/// QueueRecord is part of [`Queue`].
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueueRecord {
    /// Whether the download client has a post-import category.
    #[serde(default, rename = "downloadClientHasPostImportCategory")]
    pub has_post_import_category: bool,
    /// Queue item ID.
    #[serde(default)]
    pub id: i64,
    /// Series the item belongs to.
    #[serde(default)]
    pub series_id: i64,
    /// Episode the item belongs to.
    #[serde(default)]
    pub episode_id: i64,
    /// Language of the release.
    #[serde(default)]
    pub language: Option<Value>,
    /// Quality of the release.
    #[serde(default)]
    pub quality: Option<Quality>,
    /// Total size of the release in bytes.
    #[serde(default)]
    pub size: f64,
    /// Release title.
    #[serde(default)]
    pub title: String,
    /// Bytes left to download.
    #[serde(default)]
    pub sizeleft: f64,
    /// Time left to download.
    #[serde(default)]
    pub timeleft: String,
    /// When the download is expected to complete.
    #[serde(default)]
    pub estimated_completion_time: Option<DateTime<Utc>>,
    /// Download status.
    #[serde(default)]
    pub status: String,
    /// Tracked download status.
    #[serde(default)]
    pub tracked_download_status: String,
    /// Tracked download state.
    #[serde(default)]
    pub tracked_download_state: String,
    /// Messages attached to the item.
    #[serde(default)]
    pub status_messages: Vec<StatusMessage>,
    /// Download client ID for the release.
    #[serde(default)]
    pub download_id: String,
    /// Protocol the release was grabbed with.
    #[serde(default)]
    pub protocol: Protocol,
    /// Download client handling the release.
    #[serde(default)]
    pub download_client: String,
    /// Indexer the release came from.
    #[serde(default)]
    pub indexer: String,
    /// Where the download client puts the files.
    #[serde(default)]
    pub output_path: String,
    /// Error message, when the item failed.
    #[serde(default)]
    pub error_message: String,
}

/// QueueStatus is the aggregate queue status from `/api/v3/queue/status`.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueueStatus {
    /// Status ID.
    #[serde(default, skip_serializing_if = "is_default")]
    pub id: i32,
    /// Total number of items.
    #[serde(default, skip_serializing_if = "is_default")]
    pub total_count: i32,
    /// Number of known items.
    #[serde(default, skip_serializing_if = "is_default")]
    pub count: i32,
    /// Number of unknown items.
    #[serde(default, skip_serializing_if = "is_default")]
    pub unknown_count: i32,
    /// Whether any item has an error.
    #[serde(default)]
    pub errors: bool,
    /// Whether any item has a warning.
    #[serde(default)]
    pub warnings: bool,
    /// Whether any unknown item has an error.
    #[serde(default)]
    pub unknown_errors: bool,
    /// Whether any unknown item has a warning.
    #[serde(default)]
    pub unknown_warnings: bool,
}

impl Sonarr {
    /// Returns the Sonarr Queue (processing, but not yet imported).
    ///
    /// If you need control over the page, use [`Sonarr::get_queue_page`].
    /// This function simply returns the number of queue records desired, up to
    /// the number of records present in the application. It grabs records in
    /// (paginated) batches of `per_page`, and concatenates them into one list.
    /// Passing zero for `records` will return all of them.
    pub async fn get_queue(&self, records: i32, per_page: i32) -> Result<Queue> {
        let mut queue = Queue::default();
        let mut per_page = set_per_page(records, per_page);

        let mut page = 1;
        loop {
            let curr = self
                .get_queue_page(Some(&PageReq::new().page_size(per_page).page(page)))
                .await?;

            let curr_len = curr.records.len();
            queue.records.extend(curr.records);

            let collected = queue.records.len() as i32;
            if collected >= curr.total_records
                || (collected >= records && records != 0)
                || curr_len == 0
            {
                queue.page_size = curr.total_records;
                queue.total_records = curr.total_records;
                queue.sort_direction = curr.sort_direction;
                queue.sort_key = curr.sort_key;

                break;
            }

            per_page = adjust_per_page(records, curr.total_records, collected, per_page);
            page += 1;
        }

        Ok(queue)
    }

    /// Returns a single page from the Sonarr Queue.
    ///
    /// The page size and number is configurable with the input request parameters.
    pub async fn get_queue_page(&self, params: Option<&PageReq>) -> Result<Queue> {
        let mut params = params.cloned().unwrap_or_default();
        params.check_set("sortKey", "timeleft");
        params.check_set("includeUnknownSeriesItems", "true");

        self.api
            .get_into(Request::new(BP_QUEUE).with_query(params.params()))
            .await
    }

    /// Deletes an item from the Activity Queue.
    pub async fn delete_queue(&self, queue_id: i64, opts: Option<&QueueDeleteOpts>) -> Result<()> {
        let req = Request::new(path_join(&[BP_QUEUE, &str_val(queue_id)]))
            .with_query(queue_delete_values(opts));
        self.api.delete_any(req).await
    }

    /// Tells the app to grab items that are in the queue.
    ///
    /// Most often used on items with a delay set from a delay profile.
    pub async fn queue_grab(&self, ids: &[i64]) -> Result<()> {
        #[derive(Serialize)]
        struct IdList<'a> {
            ids: &'a [i64],
        }

        let req =
            Request::new(path_join(&[BP_QUEUE, "grab", "bulk"])).with_json(&IdList { ids })?;
        self.api.post_any(req).await
    }

    /// Returns the raw JSON body of the queue details endpoint.
    pub async fn get_queue_details(&self, query: Values) -> Result<Vec<u8>> {
        let uri = set_api_path(&path_join(&[BP_QUEUE, "details"]));

        let resp = self
            .api
            .get(Request::new(uri.clone()).with_query(query))
            .await?;

        let body = resp.bytes().await.map_err(|source| Error::Http {
            context: format!("reading response body from {uri}"),
            source,
        })?;

        Ok(body.to_vec())
    }

    /// Returns the aggregate queue status.
    pub async fn get_queue_status(&self) -> Result<QueueStatus> {
        self.api
            .get_into(Request::new(path_join(&[BP_QUEUE, "status"])))
            .await
    }

    /// Deletes many items from the Activity Queue.
    pub async fn delete_queue_bulk(
        &self,
        ids: &[i64],
        opts: Option<&QueueDeleteOpts>,
    ) -> Result<()> {
        #[derive(Serialize)]
        struct IdList<'a> {
            ids: &'a [i64],
        }

        let uri = set_api_path(&path_join(&[BP_QUEUE, "bulk"]));
        let req = Request::new(uri)
            .with_json(&IdList { ids })?
            .with_query(queue_delete_values(opts));

        self.api.delete(req).await?;

        Ok(())
    }

    /// Tells the app to grab one item that is in the queue.
    pub async fn queue_grab_one(&self, queue_id: i64) -> Result<()> {
        self.api
            .post_any(Request::new(path_join(&[
                BP_QUEUE,
                "grab",
                &str_val(queue_id),
            ])))
            .await
    }
}
