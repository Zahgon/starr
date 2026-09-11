//! The SDK client for the Lidarr API.

mod album;
mod artist;
mod autotagging;
mod blocklist;
mod calendar;
mod command;
mod customfilter;
mod customformat;
mod delayprofile;
mod diskspace;
mod downloadclient;
mod downloadclientconfig;
mod exclusions;
mod feed;
mod health;
mod history;
mod importlist;
mod indexer;
mod indexerconfig;
mod manualimport;
mod mediamanagement;
mod metadataprofile;
mod naming;
mod notification;
mod parse;
mod qualitydefinition;
mod qualityprofile;
mod queue;
mod remotepathmapping;
mod rename;
mod rootfolder;
mod system;
mod tag;
mod track;
mod trackfile;
mod update;
mod wanted;

pub use album::*;
pub use artist::*;
pub use autotagging::*;
pub use blocklist::*;
pub use calendar::*;
pub use command::*;
pub use customfilter::*;
pub use customformat::*;
pub use delayprofile::*;
pub use diskspace::*;
pub use downloadclient::*;
pub use downloadclientconfig::*;
pub use exclusions::*;
pub use feed::*;
pub use health::*;
pub use history::*;
pub use importlist::*;
pub use indexer::*;
pub use indexerconfig::*;
pub use manualimport::*;
pub use mediamanagement::*;
pub use metadataprofile::*;
pub use naming::*;
pub use notification::*;
pub use parse::*;
pub use qualitydefinition::*;
pub use qualityprofile::*;
pub use queue::*;
pub use rename::*;
pub use rootfolder::*;
pub use system::*;
pub use tag::*;
pub use track::*;
pub use trackfile::*;
pub use update::*;
pub use wanted::*;

use crate::config::Config;
use crate::error::Result;
use crate::interface::{ApiClient, ApiClientExt};
use crate::paginate::Filtering;
use crate::req::Request;
use std::sync::Arc;

/// The Lidarr API version supported by this library.
pub const APIVER: &str = "v1";

/// Contains all the methods to interact with a Lidarr server.
#[derive(Debug, Clone)]
pub struct Lidarr {
    /// The underlying API client. Swap it out to mock requests in tests.
    pub api: Arc<dyn ApiClient>,
}

impl Lidarr {
    /// Returns a Lidarr object used to interact with the Lidarr API.
    pub fn new(config: Config) -> Self {
        Self {
            api: Arc::new(config.prepare()),
        }
    }

    /// Returns a Lidarr object backed by any [`ApiClient`] implementation.
    pub fn with_api(api: Arc<dyn ApiClient>) -> Self {
        Self { api }
    }
}

/// Unknown history event.
pub const FILTER_UNKNOWN: Filtering = Filtering(0);
/// Release was grabbed.
pub const FILTER_GRABBED: Filtering = Filtering(1);
/// Artist folder was imported.
pub const FILTER_ARTIST_FOLDER_IMPORTED: Filtering = Filtering(2);
/// Track file was imported.
pub const FILTER_TRACK_FILE_IMPORTED: Filtering = Filtering(3);
/// Download failed.
pub const FILTER_DOWNLOAD_FAILED: Filtering = Filtering(4);
/// Item was deleted.
pub const FILTER_DELETED: Filtering = Filtering(5);
/// Item was renamed.
pub const FILTER_RENAMED: Filtering = Filtering(6);
/// Import failed.
pub const FILTER_IMPORT_FAILED: Filtering = Filtering(7);
/// Download was imported.
pub const FILTER_DOWNLOAD_IMPORTED: Filtering = Filtering(8);
/// Track file was retagged.
pub const FILTER_RETAGGED: Filtering = Filtering(9);
/// Item was ignored.
pub const FILTER_IGNORED: Filtering = Filtering(10);

/// `bp` means base path. You'll see it a lot in these files.
/// Ping has no api or version prefix.
const BP_PING: &str = "/ping";

impl Lidarr {
    /// Returns an error if the starr instance does not respond with a 200
    /// to an HTTP `/ping` request.
    pub async fn ping(&self) -> Result<()> {
        self.api.get(Request::new(BP_PING)).await?;
        Ok(())
    }
}
