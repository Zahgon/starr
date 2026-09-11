use super::{
    BaseEvent, CustomFormatInfo, DownloadClientItem, DownloadStatusMessage, EventType, Image,
    Language, WebhookError, WebhookResult, decode_webhook_payload,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Series is series metadata in a Sonarr webhook.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Series {
    /// Series ID.
    #[serde(default)]
    pub id: i64,
    /// Series title.
    #[serde(default)]
    pub title: String,
    /// Slug used in URLs.
    #[serde(default)]
    pub title_slug: String,
    /// Path to the series folder.
    #[serde(default)]
    pub path: String,
    /// TheTVDB ID.
    #[serde(default, rename = "tvdbId")]
    pub tvdb_id: i64,
    /// TVMaze ID.
    #[serde(default, rename = "tvMazeId")]
    pub tv_maze_id: i64,
    /// TMDb ID.
    #[serde(default, rename = "tmdbId")]
    pub tmdb_id: i64,
    /// IMDb ID.
    #[serde(default, rename = "imdbId")]
    pub imdb_id: String,
    /// MyAnimeList IDs.
    #[serde(default, rename = "malIds")]
    pub mal_ids: Vec<i64>,
    /// AniList IDs.
    #[serde(default, rename = "aniListIds")]
    pub ani_list_ids: Vec<i64>,
    /// Type of the series.
    #[serde(default, rename = "type")]
    pub series_type: String,
    /// Year the series first aired.
    #[serde(default)]
    pub year: i32,
    /// Genres this series belongs to.
    #[serde(default)]
    pub genres: Vec<String>,
    /// Series images.
    #[serde(default)]
    pub images: Vec<Image>,
    /// Tags applied to this series.
    #[serde(default)]
    pub tags: Vec<String>,
    /// Original language of the series.
    #[serde(default)]
    pub original_language: Option<Language>,
    /// Country the series came from.
    #[serde(default)]
    pub original_country: String,
}

/// Episode is episode metadata in a Sonarr webhook.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Episode {
    /// Episode ID.
    #[serde(default)]
    pub id: i64,
    /// Number of the episode in its season.
    #[serde(default)]
    pub episode_number: i32,
    /// Season this episode belongs to.
    #[serde(default)]
    pub season_number: i32,
    /// Episode title.
    #[serde(default)]
    pub title: String,
    /// Episode overview.
    #[serde(default)]
    pub overview: String,
    /// When the episode aired, in the network's time zone.
    #[serde(default)]
    pub air_date: String,
    /// When the episode aired, in UTC.
    #[serde(default, rename = "airDateUtc")]
    pub air_date_utc: Option<DateTime<Utc>>,
    /// Series this episode belongs to.
    #[serde(default)]
    pub series_id: i64,
    /// TheTVDB ID.
    #[serde(default, rename = "tvdbId")]
    pub tvdb_id: i64,
    /// Which finale this episode is, when it is one.
    #[serde(default)]
    pub finale_type: String,
}

/// EpisodeFileMediaInfo is optional media info on an episode file.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EpisodeFileMediaInfo {
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

/// EpisodeFile is an on-disk episode file in a Sonarr webhook.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EpisodeFile {
    /// Episode file ID.
    #[serde(default)]
    pub id: i64,
    /// Path relative to the series folder.
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
    pub media_info: Option<EpisodeFileMediaInfo>,
    /// Path the file was imported from.
    #[serde(default)]
    pub source_path: String,
    /// Path in the recycle bin, when the file was deleted.
    #[serde(default)]
    pub recycle_bin_path: String,
}

/// RenamedEpisodeFile extends [`EpisodeFile`] with previous paths.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RenamedEpisodeFile {
    /// The file that was renamed.
    #[serde(flatten)]
    pub episode_file: EpisodeFile,
    /// Path relative to the series folder before the rename.
    #[serde(default)]
    pub previous_relative_path: String,
    /// Full path before the rename.
    #[serde(default)]
    pub previous_path: String,
}

/// SonarrRelease is pre-grab release info (Grab / Test).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SonarrRelease {
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

/// SonarrGrabbedRelease is post-grab release info
/// (Download / ManualInteraction / ImportComplete).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SonarrGrabbedRelease {
    /// Release title.
    #[serde(default)]
    pub release_title: String,
    /// Indexer the release came from.
    #[serde(default)]
    pub indexer: String,
    /// Size of the release in bytes.
    #[serde(default)]
    pub size: Option<i64>,
    /// Indexer flags set on the release.
    #[serde(default)]
    pub indexer_flags: Vec<String>,
    /// Type of the release.
    #[serde(default)]
    pub release_type: String,
}

/// SonarrGrab is the Grab (and Test) webhook payload.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SonarrGrab {
    /// The webhook envelope.
    #[serde(flatten)]
    pub base: BaseEvent,
    /// Series the release belongs to.
    #[serde(default)]
    pub series: Option<Series>,
    /// Episodes the release covers.
    #[serde(default)]
    pub episodes: Vec<Episode>,
    /// The grabbed release.
    #[serde(default)]
    pub release: Option<SonarrRelease>,
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

/// SonarrDownload is the Download webhook for a single imported episode file.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SonarrDownload {
    /// The webhook envelope.
    #[serde(flatten)]
    pub base: BaseEvent,
    /// Series the file belongs to.
    #[serde(default)]
    pub series: Option<Series>,
    /// Episodes the file covers.
    #[serde(default)]
    pub episodes: Vec<Episode>,
    /// The imported file.
    #[serde(default)]
    pub episode_file: Option<EpisodeFile>,
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
    pub deleted_files: Vec<EpisodeFile>,
    /// Custom formats matched by the release.
    #[serde(default)]
    pub custom_format_info: Option<CustomFormatInfo>,
    /// The release the file came from.
    #[serde(default)]
    pub release: Option<SonarrGrabbedRelease>,
}

/// SonarrImportComplete is the Download webhook when a batch import completes
/// (`episodeFiles` set).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SonarrImportComplete {
    /// The webhook envelope.
    #[serde(flatten)]
    pub base: BaseEvent,
    /// Series the files belong to.
    #[serde(default)]
    pub series: Option<Series>,
    /// Episodes the files cover.
    #[serde(default)]
    pub episodes: Vec<Episode>,
    /// The imported files.
    #[serde(default)]
    pub episode_files: Vec<EpisodeFile>,
    /// Download client that grabbed the release.
    #[serde(default)]
    pub download_client: String,
    /// Type of the download client.
    #[serde(default)]
    pub download_client_type: String,
    /// Download client ID for the release.
    #[serde(default)]
    pub download_id: String,
    /// The release the files came from.
    #[serde(default)]
    pub release: Option<SonarrGrabbedRelease>,
    /// Number of files imported.
    #[serde(default)]
    pub file_count: i32,
    /// Path the files were imported from.
    #[serde(default)]
    pub source_path: String,
    /// Path the files were imported to.
    #[serde(default)]
    pub destination_path: String,
}

/// SonarrRename is the Rename webhook payload.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SonarrRename {
    /// The webhook envelope.
    #[serde(flatten)]
    pub base: BaseEvent,
    /// Series the files belong to.
    #[serde(default)]
    pub series: Option<Series>,
    /// The renamed files.
    #[serde(default)]
    pub renamed_episode_files: Vec<RenamedEpisodeFile>,
}

/// SeriesAdd is the SeriesAdd webhook payload.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SeriesAdd {
    /// The webhook envelope.
    #[serde(flatten)]
    pub base: BaseEvent,
    /// The series that was added.
    #[serde(default)]
    pub series: Option<Series>,
}

/// SeriesDelete is the SeriesDelete webhook payload.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SeriesDelete {
    /// The webhook envelope.
    #[serde(flatten)]
    pub base: BaseEvent,
    /// The series that was deleted.
    #[serde(default)]
    pub series: Option<Series>,
    /// Whether the files were deleted too.
    #[serde(default)]
    pub deleted_files: bool,
}

/// EpisodeFileDelete is the EpisodeFileDelete webhook payload.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EpisodeFileDelete {
    /// The webhook envelope.
    #[serde(flatten)]
    pub base: BaseEvent,
    /// Series the file belonged to.
    #[serde(default)]
    pub series: Option<Series>,
    /// Episodes the file covered.
    #[serde(default)]
    pub episodes: Vec<Episode>,
    /// The file that was deleted.
    #[serde(default)]
    pub episode_file: Option<EpisodeFile>,
    /// Why the file was deleted.
    #[serde(default)]
    pub delete_reason: String,
}

/// SonarrHealth is the Health or HealthRestored webhook payload.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SonarrHealth {
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

/// SonarrApplicationUpdate is the ApplicationUpdate webhook payload.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SonarrApplicationUpdate {
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

/// SonarrManualInteraction is the ManualInteractionRequired webhook payload.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SonarrManualInteraction {
    /// The webhook envelope.
    #[serde(flatten)]
    pub base: BaseEvent,
    /// Series the item belongs to.
    #[serde(default)]
    pub series: Option<Series>,
    /// Episodes the item covers.
    #[serde(default)]
    pub episodes: Vec<Episode>,
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
    pub release: Option<SonarrGrabbedRelease>,
}

/// SonarrEvent is a parsed Sonarr webhook envelope plus the raw JSON body.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct SonarrEvent {
    /// The decoded envelope.
    pub base: BaseEvent,
    body: Vec<u8>,
}

/// Parses the raw JSON body and returns the envelope.
///
/// Use the `get_*` methods to decode the full payload.
pub fn parse_sonarr(body: &[u8]) -> WebhookResult<SonarrEvent> {
    let base: BaseEvent = serde_json::from_slice(body).map_err(|source| WebhookError::Json {
        context: "decoding Sonarr event envelope".to_string(),
        source,
    })?;

    Ok(SonarrEvent {
        base,
        body: body.to_vec(),
    })
}

impl SonarrEvent {
    /// Returns the event type from the envelope.
    pub fn event_type(&self) -> &EventType {
        &self.base.event_type
    }

    /// Decodes a Grab or Test payload (Test uses the same shape as Grab).
    pub fn get_grab(&self) -> WebhookResult<SonarrGrab> {
        decode_webhook_payload(
            &self.body,
            &self.base.event_type,
            &[EventType::GRAB, EventType::TEST],
        )
    }

    /// Decodes a single-file Download payload (not an import-complete batch).
    pub fn get_download(&self) -> WebhookResult<SonarrDownload> {
        if self.base.event_type != EventType::DOWNLOAD {
            return Err(WebhookError::WrongEvent {
                got: self.base.event_type.clone(),
                want: vec![EventType::DOWNLOAD],
            });
        }

        if sonarr_is_import_complete_body(&self.body) {
            return Err(WebhookError::WrongEvent {
                got: self.base.event_type.clone(),
                want: vec![EventType::DOWNLOAD],
            });
        }

        decode_webhook_payload(&self.body, &self.base.event_type, &[EventType::DOWNLOAD])
    }

    /// Decodes a batch Download (import complete) payload.
    pub fn get_import_complete(&self) -> WebhookResult<SonarrImportComplete> {
        if self.base.event_type != EventType::DOWNLOAD {
            return Err(WebhookError::WrongEvent {
                got: self.base.event_type.clone(),
                want: vec![EventType::DOWNLOAD],
            });
        }

        if !sonarr_is_import_complete_body(&self.body) {
            return Err(WebhookError::WrongEvent {
                got: self.base.event_type.clone(),
                want: vec![EventType::DOWNLOAD],
            });
        }

        decode_webhook_payload(&self.body, &self.base.event_type, &[EventType::DOWNLOAD])
    }

    /// Decodes a Rename payload.
    pub fn get_rename(&self) -> WebhookResult<SonarrRename> {
        decode_webhook_payload(&self.body, &self.base.event_type, &[EventType::RENAME])
    }

    /// Decodes a SeriesAdd payload.
    pub fn get_series_add(&self) -> WebhookResult<SeriesAdd> {
        decode_webhook_payload(&self.body, &self.base.event_type, &[EventType::SERIES_ADD])
    }

    /// Decodes a SeriesDelete payload.
    pub fn get_series_delete(&self) -> WebhookResult<SeriesDelete> {
        decode_webhook_payload(
            &self.body,
            &self.base.event_type,
            &[EventType::SERIES_DELETE],
        )
    }

    /// Decodes an EpisodeFileDelete payload.
    pub fn get_episode_file_delete(&self) -> WebhookResult<EpisodeFileDelete> {
        decode_webhook_payload(
            &self.body,
            &self.base.event_type,
            &[EventType::EPISODE_FILE_DELETE],
        )
    }

    /// Decodes a Health payload.
    pub fn get_health(&self) -> WebhookResult<SonarrHealth> {
        decode_webhook_payload(&self.body, &self.base.event_type, &[EventType::HEALTH])
    }

    /// Decodes a HealthRestored payload.
    pub fn get_health_restored(&self) -> WebhookResult<SonarrHealth> {
        decode_webhook_payload(
            &self.body,
            &self.base.event_type,
            &[EventType::HEALTH_RESTORED],
        )
    }

    /// Decodes an ApplicationUpdate payload.
    pub fn get_application_update(&self) -> WebhookResult<SonarrApplicationUpdate> {
        decode_webhook_payload(
            &self.body,
            &self.base.event_type,
            &[EventType::APPLICATION_UPDATE],
        )
    }

    /// Decodes a ManualInteractionRequired payload.
    pub fn get_manual_interaction(&self) -> WebhookResult<SonarrManualInteraction> {
        decode_webhook_payload(
            &self.body,
            &self.base.event_type,
            &[EventType::MANUAL_INTERACTION_REQUIRED],
        )
    }
}

/// Reports whether the JSON is the batch import-complete shape (`episodeFiles`).
pub(crate) fn sonarr_is_import_complete_body(body: &[u8]) -> bool {
    let Ok(keys) = serde_json::from_slice::<std::collections::HashMap<String, serde_json::Value>>(
        body,
    ) else {
        return false;
    };

    match keys.get("episodeFiles") {
        Some(serde_json::Value::Null) | None => false,
        Some(serde_json::Value::Array(items)) => !items.is_empty(),
        Some(_) => true,
    }
}
