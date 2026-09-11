use super::Lidarr;
use crate::error::Result;
use crate::helpers::str_val;
use crate::interface::ApiClientExt;
use crate::req::{Request, path_join};
use crate::shared::{PlayTime, Quality};
use crate::values::Values;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

const BP_TRACK_FILE: &str = "v1/trackfile";

/// TrackFile represents the data sent to and returned from the trackfile endpoint.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrackFile {
    /// Track file ID.
    #[serde(default)]
    pub id: i64,
    /// Artist the file belongs to.
    #[serde(default)]
    pub artist_id: i64,
    /// Album the file belongs to.
    #[serde(default)]
    pub album_id: i64,
    /// Path to the file on disk.
    #[serde(default)]
    pub path: String,
    /// File size in bytes.
    #[serde(default)]
    pub size: i64,
    /// When the file was added.
    #[serde(default)]
    pub date_added: Option<DateTime<Utc>>,
    /// Quality of the file.
    #[serde(default)]
    pub quality: Option<Quality>,
    /// Weight of the quality.
    #[serde(default)]
    pub quality_weight: i32,
    /// Media information read from the file.
    #[serde(default)]
    pub media_info: MediaInfo,
    /// Whether the quality cutoff was not met.
    #[serde(default, rename = "qualityCutoffNotMet")]
    pub cutoff_not_met: bool,
    /// Audio tags read from the file.
    #[serde(default)]
    pub audio_tags: Option<AudioTags>,
}

/// MediaInfo is part of a [`TrackFile`].
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaInfo {
    /// Media info ID.
    #[serde(default)]
    pub id: i64,
    /// Number of audio channels.
    #[serde(default)]
    pub audio_channels: i32,
    /// Audio bit rate.
    #[serde(default)]
    pub audio_bit_rate: String,
    /// Audio codec.
    #[serde(default)]
    pub audio_codec: String,
    /// Audio bit depth.
    #[serde(default)]
    pub audio_bits: String,
    /// Audio sample rate.
    #[serde(default)]
    pub audio_sample_rate: String,
}

/// AudioTags is (optionally) part of a [`TrackFile`].
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AudioTags {
    /// Track title.
    #[serde(default)]
    pub title: String,
    /// Title with special characters removed.
    #[serde(default)]
    pub clean_title: String,
    /// Artist title.
    #[serde(default)]
    pub artist_title: String,
    /// Album title.
    #[serde(default)]
    pub album_title: String,
    /// Parsed artist title.
    #[serde(default)]
    pub artist_title_info: Option<ArtistTitleInfo>,
    /// MusicBrainz artist ID.
    #[serde(default, rename = "artistMBId")]
    pub artist_mbid: String,
    /// MusicBrainz album ID.
    #[serde(default, rename = "albumMBId")]
    pub album_mbid: String,
    /// MusicBrainz release ID.
    #[serde(default, rename = "releaseMBId")]
    pub release_mbid: String,
    /// MusicBrainz recording ID.
    #[serde(default, rename = "recordingMBId")]
    pub recording_mbid: String,
    /// MusicBrainz track ID.
    #[serde(default, rename = "trackMBId")]
    pub track_mbid: String,
    /// Disc this track is on.
    #[serde(default)]
    pub disc_number: i32,
    /// Number of discs in the release.
    #[serde(default)]
    pub disc_count: i32,
    /// Country the release came from.
    #[serde(default)]
    pub country: Option<AudioCountry>,
    /// Release year.
    #[serde(default)]
    pub year: i32,
    /// Record label.
    #[serde(default)]
    pub label: String,
    /// Catalog number.
    #[serde(default)]
    pub catalog_number: String,
    /// Text that distinguishes this release from others.
    #[serde(default)]
    pub disambiguation: String,
    /// Track duration.
    #[serde(default)]
    pub duration: PlayTime,
    /// Quality of the file.
    #[serde(default)]
    pub quality: Option<Quality>,
    /// Media information read from the tags.
    #[serde(default)]
    pub media_info: Option<AudioMediaInfo>,
    /// Track numbers in the release.
    #[serde(default)]
    pub track_numbers: Vec<i32>,
    /// Release group that produced the release.
    #[serde(default)]
    pub release_group: String,
    /// Hash of the release.
    #[serde(default)]
    pub release_hash: String,
}

/// AudioMediaInfo is part of [`AudioTags`].
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AudioMediaInfo {
    /// Audio format.
    #[serde(default)]
    pub audio_format: String,
    /// Audio bitrate.
    #[serde(default)]
    pub audio_bitrate: i64,
    /// Number of audio channels.
    #[serde(default)]
    pub audio_channels: i32,
    /// Audio bit depth.
    #[serde(default)]
    pub audio_bits: i32,
    /// Audio sample rate.
    #[serde(default)]
    pub audio_sample_rate: i32,
}

/// AudioCountry is part of [`AudioTags`].
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AudioCountry {
    /// Two letter country code.
    #[serde(default)]
    pub two_letter_code: String,
    /// Country name.
    #[serde(default)]
    pub name: String,
}

/// ArtistTitleInfo is part of [`AudioTags`].
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArtistTitleInfo {
    /// Artist title.
    #[serde(default)]
    pub title: String,
    /// Artist title with the year removed.
    #[serde(default)]
    pub title_without_year: String,
    /// Year parsed out of the title.
    #[serde(default)]
    pub year: i32,
}

impl Lidarr {
    /// Returns the track files for an artist.
    pub async fn get_track_files_for_artist(&self, artist_id: i64) -> Result<Vec<TrackFile>> {
        let mut query = Values::new();
        query.add("artistId", str_val(artist_id));

        self.api
            .get_into(Request::new(BP_TRACK_FILE).with_query(query))
            .await
    }

    /// Returns the track files for an album.
    pub async fn get_track_files_for_album(&self, album_id: i64) -> Result<Vec<TrackFile>> {
        let mut query = Values::new();
        query.add("albumId", str_val(album_id));

        self.api
            .get_into(Request::new(BP_TRACK_FILE).with_query(query))
            .await
    }

    /// Returns the requested track files by their IDs.
    pub async fn get_track_files(&self, track_file_ids: &[i64]) -> Result<Vec<TrackFile>> {
        if track_file_ids.is_empty() {
            return Ok(Vec::new());
        }

        let mut query = Values::new();
        for id in track_file_ids {
            query.add("trackFileIds", str_val(*id));
        }

        self.api
            .get_into(Request::new(BP_TRACK_FILE).with_query(query))
            .await
    }

    /// Updates a track file.
    pub async fn update_track_file(&self, track_file: &TrackFile) -> Result<TrackFile> {
        let req = Request::new(path_join(&[BP_TRACK_FILE, &str_val(track_file.id)]))
            .with_json(track_file)?;
        self.api.put_into(req).await
    }

    /// Deletes a track file.
    pub async fn delete_track_file(&self, track_file_id: i64) -> Result<()> {
        self.api
            .delete_any(Request::new(path_join(&[
                BP_TRACK_FILE,
                &str_val(track_file_id),
            ])))
            .await
    }

    /// Bulk deletes track files by their IDs.
    pub async fn delete_track_files(&self, track_file_ids: &[i64]) -> Result<()> {
        #[derive(Serialize)]
        struct PostData<'a> {
            #[serde(rename = "trackFileIDs")]
            track_file_ids: &'a [i64],
        }

        let req = Request::new(path_join(&[BP_TRACK_FILE, "bulk"])).with_json(&PostData {
            track_file_ids,
        })?;
        self.api.delete_any(req).await
    }
}
