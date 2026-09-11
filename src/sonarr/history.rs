use super::{Language, Sonarr};
use crate::error::{Error, Result};
use crate::helpers::str_val;
use crate::interface::ApiClientExt;
use crate::is_default;
use crate::paginate::{PageReq, adjust_per_page, set_per_page};
use crate::req::{Request, path_join};
use crate::shared::{Protocol, Quality};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

const BP_HISTORY: &str = "v3/history";

/// History is the `/api/v3/history` endpoint.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct History {
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
    /// The history items.
    #[serde(default)]
    pub records: Vec<HistoryRecord>,
}

/// HistoryRecord is part of the [`History`] data.
///
/// Not all items have all Data members. Check `event_type` for what you need.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HistoryRecord {
    /// History item ID.
    #[serde(default)]
    pub id: i64,
    /// Episode the event applies to.
    #[serde(default)]
    pub episode_id: i64,
    /// Series the event applies to.
    #[serde(default)]
    pub series_id: i64,
    /// Title of the release.
    #[serde(default)]
    pub source_title: String,
    /// Language of the release.
    #[serde(default)]
    pub language: Language,
    /// Quality of the release.
    #[serde(default)]
    pub quality: Option<Quality>,
    /// Whether the quality cutoff was not met.
    #[serde(default)]
    pub quality_cutoff_not_met: bool,
    /// Whether the language cutoff was not met.
    #[serde(default)]
    pub language_cutoff_not_met: bool,
    /// When the event happened.
    #[serde(default)]
    pub date: Option<DateTime<Utc>>,
    /// Download client ID for the release.
    #[serde(default, skip_serializing_if = "is_default")]
    pub download_id: String,
    /// Type of the event.
    #[serde(default)]
    pub event_type: String,
    /// Event-specific data.
    #[serde(default)]
    pub data: HistoryRecordData,
}

/// HistoryRecordData is the `data` member of a [`HistoryRecord`].
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HistoryRecordData {
    /// Age of the release in days.
    #[serde(default)]
    pub age: String,
    /// Age of the release in hours.
    #[serde(default)]
    pub age_hours: String,
    /// Age of the release in minutes.
    #[serde(default)]
    pub age_minutes: String,
    /// Download client that handled the release.
    #[serde(default)]
    pub download_client: String,
    /// Name of the download client.
    #[serde(default)]
    pub download_client_name: String,
    /// URL the release was downloaded from.
    #[serde(default, rename = "downloadUrl")]
    pub download_url: String,
    /// Path the release was dropped in.
    #[serde(default)]
    pub dropped_path: String,
    /// ID of the imported file.
    #[serde(default)]
    pub file_id: String,
    /// Release GUID.
    #[serde(default, rename = "guid")]
    pub guid: String,
    /// Path the release was imported to.
    #[serde(default)]
    pub imported_path: String,
    /// Indexer the release came from.
    #[serde(default)]
    pub indexer: String,
    /// Event message.
    #[serde(default)]
    pub message: String,
    /// Score of the preferred words matched.
    #[serde(default)]
    pub preferred_word_score: String,
    /// NZB info URL.
    #[serde(default, rename = "nzbInfoUrl")]
    pub nzb_info_url: String,
    /// Protocol the release was grabbed with.
    #[serde(default)]
    pub protocol: Protocol,
    /// When the release was published.
    #[serde(default)]
    pub published_date: Option<DateTime<Utc>>,
    /// Why the event happened.
    #[serde(default)]
    pub reason: String,
    /// Release group that produced the release.
    #[serde(default)]
    pub release_group: String,
    /// Size of the release in bytes.
    #[serde(default)]
    pub size: String,
    /// TVRage ID of the series.
    #[serde(default, rename = "tvRageId")]
    pub tv_rage_id: String,
    /// TheTVDB ID of the series.
    #[serde(default, rename = "tvdbId")]
    pub tvdb_id: String,
    /// Torrent info hash.
    #[serde(default)]
    pub torrent_info_hash: String,
}

impl Sonarr {
    /// Returns the Sonarr History (grabs/failures/completed).
    ///
    /// If you need control over the page, use [`Sonarr::get_history_page`].
    /// This function simply returns the number of history records desired, up
    /// to the number of records present in the application. It grabs records
    /// in (paginated) batches of `per_page`, and concatenates them into one
    /// list. Passing zero for `records` will return all of them.
    pub async fn get_history(&self, records: i32, per_page: i32) -> Result<History> {
        let mut hist = History::default();
        let mut per_page = set_per_page(records, per_page);

        let mut page = 1;
        loop {
            let curr = self
                .get_history_page(Some(&PageReq::new().page_size(per_page).page(page)))
                .await?;

            let curr_len = curr.records.len();
            hist.records.extend(curr.records);

            let collected = hist.records.len() as i32;
            if collected >= curr.total_records
                || (collected >= records && records != 0)
                || curr_len == 0
            {
                hist.page_size = curr.total_records;
                hist.total_records = curr.total_records;
                hist.sort_direction = curr.sort_direction;
                hist.sort_key = curr.sort_key;

                break;
            }

            per_page = adjust_per_page(records, curr.total_records, collected, per_page);
            page += 1;
        }

        Ok(hist)
    }

    /// Returns a single page from the Sonarr History (grabs/failures/completed).
    ///
    /// The page size and number is configurable with the input request parameters.
    pub async fn get_history_page(&self, params: Option<&PageReq>) -> Result<History> {
        let params = params.cloned().unwrap_or_default();

        self.api
            .get_into(Request::new(BP_HISTORY).with_query(params.params()))
            .await
    }

    /// Marks the given history item as failed by id.
    pub async fn fail(&self, history_id: i64) -> Result<()> {
        if history_id < 1 {
            return Err(Error::Request(format!("invalid history ID: {history_id}")));
        }

        // Strangely uses a POST without a payload.
        self.api
            .post_any(Request::new(path_join(&[
                BP_HISTORY,
                "failed",
                &str_val(history_id),
            ])))
            .await
    }
}
