//! The SDK client for the Prowlarr API.

mod app;
mod appprofile;
mod command;
mod customfilter;
mod diskspace;
mod downloadclient;
mod health;
mod history;
mod indexer;
mod indexerproxy;
mod notification;
mod search;
mod system;
mod tag;
mod update;

pub use app::*;
pub use appprofile::*;
pub use command::*;
pub use customfilter::*;
pub use diskspace::*;
pub use downloadclient::*;
pub use health::*;
pub use history::*;
pub use indexer::*;
pub use indexerproxy::*;
pub use notification::*;
pub use search::*;
pub use system::*;
pub use tag::*;
pub use update::*;

use crate::config::Config;
use crate::error::Result;
use crate::interface::{ApiClient, ApiClientExt};
use crate::req::Request;
use std::sync::Arc;

/// The Prowlarr API version supported by this library.
pub const APIVER: &str = "v1";

/// Contains all the methods to interact with a Prowlarr server.
#[derive(Debug, Clone)]
pub struct Prowlarr {
    /// The underlying API client. Swap it out to mock requests in tests.
    pub api: Arc<dyn ApiClient>,
}

impl Prowlarr {
    /// Returns a Prowlarr object used to interact with the Prowlarr API.
    pub fn new(config: Config) -> Self {
        Self {
            api: Arc::new(config.prepare()),
        }
    }

    /// Returns a Prowlarr object backed by any [`ApiClient`] implementation.
    pub fn with_api(api: Arc<dyn ApiClient>) -> Self {
        Self { api }
    }
}

/// `bp` means base path. You'll see it a lot in these files.
/// Ping has no api or version prefix.
const BP_PING: &str = "/ping";

impl Prowlarr {
    /// Returns an error if the Prowlarr instance does not respond with a 200
    /// to an HTTP `/ping` request.
    pub async fn ping(&self) -> Result<()> {
        self.api.get(Request::new(BP_PING)).await?;
        Ok(())
    }
}
