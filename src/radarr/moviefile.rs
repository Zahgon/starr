use super::{CustomFormatOutput, Radarr};
use crate::error::Result;
use crate::helpers::str_val;
use crate::interface::ApiClientExt;
use crate::req::{Request, path_join};
use crate::shared::{Quality, Value};
use crate::values::Values;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

const BP_MOVIE_FILE: &str = "v3/moviefile";

/// MovieFile is part of a [`super::Movie`].
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MovieFile {
    /// Movie file ID.
    #[serde(default)]
    pub id: i64,
    /// Movie the file belongs to.
    #[serde(default)]
    pub movie_id: i64,
    /// Path relative to the movie folder.
    #[serde(default)]
    pub relative_path: String,
    /// Full path to the file.
    #[serde(default)]
    pub path: String,
    /// File size in bytes.
    #[serde(default)]
    pub size: i64,
    /// When the file was added.
    #[serde(default)]
    pub date_added: Option<DateTime<Utc>>,
    /// Scene name of the release.
    #[serde(default)]
    pub scene_name: String,
    /// Indexer flags set on the release.
    #[serde(default)]
    pub indexer_flags: i64,
    /// Quality of the file.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quality: Option<Quality>,
    /// Custom formats matched by the file.
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub custom_formats: Vec<CustomFormatOutput>,
    /// Total score of the matched custom formats.
    #[serde(default)]
    pub custom_format_score: i32,
    /// Media information read from the file.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub media_info: Option<MediaInfo>,
    /// Path the file was imported from.
    #[serde(default)]
    pub original_file_path: String,
    /// Whether the quality cutoff was not met.
    #[serde(default)]
    pub quality_cutoff_not_met: bool,
    /// Languages present in the file.
    #[serde(default)]
    pub languages: Vec<Value>,
    /// Release group that produced the release.
    #[serde(default)]
    pub release_group: String,
    /// Edition of the movie.
    #[serde(default)]
    pub edition: String,
}

/// MediaInfo is part of a [`MovieFile`].
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaInfo {
    /// Media info ID.
    #[serde(default)]
    pub id: i64,
    /// Audio bitrate.
    #[serde(default)]
    pub audio_bitrate: i32,
    /// Number of audio channels.
    #[serde(default)]
    pub audio_channels: f64,
    /// Audio codec.
    #[serde(default)]
    pub audio_codec: String,
    /// Audio languages.
    #[serde(default)]
    pub audio_languages: String,
    /// Number of audio streams.
    #[serde(default)]
    pub audio_stream_count: i32,
    /// Video bit depth.
    #[serde(default)]
    pub video_bit_depth: i32,
    /// Video bitrate.
    #[serde(default)]
    pub video_bitrate: i32,
    /// Video codec.
    #[serde(default)]
    pub video_codec: String,
    /// Video dynamic range type.
    #[serde(default)]
    pub video_dynamic_range_type: String,
    /// Video frames per second.
    #[serde(default)]
    pub video_fps: f64,
    /// Video resolution.
    #[serde(default)]
    pub resolution: String,
    /// Run time of the file.
    #[serde(default)]
    pub run_time: String,
    /// Scan type of the video.
    #[serde(default)]
    pub scan_type: String,
    /// Subtitles present in the file.
    #[serde(default)]
    pub subtitles: String,
}

impl Radarr {
    /// Returns the movie file(s) for a movie.
    pub async fn get_movie_file(&self, movie_id: i64) -> Result<Vec<MovieFile>> {
        let mut query = Values::new();
        query.add("movieID", str_val(movie_id));

        self.api
            .get_into(Request::new(BP_MOVIE_FILE).with_query(query))
            .await
    }

    /// Grabs a movie file from the database by DB [movieFile] ID.
    pub async fn get_movie_file_by_id(&self, movie_file_id: i64) -> Result<MovieFile> {
        self.api
            .get_into(Request::new(path_join(&[
                BP_MOVIE_FILE,
                &str_val(movie_file_id),
            ])))
            .await
    }

    /// Returns the movie file(s) requested.
    pub async fn get_movie_files(&self, movie_file_ids: &[i64]) -> Result<Vec<MovieFile>> {
        let mut query = Values::new();
        for id in movie_file_ids {
            query.add("movieFileIds", str_val(*id));
        }

        self.api
            .get_into(Request::new(BP_MOVIE_FILE).with_query(query))
            .await
    }

    /// Updates the movie file provided.
    pub async fn update_movie_file(&self, movie_file: &MovieFile) -> Result<MovieFile> {
        let req = Request::new(path_join(&[BP_MOVIE_FILE, &str_val(movie_file.id)]))
            .with_json(movie_file)?;
        self.api.put_into(req).await
    }

    /// Deletes movie files by their IDs.
    pub async fn delete_movie_files(&self, movie_file_ids: &[i64]) -> Result<()> {
        #[derive(Serialize)]
        struct PostData<'a> {
            #[serde(rename = "movieFileIds")]
            movie_file_ids: &'a [i64],
        }

        let req = Request::new(path_join(&[BP_MOVIE_FILE, "bulk"])).with_json(&PostData {
            movie_file_ids,
        })?;
        self.api.delete_any(req).await
    }
}
