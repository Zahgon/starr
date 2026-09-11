use super::{CmdEvent, CmdResult, Dispatcher, Event, execute_get};
use crate::env_struct;
use crate::helpers::App;
use chrono::{DateTime, Utc};

env_struct! {
    /// LidarrApplicationUpdate is the ApplicationUpdate event.
    pub struct LidarrApplicationUpdate {
        /// 4.0.3.5875
        previous_version: String = "lidarr_update_previousversion";
        /// 4.0.4.5909
        new_version: String = "lidarr_update_newversion";
        /// Lidarr updated from 4.0.3.5875 to 4.0.4.5909
        message: String = "lidarr_update_message";
    }
}

env_struct! {
    /// LidarrHealthIssue is the HealthIssue event.
    pub struct LidarrHealthIssue {
        /// Lists unavailable due to failures: List name here
        message: String = "lidarr_health_issue_message";
        /// ImportListStatusCheck
        issue_type: String = "lidarr_health_issue_type";
        /// https://wiki.servarr.com/lidarr/
        wiki: String = "lidarr_health_issue_wiki";
        /// Warning
        level: String = "lidarr_health_issue_level";
    }
}

env_struct! {
    /// LidarrGrab is the Grab event.
    pub struct LidarrGrab {
        /// Deluge
        download_client: String = "lidarr_download_client";
        /// Tom Petty and the Heartbreakers
        artist_name: String = "lidarr_artist_name";
        /// f93dbc64-6f08-4033-bcc7-8a0bb4689849
        mbid: String = "lidarr_artist_mbid";
        /// Indexilate (Prowlarr)
        indexer: String = "lidarr_release_indexer";
        /// FLAC
        quality: String = "lidarr_release_quality";
        /// Value of the `lidarr_release_releasegroup` environment variable.
        release_group: String = "lidarr_release_releasegroup";
        /// Tom Petty & The Heartbreakers - Mojo (2010) (FLAC (tracks + cue))
        release_title: String = "lidarr_release_title";
        /// 4A87D9F5F92D82DF4076463E90CC49F27077CB10
        download_id: String = "lidarr_download_id";
        /// Group
        artist_type: String = "lidarr_artist_type";
        /// 4/21/2010 12:00:00 AM
        release_dates: Vec<DateTime<Utc>> = "lidarr_release_albumreleasedates", split = ',';
        /// 75f6f410-73e6-485b-898d-6fdaea4c0266
        album_mbids: Vec<String> = "lidarr_release_albummbids", split = '|';
        /// Mojo
        titles: Vec<String> = "lidarr_release_albumtitles", split = '|';
        /// 1
        album_count: i32 = "lidarr_release_albumcount";
        /// 433061888
        size: i64 = "lidarr_release_size";
        /// 262
        artist_id: i64 = "lidarr_artist_id";
        /// 1
        quality_verson: i64 = "lidarr_release_qualityversion";
    }
}

env_struct! {
    /// LidarrAlbumDownload is the AlbumDownload event.
    pub struct LidarrAlbumDownload {
        /// album.ReleaseDate.ToString())
        release_date: DateTime<Utc> = "lidarr_album_releasedate";
        /// artist.Metadata.Value.Name)
        artist_name: String = "lidarr_artist_name";
        /// artist.Path)
        path: String = "lidarr_artist_path";
        /// artist.Metadata.Value.ForeignArtistId)
        artist_mbid: String = "lidarr_artist_mbid";
        /// artist.Metadata.Value.Type)
        artist_type: String = "lidarr_artist_type";
        /// album.Title)
        title: String = "lidarr_album_title";
        /// album.ForeignAlbumId)
        mbid: String = "lidarr_album_mbid";
        /// release.ForeignReleaseId)
        album_release_mbid: String = "lidarr_albumrelease_mbid";
        /// message.DownloadClient ?? string.Empty)
        download_client: String = "lidarr_download_client";
        /// message.DownloadId ?? string.Empty)
        download_id: String = "lidarr_download_id";
        /// string.Join("|", message.TrackFiles.Select(e => e.Path)))
        added_track_paths: Vec<String> = "lidarr_addedtrackpaths", split = '|';
        /// string.Join("|", message.OldFiles.Select(e => e.Path)))
        deleted_paths: Vec<String> = "lidarr_deletedpaths", split = '|';
        /// artist.Id.ToString())
        artist_id: i64 = "lidarr_artist_id";
        /// album.Id.ToString())
        album_id: i64 = "lidarr_album_id";
    }
}

env_struct! {
    /// LidarrRename is the Rename event.
    pub struct LidarrRename {
        /// artist.Metadata.Value.Name)
        artist_name: String = "lidarr_artist_name";
        /// artist.Path)
        path: String = "lidarr_artist_path";
        /// artist.Metadata.Value.ForeignArtistId)
        artist_mbid: String = "lidarr_artist_mbid";
        /// artist.Metadata.Value.Type)
        artist_type: String = "lidarr_artist_type";
        /// artist.Id.ToString())
        artist_id: i64 = "lidarr_artist_id";
    }
}

env_struct! {
    /// LidarrTrackRetag is the TrackRetag event.
    pub struct LidarrTrackRetag {
        /// album.ReleaseDate.ToString())
        release_date: DateTime<Utc> = "lidarr_album_releasedate";
        /// artist.Metadata.Value.Name)
        artist_name: String = "lidarr_artist_name";
        /// artist.Path)
        path: String = "lidarr_artist_path";
        /// artist.Metadata.Value.ForeignArtistId)
        artist_mbid: String = "lidarr_artist_mbid";
        /// artist.Metadata.Value.Type)
        artist_type: String = "lidarr_artist_type";
        /// album.Title)
        title: String = "lidarr_album_title";
        /// album.ForeignAlbumId)
        mbid: String = "lidarr_album_mbid";
        /// release.ForeignReleaseId)
        album_release_mbid: String = "lidarr_albumrelease_mbid";
        /// trackFile.Tracks.Value.Count.ToString())
        track_count: String = "lidarr_trackfile_trackcount";
        /// trackFile.Path)
        file_path: String = "lidarr_trackfile_path";
        /// trackFile.Quality.Quality.Name)
        quality: String = "lidarr_trackfile_quality";
        /// trackFile.ReleaseGroup ?? string.Empty)
        release_group: String = "lidarr_trackfile_releasegroup";
        /// trackFile.SceneName ?? string.Empty)
        scene_name: String = "lidarr_trackfile_scenename";
        /// message.Diff.ToJson())
        tags_diff: String = "lidarr_tags_diff";
        /// string.Join(",", trackFile.Tracks.Value.Select(e => e.TrackNumber)))
        track_numbers: Vec<i32> = "lidarr_trackfile_tracknumbers", split = ',';
        /// string.Join("|", trackFile.Tracks.Value.Select(e => e.Title)))
        track_titles: Vec<String> = "lidarr_trackfile_tracktitles", split = '|';
        /// artist.Id.ToString())
        artist_id: i64 = "lidarr_artist_id";
        /// album.Id.ToString())
        id: i64 = "lidarr_album_id";
        /// trackFile.Id.ToString())
        file_id: i64 = "lidarr_trackfile_id";
        /// trackFile.Quality.Revision.Version.ToString())
        quality_version: i64 = "lidarr_trackfile_qualityversion";
        /// message.Scrubbed.ToString())
        tags_scrubbed: bool = "lidarr_tags_scrubbed";
    }
}

env_struct! {
    /// LidarrTest has no members.
    pub struct LidarrTest {
    }
}

impl CmdEvent {
    /// returns the ApplicationUpdate event data.
    pub fn get_lidarr_application_update(&self) -> CmdResult<LidarrApplicationUpdate> {
        self.check(&Event::APPLICATION_UPDATE)?;
        LidarrApplicationUpdate::from_env()
    }

    /// returns the ApplicationUpdate event data.
    pub fn get_lidarr_health_issue(&self) -> CmdResult<LidarrHealthIssue> {
        self.check(&Event::HEALTH_ISSUE)?;
        LidarrHealthIssue::from_env()
    }

    /// returns the Grab event data.
    pub fn get_lidarr_grab(&self) -> CmdResult<LidarrGrab> {
        self.check(&Event::GRAB)?;
        LidarrGrab::from_env()
    }

    /// returns the AlbumDownload event data.
    pub fn get_lidarr_album_download(&self) -> CmdResult<LidarrAlbumDownload> {
        self.check(&Event::ALBUM_DOWNLOAD)?;
        LidarrAlbumDownload::from_env()
    }

    /// returns the Rename event data.
    pub fn get_lidarr_rename(&self) -> CmdResult<LidarrRename> {
        self.check(&Event::RENAME)?;
        LidarrRename::from_env()
    }

    /// returns the TrackRetag event data.
    pub fn get_lidarr_track_retag(&self) -> CmdResult<LidarrTrackRetag> {
        self.check(&Event::TRACK_RETAG)?;
        LidarrTrackRetag::from_env()
    }

    /// returns the ApplicationUpdate event data.
    pub fn get_lidarr_test(&self) -> CmdResult<LidarrTest> {
        self.check(&Event::TEST)?;
        LidarrTest::from_env()
    }
}

impl Dispatcher {
    /// registers a Lidarr ApplicationUpdate callback.
    pub fn on_lidarr_application_update<F>(&self, handler: F)
    where
        F: Fn(&LidarrApplicationUpdate) -> CmdResult<()> + Send + Sync + 'static,
    {
        self.register(App::LIDARR, Event::APPLICATION_UPDATE, move |cmd| {
            execute_get(cmd, CmdEvent::get_lidarr_application_update, &handler)
        });
    }

    /// registers a Lidarr HealthIssue callback.
    pub fn on_lidarr_health_issue<F>(&self, handler: F)
    where
        F: Fn(&LidarrHealthIssue) -> CmdResult<()> + Send + Sync + 'static,
    {
        self.register(App::LIDARR, Event::HEALTH_ISSUE, move |cmd| {
            execute_get(cmd, CmdEvent::get_lidarr_health_issue, &handler)
        });
    }

    /// registers a Lidarr Grab callback.
    pub fn on_lidarr_grab<F>(&self, handler: F)
    where
        F: Fn(&LidarrGrab) -> CmdResult<()> + Send + Sync + 'static,
    {
        self.register(App::LIDARR, Event::GRAB, move |cmd| {
            execute_get(cmd, CmdEvent::get_lidarr_grab, &handler)
        });
    }

    /// registers a Lidarr AlbumDownload callback.
    pub fn on_lidarr_album_download<F>(&self, handler: F)
    where
        F: Fn(&LidarrAlbumDownload) -> CmdResult<()> + Send + Sync + 'static,
    {
        self.register(App::LIDARR, Event::ALBUM_DOWNLOAD, move |cmd| {
            execute_get(cmd, CmdEvent::get_lidarr_album_download, &handler)
        });
    }

    /// registers a Lidarr Rename callback.
    pub fn on_lidarr_rename<F>(&self, handler: F)
    where
        F: Fn(&LidarrRename) -> CmdResult<()> + Send + Sync + 'static,
    {
        self.register(App::LIDARR, Event::RENAME, move |cmd| {
            execute_get(cmd, CmdEvent::get_lidarr_rename, &handler)
        });
    }

    /// registers a Lidarr TrackRetag callback.
    pub fn on_lidarr_track_retag<F>(&self, handler: F)
    where
        F: Fn(&LidarrTrackRetag) -> CmdResult<()> + Send + Sync + 'static,
    {
        self.register(App::LIDARR, Event::TRACK_RETAG, move |cmd| {
            execute_get(cmd, CmdEvent::get_lidarr_track_retag, &handler)
        });
    }

    /// registers a Lidarr Test callback.
    pub fn on_lidarr_test<F>(&self, handler: F)
    where
        F: Fn(&LidarrTest) -> CmdResult<()> + Send + Sync + 'static,
    {
        self.register(App::LIDARR, Event::TEST, move |cmd| {
            execute_get(cmd, CmdEvent::get_lidarr_test, &handler)
        });
    }
}
