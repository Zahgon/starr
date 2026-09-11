use super::{
    BaseEvent, CustomFormatInfo, DownloadClientItem, DownloadStatusMessage, EventType, Image,
    Language, WebhookError, WebhookResult, decode_webhook_payload,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Movie is movie metadata in a Radarr webhook.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Movie {
    /// Movie ID.
    #[serde(default)]
    pub id: i64,
    /// Movie title.
    #[serde(default)]
    pub title: String,
    /// Release year.
    #[serde(default)]
    pub year: i32,
    /// Path to the movie file.
    #[serde(default)]
    pub file_path: String,
    /// When the movie was released.
    #[serde(default)]
    pub release_date: String,
    /// Path to the movie folder.
    #[serde(default)]
    pub folder_path: String,
    /// TMDb ID.
    #[serde(default, rename = "tmdbId")]
    pub tmdb_id: i64,
    /// IMDb ID.
    #[serde(default, rename = "imdbId")]
    pub imdb_id: String,
    /// Movie overview.
    #[serde(default)]
    pub overview: String,
    /// Genres this movie belongs to.
    #[serde(default)]
    pub genres: Vec<String>,
    /// Movie images.
    #[serde(default)]
    pub images: Vec<Image>,
    /// Tags applied to this movie.
    #[serde(default)]
    pub tags: Vec<String>,
    /// Original language of the movie.
    #[serde(default)]
    pub original_language: Option<Language>,
}

/// RemoteMovie is the movie the release was matched to.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoteMovie {
    /// TMDb ID.
    #[serde(default, rename = "tmdbId")]
    pub tmdb_id: i64,
    /// IMDb ID.
    #[serde(default, rename = "imdbId")]
    pub imdb_id: String,
    /// Movie title.
    #[serde(default)]
    pub title: String,
    /// Release year.
    #[serde(default)]
    pub year: i32,
}

/// MovieFileMediaInfo is optional media info on a movie file.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MovieFileMediaInfo {
    /// Number of audio channels.
    #[serde(default)]
    pub audio_channels: f64,
    /// Audio codec.
    #[serde(default)]
    pub audio_codec: String,
    /// Audio languages.
    #[serde(default)]
    pub audio_languages: Vec<String>,
    /// Video height in pixels.
    #[serde(default)]
    pub height: i32,
    /// Video width in pixels.
    #[serde(default)]
    pub width: i32,
    /// Subtitles present in the file.
    #[serde(default)]
    pub subtitles: Vec<String>,
    /// Video codec.
    #[serde(default)]
    pub video_codec: String,
    /// Video dynamic range.
    #[serde(default)]
    pub video_dynamic_range: String,
    /// Video dynamic range type.
    #[serde(default)]
    pub video_dynamic_range_type: String,
}

/// MovieFile is an on-disk movie file in a Radarr webhook.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MovieFile {
    /// Movie file ID.
    #[serde(default)]
    pub id: i64,
    /// Path relative to the movie folder.
    #[serde(default)]
    pub relative_path: String,
    /// Full path to the file.
    #[serde(default)]
    pub path: String,
    /// Quality of the file.
    #[serde(default)]
    pub quality: String,
    /// Revision of the quality.
    #[serde(default)]
    pub quality_version: i32,
    /// Release group that produced the release.
    #[serde(default)]
    pub release_group: String,
    /// Scene name of the release.
    #[serde(default)]
    pub scene_name: String,
    /// Indexer flags set on the release.
    #[serde(default)]
    pub indexer_flags: String,
    /// File size in bytes.
    #[serde(default)]
    pub size: i64,
    /// When the file was added.
    #[serde(default)]
    pub date_added: Option<DateTime<Utc>>,
    /// Languages present in the file.
    #[serde(default)]
    pub languages: Vec<Language>,
    /// Media information read from the file.
    #[serde(default)]
    pub media_info: Option<MovieFileMediaInfo>,
    /// Path the file was imported from.
    #[serde(default)]
    pub source_path: String,
    /// Path in the recycle bin, when the file was deleted.
    #[serde(default)]
    pub recycle_bin_path: String,
}

/// RenamedMovieFile extends [`MovieFile`] with previous paths.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RenamedMovieFile {
    /// The file that was renamed.
    #[serde(flatten)]
    pub movie_file: MovieFile,
    /// Path relative to the movie folder before the rename.
    #[serde(default)]
    pub previous_relative_path: String,
    /// Full path before the rename.
    #[serde(default)]
    pub previous_path: String,
}

/// RadarrRelease is pre-grab release info (Grab / Test).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RadarrRelease {
    /// Quality of the release.
    #[serde(default)]
    pub quality: String,
    /// Revision of the quality.
    #[serde(default)]
    pub quality_version: i32,
    /// Release group that produced the release.
    #[serde(default)]
    pub release_group: String,
    /// Release title.
    #[serde(default)]
    pub release_title: String,
    /// Indexer the release came from.
    #[serde(default)]
    pub indexer: String,
    /// Size of the release in bytes.
    #[serde(default)]
    pub size: i64,
    /// Total score of the matched custom formats.
    #[serde(default)]
    pub custom_format_score: i32,
    /// Custom formats matched by the release.
    #[serde(default)]
    pub custom_formats: Vec<String>,
    /// Languages found in the release.
    #[serde(default)]
    pub languages: Vec<Language>,
    /// Indexer flags set on the release.
    #[serde(default)]
    pub indexer_flags: Vec<String>,
}

/// RadarrGrabbedRelease is post-grab release info (Download / ManualInteraction).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RadarrGrabbedRelease {
    /// Release title.
    #[serde(default)]
    pub release_title: String,
    /// Indexer the release came from.
    #[serde(default)]
    pub indexer: String,
    /// Size of the release in bytes.
    #[serde(default)]
    pub size: i64,
    /// Indexer flags set on the release.
    #[serde(default)]
    pub indexer_flags: Vec<String>,
}

/// RadarrGrab is the Grab (and Test) webhook payload.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RadarrGrab {
    /// The webhook envelope.
    #[serde(flatten)]
    pub base: BaseEvent,
    /// Movie the release belongs to.
    #[serde(default)]
    pub movie: Option<Movie>,
    /// The movie the release was matched to.
    #[serde(default)]
    pub remote_movie: Option<RemoteMovie>,
    /// The grabbed release.
    #[serde(default)]
    pub release: Option<RadarrRelease>,
    /// Download client that grabbed the release.
    #[serde(default)]
    pub download_client: String,
    /// Type of the download client.
    #[serde(default)]
    pub download_client_type: String,
    /// Download client ID for the release.
    #[serde(default)]
    pub download_id: String,
    /// Custom formats matched by the release.
    #[serde(default)]
    pub custom_format_info: Option<CustomFormatInfo>,
}

/// RadarrDownload is the Download webhook payload.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RadarrDownload {
    /// The webhook envelope.
    #[serde(flatten)]
    pub base: BaseEvent,
    /// Movie the file belongs to.
    #[serde(default)]
    pub movie: Option<Movie>,
    /// The movie the release was matched to.
    #[serde(default)]
    pub remote_movie: Option<RemoteMovie>,
    /// The imported file.
    #[serde(default)]
    pub movie_file: Option<MovieFile>,
    /// Whether the import replaced an existing file.
    #[serde(default)]
    pub is_upgrade: bool,
    /// Download client that grabbed the release.
    #[serde(default)]
    pub download_client: String,
    /// Type of the download client.
    #[serde(default)]
    pub download_client_type: String,
    /// Download client ID for the release.
    #[serde(default)]
    pub download_id: String,
    /// Files deleted by the import.
    #[serde(default)]
    pub deleted_files: Vec<MovieFile>,
    /// Custom formats matched by the release.
    #[serde(default)]
    pub custom_format_info: Option<CustomFormatInfo>,
    /// The release the file came from.
    #[serde(default)]
    pub release: Option<RadarrGrabbedRelease>,
}

/// RadarrRename is the Rename webhook payload.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RadarrRename {
    /// The webhook envelope.
    #[serde(flatten)]
    pub base: BaseEvent,
    /// Movie the files belong to.
    #[serde(default)]
    pub movie: Option<Movie>,
    /// The renamed files.
    #[serde(default)]
    pub renamed_movie_files: Vec<RenamedMovieFile>,
}

/// MovieAdded is the MovieAdded webhook payload.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MovieAdded {
    /// The webhook envelope.
    #[serde(flatten)]
    pub base: BaseEvent,
    /// The movie that was added.
    #[serde(default)]
    pub movie: Option<Movie>,
    /// How the movie was added.
    #[serde(default)]
    pub add_method: String,
}

/// MovieDelete is the MovieDelete webhook payload.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MovieDelete {
    /// The webhook envelope.
    #[serde(flatten)]
    pub base: BaseEvent,
    /// The movie that was deleted.
    #[serde(default)]
    pub movie: Option<Movie>,
    /// Whether the files were deleted too.
    #[serde(default)]
    pub deleted_files: bool,
    /// Size of the deleted folder in bytes.
    #[serde(default)]
    pub movie_folder_size: i64,
}

/// MovieFileDelete is the MovieFileDelete webhook payload.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MovieFileDelete {
    /// The webhook envelope.
    #[serde(flatten)]
    pub base: BaseEvent,
    /// Movie the file belonged to.
    #[serde(default)]
    pub movie: Option<Movie>,
    /// The file that was deleted.
    #[serde(default)]
    pub movie_file: Option<MovieFile>,
    /// Why the file was deleted.
    #[serde(default)]
    pub delete_reason: String,
}

/// RadarrHealth is the Health or HealthRestored webhook payload.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RadarrHealth {
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

/// RadarrApplicationUpdate is the ApplicationUpdate webhook payload.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RadarrApplicationUpdate {
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

/// RadarrManualInteraction is the ManualInteractionRequired webhook payload.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RadarrManualInteraction {
    /// The webhook envelope.
    #[serde(flatten)]
    pub base: BaseEvent,
    /// Movie the item belongs to.
    #[serde(default)]
    pub movie: Option<Movie>,
    /// The download client queue item.
    #[serde(default)]
    pub download_info: Option<DownloadClientItem>,
    /// Download client handling the release.
    #[serde(default)]
    pub download_client: String,
    /// Type of the download client.
    #[serde(default)]
    pub download_client_type: String,
    /// Download client ID for the release.
    #[serde(default)]
    pub download_id: String,
    /// Status of the download.
    #[serde(default)]
    pub download_status: String,
    /// Messages attached to the download.
    #[serde(default)]
    pub download_status_messages: Vec<DownloadStatusMessage>,
    /// Custom formats matched by the release.
    #[serde(default)]
    pub custom_format_info: Option<CustomFormatInfo>,
    /// The release the item came from.
    #[serde(default)]
    pub release: Option<RadarrGrabbedRelease>,
}

/// RadarrEvent is a parsed Radarr webhook envelope plus the raw JSON body.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct RadarrEvent {
    /// The decoded envelope.
    pub base: BaseEvent,
    body: Vec<u8>,
}

/// Parses the raw JSON body and returns the envelope.
///
/// Use the `get_*` methods to decode the full payload.
pub fn parse_radarr(body: &[u8]) -> WebhookResult<RadarrEvent> {
    let base: BaseEvent = serde_json::from_slice(body).map_err(|source| WebhookError::Json {
        context: "decoding Radarr event envelope".to_string(),
        source,
    })?;

    Ok(RadarrEvent {
        base,
        body: body.to_vec(),
    })
}

impl RadarrEvent {
    /// Returns the event type from the envelope.
    pub fn event_type(&self) -> &EventType {
        &self.base.event_type
    }

    /// Decodes a Grab or Test payload (Test uses the same shape as Grab).
    pub fn get_grab(&self) -> WebhookResult<RadarrGrab> {
        decode_webhook_payload(
            &self.body,
            &self.base.event_type,
            &[EventType::GRAB, EventType::TEST],
        )
    }

    /// Decodes a Download payload.
    pub fn get_download(&self) -> WebhookResult<RadarrDownload> {
        decode_webhook_payload(&self.body, &self.base.event_type, &[EventType::DOWNLOAD])
    }

    /// Decodes a Rename payload.
    pub fn get_rename(&self) -> WebhookResult<RadarrRename> {
        decode_webhook_payload(&self.body, &self.base.event_type, &[EventType::RENAME])
    }

    /// Decodes a MovieAdded payload.
    pub fn get_movie_added(&self) -> WebhookResult<MovieAdded> {
        decode_webhook_payload(&self.body, &self.base.event_type, &[EventType::MOVIE_ADDED])
    }

    /// Decodes a MovieDelete payload.
    pub fn get_movie_delete(&self) -> WebhookResult<MovieDelete> {
        decode_webhook_payload(&self.body, &self.base.event_type, &[EventType::MOVIE_DELETE])
    }

    /// Decodes a MovieFileDelete payload.
    pub fn get_movie_file_delete(&self) -> WebhookResult<MovieFileDelete> {
        decode_webhook_payload(
            &self.body,
            &self.base.event_type,
            &[EventType::MOVIE_FILE_DELETE],
        )
    }

    /// Decodes a Health payload.
    pub fn get_health(&self) -> WebhookResult<RadarrHealth> {
        decode_webhook_payload(&self.body, &self.base.event_type, &[EventType::HEALTH])
    }

    /// Decodes a HealthRestored payload.
    pub fn get_health_restored(&self) -> WebhookResult<RadarrHealth> {
        decode_webhook_payload(
            &self.body,
            &self.base.event_type,
            &[EventType::HEALTH_RESTORED],
        )
    }

    /// Decodes an ApplicationUpdate payload.
    pub fn get_application_update(&self) -> WebhookResult<RadarrApplicationUpdate> {
        decode_webhook_payload(
            &self.body,
            &self.base.event_type,
            &[EventType::APPLICATION_UPDATE],
        )
    }

    /// Decodes a ManualInteractionRequired payload.
    pub fn get_manual_interaction(&self) -> WebhookResult<RadarrManualInteraction> {
        decode_webhook_payload(
            &self.body,
            &self.base.event_type,
            &[EventType::MANUAL_INTERACTION_REQUIRED],
        )
    }
}
