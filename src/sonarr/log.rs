use super::Sonarr;
use crate::error::Result;
use crate::interface::ApiClientExt;
use crate::is_default;
use crate::paginate::PageReq;
use crate::req::{Request, path_join};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

const BP_LOG: &str = "v3/log";

/// LogLine is one record from `/api/v3/log`.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LogLine {
    /// Log line ID.
    #[serde(default)]
    pub id: i32,
    /// When the line was logged.
    #[serde(default)]
    pub time: Option<DateTime<Utc>>,
    /// Exception message, when there is one.
    #[serde(default, skip_serializing_if = "is_default")]
    pub exception: String,
    /// Type of the exception.
    #[serde(default, skip_serializing_if = "is_default")]
    pub exception_type: String,
    /// Log level.
    #[serde(default, skip_serializing_if = "is_default")]
    pub level: String,
    /// Logger that produced the line.
    #[serde(default, skip_serializing_if = "is_default")]
    pub logger: String,
    /// Log message.
    #[serde(default, skip_serializing_if = "is_default")]
    pub message: String,
    /// Method that produced the line.
    #[serde(default, skip_serializing_if = "is_default")]
    pub method: String,
}

/// LogPage is a page of log lines from `/api/v3/log`.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LogPage {
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
    /// The log lines.
    #[serde(default)]
    pub records: Vec<LogLine>,
}

/// LogFile describes a log file on disk.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LogFile {
    /// Name of the file.
    #[serde(default, skip_serializing_if = "is_default")]
    pub filename: String,
    /// Contents of the file.
    #[serde(default, skip_serializing_if = "is_default")]
    pub contents: String,
    /// When the file was last written to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_write: Option<DateTime<Utc>>,
}

impl Sonarr {
    /// Returns a page of application log lines.
    pub async fn get_log_page(&self, params: Option<&PageReq>) -> Result<LogPage> {
        let params = params.cloned().unwrap_or_default();

        self.api
            .get_into(Request::new(BP_LOG).with_query(params.params()))
            .await
    }

    /// Returns the list of log files.
    pub async fn get_log_files(&self) -> Result<Vec<LogFile>> {
        self.api
            .get_into(Request::new(path_join(&[BP_LOG, "file"])))
            .await
    }

    /// Returns the contents of a named log file.
    pub async fn get_log_file(&self, filename: &str) -> Result<LogFile> {
        self.api
            .get_into(Request::new(path_join(&[BP_LOG, "file", filename])))
            .await
    }

    /// Triggers a log file update/roll.
    pub async fn update_log_files(&self) -> Result<Vec<LogFile>> {
        self.api
            .get_into(Request::new(path_join(&[BP_LOG, "file", "update"])))
            .await
    }

    /// Triggers an update for a specific log file.
    pub async fn update_log_file(&self, filename: &str) -> Result<Vec<LogFile>> {
        self.api
            .get_into(Request::new(path_join(&[
                BP_LOG, "file", "update", filename,
            ])))
            .await
    }
}
