use super::Sonarr;
use crate::error::Result;
use crate::interface::ApiClientExt;
use crate::is_default;
use crate::helpers::str_val;
use crate::req::{Request, path_join};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

const BP_COMMAND: &str = "v3/command";

/// CommandRequest goes into the `/api/v3/command` endpoint.
///
/// This was created from the search command and may not support other commands yet.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommandRequest {
    /// Season the command applies to.
    #[serde(default, skip_serializing_if = "is_default")]
    pub season_number: i32,
    /// Series the command applies to.
    #[serde(default, skip_serializing_if = "is_default")]
    pub series_id: i64,
    /// Episode the command applies to.
    #[serde(default, skip_serializing_if = "is_default")]
    pub episode_id: i64,
    /// Name of the command to run.
    #[serde(default)]
    pub name: String,
    /// Files the command applies to. RenameFiles only.
    #[serde(default, skip_serializing_if = "is_default")]
    pub files: Vec<i64>,
    /// Series the command applies to.
    #[serde(default, skip_serializing_if = "is_default")]
    pub series_ids: Vec<i64>,
    /// Episodes the command applies to.
    #[serde(default, skip_serializing_if = "is_default")]
    pub episode_ids: Vec<i64>,
}

/// CommandResponse comes from the `/api/v3/command` endpoint.
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

impl Sonarr {
    /// Returns all available Sonarr commands.
    pub async fn get_commands(&self) -> Result<Vec<CommandResponse>> {
        self.api.get_into(Request::new(BP_COMMAND)).await
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

    /// Sends a command to Sonarr.
    pub async fn send_command(&self, cmd: &CommandRequest) -> Result<CommandResponse> {
        if cmd.name.is_empty() {
            return Ok(CommandResponse::default());
        }

        self.api
            .post_into(Request::new(BP_COMMAND).with_json(cmd)?)
            .await
    }
}
