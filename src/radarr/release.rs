use super::Radarr;
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

/// Release is the output from the Radarr release endpoint.
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
    pub quality: Option<Quality>,
    /// Custom formats matched by the release.
    #[serde(default)]
    pub custom_formats: Vec<serde_json::Value>,
    /// Total score of the matched custom formats.
    #[serde(default)]
    pub custom_format_score: i64,
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
    /// Hash of the release.
    #[serde(default)]
    pub release_hash: String,
    /// Release title.
    #[serde(default)]
    pub title: String,
    /// Whether the release came from a scene source.
    #[serde(default)]
    pub scene_source: bool,
    /// Movie titles found in the release.
    #[serde(default)]
    pub movie_titles: Vec<String>,
    /// Languages found in the release.
    #[serde(default)]
    pub languages: Vec<Value>,
    /// Movie the release was mapped to.
    #[serde(default)]
    pub mapped_movie_id: i64,
    /// Whether the release is approved.
    #[serde(default)]
    pub approved: bool,
    /// Whether the release was temporarily rejected.
    #[serde(default)]
    pub temporarily_rejected: bool,
    /// Whether the release was rejected.
    #[serde(default)]
    pub rejected: bool,
    /// TMDb ID of the movie.
    #[serde(default, rename = "tmdbId")]
    pub tmdb_id: i64,
    /// IMDb ID of the movie.
    #[serde(default, rename = "imdbId")]
    pub imdb_id: i64,
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
    /// Whether the release may be downloaded.
    #[serde(default)]
    pub download_allowed: bool,
    /// Weight used to sort releases.
    #[serde(default)]
    pub release_weight: i64,
    /// Edition of the movie.
    #[serde(default)]
    pub edition: String,
    /// Magnet URL for the release.
    #[serde(default, rename = "magnetUrl")]
    pub magnet_url: String,
    /// Torrent info hash.
    #[serde(default)]
    pub info_hash: String,
    /// Sub group that produced the release.
    #[serde(default)]
    pub sub_group: String,
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
    pub indexer_flags: Vec<String>,
    /// Movie the release belongs to.
    #[serde(default)]
    pub movie_id: i64,
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

impl Radarr {
    /// Searches for and returns a list of releases available for download.
    pub async fn search_release(&self, movie_id: i64) -> Result<Vec<Release>> {
        let mut query = Values::new();
        query.set("movieId", str_val(movie_id));

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

    /// Attempts to download a release for a movie from a search.
    ///
    /// Pass the release from the [`Radarr::search_release`] output for the item
    /// you wish to download. If `release.movie_id` is 0 then
    /// `release.mapped_movie_id` is used. Both may be 0, and that's OK unless
    /// `release.should_override` is true. If `release.should_override` is true,
    /// then languages, movie_id and quality must be present in the release.
    pub async fn grab_release(&self, release: &Release) -> Result<Release> {
        /// These are the required fields on the Radarr `POST /release` endpoint.
        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Grab<'a> {
            guid: &'a str,
            indexer_id: i64,
            should_override: bool,
            #[serde(skip_serializing_if = "is_default")]
            languages: &'a [Value],
            #[serde(skip_serializing_if = "is_default")]
            movie_id: i64,
            #[serde(skip_serializing_if = "Option::is_none")]
            quality: Option<&'a Quality>,
        }

        let movie_id = if release.movie_id == 0 {
            release.mapped_movie_id // Best effort?
        } else {
            release.movie_id
        };

        let grab = Grab {
            guid: &release.guid,
            indexer_id: release.indexer_id,
            should_override: release.should_override,
            languages: &release.languages,
            movie_id,
            quality: release.quality.as_ref(),
        };

        self.api
            .post_into(Request::new(BP_RELEASE).with_json(&grab)?)
            .await
    }
}
