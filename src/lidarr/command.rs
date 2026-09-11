use super::Lidarr;
use crate::error::Result;
use crate::helpers::str_val;
use crate::interface::ApiClientExt;
use crate::is_default;
use crate::req::{Request, path_join};
use crate::shared::Quality;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

const BP_COMMAND: &str = "v1/command";

/// CommandRequest goes into the `/api/v1/command` endpoint.
///
/// This was created from the search command and may not support other commands yet.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommandRequest {
    /// Name of the command to run.
    #[serde(default)]
    pub name: String,
    /// Albums the command applies to.
    #[serde(default, skip_serializing_if = "is_default")]
    pub album_ids: Vec<i64>,
    /// Album the command applies to.
    #[serde(default, skip_serializing_if = "is_default")]
    pub album_id: i64,
    /// Folders the command applies to.
    #[serde(default, skip_serializing_if = "is_default")]
    pub folders: Vec<String>,
    /// Artist the command applies to.
    #[serde(default, skip_serializing_if = "is_default")]
    pub artist_id: i64,
}

/// ManualImportFile is one file in a [`ManualImportCommandRequest`].
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ManualImportFile {
    /// Path to the file being imported.
    #[serde(default)]
    pub path: String,
    /// Artist the file belongs to.
    #[serde(default)]
    pub artist_id: i64,
    /// Album the file belongs to.
    #[serde(default)]
    pub album_id: i64,
    /// Album release the file belongs to.
    #[serde(default)]
    pub album_release_id: i64,
    /// Tracks the file contains.
    #[serde(default)]
    pub track_ids: Vec<i64>,
    /// Quality of the file.
    #[serde(default)]
    pub quality: Option<Quality>,
    /// Indexer flags set on the release.
    #[serde(default)]
    pub indexer_flags: i32,
    /// Download client ID the file came from.
    #[serde(default)]
    pub download_id: String,
    /// Whether release switching is disabled for this import.
    #[serde(default)]
    pub disable_release_switching: bool,
}

/// ManualImportCommandRequest is the body for the `ManualImport` command
/// (`POST /api/v1/command`). It triggers Lidarr to import the listed files.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ManualImportCommandRequest {
    /// Command name. Defaults to `ManualImport`.
    #[serde(default)]
    pub name: String,
    /// Files to import.
    #[serde(default)]
    pub files: Vec<ManualImportFile>,
    /// Import mode. Defaults to `auto`.
    #[serde(default)]
    pub import_mode: String,
    /// Whether existing files are replaced.
    #[serde(default)]
    pub replace_existing_files: bool,
}

/// CommandResponse comes from the `/api/v1/command` endpoint.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommandResponse {
    /// Command ID.
    #[serde(default)]
    pub id: i64,
    /// Command name.
    #[serde(default)]
    pub name: String,
    /// Internal command name.
    #[serde(default)]
    pub command_name: String,
    /// Status message, when there is one.
    #[serde(default, skip_serializing_if = "is_default")]
    pub message: String,
    /// Command priority.
    #[serde(default)]
    pub priority: String,
    /// Command status.
    #[serde(default)]
    pub status: String,
    /// When the command was queued.
    #[serde(default)]
    pub queued: Option<DateTime<Utc>>,
    /// When the command started.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub started: Option<DateTime<Utc>>,
    /// When the command ended.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ended: Option<DateTime<Utc>>,
    /// When the command last changed state.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state_change_time: Option<DateTime<Utc>>,
    /// When the command last ran.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_execution_time: Option<DateTime<Utc>>,
    /// How long the command ran for.
    #[serde(default, skip_serializing_if = "is_default")]
    pub duration: String,
    /// What triggered the command.
    #[serde(default)]
    pub trigger: String,
    /// Whether progress is sent to connected clients.
    #[serde(default)]
    pub send_updates_to_client: bool,
    /// Whether the scheduled task is updated when this runs.
    #[serde(default)]
    pub update_scheduled_task: bool,
    /// The command payload.
    #[serde(default)]
    pub body: HashMap<String, serde_json::Value>,
}

impl Lidarr {
    /// Returns all available Lidarr commands.
    pub async fn get_commands(&self) -> Result<Vec<CommandResponse>> {
        self.api.get_into(Request::new(BP_COMMAND)).await
    }

    /// Sends a command to Lidarr.
    pub async fn send_command(&self, cmd: &CommandRequest) -> Result<CommandResponse> {
        if cmd.name.is_empty() {
            return Ok(CommandResponse::default());
        }

        self.api
            .post_into(Request::new(BP_COMMAND).with_json(cmd)?)
            .await
    }

    /// Returns the status of an already started command.
    pub async fn get_command_status(&self, command_id: i64) -> Result<CommandResponse> {
        if command_id == 0 {
            return Ok(CommandResponse::default());
        }

        self.api
            .get_into(Request::new(path_join(&[BP_COMMAND, &str_val(command_id)])))
            .await
    }

    /// Sends the `ManualImport` command to import the given files
    /// (e.g. after a FLAC+CUE split).
    pub async fn send_manual_import_command(
        &self,
        cmd: &ManualImportCommandRequest,
    ) -> Result<CommandResponse> {
        if cmd.files.is_empty() {
            return Ok(CommandResponse::default());
        }

        let mut cmd = cmd.clone();
        if cmd.name.is_empty() {
            cmd.name = "ManualImport".to_string();
        }

        if cmd.import_mode.is_empty() {
            cmd.import_mode = "auto".to_string();
        }

        self.api
            .post_into(Request::new(BP_COMMAND).with_json(&cmd)?)
            .await
    }
}
