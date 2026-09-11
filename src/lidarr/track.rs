use super::{Artist, Lidarr, TrackFile};
use crate::error::Result;
use crate::helpers::str_val;
use crate::interface::ApiClientExt;
use crate::req::Request;
use crate::shared::Ratings;
use crate::values::Values;
use serde::{Deserialize, Serialize};

const BP_TRACK: &str = "v1/track";

/// Track is an album track.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Track {
    /// Artist the track belongs to.
    #[serde(default)]
    pub artist_id: i64,
    /// MusicBrainz track ID.
    #[serde(default)]
    pub foreign_track_id: String,
    /// MusicBrainz recording ID.
    #[serde(default)]
    pub foreign_recording_id: String,
    /// Track file backing this track.
    #[serde(default)]
    pub track_file_id: i64,
    /// Album the track belongs to.
    #[serde(default)]
    pub album_id: i64,
    /// Whether the track is explicit.
    #[serde(default)]
    pub explicit: bool,
    /// Track number across all media.
    #[serde(default)]
    pub absolute_track_number: i32,
    /// Track number on its medium.
    #[serde(default)]
    pub track_number: String,
    /// Track title.
    #[serde(default)]
    pub title: String,
    /// Track duration in milliseconds.
    #[serde(default)]
    pub duration: i32,
    /// Medium this track is on.
    #[serde(default)]
    pub medium_number: i32,
    /// Whether a file exists for this track.
    #[serde(default)]
    pub has_file: bool,
    /// Track ratings.
    #[serde(default)]
    pub ratings: Option<Ratings>,
    /// Whether the track was grabbed.
    #[serde(default)]
    pub grabbed: bool,
    /// Track ID.
    #[serde(default)]
    pub id: i64,
    /// The artist; probably empty.
    #[serde(default)]
    pub artist: Option<Artist>,
    /// The track file; probably empty.
    #[serde(default)]
    pub track_file: Option<TrackFile>,
}

impl Lidarr {
    /// Gets tracks by their IDs.
    pub async fn get_tracks(&self, track_ids: &[i64]) -> Result<Vec<Track>> {
        let mut query = Values::new();
        for id in track_ids {
            query.add("trackIds", str_val(*id));
        }

        self.api
            .get_into(Request::new(BP_TRACK).with_query(query))
            .await
    }

    /// Gets track files using an album ID.
    pub async fn get_tracks_by_album(&self, album_id: i64) -> Result<Vec<Track>> {
        let mut query = Values::new();
        query.add("albumId", str_val(album_id));

        self.api
            .get_into(Request::new(BP_TRACK).with_query(query))
            .await
    }

    /// Gets track files using an artist ID.
    pub async fn get_tracks_by_artist(&self, artist_id: i64) -> Result<Vec<Track>> {
        let mut query = Values::new();
        query.add("artistId", str_val(artist_id));

        self.api
            .get_into(Request::new(BP_TRACK).with_query(query))
            .await
    }

    /// Gets track files using an album release ID.
    pub async fn get_tracks_by_album_release(&self, album_release_id: i64) -> Result<Vec<Track>> {
        let mut query = Values::new();
        query.add("albumReleaseId", str_val(album_release_id));

        self.api
            .get_into(Request::new(BP_TRACK).with_query(query))
            .await
    }
}
