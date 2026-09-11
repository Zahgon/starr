use super::{CustomFormatOutput, Episode, Sonarr};
use crate::error::Result;
use crate::interface::ApiClientExt;
use crate::req::Request;
use crate::shared::{Quality, Value};
use crate::values::Values;
use serde::{Deserialize, Serialize};

const BP_PARSE: &str = "v3/parse";

/// ParseInput is the input for the Sonarr parse endpoint.
///
/// Must provide either a title or a path.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ParseInput {
    /// Release title to parse.
    pub title: String,
    /// Path to parse.
    pub path: String,
}

/// SeriesTitleInfo has only been seen in the parse endpoint so far.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SeriesTitleInfo {
    /// Year parsed out of the title.
    #[serde(default)]
    pub year: i32,
    /// Parsed series title.
    #[serde(default)]
    pub title: String,
    /// Title with the year removed.
    #[serde(default)]
    pub title_without_year: String,
}

/// ParsedEpisodeInfo is provided in [`ParseOutput`] when an item was properly parsed.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParsedEpisodeInfo {
    /// Episodes found in the title.
    #[serde(default)]
    pub episode_numbers: Vec<i32>,
    /// Absolute episode numbers found in the title.
    #[serde(default)]
    pub absolute_episode_numbers: Vec<i32>,
    /// Special absolute episode numbers found in the title.
    #[serde(default)]
    pub special_absolute_episode_numbers: Vec<i32>,
    /// Languages found in the title.
    #[serde(default)]
    pub languages: Vec<Value>,
    /// Season found in the title.
    #[serde(default)]
    pub season_number: i32,
    /// Part of the season the release covers.
    #[serde(default)]
    pub season_part: i64,
    /// Whether the release is a full season.
    #[serde(default)]
    pub full_season: bool,
    /// Whether the release is a partial season.
    #[serde(default)]
    pub is_partial_season: bool,
    /// Whether the release spans several seasons.
    #[serde(default)]
    pub is_multi_season: bool,
    /// Whether the release is a season extra.
    #[serde(default)]
    pub is_season_extra: bool,
    /// Whether the release is a split episode.
    #[serde(default)]
    pub is_split_episode: bool,
    /// Whether the series is a mini series.
    #[serde(default)]
    pub is_mini_series: bool,
    /// Whether the release is a special.
    #[serde(default)]
    pub special: bool,
    /// Whether the series airs daily.
    #[serde(default)]
    pub is_daily: bool,
    /// Whether the numbering is absolute.
    #[serde(default)]
    pub is_absolute_numbering: bool,
    /// Whether the release may be a special episode.
    #[serde(default)]
    pub is_possible_special_episode: bool,
    /// Whether the release may be a scene season special.
    #[serde(default)]
    pub is_possible_scene_season_special: bool,
    /// The title that was parsed.
    #[serde(default)]
    pub release_title: String,
    /// Series title found in the release.
    #[serde(default)]
    pub series_title: String,
    /// Release group that produced the release.
    #[serde(default)]
    pub release_group: String,
    /// Hash of the release.
    #[serde(default)]
    pub release_hash: String,
    /// Tokens parsed out of the release.
    #[serde(default)]
    pub release_tokens: String,
    /// Type of the release.
    #[serde(default)]
    pub release_type: String,
    /// Parsed series title details.
    #[serde(default)]
    pub series_title_info: Option<SeriesTitleInfo>,
    /// Quality found in the title.
    #[serde(default)]
    pub quality: Option<Quality>,
}

/// ParseOutput is what you get from the parse endpoint when you provide a
/// parsable path or title.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParseOutput {
    /// Episodes the release covers.
    #[serde(default)]
    pub episodes: Vec<Episode>,
    /// Languages found in the release.
    #[serde(default)]
    pub languages: Vec<Value>,
    /// Custom formats matched by the release.
    #[serde(default)]
    pub custom_formats: Vec<CustomFormatOutput>,
    /// Total score of the matched custom formats.
    #[serde(default)]
    pub custom_format_score: i64,
    /// Parse result ID.
    #[serde(default)]
    pub id: i64,
    /// The title that was parsed.
    #[serde(default)]
    pub title: String,
    /// Everything parsed out of the title.
    ///
    /// If the parse failed, this is `None`, and you won't get an error.
    #[serde(default)]
    pub parsed_episode_info: Option<ParsedEpisodeInfo>,
}

impl Sonarr {
    /// Parses a title or path into episode info.
    pub async fn parse(&self, input: &ParseInput) -> Result<ParseOutput> {
        let mut query = Values::new();
        query.set("title", input.title.clone());
        query.set("path", input.path.clone());

        self.api
            .get_into(Request::new(BP_PARSE).with_query(query))
            .await
    }
}
