//! Bindings to consume a custom script command hook from any Starr app.
//!
//! Use this when you want to write an application that is executed by the
//! Starr app through a custom script command hook. Create these by going into
//! Settings → Connect → Custom Script in Lidarr, Prowlarr, Radarr, Readarr, or
//! Sonarr.
//!
//! Notes to future developers of this module:
//!
//! * Counters are `i32`, IDs are `i64`, sizes are `i64` (bytes).
//! * Slices need a split character, `,` or `|` (usually). A list field
//!   declared without one fails to parse instead of panicking like Go does.
//! * The time format is hard coded twice. If new formats arise, find a way to fix it.
//!
//! # Differences from the Go library
//!
//! Go fills the event structs by reflecting over `env` struct tags. Rust has no
//! runtime reflection, so [`env_struct!`] generates the same readers at compile
//! time from the same declarative table.

mod dispatcher;
mod lidarr;
mod prowlarr;
mod radarr;
mod readarr;
mod sonarr;

pub use dispatcher::*;
pub use lidarr::*;
pub use prowlarr::*;
pub use radarr::*;
pub use readarr::*;
pub use sonarr::*;

use crate::helpers::App;
use crate::string_enum;
use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use std::env;

/// Matches the date output from most apps.
pub const DATE_FORMAT: &str = "%m/%d/%Y %I:%M:%S %p";

/// Matches the date output from Readarr.
pub const DATE_FORMAT2: &str = "%m/%d/%Y %H:%M:%S";

/// Errors you may receive from this module.
#[derive(Debug, thiserror::Error)]
pub enum CmdError {
    /// Returned if you invoke a procedure for the wrong event.
    #[error("incorrect event type requested: requested '{wanted}' have '{have}'")]
    InvalidEvent {
        /// The event the caller asked for.
        wanted: Event,
        /// The event found in the environment.
        have: Event,
    },

    /// Returned if an event type is not found.
    ///
    /// This should only happen when testing and you forget a variable.
    #[error("no eventType environment variable found")]
    NoEventFound,

    /// Returned by `run` or `dispatch` when the dispatcher is missing.
    #[error("starrcmd: nil *Dispatcher")]
    NilDispatcher,

    /// Returned by `dispatch` when the command event is missing.
    #[error("starrcmd: nil *CmdEvent")]
    NilCmdEvent,

    /// An environment variable could not be parsed into its field type.
    #[error("reading environment: {name}: ({value}) {reason}")]
    Parse {
        /// Name of the environment variable.
        name: String,
        /// Value that failed to parse.
        value: String,
        /// Why it failed.
        reason: String,
    },

    /// A payload failed to parse before its handler ran.
    #[error("parse env payload: {0}")]
    ParsePayload(Box<CmdError>),

    /// A callback returned an error.
    #[error("{0}")]
    Callback(String),
}

/// Convenient result alias used by this module.
pub type CmdResult<T> = std::result::Result<T, CmdError>;

string_enum! {
    /// Event is a custom type to hold our EventType.
    ///
    /// This list represents all available and existing event types for all
    /// five Starr apps.
    pub struct Event {
        /// All apps, useless.
        const TEST = "Test";
        /// All apps.
        const HEALTH_ISSUE = "HealthIssue";
        /// All apps.
        const APPLICATION_UPDATE = "ApplicationUpdate";
        /// All apps except Prowlarr.
        const GRAB = "Grab";
        /// All apps except Prowlarr.
        const RENAME = "Rename";
        /// All apps except Prowlarr and Lidarr.
        const DOWNLOAD = "Download";
        /// Lidarr and Readarr.
        const TRACK_RETAG = "TrackRetag";
        /// Lidarr.
        const ALBUM_DOWNLOAD = "AlbumDownload";
        /// Radarr.
        const MOVIE_FILE_DELETE = "MovieFileDelete";
        /// Radarr.
        const MOVIE_DELETE = "MovieDelete";
        /// Readarr.
        const BOOK_DELETE = "BookDelete";
        /// Readarr.
        const AUTHOR_DELETE = "AuthorDelete";
        /// Readarr.
        const BOOK_FILE_DELETE = "BookFileDelete";
        /// Sonarr.
        const SERIES_DELETE = "SeriesDelete";
        /// Sonarr.
        const EPISODE_FILE_DELETE = "EpisodeFileDelete";
    }
}

/// CmdEvent holds the current event type and the app that triggered it.
///
/// Get one of these by calling [`CmdEvent::new`].
#[derive(Debug, Clone, Default, PartialEq)]
pub struct CmdEvent {
    /// The app that ran this custom script.
    pub app: App,
    /// The event that triggered the run.
    pub event_type: Event,
}

impl CmdEvent {
    /// Returns the current Event and Application it's from, or an error if the
    /// type doesn't exist.
    ///
    /// When running from a Starr App Custom Script this should not return an error.
    pub fn new() -> CmdResult<Self> {
        for (app, var) in [
            (App::RADARR, "radarr_eventtype"),
            (App::SONARR, "sonarr_eventtype"),
            (App::LIDARR, "lidarr_eventtype"),
            (App::READARR, "readarr_eventtype"),
            (App::PROWLARR, "prowlarr_eventtype"),
        ] {
            let value = env::var(var).unwrap_or_default();
            if !value.is_empty() {
                return Ok(Self {
                    app,
                    event_type: Event::new(value),
                });
            }
        }

        Err(CmdError::NoEventFound)
    }

    /// Returns a command event without returning an error.
    ///
    /// # Panics
    ///
    /// Panics if the event does not exist. When running from a Starr App Custom
    /// Script this should not panic.
    pub fn new_must() -> Self {
        Self::new().expect("no eventType environment variable found")
    }

    /// Returns a command event and allows your code to handle the problem.
    ///
    /// When running from a Starr App Custom Script this always returns a proper event.
    pub fn new_must_no_panic() -> Self {
        Self::new().unwrap_or_default()
    }

    /// Offloads the error checking from all the other routines.
    ///
    /// This is where our journey into the data truly begins.
    pub(crate) fn check(&self, wanted: &Event) -> CmdResult<()> {
        if self.event_type != *wanted {
            return Err(CmdError::InvalidEvent {
                wanted: wanted.clone(),
                have: self.event_type.clone(),
            });
        }

        Ok(())
    }
}

/// Parses one environment variable value into a struct member.
pub trait FromEnvValue: Sized {
    /// Parses `value`, splitting on `split` for list members.
    fn parse_env(value: &str, split: Option<char>) -> std::result::Result<Self, String>;
}

impl FromEnvValue for String {
    fn parse_env(value: &str, _split: Option<char>) -> std::result::Result<Self, String> {
        Ok(value.to_string())
    }
}

impl FromEnvValue for i32 {
    fn parse_env(value: &str, _split: Option<char>) -> std::result::Result<Self, String> {
        value
            .parse()
            .map_err(|err| format!("parsing integer: {err}"))
    }
}

impl FromEnvValue for i64 {
    fn parse_env(value: &str, _split: Option<char>) -> std::result::Result<Self, String> {
        value
            .parse()
            .map_err(|err| format!("parsing integer: {err}"))
    }
}

impl FromEnvValue for bool {
    fn parse_env(value: &str, _split: Option<char>) -> std::result::Result<Self, String> {
        match value.to_ascii_lowercase().as_str() {
            "1" | "t" | "true" => Ok(true),
            "0" | "f" | "false" => Ok(false),
            other => Err(format!("parsing bool: invalid syntax: {other}")),
        }
    }
}

impl FromEnvValue for DateTime<Utc> {
    fn parse_env(value: &str, _split: Option<char>) -> std::result::Result<Self, String> {
        let parsed = NaiveDateTime::parse_from_str(value, DATE_FORMAT)
            .or_else(|_| NaiveDateTime::parse_from_str(value, DATE_FORMAT2));

        match parsed {
            Ok(naive) => Ok(Utc.from_utc_datetime(&naive)),
            Err(err) => Err(format!("parsing time: {err}")),
        }
    }
}

impl<T: FromEnvValue> FromEnvValue for Vec<T> {
    fn parse_env(value: &str, split: Option<char>) -> std::result::Result<Self, String> {
        // Go panics when a list member has no split character in its env tag.
        let Some(split) = split else {
            return Err("list member declared without a split character".to_string());
        };

        value
            .split(split)
            .map(|item| T::parse_env(item, None))
            .collect()
    }
}

/// Reads one member value out of the environment.
pub fn read_env_member<T: FromEnvValue + Default>(
    name: &str,
    split: Option<char>,
) -> CmdResult<T> {
    let value = env::var(name).unwrap_or_default();
    if value.is_empty() {
        return Ok(T::default());
    }

    T::parse_env(&value, split).map_err(|reason| CmdError::Parse {
        name: name.to_string(),
        value,
        reason,
    })
}

/// Declares a Custom Script event payload and its environment reader.
///
/// This replaces the `env` struct tags and reflection the Go library uses.
#[macro_export]
macro_rules! env_struct {
    (
        $(#[$meta:meta])*
        pub struct $name:ident {
            $(
                $(#[$fmeta:meta])*
                $field:ident : $ty:ty = $env:literal $(, split = $split:literal)? ;
            )*
        }
    ) => {
        $(#[$meta])*
        #[derive(Debug, Clone, Default, PartialEq)]
        pub struct $name {
            $(
                $(#[$fmeta])*
                pub $field: $ty,
            )*
        }

        impl $name {
            /// Reads this event's members from the process environment.
            pub fn from_env() -> $crate::starrcmd::CmdResult<Self> {
                #[allow(unused_mut)]
                let mut out = Self::default();
                $(
                    #[allow(unused_variables, unused_assignments)]
                    let split: ::std::option::Option<char> = ::std::option::Option::None;
                    $( let split: ::std::option::Option<char> = ::std::option::Option::Some($split); )?
                    out.$field = $crate::starrcmd::read_env_member($env, split)?;
                )*

                Ok(out)
            }
        }
    };
}
