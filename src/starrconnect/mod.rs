//! Decodes HTTP webhook JSON payloads from Sonarr, Radarr, Lidarr, and Prowlarr.
//!
//! Configure webhooks in each app under Settings → Connect → Webhook.
//!
//! For Custom Script (environment variables) instead of HTTP webhooks, see
//! [`crate::starrcmd`].
//!
//! # Differences from the Go library
//!
//! Go implements `http.Handler` directly. Rust has no HTTP server in its
//! standard library, so the handlers here take the request method and body and
//! return the [`WebhookResponse`] the caller should write back. Wire that into
//! whatever server you use.
//!
//! Notes to future developers of this module:
//!
//! * IDs are `i64` where they may exceed 32 bits; C# `int` maps to `i32` where safe.
//! * Sizes are `i64` (bytes).
//! * JSON uses camelCase property names (Servarr Newtonsoft / System.Text.Json settings).
//! * Sonarr sends eventType `Download` for both single-file import and
//!   import-complete; use `episodeFile` vs `episodeFiles` in the JSON to tell
//!   them apart (see [`sonarr`]).
//! * Lidarr sends eventType `ImportFailure` with the same shape as `Download`
//!   but album is null.
//! * Health payload `level` is a string enum: ok, notice, warning, error.
//! * Prowlarr currently emits Test, Grab, Health, HealthRestored, and
//!   ApplicationUpdate; Download and Rename exist on WebhookEventType but have
//!   no dedicated payload types upstream yet.

mod lidarr;
mod lidarr_handler;
mod prowlarr;
mod prowlarr_handler;
mod radarr;
mod radarr_handler;
mod sonarr;
mod sonarr_handler;

pub use lidarr::*;
pub use lidarr_handler::*;
pub use prowlarr::*;
pub use prowlarr_handler::*;
pub use radarr::*;
pub use radarr_handler::*;
pub use sonarr::*;
pub use sonarr_handler::*;

use crate::string_enum;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

/// Largest webhook body this module accepts.
pub const MAX_BODY_BYTES: usize = 10 << 20; // 10 MiB

/// Errors returned while decoding a webhook payload.
#[derive(Debug, thiserror::Error)]
pub enum WebhookError {
    /// The envelope eventType does not match the requested payload type.
    #[error("starrconnect: event type mismatch: got {got} want {want:?}")]
    WrongEvent {
        /// The event type found in the payload.
        got: EventType,
        /// The event types the caller accepts.
        want: Vec<EventType>,
    },

    /// The JSON eventType is not recognized for this app.
    #[error("starrconnect: unknown event type: {0}")]
    UnknownEvent(EventType),

    /// The webhook POST exceeds [`MAX_BODY_BYTES`].
    #[error("starrconnect: request body too large")]
    BodyTooLarge,

    /// The payload could not be decoded.
    #[error("{context}: {source}")]
    Json {
        /// What we were decoding.
        context: String,
        /// The serde error.
        source: serde_json::Error,
    },
}

/// Convenient result alias used by this module.
pub type WebhookResult<T> = std::result::Result<T, WebhookError>;

/// The error type a webhook callback may return.
pub type CallbackError = Box<dyn std::error::Error + Send + Sync>;

/// The result a webhook callback returns.
pub type CallbackResult = std::result::Result<(), CallbackError>;

/// A callback invoked with a decoded webhook payload.
pub type Callback<T> = Box<dyn Fn(&T) -> CallbackResult + Send + Sync>;

/// A callback invoked when a webhook fails to decode or a callback errors.
pub type ErrorCallback = Box<dyn Fn(&(dyn std::error::Error + 'static)) + Send + Sync>;

/// The HTTP status and body a handler wants written back to the Starr app.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct WebhookResponse {
    /// HTTP status code to return.
    pub status: u16,
    /// Response body to return. Empty on success.
    pub message: String,
}

impl WebhookResponse {
    /// Returns the response for a webhook that was handled successfully.
    pub fn ok() -> Self {
        Self {
            status: 200,
            message: String::new(),
        }
    }

    /// Returns an error response with the given status and public message.
    pub fn error(status: u16, message: &str) -> Self {
        Self {
            status,
            message: message.to_string(),
        }
    }

    /// Reports whether the response is a success.
    pub fn is_ok(&self) -> bool {
        self.status == 200
    }
}

string_enum! {
    /// EventType is the webhook `eventType` field (JSON string).
    pub struct EventType {
        /// Test event, sent from the app's UI.
        const TEST = "Test";
        /// A release was grabbed.
        const GRAB = "Grab";
        /// A release was imported.
        const DOWNLOAD = "Download";
        /// Files were renamed.
        const RENAME = "Rename";
        /// A health check failed.
        const HEALTH = "Health";
        /// A health check recovered.
        const HEALTH_RESTORED = "HealthRestored";
        /// The application updated itself.
        const APPLICATION_UPDATE = "ApplicationUpdate";
        /// A queue item needs attention.
        const MANUAL_INTERACTION_REQUIRED = "ManualInteractionRequired";
        /// A series was added.
        const SERIES_ADD = "SeriesAdd";
        /// A series was deleted.
        const SERIES_DELETE = "SeriesDelete";
        /// An episode file was deleted.
        const EPISODE_FILE_DELETE = "EpisodeFileDelete";
        /// A movie was added.
        const MOVIE_ADDED = "MovieAdded";
        /// A movie was deleted.
        const MOVIE_DELETE = "MovieDelete";
        /// A movie file was deleted.
        const MOVIE_FILE_DELETE = "MovieFileDelete";
        /// A download failed.
        const DOWNLOAD_FAILURE = "DownloadFailure";
        /// An import failed.
        const IMPORT_FAILURE = "ImportFailure";
        /// An artist was added.
        const ARTIST_ADD = "ArtistAdd";
        /// An artist was deleted.
        const ARTIST_DELETE = "ArtistDelete";
        /// An album was deleted.
        const ALBUM_DELETE = "AlbumDelete";
        /// A track file was retagged.
        const RETAG = "Retag";
    }
}

/// BaseEvent is embedded in every concrete webhook payload.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BaseEvent {
    /// Type of the event.
    #[serde(default)]
    pub event_type: EventType,
    /// Name given to the instance that sent the webhook.
    #[serde(default)]
    pub instance_name: String,
    /// External URL of the instance that sent the webhook.
    #[serde(default, rename = "applicationUrl")]
    pub application_url: String,
}

/// Image is a cover / media image reference in webhook payloads.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Image {
    /// What the image depicts.
    #[serde(default)]
    pub cover_type: String,
    /// Local URL of the image.
    #[serde(default, rename = "url")]
    pub url: String,
    /// Remote URL of the image.
    #[serde(default, rename = "remoteUrl")]
    pub remote_url: String,
}

/// Language is an audio/subtitle language entry (id + name).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Language {
    /// Language ID.
    #[serde(default)]
    pub id: i32,
    /// Language name.
    #[serde(default)]
    pub name: String,
}

/// CustomFormat is a scored custom format on a release.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomFormat {
    /// Format ID.
    #[serde(default)]
    pub id: i64,
    /// Format name.
    #[serde(default)]
    pub name: String,
}

/// CustomFormatInfo groups custom formats and score (Sonarr/Radarr).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomFormatInfo {
    /// Formats matched by the release.
    #[serde(default)]
    pub custom_formats: Vec<CustomFormat>,
    /// Total score of the matched formats.
    #[serde(default)]
    pub custom_format_score: i32,
}

/// DownloadStatusMessage is a tracked-download status message block.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DownloadStatusMessage {
    /// Title of the message.
    #[serde(default)]
    pub title: String,
    /// The messages themselves.
    #[serde(default)]
    pub messages: Vec<String>,
}

/// DownloadClientItem describes the download client queue item
/// (e.g. manual interaction).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DownloadClientItem {
    /// Quality of the release.
    #[serde(default)]
    pub quality: String,
    /// Revision of the quality.
    #[serde(default)]
    pub quality_version: i32,
    /// Release title.
    #[serde(default)]
    pub title: String,
    /// Indexer the release came from.
    #[serde(default)]
    pub indexer: String,
    /// Size of the release in bytes.
    #[serde(default)]
    pub size: i64,
}

/// Checks that `got` is in `want`, then decodes `body` into `T`.
pub fn decode_webhook_payload<T: DeserializeOwned>(
    body: &[u8],
    got: &EventType,
    want: &[EventType],
) -> WebhookResult<T> {
    if !want.contains(got) {
        return Err(WebhookError::WrongEvent {
            got: got.clone(),
            want: want.to_vec(),
        });
    }

    serde_json::from_slice(body).map_err(|source| WebhookError::Json {
        context: format!("decoding {got} event as {}", std::any::type_name::<T>()),
        source,
    })
}

/// Checks a webhook body against [`MAX_BODY_BYTES`].
pub fn check_body_size(body: &[u8]) -> WebhookResult<()> {
    if body.len() > MAX_BODY_BYTES {
        return Err(WebhookError::BodyTooLarge);
    }

    Ok(())
}

/// A failed webhook, carrying both the public response and the private cause.
pub(crate) struct HandlerFailure {
    /// What the Starr app is told.
    pub response: WebhookResponse,
    /// What the `on_error` callback is told.
    pub cause: CallbackError,
}

impl HandlerFailure {
    pub(crate) fn new(status: u16, message: &str, cause: impl Into<CallbackError>) -> Self {
        Self {
            response: WebhookResponse::error(status, message),
            cause: cause.into(),
        }
    }
}

/// Runs an optional user callback after a successful typed decode.
pub(crate) fn run_webhook_callback<T, F>(
    callback: Option<&Callback<T>>,
    decode: F,
) -> std::result::Result<(), HandlerFailure>
where
    F: FnOnce() -> WebhookResult<T>,
{
    let Some(callback) = callback else {
        return Ok(());
    };

    let value = decode().map_err(|err| HandlerFailure::new(500, "handler error", err))?;

    callback(&value).map_err(|err| HandlerFailure::new(500, "handler error", err))
}
