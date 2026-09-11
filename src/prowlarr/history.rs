use super::Prowlarr;
use crate::error::Result;
use crate::helpers::str_val;
use crate::interface::ApiClientExt;
use crate::is_default;
use crate::paginate::PageReq;
use crate::req::{Request, path_join};
use crate::values::Values;
use chrono::{DateTime, SecondsFormat, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

const BP_HISTORY: &str = "v1/history";

/// HistoryPage is a paged history response.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HistoryPage {
    /// Page number.
    #[serde(default)]
    pub page: i32,
    /// Records per page.
    #[serde(default)]
    pub page_size: i32,
    /// Sort key used.
    #[serde(default, skip_serializing_if = "is_default")]
    pub sort_key: String,
    /// Sort direction used.
    #[serde(default, skip_serializing_if = "is_default")]
    pub sort_direction: String,
    /// Total records available.
    #[serde(default)]
    pub total_records: i32,
    /// The records on this page.
    #[serde(default)]
    pub records: Vec<HistoryRecord>,
}

/// HistoryRecord is one history entry.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HistoryRecord {
    /// Record ID.
    #[serde(default)]
    pub id: i64,
    /// Title of the release.
    #[serde(default, skip_serializing_if = "is_default")]
    pub source_title: String,
    /// When the event happened.
    #[serde(default)]
    pub date: Option<DateTime<Utc>>,
    /// The event type.
    #[serde(default, skip_serializing_if = "is_default")]
    pub event_type: String,
    /// Download client ID.
    #[serde(default, rename = "downloadId", skip_serializing_if = "is_default")]
    pub download_id: String,
    /// Event-specific data.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<HashMap<String, serde_json::Value>>,
}

impl Prowlarr {
    /// Returns a page of history.
    pub async fn get_history_page(&self, params: Option<&PageReq>) -> Result<HistoryPage> {
        let params = params.cloned().unwrap_or_default();
        let req = Request::new(BP_HISTORY).with_query(params.params());

        self.api.get_into(req).await
    }

    /// Returns history since a date.
    pub async fn get_history_since(
        &self,
        date: DateTime<Utc>,
        event_type: &str,
    ) -> Result<Vec<HistoryRecord>> {
        let mut params = Values::new();
        params.set("date", date.to_rfc3339_opts(SecondsFormat::Secs, true));

        if !event_type.is_empty() {
            params.set("eventType", event_type);
        }

        let req = Request::new(path_join(&[BP_HISTORY, "since"])).with_query(params);

        self.api.get_into(req).await
    }

    /// Returns history for an indexer.
    pub async fn get_history_by_indexer(
        &self,
        indexer_id: i64,
        event_type: &str,
        limit: i32,
    ) -> Result<Vec<HistoryRecord>> {
        let mut params = Values::new();
        params.set("indexerId", str_val(indexer_id));

        if !event_type.is_empty() {
            params.set("eventType", event_type);
        }

        if limit > 0 {
            params.set("limit", str_val(limit));
        }

        let req = Request::new(path_join(&[BP_HISTORY, "indexer"])).with_query(params);

        self.api.get_into(req).await
    }
}
