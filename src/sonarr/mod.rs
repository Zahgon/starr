//! The SDK client for the Sonarr API.

mod autotagging;
mod blocklist;
mod calendar;
mod command;
mod config_host_ui_importlist;
mod customfilter;
mod customformat;
mod delayprofile;
mod diskspace;
mod downloadclient;
mod downloadclientconfig;
mod episode;
mod episodefile;
mod exclusions;
mod feed;
mod filesystem;
mod health;
mod history;
mod importlist;
mod indexer;
mod indexerconfig;
mod indexerflag;
mod language;
mod languageprofile;
mod localization;
mod log;
mod manualimport;
mod mediacover;
mod mediamanagement;
mod metadata;
mod naming;
mod notification;
mod parse;
mod qualitydefinition;
mod qualityprofile;
mod queue;
mod release;
mod releaseprofile;
mod remotepathmapping;
mod rename;
mod rootfolder;
mod seasonpass;
mod series;
mod system;
mod tag;
mod update;
mod wanted;

pub use autotagging::*;
pub use blocklist::*;
pub use calendar::*;
pub use command::*;
pub use config_host_ui_importlist::*;
pub use customfilter::*;
pub use customformat::*;
pub use delayprofile::*;
pub use diskspace::*;
pub use downloadclient::*;
pub use downloadclientconfig::*;
pub use episode::*;
pub use episodefile::*;
pub use exclusions::*;
pub use feed::*;
pub use filesystem::*;
pub use health::*;
pub use history::*;
pub use importlist::*;
pub use indexer::*;
pub use indexerconfig::*;
pub use indexerflag::*;
pub use language::*;
pub use languageprofile::*;
pub use localization::*;
pub use log::*;
pub use manualimport::*;
pub use mediamanagement::*;
pub use metadata::*;
pub use naming::*;
pub use notification::*;
pub use parse::*;
pub use qualitydefinition::*;
pub use qualityprofile::*;
pub use queue::*;
pub use release::*;
pub use releaseprofile::*;
pub use rename::*;
pub use rootfolder::*;
pub use seasonpass::*;
pub use series::*;
pub use system::*;
pub use update::*;
pub use wanted::*;

use crate::config::Config;
use crate::error::Result;
use crate::interface::{ApiClient, ApiClientExt};
use crate::paginate::Filtering;
use crate::req::Request;
use std::sync::Arc;

/// The Sonarr API version supported by this library.
pub const APIVER: &str = "v3";

/// Contains all the methods to interact with a Sonarr server.
#[derive(Debug, Clone)]
pub struct Sonarr {
    /// The underlying API client. Swap it out to mock requests in tests.
    pub api: Arc<dyn ApiClient>,
}

impl Sonarr {
    /// Returns a Sonarr object used to interact with the Sonarr API.
    pub fn new(config: Config) -> Self {
        Self {
            api: Arc::new(config.prepare()),
        }
    }

    /// Returns a Sonarr object backed by any [`ApiClient`] implementation.
    pub fn with_api(api: Arc<dyn ApiClient>) -> Self {
        Self { api }
    }
}

/// Unknown history event.
pub const FILTER_UNKNOWN: Filtering = Filtering(0);
/// Release was grabbed.
pub const FILTER_GRABBED: Filtering = Filtering(1);
/// Series folder was imported.
pub const FILTER_SERIES_FOLDER_IMPORTED: Filtering = Filtering(2);
/// Download folder was imported.
pub const FILTER_DOWNLOAD_FOLDER_IMPORTED: Filtering = Filtering(3);
/// Download failed.
pub const FILTER_DOWNLOAD_FAILED: Filtering = Filtering(4);
/// Item was deleted.
pub const FILTER_DELETED: Filtering = Filtering(5);
/// Item was renamed.
pub const FILTER_RENAMED: Filtering = Filtering(6);
/// Import failed.
pub const FILTER_IMPORT_FAILED: Filtering = Filtering(7);

/// `bp` means base path. You'll see it a lot in these files.
/// Ping has no api or version prefix.
const BP_PING: &str = "/ping";

impl Sonarr {
    /// Returns an error if the starr instance does not respond with a 200
    /// to an HTTP `/ping` request.
    pub async fn ping(&self) -> Result<()> {
        self.api.get(Request::new(BP_PING)).await?;
        Ok(())
    }
}
