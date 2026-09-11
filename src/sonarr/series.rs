use super::Sonarr;
use crate::error::Result;
use crate::helpers::str_val;
use crate::interface::ApiClientExt;
use crate::is_default;
use crate::req::{Request, path_join};
use crate::shared::{Image, Ratings};
use crate::string_enum;
use crate::values::Values;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

const BP_SERIES: &str = "v3/series";

/// AddSeriesInput is the input for the `/api/v3/series` endpoint.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddSeriesInput {
    /// Whether the series is monitored.
    #[serde(default)]
    pub monitored: bool,
    /// Whether episodes go into season folders.
    #[serde(default, skip_serializing_if = "is_default")]
    pub season_folder: bool,
    /// Whether scene numbering is used.
    #[serde(default, skip_serializing_if = "is_default")]
    pub use_scene_numbering: bool,
    /// Series ID. Leave zero when creating.
    #[serde(default, skip_serializing_if = "is_default")]
    pub id: i64,
    /// Language profile applied to this series.
    #[serde(default, skip_serializing_if = "is_default")]
    pub language_profile_id: i64,
    /// Quality profile applied to this series.
    #[serde(default, skip_serializing_if = "is_default")]
    pub quality_profile_id: i64,
    /// TheTVDB ID.
    #[serde(default, skip_serializing_if = "is_default", rename = "tvdbId")]
    pub tvdb_id: i64,
    /// IMDb ID.
    #[serde(default, skip_serializing_if = "is_default", rename = "imdbId")]
    pub imdb_id: String,
    /// TVMaze ID.
    #[serde(default, skip_serializing_if = "is_default", rename = "tvMazeId")]
    pub tv_maze_id: i64,
    /// TVRage ID.
    #[serde(default, skip_serializing_if = "is_default", rename = "tvRageId")]
    pub tv_rage_id: i64,
    /// Full path to the series folder.
    #[serde(default, skip_serializing_if = "is_default")]
    pub path: String,
    /// Type of the series.
    #[serde(default, skip_serializing_if = "is_default")]
    pub series_type: String,
    /// Series title.
    #[serde(default, skip_serializing_if = "is_default")]
    pub title: String,
    /// Slug used in URLs.
    #[serde(default, skip_serializing_if = "is_default")]
    pub title_slug: String,
    /// Root folder the series is placed in.
    #[serde(default, skip_serializing_if = "is_default")]
    pub root_folder_path: String,
    /// Tags applied to this series.
    #[serde(default, skip_serializing_if = "is_default")]
    pub tags: Vec<i32>,
    /// Seasons of this series.
    #[serde(default, skip_serializing_if = "is_default")]
    pub seasons: Vec<Season>,
    /// Series images.
    #[serde(default, skip_serializing_if = "is_default")]
    pub images: Vec<Image>,
    /// To be used only on POST, not for PUT.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub add_options: Option<AddSeriesOptions>,
}

/// Series is the output of the `/api/v3/series` endpoint.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Series {
    /// Whether the series has ended.
    #[serde(default, skip_serializing_if = "is_default")]
    pub ended: bool,
    /// Whether the series is monitored.
    #[serde(default)]
    pub monitored: bool,
    /// Whether episodes go into season folders.
    #[serde(default, skip_serializing_if = "is_default")]
    pub season_folder: bool,
    /// Whether scene numbering is used.
    #[serde(default, skip_serializing_if = "is_default")]
    pub use_scene_numbering: bool,
    /// Runtime in minutes.
    #[serde(default, skip_serializing_if = "is_default")]
    pub runtime: i32,
    /// Year the series first aired.
    #[serde(default, skip_serializing_if = "is_default")]
    pub year: i32,
    /// Series ID.
    #[serde(default, skip_serializing_if = "is_default")]
    pub id: i64,
    /// Language profile applied to this series.
    #[serde(default, skip_serializing_if = "is_default")]
    pub language_profile_id: i64,
    /// Quality profile applied to this series.
    #[serde(default, skip_serializing_if = "is_default")]
    pub quality_profile_id: i64,
    /// TheTVDB ID.
    #[serde(default, skip_serializing_if = "is_default", rename = "tvdbId")]
    pub tvdb_id: i64,
    /// TVMaze ID.
    #[serde(default, skip_serializing_if = "is_default", rename = "tvMazeId")]
    pub tv_maze_id: i64,
    /// TVRage ID.
    #[serde(default, skip_serializing_if = "is_default", rename = "tvRageId")]
    pub tv_rage_id: i64,
    /// Time of day the series airs.
    #[serde(default, skip_serializing_if = "is_default")]
    pub air_time: String,
    /// Series certification.
    #[serde(default, skip_serializing_if = "is_default")]
    pub certification: String,
    /// Title with special characters removed.
    #[serde(default, skip_serializing_if = "is_default")]
    pub clean_title: String,
    /// IMDb ID.
    #[serde(default, skip_serializing_if = "is_default", rename = "imdbId")]
    pub imdb_id: String,
    /// Network that airs the series.
    #[serde(default, skip_serializing_if = "is_default")]
    pub network: String,
    /// Series overview.
    #[serde(default, skip_serializing_if = "is_default")]
    pub overview: String,
    /// Full path to the series folder.
    #[serde(default, skip_serializing_if = "is_default")]
    pub path: String,
    /// Type of the series.
    #[serde(default, skip_serializing_if = "is_default")]
    pub series_type: String,
    /// Title used for sorting.
    #[serde(default, skip_serializing_if = "is_default")]
    pub sort_title: String,
    /// Series status.
    #[serde(default, skip_serializing_if = "is_default")]
    pub status: String,
    /// Series title.
    #[serde(default, skip_serializing_if = "is_default")]
    pub title: String,
    /// Slug used in URLs.
    #[serde(default, skip_serializing_if = "is_default")]
    pub title_slug: String,
    /// Root folder the series lives under.
    #[serde(default, skip_serializing_if = "is_default")]
    pub root_folder_path: String,
    /// When the series was added.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub added: Option<DateTime<Utc>>,
    /// When the series first aired.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub first_aired: Option<DateTime<Utc>>,
    /// When the next episode airs.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_airing: Option<DateTime<Utc>>,
    /// When the previous episode aired.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub previous_airing: Option<DateTime<Utc>>,
    /// Series ratings.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ratings: Option<Ratings>,
    /// Library statistics for this series.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub statistics: Option<Statistics>,
    /// Tags applied to this series.
    #[serde(default, skip_serializing_if = "is_default")]
    pub tags: Vec<i32>,
    /// Genres this series belongs to.
    #[serde(default, skip_serializing_if = "is_default")]
    pub genres: Vec<String>,
    /// Alternate titles for this series.
    #[serde(default, skip_serializing_if = "is_default")]
    pub alternate_titles: Vec<AlternateTitle>,
    /// Seasons of this series.
    #[serde(default, skip_serializing_if = "is_default")]
    pub seasons: Vec<Season>,
    /// Series images.
    #[serde(default, skip_serializing_if = "is_default")]
    pub images: Vec<Image>,
}

/// AddSeriesOptions is part of [`AddSeriesInput`].
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddSeriesOptions {
    /// Whether missing episodes are searched for.
    #[serde(default)]
    pub search_for_missing_episodes: bool,
    /// Whether episodes below the cutoff are searched for.
    #[serde(default, skip_serializing_if = "is_default")]
    pub search_for_cutoff_unmet_episodes: bool,
    /// Whether episodes that have files are ignored.
    #[serde(default, skip_serializing_if = "is_default")]
    pub ignore_episodes_with_files: bool,
    /// Whether episodes without files are ignored.
    #[serde(default, skip_serializing_if = "is_default")]
    pub ignore_episodes_without_files: bool,
    /// What gets monitored on the series.
    #[serde(default, skip_serializing_if = "is_default")]
    pub monitor: MonitorType,
}

string_enum! {
    /// MonitorType is part of the [`AddSeriesOptions`].
    ///
    /// These are the possible values for the monitor option when adding a new series.
    pub struct MonitorType {
        /// Unknown monitor type.
        const UNKNOWN = "unknown";
        /// Monitor all episodes.
        const ALL = "all";
        /// Monitor future episodes.
        const FUTURE = "future";
        /// Monitor missing episodes.
        const MISSING = "missing";
        /// Monitor existing episodes.
        const EXISTING = "existing";
        /// Monitor the first season.
        const FIRST_SEASON = "firstSeason";
        /// Monitor the last season.
        const LAST_SEASON = "lastSeason";
        /// Monitor the latest season. Obsolete.
        const LATEST_SEASON = "latestSeason";
        /// Monitor the pilot episode.
        const PILOT = "pilot";
        /// Monitor recent episodes.
        const RECENT = "recent";
        /// Monitor the specials.
        const MONITOR_SPECIALS = "monitorSpecials";
        /// Unmonitor the specials.
        const UNMONITOR_SPECIALS = "unmonitorSpecials";
        /// Monitor nothing.
        const NONE = "none";
        /// Skip monitoring.
        const SKIP = "skip";
    }
}

/// AlternateTitle is part of an [`AddSeriesInput`].
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlternateTitle {
    /// Season the title applies to.
    #[serde(default)]
    pub season_number: i32,
    /// The alternate title.
    #[serde(default)]
    pub title: String,
}

/// Season is part of [`AddSeriesInput`] and Queue, and is used in a few places.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Season {
    /// Whether the season is monitored.
    #[serde(default)]
    pub monitored: bool,
    /// Number of this season.
    #[serde(default)]
    pub season_number: i32,
    /// Library statistics for this season.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub statistics: Option<Statistics>,
    /// Season images.
    #[serde(default, skip_serializing_if = "is_default")]
    pub images: Vec<Image>,
}

/// Statistics is part of [`AddSeriesInput`] and Queue.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Statistics {
    /// Number of seasons.
    #[serde(default)]
    pub season_count: i32,
    /// Number of episode files on disk.
    #[serde(default)]
    pub episode_file_count: i32,
    /// Number of episodes.
    #[serde(default)]
    pub episode_count: i32,
    /// Total number of episodes.
    #[serde(default)]
    pub total_episode_count: i32,
    /// Bytes used on disk.
    #[serde(default)]
    pub size_on_disk: i64,
    /// Percentage of episodes present on disk.
    #[serde(default)]
    pub percent_of_episodes: f64,
    /// When the previous episode aired.
    #[serde(default)]
    pub previous_airing: Option<DateTime<Utc>>,
    /// When the next episode airs.
    #[serde(default)]
    pub next_airing: Option<DateTime<Utc>>,
    /// Release groups present in the library.
    #[serde(default)]
    pub release_groups: Vec<String>,
}

impl Sonarr {
    /// Returns all configured series.
    ///
    /// This may not deal well with pagination atm, let us know?
    pub async fn get_all_series(&self) -> Result<Vec<Series>> {
        self.get_series(0).await
    }

    /// Locates and returns a series by `tvdb_id`. If `tvdb_id` is 0, returns all series.
    pub async fn get_series(&self, tvdb_id: i64) -> Result<Vec<Series>> {
        let mut query = Values::new();
        if tvdb_id != 0 {
            query.add("tvdbId", str_val(tvdb_id));
        }

        query.add("includeSeasonImages", "true");

        self.api
            .get_into(Request::new(BP_SERIES).with_query(query))
            .await
    }

    /// Updates a series in place.
    pub async fn update_series(
        &self,
        series: &AddSeriesInput,
        move_files: bool,
    ) -> Result<Series> {
        let mut query = Values::new();
        query.add("moveFiles", str_val(move_files));

        let req = Request::new(path_join(&[BP_SERIES, &str_val(series.id)]))
            .with_json(series)?
            .with_query(query);
        self.api.put_into(req).await
    }

    /// Adds a new series to Sonarr.
    pub async fn add_series(&self, series: &AddSeriesInput) -> Result<Series> {
        self.api
            .post_into(Request::new(BP_SERIES).with_json(series)?)
            .await
    }

    /// Locates and returns a series by DB [series] ID.
    pub async fn get_series_by_id(&self, series_id: i64) -> Result<Series> {
        let mut query = Values::new();
        query.add("includeSeasonImages", "true");

        self.api
            .get_into(Request::new(path_join(&[BP_SERIES, &str_val(series_id)])).with_query(query))
            .await
    }

    /// Searches for a series [in Servarr] using a search term or a tvdbid.
    ///
    /// Provide a search term or a tvdbid. If you provide both, `tvdb_id` is used.
    pub async fn get_series_lookup(&self, term: &str, tvdb_id: i64) -> Result<Vec<Series>> {
        let mut query = Values::new();
        if tvdb_id > 0 {
            query.add("term", format!("tvdbid:{}", str_val(tvdb_id)));
        } else {
            query.add("term", term);
        }

        self.api
            .get_into(Request::new(path_join(&[BP_SERIES, "lookup"])).with_query(query))
            .await
    }

    /// Searches for series matching the specified search term.
    ///
    /// Searches for new shows on TheTVDB.com utilizing sonarr.tv's caching and
    /// augmentation proxy.
    pub async fn lookup(&self, term: &str) -> Result<Vec<Series>> {
        self.get_series_lookup(term, 0).await
    }

    /// Removes a single Series.
    ///
    /// `delete_files` defines the `deleteFiles` query parameter.
    /// `import_exclude` defines the `addImportListExclusion` query parameter.
    pub async fn delete_series(
        &self,
        series_id: i32,
        delete_files: bool,
        import_exclude: bool,
    ) -> Result<()> {
        let mut query = Values::new();
        query.add("deleteFiles", str_val(delete_files));
        query.add("addImportListExclusion", str_val(import_exclude));

        self.api
            .delete_any(Request::new(path_join(&[BP_SERIES, &str_val(series_id)])).with_query(query))
            .await
    }

    /// Removes a single Series, setting `deleteFiles` to true and
    /// `addImportListExclusion` to false.
    pub async fn delete_series_default(&self, series_id: i32) -> Result<()> {
        self.delete_series(series_id, true, false).await
    }
}
