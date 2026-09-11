use super::{BaseEvent, EventType, Image, WebhookError, WebhookResult, decode_webhook_payload};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Artist is artist metadata in a Lidarr webhook.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Artist {
    /// Artist ID.
    #[serde(default)]
    pub id: i64,
    /// Artist name.
    #[serde(default)]
    pub name: String,
    /// Text that distinguishes this artist from others with the same name.
    #[serde(default)]
    pub disambiguation: String,
    /// Path to the artist folder.
    #[serde(default)]
    pub path: String,
    /// MusicBrainz ID.
    #[serde(default, rename = "mbId")]
    pub mbid: String,
    /// Type of artist.
    #[serde(default, rename = "type")]
    pub artist_type: String,
    /// Artist overview.
    #[serde(default)]
    pub overview: String,
    /// Genres this artist belongs to.
    #[serde(default)]
    pub genres: Vec<String>,
    /// Artist images.
    #[serde(default)]
    pub images: Vec<Image>,
    /// Tags applied to this artist.
    #[serde(default)]
    pub tags: Vec<String>,
}

/// Album is album metadata in a Lidarr webhook.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Album {
    /// Album ID.
    #[serde(default)]
    pub id: i64,
    /// MusicBrainz ID.
    #[serde(default, rename = "mbId")]
    pub mbid: String,
    /// Album title.
    #[serde(default)]
    pub title: String,
    /// Text that distinguishes this album from others with the same title.
    #[serde(default)]
    pub disambiguation: String,
    /// Album overview.
    #[serde(default)]
    pub overview: String,
    /// Type of the album.
    #[serde(default)]
    pub album_type: String,
    /// Secondary types of the album.
    #[serde(default)]
    pub secondary_album_types: Vec<String>,
    /// When the album was released.
    #[serde(default)]
    pub release_date: Option<DateTime<Utc>>,
    /// Genres this album belongs to.
    #[serde(default)]
    pub genres: Vec<String>,
    /// Album images.
    #[serde(default)]
    pub images: Vec<Image>,
}

/// Track is track metadata in a Lidarr webhook.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Track {
    /// Track ID.
    #[serde(default)]
    pub id: i32,
    /// Track title.
    #[serde(default)]
    pub title: String,
    /// Track number.
    #[serde(default)]
    pub track_number: String,
    /// Quality of the track.
    #[serde(default)]
    pub quality: String,
    /// Revision of the quality.
    #[serde(default)]
    pub quality_version: i32,
    /// Release group that produced the release.
    #[serde(default)]
    pub release_group: String,
}

/// TrackFile is an on-disk track file in a Lidarr webhook.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrackFile {
    /// Track file ID.
    #[serde(default)]
    pub id: i64,
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
}

/// RenamedTrackFile extends [`TrackFile`] with the previous path.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RenamedTrackFile {
    /// The file that was renamed.
    #[serde(flatten)]
    pub track_file: TrackFile,
    /// Full path before the rename.
    #[serde(default)]
    pub previous_path: String,
}

/// LidarrRelease is release info in a Lidarr webhook.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LidarrRelease {
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
}

/// LidarrGrab is the Grab (and Test) webhook payload.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LidarrGrab {
    /// The webhook envelope.
    #[serde(flatten)]
    pub base: BaseEvent,
    /// Artist the release belongs to.
    #[serde(default)]
    pub artist: Option<Artist>,
    /// Albums the release covers.
    #[serde(default)]
    pub albums: Vec<Album>,
    /// The grabbed release.
    #[serde(default)]
    pub release: Option<LidarrRelease>,
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

/// LidarrDownload is the Download webhook payload.
///
/// Lidarr sends eventType `ImportFailure` with this same shape, but with a
/// null album; see [`LidarrImportFailure`].
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LidarrDownload {
    /// The webhook envelope.
    #[serde(flatten)]
    pub base: BaseEvent,
    /// Artist the files belong to.
    #[serde(default)]
    pub artist: Option<Artist>,
    /// Album the files belong to.
    #[serde(default)]
    pub album: Option<Album>,
    /// Tracks the files cover.
    #[serde(default)]
    pub tracks: Vec<Track>,
    /// The imported files.
    #[serde(default)]
    pub track_files: Vec<TrackFile>,
    /// Files deleted by the import.
    #[serde(default)]
    pub deleted_files: Vec<TrackFile>,
    /// Whether the import replaced existing files.
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
}

/// LidarrImportFailure is the ImportFailure webhook payload.
pub type LidarrImportFailure = LidarrDownload;

/// LidarrDownloadFailure is the DownloadFailure webhook payload.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LidarrDownloadFailure {
    /// The webhook envelope.
    #[serde(flatten)]
    pub base: BaseEvent,
    /// Quality of the release.
    #[serde(default)]
    pub quality: String,
    /// Revision of the quality.
    #[serde(default)]
    pub quality_version: i32,
    /// Release title.
    #[serde(default)]
    pub release_title: String,
    /// Download client that grabbed the release.
    #[serde(default)]
    pub download_client: String,
    /// Download client ID for the release.
    #[serde(default)]
    pub download_id: String,
}

/// LidarrRename is the Rename webhook payload.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LidarrRename {
    /// The webhook envelope.
    #[serde(flatten)]
    pub base: BaseEvent,
    /// Artist the files belong to.
    #[serde(default)]
    pub artist: Option<Artist>,
    /// The renamed files.
    #[serde(default)]
    pub renamed_track_files: Vec<RenamedTrackFile>,
}

/// LidarrRetag is the Retag webhook payload.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LidarrRetag {
    /// The webhook envelope.
    #[serde(flatten)]
    pub base: BaseEvent,
    /// Artist the file belongs to.
    #[serde(default)]
    pub artist: Option<Artist>,
    /// The retagged file.
    #[serde(default)]
    pub track_file: Option<TrackFile>,
}

/// ArtistAdd is the ArtistAdd webhook payload.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArtistAdd {
    /// The webhook envelope.
    #[serde(flatten)]
    pub base: BaseEvent,
    /// The artist that was added.
    #[serde(default)]
    pub artist: Option<Artist>,
}

/// ArtistDelete is the ArtistDelete webhook payload.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArtistDelete {
    /// The webhook envelope.
    #[serde(flatten)]
    pub base: BaseEvent,
    /// The artist that was deleted.
    #[serde(default)]
    pub artist: Option<Artist>,
    /// Whether the files were deleted too.
    #[serde(default)]
    pub deleted_files: bool,
}

/// AlbumDelete is the AlbumDelete webhook payload.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlbumDelete {
    /// The webhook envelope.
    #[serde(flatten)]
    pub base: BaseEvent,
    /// Artist the album belonged to.
    #[serde(default)]
    pub artist: Option<Artist>,
    /// The album that was deleted.
    #[serde(default)]
    pub album: Option<Album>,
    /// Whether the files were deleted too.
    #[serde(default)]
    pub deleted_files: bool,
}

/// LidarrHealth is the Health or HealthRestored webhook payload.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LidarrHealth {
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

/// LidarrApplicationUpdate is the ApplicationUpdate webhook payload.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LidarrApplicationUpdate {
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

/// LidarrEvent is a parsed Lidarr webhook envelope plus the raw JSON body.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct LidarrEvent {
    /// The decoded envelope.
    pub base: BaseEvent,
    body: Vec<u8>,
}

/// Parses the raw JSON body and returns the envelope.
///
/// Use the `get_*` methods to decode the full payload.
pub fn parse_lidarr(body: &[u8]) -> WebhookResult<LidarrEvent> {
    let base: BaseEvent = serde_json::from_slice(body).map_err(|source| WebhookError::Json {
        context: "decoding Lidarr event envelope".to_string(),
        source,
    })?;

    Ok(LidarrEvent {
        base,
        body: body.to_vec(),
    })
}

impl LidarrEvent {
    /// Returns the event type from the envelope.
    pub fn event_type(&self) -> &EventType {
        &self.base.event_type
    }

    /// Decodes a Grab or Test payload (Test uses the same shape as Grab).
    pub fn get_grab(&self) -> WebhookResult<LidarrGrab> {
        decode_webhook_payload(
            &self.body,
            &self.base.event_type,
            &[EventType::GRAB, EventType::TEST],
        )
    }

    /// Decodes a Download payload.
    pub fn get_download(&self) -> WebhookResult<LidarrDownload> {
        decode_webhook_payload(&self.body, &self.base.event_type, &[EventType::DOWNLOAD])
    }

    /// Decodes an ImportFailure payload, which has the Download shape.
    pub fn get_import_failure(&self) -> WebhookResult<LidarrImportFailure> {
        decode_webhook_payload(
            &self.body,
            &self.base.event_type,
            &[EventType::IMPORT_FAILURE],
        )
    }

    /// Decodes a DownloadFailure payload.
    pub fn get_download_failure(&self) -> WebhookResult<LidarrDownloadFailure> {
        decode_webhook_payload(
            &self.body,
            &self.base.event_type,
            &[EventType::DOWNLOAD_FAILURE],
        )
    }

    /// Decodes a Rename payload.
    pub fn get_rename(&self) -> WebhookResult<LidarrRename> {
        decode_webhook_payload(&self.body, &self.base.event_type, &[EventType::RENAME])
    }

    /// Decodes a Retag payload.
    pub fn get_retag(&self) -> WebhookResult<LidarrRetag> {
        decode_webhook_payload(&self.body, &self.base.event_type, &[EventType::RETAG])
    }

    /// Decodes an ArtistAdd payload.
    pub fn get_artist_add(&self) -> WebhookResult<ArtistAdd> {
        decode_webhook_payload(&self.body, &self.base.event_type, &[EventType::ARTIST_ADD])
    }

    /// Decodes an ArtistDelete payload.
    pub fn get_artist_delete(&self) -> WebhookResult<ArtistDelete> {
        decode_webhook_payload(
            &self.body,
            &self.base.event_type,
            &[EventType::ARTIST_DELETE],
        )
    }

    /// Decodes an AlbumDelete payload.
    pub fn get_album_delete(&self) -> WebhookResult<AlbumDelete> {
        decode_webhook_payload(&self.body, &self.base.event_type, &[EventType::ALBUM_DELETE])
    }

    /// Decodes a Health payload.
    pub fn get_health(&self) -> WebhookResult<LidarrHealth> {
        decode_webhook_payload(&self.body, &self.base.event_type, &[EventType::HEALTH])
    }

    /// Decodes a HealthRestored payload.
    pub fn get_health_restored(&self) -> WebhookResult<LidarrHealth> {
        decode_webhook_payload(
            &self.body,
            &self.base.event_type,
            &[EventType::HEALTH_RESTORED],
        )
    }

    /// Decodes an ApplicationUpdate payload.
    pub fn get_application_update(&self) -> WebhookResult<LidarrApplicationUpdate> {
        decode_webhook_payload(
            &self.body,
            &self.base.event_type,
            &[EventType::APPLICATION_UPDATE],
        )
    }
}
