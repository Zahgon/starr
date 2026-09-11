use super::{CustomFormatOutput, Sonarr};
use crate::error::Result;
use crate::helpers::str_val;
use crate::interface::ApiClientExt;
use crate::req::{Request, path_join};
use crate::shared::{BaseQuality, PlayTime, Quality, Value};
use crate::values::Values;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

const BP_EPISODE_FILE: &str = "v3/episodeFile";

/// EpisodeFile is the output from the `/api/v3/episodeFile` endpoint.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EpisodeFile {
    /// Episode file ID.
    #[serde(default)]
    pub id: i64,
    /// Series the file belongs to.
    #[serde(default)]
    pub series_id: i64,
    /// Season the file belongs to.
    #[serde(default)]
    pub season_number: i32,
    /// Path relative to the series folder.
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
    /// Release group that produced the release.
    #[serde(default)]
    pub release_group: String,
    /// Language of the file.
    #[serde(default)]
    pub language: Option<Value>,
    /// Quality of the file.
    #[serde(default)]
    pub quality: Option<Quality>,
    /// Media information read from the file.
    #[serde(default)]
    pub media_info: Option<MediaInfo>,
    /// Whether the quality cutoff was not met.
    #[serde(default)]
    pub quality_cutoff_not_met: bool,
    /// Whether the language cutoff was not met.
    #[serde(default)]
    pub language_cutoff_not_met: bool,
    /// Custom formats matched by the file. v4 only.
    #[serde(default)]
    pub custom_formats: Vec<CustomFormatOutput>,
}

/// MediaInfo is part of an [`EpisodeFile`].
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaInfo {
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
    /// Video frames per second.
    #[serde(default, rename = "videoFps")]
    pub video_fps: f64,
    /// Video resolution.
    #[serde(default)]
    pub resolution: String,
    /// Run time of the file.
    #[serde(default)]
    pub run_time: PlayTime,
    /// Scan type of the video.
    #[serde(default)]
    pub scan_type: String,
    /// Subtitles present in the file.
    #[serde(default)]
    pub subtitles: String,
}

impl Sonarr {
    /// Returns information about episode files by episode file IDs.
    pub async fn get_episode_files(&self, episode_file_ids: &[i64]) -> Result<Vec<EpisodeFile>> {
        let mut query = Values::new();
        for id in episode_file_ids {
            query.add("episodeFileIds", str_val(*id));
        }

        self.api
            .get_into(Request::new(BP_EPISODE_FILE).with_query(query))
            .await
    }

    /// Returns information about all episode files in a series.
    pub async fn get_series_episode_files(&self, series_id: i64) -> Result<Vec<EpisodeFile>> {
        let mut query = Values::new();
        query.add("seriesId", str_val(series_id));

        self.api
            .get_into(Request::new(BP_EPISODE_FILE).with_query(query))
            .await
    }

    /// Updates an episode file's quality.
    ///
    /// Use [`Sonarr::get_quality_profiles`] to find the available IDs.
    pub async fn update_episode_file_quality(
        &self,
        episode_file_id: i64,
        quality_id: i64,
    ) -> Result<EpisodeFile> {
        let file = EpisodeFile {
            id: episode_file_id,
            quality: Some(Quality {
                quality: Some(BaseQuality {
                    id: quality_id,
                    ..Default::default()
                }),
                ..Default::default()
            }),
            ..Default::default()
        };

        let req = Request::new(path_join(&[BP_EPISODE_FILE, &str_val(episode_file_id)]))
            .with_json(&file)?;
        self.api.put_into(req).await
    }

    /// Deletes an episode file.
    pub async fn delete_episode_file(&self, episode_file_id: i64) -> Result<()> {
        self.api
            .delete_any(Request::new(path_join(&[
                BP_EPISODE_FILE,
                &str_val(episode_file_id),
            ])))
            .await
    }
}
