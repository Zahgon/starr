use super::{Artist, Lidarr, Statistics};
use crate::error::Result;
use crate::helpers::str_val;
use crate::interface::ApiClientExt;
use crate::is_default;
use crate::req::{Request, path_join};
use crate::shared::{Image, Link, Ratings};
use crate::values::Values;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

const BP_ALBUM: &str = "v1/album";
const BP_ALBUM_STUDIO: &str = "v1/albumstudio";

/// Album is the `/api/v1/album` endpoint.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Album {
    /// Album ID.
    #[serde(default, skip_serializing_if = "is_default")]
    pub id: i64,
    /// Album title.
    #[serde(default)]
    pub title: String,
    /// Text that distinguishes this album from others with the same title.
    #[serde(default)]
    pub disambiguation: String,
    /// Album overview.
    #[serde(default)]
    pub overview: String,
    /// ID of the artist that released this album.
    #[serde(default)]
    pub artist_id: i64,
    /// MusicBrainz release-group ID.
    #[serde(default)]
    pub foreign_album_id: String,
    /// Quality profile applied to this album.
    #[serde(default)]
    pub profile_id: i64,
    /// Album duration in milliseconds.
    #[serde(default)]
    pub duration: i32,
    /// Primary album type.
    #[serde(default)]
    pub album_type: String,
    /// Secondary album types.
    #[serde(default)]
    pub secondary_types: Vec<serde_json::Value>,
    /// Number of media in the album.
    #[serde(default)]
    pub medium_count: i32,
    /// Album ratings.
    #[serde(default)]
    pub ratings: Option<Ratings>,
    /// When the album was released.
    #[serde(default)]
    pub release_date: Option<DateTime<Utc>>,
    /// Known releases of this album.
    #[serde(default)]
    pub releases: Vec<Release>,
    /// Genres this album belongs to.
    #[serde(default)]
    pub genres: Vec<String>,
    /// Media that make up this album.
    #[serde(default)]
    pub media: Vec<Media>,
    /// The artist that released this album.
    #[serde(default)]
    pub artist: Option<Box<Artist>>,
    /// External links for this album.
    #[serde(default)]
    pub links: Vec<Link>,
    /// Album images.
    #[serde(default)]
    pub images: Vec<Image>,
    /// Library statistics for this album.
    #[serde(default)]
    pub statistics: Option<Statistics>,
    /// Remote cover art URL.
    #[serde(default, skip_serializing_if = "is_default")]
    pub remote_cover: String,
    /// Options applied when adding the album.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub add_options: Option<AlbumAddOptions>,
    /// Whether the album is monitored.
    #[serde(default)]
    pub monitored: bool,
    /// Whether any release of this album is acceptable.
    #[serde(default)]
    pub any_release_ok: bool,
    /// Whether the album has been grabbed.
    #[serde(default)]
    pub grabbed: bool,
}

/// Release is part of an [`Album`].
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Release {
    /// Release ID.
    #[serde(default)]
    pub id: i64,
    /// ID of the album this release belongs to.
    #[serde(default)]
    pub album_id: i64,
    /// MusicBrainz release ID.
    #[serde(default)]
    pub foreign_release_id: String,
    /// Release title.
    #[serde(default)]
    pub title: String,
    /// Release status.
    #[serde(default)]
    pub status: String,
    /// Release duration in milliseconds.
    #[serde(default)]
    pub duration: i32,
    /// Number of tracks in the release.
    #[serde(default)]
    pub track_count: i32,
    /// Media that make up this release.
    #[serde(default)]
    pub media: Vec<Media>,
    /// Number of media in the release.
    #[serde(default)]
    pub medium_count: i32,
    /// Text that distinguishes this release from others.
    #[serde(default)]
    pub disambiguation: String,
    /// Countries the release was published in.
    #[serde(default)]
    pub country: Vec<String>,
    /// Labels that published the release.
    #[serde(default)]
    pub label: Vec<String>,
    /// Release format.
    #[serde(default)]
    pub format: String,
    /// Whether the release is monitored.
    #[serde(default)]
    pub monitored: bool,
}

/// Media is part of an [`Album`].
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Media {
    /// Medium number within the release.
    #[serde(default)]
    pub medium_number: i64,
    /// Medium name.
    #[serde(default)]
    pub medium_name: String,
    /// Medium format, like `CD` or `Digital Media`.
    #[serde(default)]
    pub medium_format: String,
}

/// ArtistAddOptions is part of an [`Artist`] and an [`Album`].
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArtistAddOptions {
    /// Which albums to monitor when adding.
    #[serde(default, skip_serializing_if = "is_default")]
    pub monitor: String,
    /// Whether the artist is monitored after adding.
    #[serde(default, skip_serializing_if = "is_default")]
    pub monitored: bool,
    /// Whether to search for missing albums after adding.
    #[serde(default, skip_serializing_if = "is_default")]
    pub search_for_missing_albums: bool,
}

/// AddAlbumInput is currently unknown.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddAlbumInput {
    /// MusicBrainz release-group ID.
    #[serde(default)]
    pub foreign_album_id: String,
    /// Whether the album is monitored.
    #[serde(default)]
    pub monitored: bool,
    /// Releases to add with the album.
    #[serde(default)]
    pub releases: Vec<AddAlbumInputRelease>,
    /// Options applied when adding the album.
    #[serde(default)]
    pub add_options: Option<AlbumAddOptions>,
    /// The artist that released this album.
    #[serde(default)]
    pub artist: Option<Artist>,
}

/// AddAlbumInputRelease is part of [`AddAlbumInput`].
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddAlbumInputRelease {
    /// MusicBrainz release ID.
    #[serde(default)]
    pub foreign_release_id: String,
    /// Release title.
    #[serde(default)]
    pub title: String,
    /// Media that make up this release.
    #[serde(default)]
    pub media: Vec<Media>,
    /// Whether the release is monitored.
    #[serde(default)]
    pub monitored: bool,
}

/// AlbumAddOptions is part of an [`Album`].
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlbumAddOptions {
    /// Whether to search for the new album after adding.
    #[serde(default, skip_serializing_if = "is_default")]
    pub search_for_new_album: bool,
}

/// AlbumsMonitoredInput is the body for `PUT /album/monitor`.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlbumsMonitoredInput {
    /// Albums to change.
    #[serde(default)]
    pub album_ids: Vec<i64>,
    /// Monitored state to apply.
    #[serde(default)]
    pub monitored: bool,
}

/// MonitoringOptions configures album studio monitoring.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MonitoringOptions {
    /// Which albums to monitor.
    #[serde(default, skip_serializing_if = "is_default")]
    pub monitor: String,
    /// Specific albums to monitor.
    #[serde(default, skip_serializing_if = "is_default")]
    pub albums_to_monitor: Vec<String>,
    /// Monitored state to apply.
    #[serde(default, skip_serializing_if = "is_default")]
    pub monitored: bool,
}

/// AlbumStudioArtist is one artist block for album studio.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlbumStudioArtist {
    /// Artist ID.
    #[serde(default)]
    pub id: i64,
    /// Monitored state to apply.
    #[serde(default, skip_serializing_if = "is_default")]
    pub monitored: bool,
    /// Albums to change for this artist.
    #[serde(default, skip_serializing_if = "is_default")]
    pub albums: Vec<Album>,
}

/// AlbumStudioInput is the body for `POST /albumstudio`.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlbumStudioInput {
    /// Artists to change.
    #[serde(default, skip_serializing_if = "is_default")]
    pub artist: Vec<AlbumStudioArtist>,
    /// Monitoring options to apply.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub monitoring_options: Option<MonitoringOptions>,
    /// How new items are monitored.
    #[serde(default, skip_serializing_if = "is_default")]
    pub monitor_new_items: String,
}

impl Lidarr {
    /// Returns an album, or all albums when `mb_id` is empty.
    ///
    /// `mb_id` is the MusicBrainz UUID for a release-group.
    pub async fn get_album(&self, mb_id: &str) -> Result<Vec<Album>> {
        let mut query = Values::new();
        if !mb_id.is_empty() {
            query.add("ForeignAlbumId", mb_id);
        }

        self.api
            .get_into(Request::new(BP_ALBUM).with_query(query))
            .await
    }

    /// Returns an album by database ID.
    pub async fn get_album_by_id(&self, album_id: i64) -> Result<Album> {
        self.api
            .get_into(Request::new(path_join(&[BP_ALBUM, &str_val(album_id)])))
            .await
    }

    /// Updates an album in place; the output of this is currently unknown.
    pub async fn update_album(
        &self,
        album_id: i64,
        album: &Album,
        move_files: bool,
    ) -> Result<Album> {
        let mut query = Values::new();
        query.add("moveFiles", str_val(move_files));

        let req = Request::new(path_join(&[BP_ALBUM, &str_val(album_id)]))
            .with_json(album)?
            .with_query(query);
        self.api.put_into(req).await
    }

    /// Adds a new album to Lidarr, and probably does not yet work.
    pub async fn add_album(&self, album: &AddAlbumInput) -> Result<Album> {
        self.api
            .post_into(Request::new(BP_ALBUM).with_json(album)?)
            .await
    }

    /// Searches for albums matching the specified search term.
    pub async fn lookup(&self, term: &str) -> Result<Vec<Album>> {
        if term.is_empty() {
            return Ok(Vec::new());
        }

        let mut query = Values::new();
        query.set("term", term);

        self.api
            .get_into(Request::new(path_join(&[BP_ALBUM, "lookup"])).with_query(query))
            .await
    }

    /// Removes an album from the database.
    ///
    /// Setting `delete_files` true will delete all content for the album.
    pub async fn delete_album(
        &self,
        album_id: i64,
        delete_files: bool,
        add_import_exclusion: bool,
    ) -> Result<()> {
        let mut query = Values::new();
        query.set("deleteFiles", str_val(delete_files));
        query.set("addImportListExclusion", str_val(add_import_exclusion));

        self.api
            .delete_any(Request::new(path_join(&[BP_ALBUM, &str_val(album_id)])).with_query(query))
            .await
    }

    /// Sets monitored state for the given album IDs.
    pub async fn monitor_albums(&self, album_ids: &[i64], monitored: bool) -> Result<Vec<Album>> {
        let input = AlbumsMonitoredInput {
            album_ids: album_ids.to_vec(),
            monitored,
        };

        let req = Request::new(path_join(&[BP_ALBUM, "monitor"])).with_json(&input)?;
        self.api.put_into(req).await
    }

    /// Triggers album studio actions (monitoring) for artists and albums.
    pub async fn album_studio(&self, input: &AlbumStudioInput) -> Result<()> {
        self.api
            .post_any(Request::new(BP_ALBUM_STUDIO).with_json(input)?)
            .await
    }
}
