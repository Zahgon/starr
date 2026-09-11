use super::{CmdEvent, CmdResult, Dispatcher, Event, execute_get};
use crate::env_struct;
use crate::helpers::App;
use chrono::{DateTime, Utc};

env_struct! {
    /// ReadarrApplicationUpdate is the ApplicationUpdate event.
    pub struct ReadarrApplicationUpdate {
        /// 4.0.3.5875
        previous_version: String = "readarr_update_previousversion";
        /// 4.0.4.5909
        new_version: String = "readarr_update_newversion";
        /// Readarr updated from 4.0.3.5875 to 4.0.4.5909
        message: String = "readarr_update_message";
    }
}

env_struct! {
    /// ReadarrHealthIssue is the HealthIssue event.
    pub struct ReadarrHealthIssue {
        /// Lists unavailable due to failures: List name here
        message: String = "readarr_health_issue_message";
        /// ImportListStatusCheck
        issue_type: String = "readarr_health_issue_type";
        /// https://wiki.servarr.com/
        wiki: String = "readarr_health_issue_wiki";
        /// Warning
        level: String = "readarr_health_issue_level";
    }
}

env_struct! {
    /// ReadarrGrab is the Grab event.
    pub struct ReadarrGrab {
        /// BitBook
        release_group: String = "readarr_release_releasegroup";
        /// J.K. Rowling
        author_name: String = "readarr_author_name";
        /// J K Rowling - Harry Potter and the Order of the Phoenix 2012 Retail EPUB eBook-BitBook
        release_title: String = "readarr_release_title";
        /// 21175582 // not sure what this looks like with 2+
        grids: String = "readarr_release_grids";
        /// qBittorrent
        download_client: String = "readarr_download_client";
        /// 1
        quality_version: String = "readarr_release_qualityversion";
        /// InfoWars (Prowlarr)
        release_indexer: String = "readarr_release_indexer";
        /// 3852BA2204A84185B2B43281E53BE93D56DE5C81
        download_id: String = "readarr_download_id";
        /// EPUB
        quality: String = "readarr_release_quality";
        /// Harry Potter and the Order of the Phoenix
        titles: Vec<String> = "readarr_release_booktitles", split = '|';
        /// 649
        ids: Vec<i64> = "readarr_release_bookids", split = '|';
        /// 07/10/2003 07:00:00
        release_dates: Vec<DateTime<Utc>> = "readarr_release_bookreleasedates", split = ',';
        /// 1077326
        author_grid: i64 = "readarr_author_grid";
        /// 1279262
        size: i64 = "readarr_release_size";
        /// 1
        book_count: i32 = "readarr_release_bookcount";
        /// 4
        author_id: i64 = "readarr_author_id";
    }
}

env_struct! {
    /// ReadarrBookDelete is the BookDelete event.
    pub struct ReadarrBookDelete {
        /// Alyssa Cole
        author_name: String = "readarr_author_name";
        /// Unti Cole #6: A Novel
        title: String = "readarr_book_title";
        /// /books/Alyssa Cole
        path: String = "readarr_author_path";
        /// 33
        author_id: String = "readarr_author_id";
        /// 88514853
        gr_id: i64 = "readarr_book_goodreadsid";
        /// 7790155
        author_gr_id: i64 = "readarr_author_goodreadsid";
        /// 636
        id: i64 = "readarr_book_id";
        /// True
        deleted_files: bool = "readarr_book_deletedfiles";
    }
}

env_struct! {
    /// ReadarrBookFileDelete is the BookFileDelete event.
    pub struct ReadarrBookFileDelete {
        /// deleteMessage.Reason.ToString())
        reason: String = "readarr_delete_reason";
        /// author.Name)
        author_name: String = "readarr_author_name";
        /// book.Id.ToString())
        id: String = "readarr_book_id";
        /// book.Title)
        title: String = "readarr_book_title";
        /// bookFile.Path)
        path: String = "readarr_bookfile_path";
        /// bookFile.Quality.Quality.Name)
        quality: String = "readarr_bookfile_quality";
        /// bookFile.ReleaseGroup ?? string.Empty)
        release_group: String = "readarr_bookfile_releasegroup";
        /// bookFile.SceneName ?? string.Empty)
        scene_name: String = "readarr_bookfile_scenename";
        /// edition.Title)
        edition_name: String = "readarr_bookfile_edition_name";
        /// edition.Isbn13)
        edition_isbn13: String = "readarr_bookfile_edition_isbn13";
        /// edition.Asin)
        edition_asin: String = "readarr_bookfile_edition_asin";
        /// author.Id.ToString())
        author_id: i64 = "readarr_author_id";
        /// author.ForeignAuthorId)
        author_gr_id: i64 = "readarr_author_goodreadsid";
        /// book.ForeignBookId)
        gr_id: i64 = "readarr_book_goodreadsid";
        /// bookFile.Id.ToString())
        file_id: i64 = "readarr_bookfile_id";
        /// bookFile.Quality.Revision.Version.ToString())
        quality_version: i64 = "readarr_bookfile_qualityversion";
        /// edition.Id.ToString())
        edition_id: i64 = "readarr_bookfile_edition_id";
        /// edition.ForeignEditionId)
        edition_gr_id: i64 = "readarr_bookfile_edition_goodreadsid";
    }
}

env_struct! {
    /// ReadarrAuthorDelete is the AuthorDelete event.
    pub struct ReadarrAuthorDelete {
        /// author.Name)
        author_name: String = "readarr_author_name";
        /// author.Path)
        path: String = "readarr_author_path";
        /// author.Id.ToString())
        author_id: i64 = "readarr_author_id";
        /// author.ForeignAuthorId)
        author_gr_id: i64 = "readarr_author_goodreadsid";
        /// deleteMessage.DeletedFiles.ToString())
        deleted_files: bool = "readarr_author_deletedfiles";
    }
}

env_struct! {
    /// ReadarrRename is the Rename event.
    pub struct ReadarrRename {
        /// author.Metadata.Value.Name)
        author_name: String = "readarr_author_name";
        /// author.Path)
        path: String = "readarr_author_path";
        /// author.Id.ToString())
        author_id: i64 = "readarr_author_id";
        /// author.Metadata.Value.ForeignAuthorId)
        author_gr_id: i64 = "readarr_author_grid";
    }
}

env_struct! {
    /// ReadarrDownload is Download event.
    pub struct ReadarrDownload {
        /// author.Metadata.Value.Name)
        author_name: String = "readarr_author_name";
        /// author.Path)
        path: String = "readarr_author_path";
        /// book.Title)
        title: String = "readarr_book_title";
        /// book.ReleaseDate.ToString())
        release_date: String = "readarr_book_releasedate";
        /// message.DownloadClient ?? string.Empty)
        download_client: String = "readarr_download_client";
        /// message.DownloadId ?? string.Empty)
        download_id: String = "readarr_download_id";
        /// string.Join("|", message.BookFiles.Select(e => e.Path)))
        added_book_paths: Vec<String> = "readarr_addedbookpaths", split = '|';
        /// string.Join("|", message.OldFiles.Select(e => e.Path)))
        deleted_paths: Vec<String> = "readarr_deletedpaths", split = '|';
        /// author.Id.ToString())
        author_id: i64 = "readarr_author_id";
        /// author.Metadata.Value.ForeignAuthorId)
        author_gr_id: i64 = "readarr_author_grid";
        /// book.Id.ToString())
        id: i64 = "readarr_book_id";
        /// book.Editions.Value.Single(e => e.Monitored).ForeignEditionId.ToString())
        gr_id: i64 = "readarr_book_grid";
    }
}

env_struct! {
    /// ReadarrTrackRetag is the TrackRetag event.
    pub struct ReadarrTrackRetag {
        /// book.ReleaseDate.ToString())
        release_date: DateTime<Utc> = "readarr_book_releasedate";
        /// author.Metadata.Value.Name)
        author_name: String = "readarr_author_name";
        /// author.Path)
        path: String = "readarr_author_path";
        /// book.Title)
        title: String = "readarr_book_title";
        /// bookFile.Path)
        file_path: String = "readarr_bookfile_path";
        /// bookFile.Quality.Quality.Name)
        quality: String = "readarr_bookfile_quality";
        /// bookFile.ReleaseGroup ?? string.Empty)
        release_group: String = "readarr_bookfile_releasegroup";
        /// bookFile.SceneName ?? string.Empty)
        scene_name: String = "readarr_bookfile_scenename";
        /// message.Diff.ToJson())
        tags_diff: String = "readarr_tags_diff";
        /// author.Id.ToString())
        author_id: i64 = "readarr_author_id";
        /// author.Metadata.Value.ForeignAuthorId)
        author_gr_id: i64 = "readarr_author_grid";
        /// book.Id.ToString())
        id: i64 = "readarr_book_id";
        /// book.Editions.Value.Single(e => e.Monitored).ForeignEditionId.ToString())
        gr_id: i64 = "readarr_book_grid";
        /// bookFile.Id.ToString())
        file_id: i64 = "readarr_bookfile_id";
        /// bookFile.Quality.Revision.Version.ToString())
        quality_version: i64 = "readarr_bookfile_qualityversion";
        /// message.Scrubbed.ToString())
        scrubbed: bool = "readarr_tags_scrubbed";
    }
}

env_struct! {
    /// ReadarrTest has no members.
    pub struct ReadarrTest {
    }
}

impl CmdEvent {
    /// returns the ApplicationUpdate event data.
    pub fn get_readarr_application_update(&self) -> CmdResult<ReadarrApplicationUpdate> {
        self.check(&Event::APPLICATION_UPDATE)?;
        ReadarrApplicationUpdate::from_env()
    }

    /// returns the ApplicationUpdate event data.
    pub fn get_readarr_health_issue(&self) -> CmdResult<ReadarrHealthIssue> {
        self.check(&Event::HEALTH_ISSUE)?;
        ReadarrHealthIssue::from_env()
    }

    /// returns the Grab event data.
    pub fn get_readarr_grab(&self) -> CmdResult<ReadarrGrab> {
        self.check(&Event::GRAB)?;
        ReadarrGrab::from_env()
    }

    /// returns the BookDelete event data.
    pub fn get_readarr_book_delete(&self) -> CmdResult<ReadarrBookDelete> {
        self.check(&Event::BOOK_DELETE)?;
        ReadarrBookDelete::from_env()
    }

    /// returns the AuthorDelete event data.
    pub fn get_readarr_author_delete(&self) -> CmdResult<ReadarrAuthorDelete> {
        self.check(&Event::AUTHOR_DELETE)?;
        ReadarrAuthorDelete::from_env()
    }

    /// returns the BookFileDelete event data.
    pub fn get_readarr_book_file_delete(&self) -> CmdResult<ReadarrBookFileDelete> {
        self.check(&Event::BOOK_FILE_DELETE)?;
        ReadarrBookFileDelete::from_env()
    }

    /// returns the Download event data.
    pub fn get_readarr_download(&self) -> CmdResult<ReadarrDownload> {
        self.check(&Event::DOWNLOAD)?;
        ReadarrDownload::from_env()
    }

    /// returns the Rename event data.
    pub fn get_readarr_rename(&self) -> CmdResult<ReadarrRename> {
        self.check(&Event::RENAME)?;
        ReadarrRename::from_env()
    }

    /// returns the TrackRetag event data.
    pub fn get_readarr_track_retag(&self) -> CmdResult<ReadarrTrackRetag> {
        self.check(&Event::TRACK_RETAG)?;
        ReadarrTrackRetag::from_env()
    }

    /// returns the ApplicationUpdate event data.
    pub fn get_readarr_test(&self) -> CmdResult<ReadarrTest> {
        self.check(&Event::TEST)?;
        ReadarrTest::from_env()
    }
}

impl Dispatcher {
    /// registers a Readarr ApplicationUpdate callback.
    pub fn on_readarr_application_update<F>(&self, handler: F)
    where
        F: Fn(&ReadarrApplicationUpdate) -> CmdResult<()> + Send + Sync + 'static,
    {
        self.register(App::READARR, Event::APPLICATION_UPDATE, move |cmd| {
            execute_get(cmd, CmdEvent::get_readarr_application_update, &handler)
        });
    }

    /// registers a Readarr AuthorDelete callback.
    pub fn on_readarr_author_delete<F>(&self, handler: F)
    where
        F: Fn(&ReadarrAuthorDelete) -> CmdResult<()> + Send + Sync + 'static,
    {
        self.register(App::READARR, Event::AUTHOR_DELETE, move |cmd| {
            execute_get(cmd, CmdEvent::get_readarr_author_delete, &handler)
        });
    }

    /// registers a Readarr BookDelete callback.
    pub fn on_readarr_book_delete<F>(&self, handler: F)
    where
        F: Fn(&ReadarrBookDelete) -> CmdResult<()> + Send + Sync + 'static,
    {
        self.register(App::READARR, Event::BOOK_DELETE, move |cmd| {
            execute_get(cmd, CmdEvent::get_readarr_book_delete, &handler)
        });
    }

    /// registers a Readarr BookFileDelete callback.
    pub fn on_readarr_book_file_delete<F>(&self, handler: F)
    where
        F: Fn(&ReadarrBookFileDelete) -> CmdResult<()> + Send + Sync + 'static,
    {
        self.register(App::READARR, Event::BOOK_FILE_DELETE, move |cmd| {
            execute_get(cmd, CmdEvent::get_readarr_book_file_delete, &handler)
        });
    }

    /// registers a Readarr Download callback.
    pub fn on_readarr_download<F>(&self, handler: F)
    where
        F: Fn(&ReadarrDownload) -> CmdResult<()> + Send + Sync + 'static,
    {
        self.register(App::READARR, Event::DOWNLOAD, move |cmd| {
            execute_get(cmd, CmdEvent::get_readarr_download, &handler)
        });
    }

    /// registers a Readarr Grab callback.
    pub fn on_readarr_grab<F>(&self, handler: F)
    where
        F: Fn(&ReadarrGrab) -> CmdResult<()> + Send + Sync + 'static,
    {
        self.register(App::READARR, Event::GRAB, move |cmd| {
            execute_get(cmd, CmdEvent::get_readarr_grab, &handler)
        });
    }

    /// registers a Readarr HealthIssue callback.
    pub fn on_readarr_health_issue<F>(&self, handler: F)
    where
        F: Fn(&ReadarrHealthIssue) -> CmdResult<()> + Send + Sync + 'static,
    {
        self.register(App::READARR, Event::HEALTH_ISSUE, move |cmd| {
            execute_get(cmd, CmdEvent::get_readarr_health_issue, &handler)
        });
    }

    /// registers a Readarr Rename callback.
    pub fn on_readarr_rename<F>(&self, handler: F)
    where
        F: Fn(&ReadarrRename) -> CmdResult<()> + Send + Sync + 'static,
    {
        self.register(App::READARR, Event::RENAME, move |cmd| {
            execute_get(cmd, CmdEvent::get_readarr_rename, &handler)
        });
    }

    /// registers a Readarr Test callback.
    pub fn on_readarr_test<F>(&self, handler: F)
    where
        F: Fn(&ReadarrTest) -> CmdResult<()> + Send + Sync + 'static,
    {
        self.register(App::READARR, Event::TEST, move |cmd| {
            execute_get(cmd, CmdEvent::get_readarr_test, &handler)
        });
    }

    /// registers a Readarr TrackRetag callback.
    pub fn on_readarr_track_retag<F>(&self, handler: F)
    where
        F: Fn(&ReadarrTrackRetag) -> CmdResult<()> + Send + Sync + 'static,
    {
        self.register(App::READARR, Event::TRACK_RETAG, move |cmd| {
            execute_get(cmd, CmdEvent::get_readarr_track_retag, &handler)
        });
    }
}
