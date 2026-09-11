use super::{Album, ArtistAddOptions, Lidarr};
use crate::error::{Error, Result};
use crate::helpers::str_val;
use crate::interface::ApiClientExt;
use crate::is_default;
use crate::req::{Request, path_join, set_api_path};
use crate::shared::{ApplyTags, Image, Link, Ratings};
use crate::values::Values;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

const BP_ARTIST: &str = "v1/artist";

/// ArtistEditorInput is the request body for `PUT` and `DELETE /artist/editor`.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArtistEditorInput {
    /// Artists to edit.
    #[serde(default, skip_serializing_if = "is_default")]
    pub artist_ids: Vec<i32>,
    /// Monitored state to apply.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub monitored: Option<bool>,
    /// How new items are monitored.
    #[serde(default, skip_serializing_if = "is_default")]
    pub monitor_new_items: String,
    /// Quality profile to apply.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quality_profile_id: Option<i32>,
    /// Metadata profile to apply.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata_profile_id: Option<i32>,
    /// Root folder to move the artists into.
    #[serde(default, skip_serializing_if = "is_default")]
    pub root_folder_path: String,
    /// Tags to apply.
    #[serde(default, skip_serializing_if = "is_default")]
    pub tags: Vec<i32>,
    /// How the tags are applied.
    #[serde(default, skip_serializing_if = "is_default")]
    pub apply_tags: ApplyTags,
    /// Whether files move with the artist.
    #[serde(default, skip_serializing_if = "is_default")]
    pub move_files: bool,
    /// Whether files are deleted on delete.
    #[serde(default, skip_serializing_if = "is_default")]
    pub delete_files: bool,
    /// Whether deleted artists are added to the import list exclusions.
    #[serde(default, skip_serializing_if = "is_default")]
    pub add_import_list_exclusion: bool,
}

/// Artist represents the `/api/v1/artist` endpoint, and it's part of an [`Album`].
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Artist {
    /// Artist ID.
    #[serde(default)]
    pub id: i64,
    /// Artist status.
    #[serde(default, skip_serializing_if = "is_default")]
    pub status: String,
    /// When metadata was last synced.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_info_sync: Option<DateTime<Utc>>,
    /// Artist name.
    #[serde(default, skip_serializing_if = "is_default")]
    pub artist_name: String,
    /// MusicBrainz artist ID.
    #[serde(default, skip_serializing_if = "is_default")]
    pub foreign_artist_id: String,
    /// TheAudioDB ID.
    #[serde(default, skip_serializing_if = "is_default")]
    pub tadb_id: i64,
    /// Discogs ID.
    #[serde(default, skip_serializing_if = "is_default")]
    pub discogs_id: i64,
    /// Quality profile applied to this artist.
    #[serde(default, skip_serializing_if = "is_default")]
    pub quality_profile_id: i64,
    /// Metadata profile applied to this artist.
    #[serde(default, skip_serializing_if = "is_default")]
    pub metadata_profile_id: i64,
    /// Artist overview.
    #[serde(default, skip_serializing_if = "is_default")]
    pub overview: String,
    /// Type of artist, like `Person` or `Group`.
    #[serde(default, skip_serializing_if = "is_default")]
    pub artist_type: String,
    /// Text that distinguishes this artist from others with the same name.
    #[serde(default, skip_serializing_if = "is_default")]
    pub disambiguation: String,
    /// Root folder the artist lives under.
    #[serde(default, skip_serializing_if = "is_default")]
    pub root_folder_path: String,
    /// Full path to the artist folder.
    #[serde(default, skip_serializing_if = "is_default")]
    pub path: String,
    /// Name with special characters removed.
    #[serde(default, skip_serializing_if = "is_default")]
    pub clean_name: String,
    /// Name used for sorting.
    #[serde(default, skip_serializing_if = "is_default")]
    pub sort_name: String,
    /// External links for this artist.
    #[serde(default, skip_serializing_if = "is_default")]
    pub links: Vec<Link>,
    /// Artist images.
    #[serde(default, skip_serializing_if = "is_default")]
    pub images: Vec<Image>,
    /// Genres this artist belongs to.
    #[serde(default, skip_serializing_if = "is_default")]
    pub genres: Vec<String>,
    /// Tags applied to this artist.
    #[serde(default, skip_serializing_if = "is_default")]
    pub tags: Vec<i32>,
    /// When the artist was added.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub added: Option<DateTime<Utc>>,
    /// Artist ratings.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ratings: Option<Ratings>,
    /// Library statistics for this artist.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub statistics: Option<Statistics>,
    /// The most recent album.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_album: Option<Box<Album>>,
    /// The next upcoming album.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_album: Option<Box<Album>>,
    /// Options applied when adding the artist.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub add_options: Option<ArtistAddOptions>,
    /// Whether albums get their own folders.
    #[serde(default, skip_serializing_if = "is_default")]
    pub album_folder: bool,
    /// Whether the artist is monitored.
    #[serde(default)]
    pub monitored: bool,
    /// Whether the artist is no longer active.
    #[serde(default, skip_serializing_if = "is_default")]
    pub ended: bool,
}

/// Statistics is part of [`Artist`] and [`Album`].
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Statistics {
    /// Number of albums.
    #[serde(default, skip_serializing_if = "is_default")]
    pub album_count: i32,
    /// Number of track files on disk.
    #[serde(default)]
    pub track_file_count: i32,
    /// Number of tracks.
    #[serde(default)]
    pub track_count: i32,
    /// Total number of tracks.
    #[serde(default)]
    pub total_track_count: i32,
    /// Bytes used on disk.
    #[serde(default)]
    pub size_on_disk: i32,
    /// Percentage of tracks present on disk.
    #[serde(default)]
    pub percent_of_tracks: f64,
}

impl Lidarr {
    /// Returns an artist, or all artists when `mb_id` is empty.
    pub async fn get_artist(&self, mb_id: &str) -> Result<Vec<Artist>> {
        let mut query = Values::new();
        if !mb_id.is_empty() {
            query.add("mbId", mb_id);
        }

        self.api
            .get_into(Request::new(BP_ARTIST).with_query(query))
            .await
    }

    /// Returns an artist from an ID.
    pub async fn get_artist_by_id(&self, artist_id: i64) -> Result<Artist> {
        self.api
            .get_into(Request::new(path_join(&[BP_ARTIST, &str_val(artist_id)])))
            .await
    }

    /// Adds a new artist to Lidarr, and probably does not yet work.
    pub async fn add_artist(&self, artist: &Artist) -> Result<Artist> {
        self.api
            .post_into(Request::new(BP_ARTIST).with_json(artist)?)
            .await
    }

    /// Updates an artist in place.
    pub async fn update_artist(&self, artist: &Artist, move_files: bool) -> Result<Artist> {
        let mut query = Values::new();
        query.add("moveFiles", str_val(move_files));

        let req = Request::new(path_join(&[BP_ARTIST, &str_val(artist.id)]))
            .with_json(artist)?
            .with_query(query);
        self.api.put_into(req).await
    }

    /// Removes an artist from the database.
    ///
    /// Setting `delete_files` true will delete all content for the artist.
    pub async fn delete_artist(
        &self,
        artist_id: i64,
        delete_files: bool,
        add_import_exclusion: bool,
    ) -> Result<()> {
        let mut query = Values::new();
        query.set("deleteFiles", str_val(delete_files));
        query.set("addImportListExclusion", str_val(add_import_exclusion));

        self.api
            .delete_any(
                Request::new(path_join(&[BP_ARTIST, &str_val(artist_id)])).with_query(query),
            )
            .await
    }

    /// Searches for artists matching the specified search term.
    pub async fn lookup_artist(&self, term: &str) -> Result<Vec<Artist>> {
        if term.is_empty() {
            return Ok(Vec::new());
        }

        let mut query = Values::new();
        query.set("term", term);

        self.api
            .get_into(Request::new(path_join(&[BP_ARTIST, "lookup"])).with_query(query))
            .await
    }

    /// Applies bulk edits to artists.
    pub async fn edit_artists(&self, input: &ArtistEditorInput) -> Result<Vec<Artist>> {
        let req = Request::new(path_join(&[BP_ARTIST, "editor"])).with_json(input)?;
        self.api.put_into(req).await
    }

    /// Applies bulk deletes to artists.
    ///
    /// This uses a `DELETE` with a request body, so it bypasses the usual
    /// decode path and reads the response directly.
    pub async fn delete_artists(&self, input: &ArtistEditorInput) -> Result<Vec<Artist>> {
        let uri = set_api_path(&path_join(&[BP_ARTIST, "editor"]));
        let req = Request::new(uri.clone()).with_json(input)?;

        let resp = self.api.delete(req).await?;
        let body = resp.bytes().await.map_err(|source| Error::Http {
            context: format!("decoding response from {uri}"),
            source,
        })?;

        serde_json::from_slice(&body).map_err(|source| Error::Json {
            context: format!("decoding response from {uri}"),
            source,
        })
    }
}
