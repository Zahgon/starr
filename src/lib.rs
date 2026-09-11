//! A Rust port of the [golift.io/starr](https://github.com/golift/starr) library
//! for interacting with the APIs in Radarr, Lidarr, Sonarr, Readarr and Prowlarr.
//!
//! It consists of the crate root and one module per Starr application. In the
//! basic use, you create a [`Config`] that contains an API key and an app URL,
//! then pass it into one of the app modules (like [`radarr`]).
//!
//! # Example
//!
//! ```no_run
//! use starr_rust::{Config, lidarr::Lidarr};
//! use std::time::Duration;
//!
//! # async fn run() -> starr_rust::Result<()> {
//! // Get a Config that can plug into any Starr app.
//! let config = Config::new("abc1234ahsuyka123jh12", "http://localhost:8686", Duration::ZERO);
//! // Let's make a lidarr server with the default starr Config.
//! let lidarr = Lidarr::new(config);
//!
//! let status = lidarr.get_system_status().await?;
//! println!("{status:?}");
//! # Ok(())
//! # }
//! ```
//!
//! # Differences from the Go library
//!
//! * Every request method is `async`; Go's `context.Context` parameter is gone
//!   because dropping a future cancels the request. The Go `XxxContext`
//!   variants therefore collapse into a single method.
//! * Optional API fields are `Option<T>` instead of Go pointers; use
//!   `Some(true)` where Go used `starr.True()`.
//! * Debug logging attaches to [`Config`] rather than to an HTTP transport,
//!   because reqwest has no public `RoundTripper` hook.

#![warn(missing_docs)]

pub mod config;
pub mod debuglog;
pub mod error;
pub mod helpers;
pub mod interface;
pub mod paginate;
pub mod req;
pub mod shared;
pub mod starrcmd;
pub mod starrconnect;
pub mod starrshared;
pub mod starrtest;
pub mod strenum;
pub mod values;

pub mod lidarr;
pub mod prowlarr;
pub mod radarr;
pub mod readarr;
pub mod sonarr;

pub use config::{Config, DEFAULT_TIMEOUT, USER_AGENT};
pub use error::{Error, ErrorContext, ReqError, Result};
pub use helpers::{App, StrVal, client, client_with_debug, force_save, ptr, str_val};
pub use interface::{ApiClient, ApiClientExt, InitializeJs, read_initialize_js};
pub use paginate::{Filtering, PageReq, Sorting, adjust_per_page, set_per_page};
pub use req::{API, Request, path_join, set_api_path};
pub use shared::*;
pub use values::Values;

/// Reports whether a value equals its default.
///
/// Used with `#[serde(skip_serializing_if = "is_default")]` to reproduce Go's
/// `omitempty`, which drops empty strings, zero numbers, `false` and empty
/// slices from the JSON payload.
pub fn is_default<T: Default + PartialEq>(value: &T) -> bool {
    *value == T::default()
}
