//! The SDK client for the Radarr API.

mod alttitle;
mod autotagging;
mod blocklist;
mod calendar;
mod collection;
mod command;
mod credit;
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
mod indexerflag;
mod language;
mod manualimport;
mod mediamanagement;
mod movie;
mod movieeditor;
mod moviefile;
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
mod restriction;
mod rootfolder;
mod system;
mod tag;
mod update;
mod wanted;

pub use autotagging::*;
pub use blocklist::*;
pub use calendar::*;
pub use collection::*;
pub use command::*;
pub use credit::*;
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
pub use indexerflag::*;
pub use language::*;
pub use manualimport::*;
pub use mediamanagement::*;
pub use movie::*;
pub use movieeditor::*;
pub use moviefile::*;
pub use naming::*;
pub use notification::*;
pub use parse::*;
pub use qualitydefinition::*;
pub use qualityprofile::*;
pub use queue::*;
pub use release::*;
pub use releaseprofile::*;
pub use rename::*;
pub use restriction::*;
pub use rootfolder::*;
pub use system::*;
pub use tag::*;
pub use update::*;
pub use wanted::*;

use crate::config::Config;
use crate::error::Result;
use crate::interface::{ApiClient, ApiClientExt};
use crate::paginate::Filtering;
use crate::req::Request;
use std::sync::Arc;

/// The Radarr API version supported by this library.
pub const APIVER: &str = "v3";

/// Contains all the methods to interact with a Radarr server.
#[derive(Debug, Clone)]
pub struct Radarr {
    /// The underlying API client. Swap it out to mock requests in tests.
    pub api: Arc<dyn ApiClient>,
}

impl Radarr {
    /// Returns a Radarr object used to interact with the Radarr API.
    pub fn new(config: Config) -> Self {
        Self {
            api: Arc::new(config.prepare()),
        }
    }

    /// Returns a Radarr object backed by any [`ApiClient`] implementation.
    pub fn with_api(api: Arc<dyn ApiClient>) -> Self {
        Self { api }
    }
}

/// Unknown history event.
pub const FILTER_UNKNOWN: Filtering = Filtering(0);
/// Release was grabbed.
pub const FILTER_GRABBED: Filtering = Filtering(1);
/// Download folder was imported.
pub const FILTER_DOWNLOAD_FOLDER_IMPORTED: Filtering = Filtering(3);
/// Download failed.
pub const FILTER_DOWNLOAD_FAILED: Filtering = Filtering(4);
/// Movie file was deleted.
pub const FILTER_FILE_DELETED: Filtering = Filtering(6);
/// Movie file was renamed.
pub const FILTER_RENAMED: Filtering = Filtering(8);
/// Item was ignored.
pub const FILTER_IGNORED: Filtering = Filtering(9);

/// `bp` means base path. You'll see it a lot in these files.
/// Ping has no api or version prefix.
const BP_PING: &str = "/ping";

impl Radarr {
    /// Returns an error if the starr instance does not respond with a 200
    /// to an HTTP `/ping` request.
    pub async fn ping(&self) -> Result<()> {
        self.api.get(Request::new(BP_PING)).await?;
        Ok(())
    }
}
