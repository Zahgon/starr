use super::{CustomFormatOutput, Sonarr};
use crate::error::Result;
use crate::helpers::str_val;
use crate::interface::ApiClientExt;
use crate::is_default;
use crate::req::Request;
use crate::shared::{Protocol, Quality, Value};
use crate::values::Values;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

const BP_RELEASE: &str = "v3/release";

/// Release is the output from the Sonarr release endpoint.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Release {
    /// Release ID.
    #[serde(default)]
    pub id: i64,
    /// Release GUID.
    #[serde(default, rename = "guid")]
    pub guid: String,
    /// Quality of the release.
    #[serde(default)]
    pub quality: Quality,
    /// Weight of the quality.
    #[serde(default)]
    pub quality_weight: i64,
    /// Age of the release in days.
    #[serde(default)]
    pub age: i64,
    /// Age of the release in hours.
    #[serde(default)]
    pub age_hours: f64,
    /// Age of the release in minutes.
    #[serde(default)]
    pub age_minutes: f64,
    /// Size of the release in bytes.
    #[serde(default)]
    pub size: i64,
    /// Indexer the release came from.
    #[serde(default, rename = "indexerId")]
    pub indexer_id: i64,
    /// Name of the indexer.
    #[serde(default)]
    pub indexer: String,
    /// Release group that produced the release.
    #[serde(default)]
    pub release_group: String,
    /// Sub group that produced the release.
    #[serde(default)]
    pub sub_group: String,
    /// Hash of the release.
    #[serde(default)]
    pub release_hash: String,
    /// Release title.
    #[serde(default)]
    pub title: String,
    /// Whether the release covers a full season.
    #[serde(default)]
    pub full_season: bool,
    /// Whether the release came from a scene source.
    #[serde(default)]
    pub scene_source: bool,
    /// Season the release covers.
    #[serde(default)]
    pub season_number: i32,
    /// Languages found in the release.
    #[serde(default)]
    pub languages: Vec<Value>,
    /// Weight of the language.
    #[serde(default)]
    pub language_weight: i64,
    /// When the episode aired.
    #[serde(default)]
    pub air_date: String,
    /// Title of the series.
    #[serde(default)]
    pub series_title: String,
    /// Episodes the release covers.
    #[serde(default)]
    pub episode_numbers: Vec<i32>,
    /// Absolute episode numbers the release covers.
    #[serde(default)]
    pub absolute_episode_numbers: Vec<i32>,
    /// Season the release was mapped to.
    #[serde(default)]
    pub mapped_season_number: i32,
    /// Episodes the release was mapped to.
    #[serde(default)]
    pub mapped_episode_numbers: Vec<i32>,
    /// Absolute episode numbers the release was mapped to.
    #[serde(default)]
    pub mapped_absolute_episode_numbers: Vec<i32>,
    /// Series the release was mapped to.
    #[serde(default)]
    pub mapped_series_id: i64,
    /// Details of the mapped episodes.
    #[serde(default)]
    pub mapped_episode_info: Vec<ReleaseEpisodeInfo>,
    /// Whether the release is approved.
    #[serde(default)]
    pub approved: bool,
    /// Whether the release was temporarily rejected.
    #[serde(default)]
    pub temporarily_rejected: bool,
    /// Whether the release was rejected.
    #[serde(default)]
    pub rejected: bool,
    /// TheTVDB ID of the series.
    #[serde(default, rename = "tvdbId")]
    pub tvdb_id: i64,
    /// TVRage ID of the series.
    #[serde(default, rename = "tvRageId")]
    pub tv_rage_id: i64,
    /// Why the release was rejected.
    #[serde(default)]
    pub rejections: Vec<String>,
    /// When the release was published.
    #[serde(default)]
    pub publish_date: Option<DateTime<Utc>>,
    /// URL to the release comments.
    #[serde(default, rename = "commentUrl")]
    pub comment_url: String,
    /// URL the release is downloaded from.
    #[serde(default, rename = "downloadUrl")]
    pub download_url: String,
    /// URL with more information about the release.
    #[serde(default, rename = "infoUrl")]
    pub info_url: String,
    /// Whether the episode was requested.
    #[serde(default)]
    pub episode_requested: bool,
    /// Whether the release may be downloaded.
    #[serde(default)]
    pub download_allowed: bool,
    /// Weight used to sort releases.
    #[serde(default)]
    pub release_weight: i64,
    /// Custom formats matched by the release.
    #[serde(default)]
    pub custom_formats: Vec<CustomFormatOutput>,
    /// Total score of the matched custom formats.
    #[serde(default)]
    pub custom_format_score: i64,
    /// Scene mapping applied to the release.
    #[serde(default)]
    pub scene_mapping: ReleaseSceneMapping,
    /// Magnet URL for the release.
    #[serde(default, rename = "magnetUrl")]
    pub magnet_url: String,
    /// Torrent info hash.
    #[serde(default)]
    pub info_hash: String,
    /// Number of seeders.
    #[serde(default)]
    pub seeders: i32,
    /// Number of leechers.
    #[serde(default)]
    pub leechers: i32,
    /// Protocol the release uses.
    #[serde(default)]
    pub protocol: Protocol,
    /// Indexer flags set on the release.
    #[serde(default, skip_serializing_if = "is_default")]
    pub indexer_flags: i64,
    /// Whether the series airs daily.
    #[serde(default)]
    pub is_daily: bool,
    /// Whether the numbering is absolute.
    #[serde(default)]
    pub is_absolute_numbering: bool,
    /// Whether the release may be a special episode.
    #[serde(default)]
    pub is_possible_special_episode: bool,
    /// Whether the release is a special.
    #[serde(default)]
    pub special: bool,
    /// Series the release belongs to.
    #[serde(default)]
    pub series_id: i64,
    /// Episode the release belongs to.
    #[serde(default)]
    pub episode_id: i64,
    /// Episodes the release belongs to.
    #[serde(default)]
    pub episode_ids: Vec<i64>,
    /// Download client used to grab the release.
    #[serde(default)]
    pub download_client_id: i64,
    /// Name of the download client.
    #[serde(default)]
    pub download_client: String,
    /// Whether the release overrides the parsed data.
    #[serde(default)]
    pub should_override: bool,
}

/// ReleaseEpisodeInfo is part of a [`Release`].
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReleaseEpisodeInfo {
    /// Episode ID.
    #[serde(default)]
    pub id: i64,
    /// Season the episode belongs to.
    #[serde(default)]
    pub season_number: i32,
    /// Number of the episode in its season.
    #[serde(default)]
    pub episode_number: i32,
    /// Episode number across all seasons.
    #[serde(default)]
    pub absolute_episode_number: i32,
    /// Episode title.
    #[serde(default)]
    pub title: String,
}

/// ReleaseSceneMapping is part of a [`Release`].
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReleaseSceneMapping {
    /// Title used by the scene.
    #[serde(default)]
    pub title: String,
    /// Season number in the library.
    #[serde(default)]
    pub season_number: i32,
    /// Season number used by the scene.
    #[serde(default)]
    pub scene_season_number: i32,
    /// Where the scene mapping came from.
    #[serde(default)]
    pub scene_origin: String,
    /// Comment attached to the mapping.
    #[serde(default)]
    pub comment: String,
}

/// SearchRelease is the input to the release search endpoint.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchRelease {
    /// Series to search for.
    #[serde(default)]
    pub series_id: i64,
    /// Episode to search for.
    #[serde(default)]
    pub episode_id: i64,
    /// Season to search for.
    #[serde(default)]
    pub season_number: i32,
}

impl Sonarr {
    /// Searches for and returns a list of releases available for download.
    pub async fn search_release(&self, input: &SearchRelease) -> Result<Vec<Release>> {
        let mut query = Values::new();
        query.set("seriesId", str_val(input.series_id));
        query.set("episodeId", str_val(input.episode_id));
        query.set("seasonNumber", str_val(input.season_number));

        self.api
            .get_into(Request::new(BP_RELEASE).with_query(query))
            .await
    }

    /// Attempts to download a release by GUID.
    pub async fn grab(&self, guid: &str, indexer_id: i64) -> Result<Release> {
        self.grab_release(&Release {
            indexer_id,
            guid: guid.to_string(),
            ..Default::default()
        })
        .await
    }

    /// Attempts to download a release from a search.
    ///
    /// Pass the release from the [`Sonarr::search_release`] output for the item
    /// you wish to download.
    pub async fn grab_release(&self, release: &Release) -> Result<Release> {
        /// We only use/need the guid and indexerID from the release.
        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Grab<'a> {
            guid: &'a str,
            indexer_id: i64,
        }

        let grab = Grab {
            guid: &release.guid,
            indexer_id: release.indexer_id,
        };

        self.api
            .post_into(Request::new(BP_RELEASE).with_json(&grab)?)
            .await
    }
}
