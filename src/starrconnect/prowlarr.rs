use super::{BaseEvent, EventType, WebhookError, WebhookResult, decode_webhook_payload};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// ProwlarrRelease is indexer release metadata in a Prowlarr Grab webhook.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProwlarrRelease {
    /// Release title.
    #[serde(default)]
    pub release_title: String,
    /// Indexer the release came from.
    #[serde(default)]
    pub indexer: String,
    /// Size of the release in bytes. May be empty or null.
    #[serde(default)]
    pub size: i64,
    /// Categories the release belongs to.
    #[serde(default)]
    pub categories: Vec<String>,
    /// Genres the release belongs to.
    #[serde(default)]
    pub genres: Vec<String>,
    /// Indexer flags set on the release.
    #[serde(default)]
    pub indexer_flags: Vec<String>,
    /// When the release was published. May be empty or null.
    #[serde(default)]
    pub publish_date: Option<DateTime<Utc>>,
}

/// ProwlarrGrab is the Grab webhook payload.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProwlarrGrab {
    /// The webhook envelope.
    #[serde(flatten)]
    pub base: BaseEvent,
    /// The grabbed release.
    #[serde(default)]
    pub release: Option<ProwlarrRelease>,
    /// What triggered the grab.
    #[serde(default)]
    pub trigger: String,
    /// Application that requested the grab.
    #[serde(default)]
    pub source: String,
    /// Host that requested the grab.
    #[serde(default)]
    pub host: String,
    /// Download client that grabbed the release.
    #[serde(default)]
    pub download_client: String,
    /// Type of the download client.
    #[serde(default)]
    pub download_client_type: String,
    /// Download client ID for the release.
    #[serde(default)]
    pub download_id: String,
}

/// ProwlarrTest is the Test webhook payload (base fields only).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProwlarrTest {
    /// The webhook envelope.
    #[serde(flatten)]
    pub base: BaseEvent,
}

/// ProwlarrDownload is reserved for a future Download webhook shape
/// (WebhookEventType only today).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProwlarrDownload {
    /// The webhook envelope.
    #[serde(flatten)]
    pub base: BaseEvent,
}

/// ProwlarrRename is reserved for a future Rename webhook shape
/// (WebhookEventType only today).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProwlarrRename {
    /// The webhook envelope.
    #[serde(flatten)]
    pub base: BaseEvent,
}

/// ProwlarrHealth is the Health or HealthRestored webhook payload.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProwlarrHealth {
    /// The webhook envelope.
    #[serde(flatten)]
    pub base: BaseEvent,
    /// Severity: ok, notice, warning or error.
    #[serde(default)]
    pub level: String,
    /// The health message.
    #[serde(default)]
    pub message: String,
    /// Which health check produced the message.
    #[serde(default, rename = "type")]
    pub health_type: String,
    /// Wiki page describing the problem.
    #[serde(default, rename = "wikiUrl")]
    pub wiki_url: String,
}

/// ProwlarrApplicationUpdate is the ApplicationUpdate webhook payload.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProwlarrApplicationUpdate {
    /// The webhook envelope.
    #[serde(flatten)]
    pub base: BaseEvent,
    /// The update message.
    #[serde(default)]
    pub message: String,
    /// Version before the update.
    #[serde(default)]
    pub previous_version: String,
    /// Version after the update.
    #[serde(default)]
    pub new_version: String,
}

/// ProwlarrEvent is a parsed Prowlarr webhook envelope plus the raw JSON body.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ProwlarrEvent {
    /// The decoded envelope.
    pub base: BaseEvent,
    body: Vec<u8>,
}

/// Parses the raw JSON body and returns the envelope.
///
/// Use the `get_*` methods to decode the full payload.
pub fn parse_prowlarr(body: &[u8]) -> WebhookResult<ProwlarrEvent> {
    let base: BaseEvent = serde_json::from_slice(body).map_err(|source| WebhookError::Json {
        context: "decoding Prowlarr event envelope".to_string(),
        source,
    })?;

    Ok(ProwlarrEvent {
        base,
        body: body.to_vec(),
    })
}

impl ProwlarrEvent {
    /// Returns the event type from the envelope.
    pub fn event_type(&self) -> &EventType {
        &self.base.event_type
    }

    /// Decodes a Grab payload.
    pub fn get_grab(&self) -> WebhookResult<ProwlarrGrab> {
        decode_webhook_payload(&self.body, &self.base.event_type, &[EventType::GRAB])
    }

    /// Decodes a Test payload.
    pub fn get_test(&self) -> WebhookResult<ProwlarrTest> {
        decode_webhook_payload(&self.body, &self.base.event_type, &[EventType::TEST])
    }

    /// Decodes a Download payload (reserved; upstream may add fields later).
    pub fn get_download(&self) -> WebhookResult<ProwlarrDownload> {
        decode_webhook_payload(&self.body, &self.base.event_type, &[EventType::DOWNLOAD])
    }

    /// Decodes a Rename payload (reserved; upstream may add fields later).
    pub fn get_rename(&self) -> WebhookResult<ProwlarrRename> {
        decode_webhook_payload(&self.body, &self.base.event_type, &[EventType::RENAME])
    }

    /// Decodes a Health payload.
    pub fn get_health(&self) -> WebhookResult<ProwlarrHealth> {
        decode_webhook_payload(&self.body, &self.base.event_type, &[EventType::HEALTH])
    }

    /// Decodes a HealthRestored payload.
    pub fn get_health_restored(&self) -> WebhookResult<ProwlarrHealth> {
        decode_webhook_payload(
            &self.body,
            &self.base.event_type,
            &[EventType::HEALTH_RESTORED],
        )
    }

    /// Decodes an ApplicationUpdate payload.
    pub fn get_application_update(&self) -> WebhookResult<ProwlarrApplicationUpdate> {
        decode_webhook_payload(
            &self.body,
            &self.base.event_type,
            &[EventType::APPLICATION_UPDATE],
        )
    }
}
