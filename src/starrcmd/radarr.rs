use super::{CmdEvent, CmdResult, Dispatcher, Event, execute_get};
use crate::env_struct;
use crate::helpers::App;
use chrono::{DateTime, Utc};

env_struct! {
    /// RadarrApplicationUpdate is the ApplicationUpdate event.
    pub struct RadarrApplicationUpdate {
        /// 4.0.3.5875
        previous_version: String = "radarr_update_previousversion";
        /// 4.0.4.5909
        new_version: String = "radarr_update_newversion";
        /// Radarr updated from 4.0.3.5875 to 4.0.4.5909
        message: String = "radarr_update_message";
    }
}

env_struct! {
    /// RadarrDownload is the Download event.
    pub struct RadarrDownload {
        /// Value of the `radarr_movie_physical_release_date` environment variable.
        release_date: DateTime<Utc> = "radarr_movie_physical_release_date";
        /// 2/10/2011 12:00:00 AM
        in_cinemas: DateTime<Utc> = "radarr_movie_in_cinemas_date";
        /// /movies/Just Go with It (2011)/Just.Go.with.It.2011.Bluray-1080p.mkv
        file_path: String = "radarr_moviefile_path";
        /// tt1564367
        imdb_id: String = "radarr_movie_imdbid";
        /// Just.Go.with.It.2011.1080p.BluRay.x264-OFT
        scene_name: String = "radarr_moviefile_scenename";
        /// OFT
        release_group: String = "radarr_moviefile_releasegroup";
        /// string F3D870942BFDD643488852284E917336170CEA00
        download_id: String = "radarr_download_id";
        /// /downloads/Seeding/Just.Go.with.It.2011.1080p.BluRay.x264-OFT
        source_folder: String = "radarr_moviefile_sourcefolder";
        /// /movies/Just Go with It (2011)
        path: String = "radarr_movie_path";
        /// Just.Go.with.It.2011.Bluray-1080p.mkv
        relative_path: String = "radarr_moviefile_relativepath";
        /// Deluge
        download_client: String = "radarr_download_client";
        /// /downloads/Seeding/Just.Go.with.It.2011.1080p.BluRay.x264-OFT/Just.Go.with.It.2011.1080p.BluRay.x264-OFT.mkv
        source_path: String = "radarr_moviefile_sourcepath";
        /// Bluray-1080p
        quality: String = "radarr_moviefile_quality";
        /// Just Go with It
        title: String = "radarr_movie_title";
        /// Value of the `radarr_deletedrelativepaths` environment variable.
        deleted_relative_paths: Vec<String> = "radarr_deletedrelativepaths", split = '|';
        /// Value of the `radarr_deletedpaths` environment variable.
        deleted_paths: Vec<String> = "radarr_deletedpaths", split = '|';
        /// 3594
        file_id: i64 = "radarr_moviefile_id";
        /// 2011
        year: i32 = "radarr_movie_year";
        /// 50546
        tmdb_id: i64 = "radarr_movie_tmdbid";
        /// 924
        id: i64 = "radarr_movie_id";
        /// 1
        quality_version: i64 = "radarr_moviefile_qualityversion";
        /// False
        is_upgrade: bool = "radarr_isupgrade";
    }
}

env_struct! {
    /// RadarrGrab is the Grab event.
    pub struct RadarrGrab {
        /// 1/19/2006 12:00:00 AM
        release_date: DateTime<Utc> = "radarr_movie_physical_release_date";
        /// 11/22/2005 12:00:00 AM
        in_cinemas: DateTime<Utc> = "radarr_movie_in_cinemas_date";
        /// SLOT
        release_group: String = "radarr_release_releasegroup";
        /// tt0448172
        imdb_id: String = "radarr_movie_imdbid";
        /// E63FAFFAAA0DEE42F0846348A9C0657BC53E7AA5
        download_id: String = "radarr_download_id";
        /// 8MM 2 2005 1080p BluRay x264
        release_title: String = "radarr_release_title";
        /// Bluray-1080p
        quality: String = "radarr_release_quality";
        /// Deluge
        download_client: String = "radarr_download_client";
        /// Inexilator (Prowlarr)
        release_indexer: String = "radarr_release_indexer";
        /// 8MM 2
        title: String = "radarr_movie_title";
        /// 1
        quality_version: i64 = "radarr_release_qualityversion";
        /// 0
        indexer_flags: i64 = "radarr_indexerflags";
        /// 2158221056
        size: i64 = "radarr_release_size";
        /// 2005
        year: i32 = "radarr_movie_year";
        /// 7295
        tmdb_id: i64 = "radarr_movie_tmdbid";
        /// 339
        id: i64 = "radarr_movie_id";
    }
}

env_struct! {
    /// RadarrHealthIssue is the HealthIssue event.
    pub struct RadarrHealthIssue {
        /// Lists unavailable due to failures: List name here
        message: String = "radarr_health_issue_message";
        /// ImportListStatusCheck
        issue_type: String = "radarr_health_issue_type";
        /// https://wiki.servarr.com/radarr/system#lists-are-unavailable-due-to-failures
        wiki: String = "radarr_health_issue_wiki";
        /// Warning
        level: String = "radarr_health_issue_level";
    }
}

env_struct! {
    /// RadarrMovieFileDelete is the MovieFileDelete event.
    pub struct RadarrMovieFileDelete {
        /// Upgrade
        reason: String = "radarr_moviefile_deletereason";
        /// /movies/The French Dispatch (2021)/The.French.Dispatch.2021.Bluray-720p.mkv
        file_path: String = "radarr_moviefile_path";
        /// The.French.Dispatch.2021.720p.BluRay.x264-WoAT
        scene_name: String = "radarr_moviefile_scenename";
        /// tt8847712
        imdb_id: String = "radarr_movie_imdbid";
        /// WoAT
        release_group: String = "radarr_moviefile_releasegroup";
        /// /movies/The French Dispatch (2021)
        path: String = "radarr_movie_path";
        /// The.French.Dispatch.2021.Bluray-720p.mkv
        relative_path: String = "radarr_moviefile_relativepath";
        /// 542178
        tmdb_id: String = "radarr_movie_tmdbid";
        /// Bluray-720p
        quality: String = "radarr_moviefile_quality";
        /// The French Dispatch
        title: String = "radarr_movie_title";
        /// 3531
        file_id: i64 = "radarr_moviefile_id";
        /// 2021
        year: i32 = "radarr_movie_year";
        /// 3593317970
        size: i64 = "radarr_moviefile_size";
        /// 2173
        id: i64 = "radarr_movie_id";
        /// 1
        quality_version: i64 = "radarr_moviefile_qualityversion";
    }
}

env_struct! {
    /// RadarrMovieDelete is the MovieDelete event.
    pub struct RadarrMovieDelete {
        /// The French Dispatch
        title: String = "radarr_movie_title";
        /// /movies/The French Dispatch (2021)
        path: String = "radarr_movie_path";
        /// tt8847712
        imdb_id: String = "radarr_movie_imdbid";
        /// XXX: no example. Does this need a split?
        delete_files: String = "radarr_movie_deletedfiles";
        /// 2173
        id: i64 = "radarr_movie_id";
        /// 2021
        year: i32 = "radarr_movie_year";
        /// 542178
        tmdb_id: i64 = "radarr_movie_tmdbid";
        /// 3593317970
        size: i64 = "radarr_movie_folder_size";
    }
}

env_struct! {
    /// RadarrRename is the Rename event.
    pub struct RadarrRename {
        /// 11/22/2005 12:00:00 AM
        in_cinemas: DateTime<Utc> = "radarr_movie_in_cinemas_date";
        /// Value of the `radarr_movie_physical_release_date` environment variable.
        release_date: DateTime<Utc> = "radarr_movie_physical_release_date";
        /// /movies/The French Dispatch (2021)
        path: String = "radarr_movie_path";
        /// tt8847712
        imdb_id: String = "radarr_movie_imdbid";
        /// Value of the `radarr_moviefile_ids` environment variable.
        file_ids: Vec<i64> = "radarr_moviefile_ids", split = ',';
        /// Value of the `radarr_moviefile_relativepaths` environment variable.
        relative_paths: Vec<String> = "radarr_moviefile_relativepaths", split = '|';
        /// Value of the `radarr_moviefile_paths` environment variable.
        paths: Vec<String> = "radarr_moviefile_paths", split = '|';
        /// Value of the `radarr_moviefile_previousrelativepaths` environment variable.
        previous_relative_paths: Vec<String> = "radarr_moviefile_previousrelativepaths", split = '|';
        /// Value of the `radarr_moviefile_previouspaths` environment variable.
        previous_paths: Vec<String> = "radarr_moviefile_previouspaths", split = '|';
        /// 2173
        id: i64 = "radarr_movie_id";
        /// 2021
        year: i32 = "radarr_movie_year";
        /// 542178
        tmdb_id: i64 = "radarr_movie_tmdbid";
    }
}

env_struct! {
    /// RadarrTest has no members.
    pub struct RadarrTest {
    }
}

impl CmdEvent {
    /// returns the HealthIssue event data.
    pub fn get_radarr_health_issue(&self) -> CmdResult<RadarrHealthIssue> {
        self.check(&Event::HEALTH_ISSUE)?;
        RadarrHealthIssue::from_env()
    }

    /// returns the ApplicationUpdate event data.
    pub fn get_radarr_application_update(&self) -> CmdResult<RadarrApplicationUpdate> {
        self.check(&Event::APPLICATION_UPDATE)?;
        RadarrApplicationUpdate::from_env()
    }

    /// returns the Download event data.
    pub fn get_radarr_download(&self) -> CmdResult<RadarrDownload> {
        self.check(&Event::DOWNLOAD)?;
        RadarrDownload::from_env()
    }

    /// returns the Grab event data.
    pub fn get_radarr_grab(&self) -> CmdResult<RadarrGrab> {
        self.check(&Event::GRAB)?;
        RadarrGrab::from_env()
    }

    /// returns the MovieFileDelete event data.
    pub fn get_radarr_movie_file_delete(&self) -> CmdResult<RadarrMovieFileDelete> {
        self.check(&Event::MOVIE_FILE_DELETE)?;
        RadarrMovieFileDelete::from_env()
    }

    /// returns the Test event data.
    pub fn get_radarr_test(&self) -> CmdResult<RadarrTest> {
        self.check(&Event::TEST)?;
        RadarrTest::from_env()
    }

    /// returns the MovieDelete event data.
    pub fn get_radarr_movie_delete(&self) -> CmdResult<RadarrMovieDelete> {
        self.check(&Event::MOVIE_DELETE)?;
        RadarrMovieDelete::from_env()
    }

    /// returns the Rename event data.
    pub fn get_radarr_rename(&self) -> CmdResult<RadarrRename> {
        self.check(&Event::RENAME)?;
        RadarrRename::from_env()
    }
}

impl Dispatcher {
    /// registers a Radarr ApplicationUpdate callback.
    pub fn on_radarr_application_update<F>(&self, handler: F)
    where
        F: Fn(&RadarrApplicationUpdate) -> CmdResult<()> + Send + Sync + 'static,
    {
        self.register(App::RADARR, Event::APPLICATION_UPDATE, move |cmd| {
            execute_get(cmd, CmdEvent::get_radarr_application_update, &handler)
        });
    }

    /// registers a Radarr Download callback.
    pub fn on_radarr_download<F>(&self, handler: F)
    where
        F: Fn(&RadarrDownload) -> CmdResult<()> + Send + Sync + 'static,
    {
        self.register(App::RADARR, Event::DOWNLOAD, move |cmd| {
            execute_get(cmd, CmdEvent::get_radarr_download, &handler)
        });
    }

    /// registers a Radarr Grab callback.
    pub fn on_radarr_grab<F>(&self, handler: F)
    where
        F: Fn(&RadarrGrab) -> CmdResult<()> + Send + Sync + 'static,
    {
        self.register(App::RADARR, Event::GRAB, move |cmd| {
            execute_get(cmd, CmdEvent::get_radarr_grab, &handler)
        });
    }

    /// registers a Radarr HealthIssue callback.
    pub fn on_radarr_health_issue<F>(&self, handler: F)
    where
        F: Fn(&RadarrHealthIssue) -> CmdResult<()> + Send + Sync + 'static,
    {
        self.register(App::RADARR, Event::HEALTH_ISSUE, move |cmd| {
            execute_get(cmd, CmdEvent::get_radarr_health_issue, &handler)
        });
    }

    /// registers a Radarr MovieDelete callback.
    pub fn on_radarr_movie_delete<F>(&self, handler: F)
    where
        F: Fn(&RadarrMovieDelete) -> CmdResult<()> + Send + Sync + 'static,
    {
        self.register(App::RADARR, Event::MOVIE_DELETE, move |cmd| {
            execute_get(cmd, CmdEvent::get_radarr_movie_delete, &handler)
        });
    }

    /// registers a Radarr MovieFileDelete callback.
    pub fn on_radarr_movie_file_delete<F>(&self, handler: F)
    where
        F: Fn(&RadarrMovieFileDelete) -> CmdResult<()> + Send + Sync + 'static,
    {
        self.register(App::RADARR, Event::MOVIE_FILE_DELETE, move |cmd| {
            execute_get(cmd, CmdEvent::get_radarr_movie_file_delete, &handler)
        });
    }

    /// registers a Radarr Rename callback.
    pub fn on_radarr_rename<F>(&self, handler: F)
    where
        F: Fn(&RadarrRename) -> CmdResult<()> + Send + Sync + 'static,
    {
        self.register(App::RADARR, Event::RENAME, move |cmd| {
            execute_get(cmd, CmdEvent::get_radarr_rename, &handler)
        });
    }

    /// registers a Radarr Test callback.
    pub fn on_radarr_test<F>(&self, handler: F)
    where
        F: Fn(&RadarrTest) -> CmdResult<()> + Send + Sync + 'static,
    {
        self.register(App::RADARR, Event::TEST, move |cmd| {
            execute_get(cmd, CmdEvent::get_radarr_test, &handler)
        });
    }
}
