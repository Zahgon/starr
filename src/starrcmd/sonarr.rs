use super::{CmdEvent, CmdResult, Dispatcher, Event, execute_get};
use crate::env_struct;
use crate::helpers::App;
use chrono::{DateTime, Utc};

env_struct! {
    /// SonarrApplicationUpdate is the ApplicationUpdate event.
    pub struct SonarrApplicationUpdate {
        /// 4.0.3.5875
        previous_version: String = "sonarr_update_previousversion";
        /// 4.0.4.5909
        new_version: String = "sonarr_update_newversion";
        /// Sonarr updated from 4.0.3.5875 to 4.0.4.5909
        message: String = "sonarr_update_message";
    }
}

env_struct! {
    /// SonarrHealthIssue is the HealthIssue event.
    pub struct SonarrHealthIssue {
        /// Lists unavailable due to failures: Listnamehere
        message: String = "sonarr_health_issue_message";
        /// ImportListStatusCheck
        issue_type: String = "sonarr_health_issue_type";
        /// https://wiki.servarr.com/
        wiki: String = "sonarr_health_issue_wiki";
        /// Warning
        level: String = "sonarr_health_issue_level";
    }
}

env_struct! {
    /// SonarrGrab is the Grab event.
    pub struct SonarrGrab {
        /// HDTV-720p
        quality: String = "sonarr_release_quality";
        /// This Is Us
        title: String = "sonarr_series_title";
        /// NZBGet
        download_client: String = "sonarr_download_client";
        /// This.is.Us.S06E04.720p.HDTV.x264-SYNCOPY
        release_title: String = "sonarr_release_title";
        /// a87bda3c0e7f40a1b8fa011b421a5201
        download_id: String = "sonarr_download_id";
        /// Indexor (Prowlarr)
        release_indexer: String = "sonarr_release_indexer";
        /// Standard
        series_type: String = "sonarr_series_type";
        /// SYNCOPY
        release_group: String = "sonarr_release_releasegroup";
        /// tt5555260
        imdb_id: String = "sonarr_series_imdbid";
        /// 4
        episode_numbers: Vec<i32> = "sonarr_release_episodenumbers", split = ',';
        /// 2022-01-25
        episode_air_dates: Vec<String> = "sonarr_release_episodeairdates", split = ',';
        /// Don't Let Me Keep You
        episode_titles: Vec<String> = "sonarr_release_episodetitles", split = '|';
        /// 92
        abs_episode_numbers: Vec<i32> = "sonarr_release_absoluteepisodenumbers", split = ',';
        /// 1/26/2022 2:00:00 AM
        episode_air_dates_utc: Vec<DateTime<Utc>> = "sonarr_release_episodeairdatesutc", split = ',';
        /// 1
        quality_version: i64 = "sonarr_release_qualityversion";
        /// 47
        series_id: i64 = "sonarr_series_id";
        /// 1
        episode_count: i32 = "sonarr_release_episodecount";
        /// 885369406
        size: i64 = "sonarr_release_size";
        /// 311714
        tvdb_id: i64 = "sonarr_series_tvdbid";
        /// 17128
        tvmaze_id: i64 = "sonarr_series_tvmazeid";
        /// 6
        season_number: i32 = "sonarr_release_seasonnumber";
    }
}

env_struct! {
    /// SonarrDownload is the Download event.
    pub struct SonarrDownload {
        /// Puppy Dog Pals
        title: String = "sonarr_series_title";
        /// /downloads/completed/Series/Puppy.Dog.Pals.S05E03e04.The.Puppy.Outdoor.Play.Day.Games.for.the.Glove.of.the.Game.HULU.WEB-DL.AAC2.0.H.264-LAZY
        source_folder: String = "sonarr_episodefile_sourcefolder";
        /// WEBDL-480p
        quality: String = "sonarr_episodefile_quality";
        /// LAZY
        release_group: String = "sonarr_episodefile_releasegroup";
        /// NZBGET
        download_client: String = "sonarr_download_client";
        /// /tv/Puppy Dog Pals/Season 5/Puppy Dog Pals - S05E03-04 - The Puppy Outdoor Play Day Games + For the Glove of the Game WEBDL-480p.mkv
        episode_path: String = "sonarr_episodefile_path";
        /// Puppy.Dog.Pals.S05E03e04.The.Puppy.Outdoor.Play.Day.Games.for.the.Glove.of.the.Game.HULU.WEB-DL.AAC2.0.H.264-LAZY
        scene_name: String = "sonarr_episodefile_scenename";
        /// /tv/Puppy Dog Pals
        path: String = "sonarr_series_path";
        /// /downloads/completed/Series/Puppy.Dog.Pals.S05E03e04.The.Puppy.Outdoor.Play.Day.Games.for.the.Glove.of.the.Game.HULU.WEB-DL.AAC2.0.H.264-LAZY/9ZMAepAkHwQsOn.mkv
        source_path: String = "sonarr_episodefile_sourcepath";
        /// 977d4bd4ac3845c0a2d5c890cc5a10e4
        download_id: String = "sonarr_download_id";
        /// Standard
        series_type: String = "sonarr_series_type";
        /// tt6688750
        imdb_id: String = "sonarr_series_imdbid";
        /// Season 5/Puppy Dog Pals - S05E03-04 - The Puppy Outdoor Play Day Games + For the Glove of the Game WEBDL-480p.mkv
        relative_path: String = "sonarr_episodefile_relativepath";
        /// 22691,22692
        episode_ids: Vec<i64> = "sonarr_episodefile_episodeids", split = ',';
        /// 3,4
        episode_numbers: Vec<i32> = "sonarr_episodefile_episodenumbers", split = ',';
        /// 2022-01-21,2022-01-21
        episode_air_dates: Vec<String> = "sonarr_episodefile_episodeairdates", split = ',';
        /// The Puppy Outdoor Play Day Games|For the Glove of the Game
        episode_titles: Vec<String> = "sonarr_episodefile_episodetitles", split = '|';
        /// 1/21/2022 2:00:00 PM,1/21/2022 2:12:00 PM
        episode_air_dates_utc: Vec<DateTime<Utc>> = "sonarr_episodefile_episodeairdatesutc", split = ',';
        /// Not always present.
        deleted_relative_paths: Vec<String> = "sonarr_deletedrelativepaths", split = '|';
        /// Not always present.
        deleted_paths: Vec<String> = "sonarr_deletedpaths", split = '|';
        /// 108
        series_id: i64 = "sonarr_series_id";
        /// 1
        quality_version: i64 = "sonarr_episodefile_qualityversion";
        /// 14996
        file_id: i64 = "sonarr_episodefile_id";
        /// 325978
        tvdb_id: i64 = "sonarr_series_tvdbid";
        /// 26341
        tvmaze_id: i64 = "sonarr_series_tvmazeid";
        /// 2
        episode_count: i32 = "sonarr_episodefile_episodecount";
        /// 5
        season_number: i32 = "sonarr_episodefile_seasonnumber";
        /// False
        is_upgrade: bool = "sonarr_isupgrade";
    }
}

env_struct! {
    /// SonarrRename is the Rename event.
    pub struct SonarrRename {
        /// series.Title)
        title: String = "sonarr_series_title";
        /// series.Path)
        path: String = "sonarr_series_path";
        /// series.ImdbId ?? string.Empty)
        imdb_id: String = "sonarr_series_imdbid";
        /// series.SeriesType.ToString())
        series_type: String = "sonarr_series_type";
        /// string.Join(",", renamedFiles.Select(e => e.EpisodeFile.Id)))
        file_ids: Vec<i64> = "sonarr_episodefile_ids", split = ',';
        /// string.Join("|", renamedFiles.Select(e => e.EpisodeFile.RelativePath)))
        relative_paths: Vec<String> = "sonarr_episodefile_relativepaths", split = '|';
        /// string.Join("|", renamedFiles.Select(e => e.EpisodeFile.Path)))
        paths: Vec<String> = "sonarr_episodefile_paths", split = '|';
        /// string.Join("|", renamedFiles.Select(e => e.PreviousRelativePath)))
        previous_relative_paths: Vec<String> = "sonarr_episodefile_previousrelativepaths", split = '|';
        /// string.Join("|", renamedFiles.Select(e => e.PreviousPath)))
        previous_paths: Vec<String> = "sonarr_episodefile_previouspaths", split = '|';
        /// series.Id.ToString())
        id: i64 = "sonarr_series_id";
        /// series.TvdbId.ToString())
        tvdb_id: i64 = "sonarr_series_tvdbid";
        /// series.TvMazeId.ToString())
        tvmaze_id: i64 = "sonarr_series_tvmazeid";
    }
}

env_struct! {
    /// SonarrSeriesDelete is the SeriesDelete event.
    pub struct SonarrSeriesDelete {
        /// series.Title)
        title: String = "sonarr_series_title";
        /// series.Path)
        path: String = "sonarr_series_path";
        /// series.ImdbId ?? string.Empty)
        imdb_id: String = "sonarr_series_imdbid";
        /// series.SeriesType.ToString())
        series_type: String = "sonarr_series_type";
        /// deleteMessage.DeletedFiles.ToString())
        deleted_files: String = "sonarr_series_deletedfiles";
        /// series.Id.ToString())
        id: i64 = "sonarr_series_id";
        /// series.TvdbId.ToString())
        tvdb_id: i64 = "sonarr_series_tvdbid";
        /// series.TvMazeId.ToString())
        tvmaze_id: i64 = "sonarr_series_tvmazeid";
    }
}

env_struct! {
    /// SonarrEpisodeFileDelete is the EpisodeFileDelete event.
    pub struct SonarrEpisodeFileDelete {
        /// deleteMessage.Reason.ToString())
        reason: String = "sonarr_episodefile_deletereason";
        /// series.Title)
        title: String = "sonarr_series_title";
        /// series.Path)
        path: String = "sonarr_series_path";
        /// series.ImdbId ?? string.Empty)
        imdb_id: String = "sonarr_series_imdbid";
        /// series.SeriesType.ToString())
        series_type: String = "sonarr_series_type";
        /// episodeFile.RelativePath)
        relative_path: String = "sonarr_episodefile_relativepath";
        /// Path.Combine(series.Path, episodeFile.RelativePath))
        file_path: String = "sonarr_episodefile_path";
        /// episodeFile.SeasonNumber.ToString())
        season_number: String = "sonarr_episodefile_seasonnumber";
        /// episodeFile.Quality.Quality.Name)
        quality: String = "sonarr_episodefile_quality";
        /// episodeFile.Quality.Revision.Version.ToString())
        quality_version: String = "sonarr_episodefile_qualityversion";
        /// episodeFile.ReleaseGroup ?? string.Empty)
        release_group: String = "sonarr_episodefile_releasegroup";
        /// episodeFile.SceneName ?? string.Empty)
        scene_name: String = "sonarr_episodefile_scenename";
        /// string.Join(",", episodeFile.Episodes.Value.Select(e => e.Id)))
        episode_ids: Vec<i64> = "sonarr_episodefile_episodeids", split = ',';
        /// string.Join(",", episodeFile.Episodes.Value.Select(e => e.EpisodeNumber)))
        episode_numbers: Vec<i32> = "sonarr_episodefile_episodenumbers", split = ',';
        /// string.Join(",", episodeFile.Episodes.Value.Select(e => e.AirDate)))
        episode_air_dates: Vec<String> = "sonarr_episodefile_episodeairdates", split = ',';
        /// string.Join(",", episodeFile.Episodes.Value.Select(e => e.AirDateUtc)))
        episode_air_dates_utc: Vec<DateTime<Utc>> = "sonarr_episodefile_episodeairdatesutc", split = ',';
        /// string.Join("|", episodeFile.Episodes.Value.Select(e => e.Title)))
        episode_titles: Vec<String> = "sonarr_episodefile_episodetitles", split = '|';
        /// series.Id.ToString())
        id: i64 = "sonarr_series_id";
        /// series.TvdbId.ToString())
        tvdb_id: i64 = "sonarr_series_tvdbid";
        /// series.TvMazeId.ToString())
        tvmaze_id: i64 = "sonarr_series_tvmazeid";
        /// episodeFile.Id.ToString())
        file_id: i64 = "sonarr_episodefile_id";
        /// episodeFile.Episodes.Value.Count.ToString())
        episode_count: i32 = "sonarr_episodefile_episodecount";
    }
}

env_struct! {
    /// SonarrTest has no members.
    pub struct SonarrTest {
    }
}

impl CmdEvent {
    /// returns the ApplicationUpdate event data.
    pub fn get_sonarr_application_update(&self) -> CmdResult<SonarrApplicationUpdate> {
        self.check(&Event::APPLICATION_UPDATE)?;
        SonarrApplicationUpdate::from_env()
    }

    /// returns the ApplicationUpdate event data.
    pub fn get_sonarr_health_issue(&self) -> CmdResult<SonarrHealthIssue> {
        self.check(&Event::HEALTH_ISSUE)?;
        SonarrHealthIssue::from_env()
    }

    /// returns the ApplicationUpdate event data.
    pub fn get_sonarr_test(&self) -> CmdResult<SonarrTest> {
        self.check(&Event::TEST)?;
        SonarrTest::from_env()
    }

    /// returns the Grab event data.
    pub fn get_sonarr_grab(&self) -> CmdResult<SonarrGrab> {
        self.check(&Event::GRAB)?;
        SonarrGrab::from_env()
    }

    /// returns the Download event data.
    pub fn get_sonarr_download(&self) -> CmdResult<SonarrDownload> {
        self.check(&Event::DOWNLOAD)?;
        SonarrDownload::from_env()
    }

    /// returns the Rename event data.
    pub fn get_sonarr_rename(&self) -> CmdResult<SonarrRename> {
        self.check(&Event::RENAME)?;
        SonarrRename::from_env()
    }

    /// returns the SeriesDelete event data.
    pub fn get_sonarr_series_delete(&self) -> CmdResult<SonarrSeriesDelete> {
        self.check(&Event::SERIES_DELETE)?;
        SonarrSeriesDelete::from_env()
    }

    /// returns the EpisodeFileDelete event data.
    pub fn get_sonarr_episode_file_delete(&self) -> CmdResult<SonarrEpisodeFileDelete> {
        self.check(&Event::EPISODE_FILE_DELETE)?;
        SonarrEpisodeFileDelete::from_env()
    }
}

impl Dispatcher {
    /// registers a Sonarr ApplicationUpdate callback.
    pub fn on_sonarr_application_update<F>(&self, handler: F)
    where
        F: Fn(&SonarrApplicationUpdate) -> CmdResult<()> + Send + Sync + 'static,
    {
        self.register(App::SONARR, Event::APPLICATION_UPDATE, move |cmd| {
            execute_get(cmd, CmdEvent::get_sonarr_application_update, &handler)
        });
    }

    /// registers a Sonarr Download callback.
    pub fn on_sonarr_download<F>(&self, handler: F)
    where
        F: Fn(&SonarrDownload) -> CmdResult<()> + Send + Sync + 'static,
    {
        self.register(App::SONARR, Event::DOWNLOAD, move |cmd| {
            execute_get(cmd, CmdEvent::get_sonarr_download, &handler)
        });
    }

    /// registers a Sonarr EpisodeFileDelete callback.
    pub fn on_sonarr_episode_file_delete<F>(&self, handler: F)
    where
        F: Fn(&SonarrEpisodeFileDelete) -> CmdResult<()> + Send + Sync + 'static,
    {
        self.register(App::SONARR, Event::EPISODE_FILE_DELETE, move |cmd| {
            execute_get(cmd, CmdEvent::get_sonarr_episode_file_delete, &handler)
        });
    }

    /// registers a Sonarr Grab callback.
    pub fn on_sonarr_grab<F>(&self, handler: F)
    where
        F: Fn(&SonarrGrab) -> CmdResult<()> + Send + Sync + 'static,
    {
        self.register(App::SONARR, Event::GRAB, move |cmd| {
            execute_get(cmd, CmdEvent::get_sonarr_grab, &handler)
        });
    }

    /// registers a Sonarr HealthIssue callback.
    pub fn on_sonarr_health_issue<F>(&self, handler: F)
    where
        F: Fn(&SonarrHealthIssue) -> CmdResult<()> + Send + Sync + 'static,
    {
        self.register(App::SONARR, Event::HEALTH_ISSUE, move |cmd| {
            execute_get(cmd, CmdEvent::get_sonarr_health_issue, &handler)
        });
    }

    /// registers a Sonarr Rename callback.
    pub fn on_sonarr_rename<F>(&self, handler: F)
    where
        F: Fn(&SonarrRename) -> CmdResult<()> + Send + Sync + 'static,
    {
        self.register(App::SONARR, Event::RENAME, move |cmd| {
            execute_get(cmd, CmdEvent::get_sonarr_rename, &handler)
        });
    }

    /// registers a Sonarr SeriesDelete callback.
    pub fn on_sonarr_series_delete<F>(&self, handler: F)
    where
        F: Fn(&SonarrSeriesDelete) -> CmdResult<()> + Send + Sync + 'static,
    {
        self.register(App::SONARR, Event::SERIES_DELETE, move |cmd| {
            execute_get(cmd, CmdEvent::get_sonarr_series_delete, &handler)
        });
    }

    /// registers a Sonarr Test callback.
    pub fn on_sonarr_test<F>(&self, handler: F)
    where
        F: Fn(&SonarrTest) -> CmdResult<()> + Send + Sync + 'static,
    {
        self.register(App::SONARR, Event::TEST, move |cmd| {
            execute_get(cmd, CmdEvent::get_sonarr_test, &handler)
        });
    }
}
