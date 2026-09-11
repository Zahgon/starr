use super::{Album, Artist, CustomFormatOutput, Lidarr};
use crate::error::Result;
use crate::interface::ApiClientExt;
use crate::is_default;
use crate::req::Request;
use crate::shared::Quality;
use crate::values::Values;
use serde::{Deserialize, Serialize};

const BP_PARSE: &str = "v1/parse";

/// ParsedAlbumInfo is returned when a release title parses as an album.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParsedAlbumInfo {
    /// Title of the release that was parsed.
    #[serde(default, skip_serializing_if = "is_default")]
    pub release_title: String,
    /// Album title found in the release title.
    #[serde(default, skip_serializing_if = "is_default")]
    pub album_title: String,
    /// Artist name found in the release title.
    #[serde(default, skip_serializing_if = "is_default")]
    pub artist_name: String,
    /// Album type found in the release title.
    #[serde(default, skip_serializing_if = "is_default")]
    pub album_type: String,
    /// Quality parsed from the release title.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quality: Option<Quality>,
    /// Release date parsed from the release title.
    #[serde(default, skip_serializing_if = "is_default")]
    pub release_date: String,
    /// Whether the release is a discography.
    #[serde(default, skip_serializing_if = "is_default")]
    pub discography: bool,
    /// First year of the discography.
    #[serde(default, skip_serializing_if = "is_default")]
    pub discography_start: i32,
}

/// ParseOutput is returned from `GET /api/v1/parse`.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParseOutput {
    /// Parse result ID.
    #[serde(default)]
    pub id: i64,
    /// Title that was parsed.
    #[serde(default, skip_serializing_if = "is_default")]
    pub title: String,
    /// Album metadata extracted from the title.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parsed_album_info: Option<ParsedAlbumInfo>,
    /// Artist matched to the parsed title.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub artist: Option<Artist>,
    /// Albums matched to the parsed title.
    #[serde(default, skip_serializing_if = "is_default")]
    pub albums: Vec<Album>,
    /// Custom formats that apply to the release.
    #[serde(default, skip_serializing_if = "is_default")]
    pub custom_formats: Vec<CustomFormatOutput>,
    /// Total score of the matching custom formats.
    #[serde(default)]
    pub custom_format_score: i64,
}

impl Lidarr {
    /// Resolves a release title into parsed album metadata.
    pub async fn parse(&self, title: &str) -> Result<ParseOutput> {
        let mut query = Values::new();
        query.set("title", title);

        self.api
            .get_into(Request::new(BP_PARSE).with_query(query))
            .await
    }
}
