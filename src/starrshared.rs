//! API types used by more than one Starr app client, ported from `starrshared/`.

use crate::is_default;
use crate::shared::FieldInput;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// AutoTagging is the auto tagging API resource shared by Sonarr, Lidarr, Readarr, and Radarr.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AutoTagging {
    /// Auto tag ID.
    #[serde(default, skip_serializing_if = "is_default")]
    pub id: i32,
    /// Auto tag name.
    #[serde(default, skip_serializing_if = "is_default")]
    pub name: String,
    /// Whether tags are removed automatically when rules stop matching.
    #[serde(default, rename = "removeTagsAutomatically")]
    pub remove_tags_automatically: bool,
    /// Tag IDs applied by this rule.
    #[serde(default, skip_serializing_if = "is_default")]
    pub tags: Vec<i32>,
    /// Rules inside this definition.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub specifications: Option<Vec<AutoTaggingSpecification>>,
}

/// AutoTaggingSpecification is one rule inside an [`AutoTagging`] definition.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AutoTaggingSpecification {
    /// Specification ID.
    #[serde(default, skip_serializing_if = "is_default")]
    pub id: i32,
    /// Specification name.
    #[serde(default, skip_serializing_if = "is_default")]
    pub name: String,
    /// Implementation identifier.
    #[serde(default, skip_serializing_if = "is_default")]
    pub implementation: String,
    /// Human readable implementation name.
    #[serde(
        default,
        rename = "implementationName",
        skip_serializing_if = "is_default"
    )]
    pub implementation_name: String,
    /// Whether the match is inverted.
    #[serde(default)]
    pub negate: bool,
    /// Whether this rule must match.
    #[serde(default)]
    pub required: bool,
    /// Implementation-specific fields.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<FieldInput>>,
}

/// CustomFilter is the `/customfilter` resource (UI saved filters).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct CustomFilter {
    /// Filter ID.
    #[serde(default, skip_serializing_if = "is_default")]
    pub id: i32,
    /// Filter type.
    #[serde(default, rename = "type", skip_serializing_if = "is_default")]
    pub filter_type: String,
    /// Filter label.
    #[serde(default, skip_serializing_if = "is_default")]
    pub label: String,
    /// Raw filter definitions; shape varies by app and UI version.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<serde_json::Value>>,
}

/// DiskSpace is the `/diskspace` API resource shared by all Starr app clients.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct DiskSpace {
    /// Entry ID.
    #[serde(default)]
    pub id: i32,
    /// Mount path.
    #[serde(default, skip_serializing_if = "is_default")]
    pub path: String,
    /// Volume label.
    #[serde(default, skip_serializing_if = "is_default")]
    pub label: String,
    /// Free bytes.
    #[serde(default, rename = "freeSpace")]
    pub free_space: i64,
    /// Total bytes.
    #[serde(default, rename = "totalSpace")]
    pub total_space: i64,
}

/// Health is the `/health` API resource shared by all Starr app clients.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Health {
    /// Health entry ID.
    #[serde(default)]
    pub id: i32,
    /// Source of the health check.
    #[serde(default, skip_serializing_if = "is_default")]
    pub source: String,
    /// Severity type.
    #[serde(default, rename = "type", skip_serializing_if = "is_default")]
    pub health_type: String,
    /// Human readable message.
    #[serde(default, skip_serializing_if = "is_default")]
    pub message: String,
    /// Link to the wiki entry describing the problem.
    #[serde(default, rename = "wikiUrl", skip_serializing_if = "is_default")]
    pub wiki_url: String,
}

/// SystemTask is a scheduled task from `/system/task`.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SystemTask {
    /// Task ID.
    #[serde(default, skip_serializing_if = "is_default")]
    pub id: i32,
    /// Task name.
    #[serde(default, skip_serializing_if = "is_default")]
    pub name: String,
    /// Internal task name.
    #[serde(default, rename = "taskName", skip_serializing_if = "is_default")]
    pub task_name: String,
    /// Run interval in minutes.
    #[serde(default, skip_serializing_if = "is_default")]
    pub interval: i32,
    /// When the task last completed.
    #[serde(
        default,
        rename = "lastExecution",
        skip_serializing_if = "Option::is_none"
    )]
    pub last_execution: Option<DateTime<Utc>>,
    /// When the task last started.
    #[serde(
        default,
        rename = "lastStartTime",
        skip_serializing_if = "Option::is_none"
    )]
    pub last_start_time: Option<DateTime<Utc>>,
    /// When the task runs next.
    #[serde(
        default,
        rename = "nextExecution",
        skip_serializing_if = "Option::is_none"
    )]
    pub next_execution: Option<DateTime<Utc>>,
    /// How long the last run took.
    #[serde(default, rename = "lastDuration", skip_serializing_if = "is_default")]
    pub last_duration: String,
}

/// BackupRestoreResponse is returned when restoring a backup.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct BackupRestoreResponse {
    /// Whether the app must be restarted for the restore to complete.
    #[serde(default, rename = "restartRequired")]
    pub restart_required: bool,
}

/// UpdateChanges is the change log embedded in [`Update`].
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct UpdateChanges {
    /// New features.
    #[serde(default, skip_serializing_if = "is_default")]
    pub new: Vec<String>,
    /// Bug fixes.
    #[serde(default, skip_serializing_if = "is_default")]
    pub fixed: Vec<String>,
}

/// Update is one available or installed update from the `/update` endpoint.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Update {
    /// Update ID.
    #[serde(default, skip_serializing_if = "is_default")]
    pub id: i32,
    /// Version string.
    #[serde(default, skip_serializing_if = "is_default")]
    pub version: String,
    /// Release branch.
    #[serde(default, skip_serializing_if = "is_default")]
    pub branch: String,
    /// When the version was released.
    #[serde(
        default,
        rename = "releaseDate",
        skip_serializing_if = "Option::is_none"
    )]
    pub release_date: Option<DateTime<Utc>>,
    /// Package file name.
    #[serde(default, rename = "fileName", skip_serializing_if = "is_default")]
    pub file_name: String,
    /// Package URL.
    #[serde(default, skip_serializing_if = "is_default")]
    pub url: String,
    /// Whether this version is installed.
    #[serde(default)]
    pub installed: bool,
    /// When this version was installed.
    #[serde(
        default,
        rename = "installedOn",
        skip_serializing_if = "Option::is_none"
    )]
    pub installed_on: Option<DateTime<Utc>>,
    /// Whether this version can be installed.
    #[serde(default)]
    pub installable: bool,
    /// Whether this is the latest version.
    #[serde(default)]
    pub latest: bool,
    /// The change log.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub changes: Option<UpdateChanges>,
    /// Package hash.
    #[serde(default, skip_serializing_if = "is_default")]
    pub hash: String,
}
