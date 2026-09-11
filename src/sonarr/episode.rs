use super::{Series, Sonarr};
use crate::error::Result;
use crate::helpers::str_val;
use crate::interface::ApiClientExt;
use crate::req::{Request, path_join};
use crate::shared::Image;
use crate::values::Values;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

const BP_EPISODE: &str = "v3/episode";

/// Episode is the `/api/v3/episode` endpoint.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Episode {
    /// Episode number across all seasons.
    #[serde(default)]
    pub absolute_episode_number: i32,
    /// Season this episode belongs to.
    #[serde(default)]
    pub season_number: i32,
    /// Number of this episode in its season.
    #[serde(default)]
    pub episode_number: i32,
    /// Episode ID.
    #[serde(default)]
    pub id: i64,
    /// Series this episode belongs to.
    #[serde(default)]
    pub series_id: i64,
    /// TheTVDB ID.
    #[serde(default, rename = "tvdbId")]
    pub tvdb_id: i64,
    /// File backing this episode.
    #[serde(default)]
    pub episode_file_id: i64,
    /// When the episode aired, in UTC.
    #[serde(default, rename = "airDateUtc")]
    pub air_date_utc: Option<DateTime<Utc>>,
    /// When the episode aired, in the network's time zone.
    #[serde(default)]
    pub air_date: String,
    /// Episode title.
    #[serde(default)]
    pub title: String,
    /// Episode overview.
    #[serde(default)]
    pub overview: String,
    /// Whether the scene numbering is unverified.
    #[serde(default)]
    pub unverified_scene_numbering: bool,
    /// Whether a file exists for this episode.
    #[serde(default)]
    pub has_file: bool,
    /// Whether the episode is monitored.
    #[serde(default)]
    pub monitored: bool,
    /// Episode images.
    #[serde(default)]
    pub images: Vec<Image>,
    /// The series this episode belongs to.
    #[serde(default)]
    pub series: Option<Series>,
}

/// GetEpisode represents the input parameters for an episode api request.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct GetEpisode {
    /// Set this to get episodes for a specific series. Set to zero to get all episodes.
    pub series_id: i64,
    /// Set this to get episodes for a specific season. Set to zero to get all episodes.
    pub season_number: i32,
    /// Set this to get episodes for a specific set of IDs. Leave empty to get all episodes.
    pub episode_ids: Vec<i64>,
    /// Set this to get episodes for a specific file. Set to zero to get all episodes.
    pub episode_file_id: i64,
    /// Set this to include images for each episode.
    pub include_images: bool,
}

impl Sonarr {
    /// Returns all episodes matching the provided filters.
    ///
    /// You can get series IDs from [`Sonarr::get_all_series`] and [`Sonarr::get_series`].
    pub async fn get_series_episodes(&self, get_episode: &GetEpisode) -> Result<Vec<Episode>> {
        let mut params = Values::new();

        if get_episode.series_id > 0 {
            params.set("seriesId", str_val(get_episode.series_id));
        }

        if get_episode.season_number > 0 {
            params.set("seasonNumber", str_val(get_episode.season_number));
        }

        for id in &get_episode.episode_ids {
            params.add("episodeIds", str_val(*id));
        }

        if get_episode.episode_file_id > 0 {
            params.set("episodeFileId", str_val(get_episode.episode_file_id));
        }

        if get_episode.include_images {
            params.set("includeImages", "true");
        }

        self.api
            .get_into(Request::new(BP_EPISODE).with_query(params))
            .await
    }

    /// Locates and returns an episode by DB [episode] ID.
    pub async fn get_episode_by_id(&self, episode_id: i64) -> Result<Episode> {
        self.api
            .get_into(Request::new(path_join(&[BP_EPISODE, &str_val(episode_id)])))
            .await
    }

    /// Sends a request to monitor (true) or unmonitor (false) a list of episodes by ID.
    ///
    /// You can get episode IDs from [`Sonarr::get_series_episodes`].
    pub async fn monitor_episode(
        &self,
        episode_ids: &[i64],
        monitor: bool,
    ) -> Result<Vec<Episode>> {
        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Input<'a> {
            episode_ids: &'a [i64],
            monitored: bool,
        }

        let req = Request::new(path_join(&[BP_EPISODE, "monitor"])).with_json(&Input {
            episode_ids,
            monitored: monitor,
        })?;
        self.api.put_into(req).await
    }
}
