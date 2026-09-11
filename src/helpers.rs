//! Small helpers shared by the whole crate, ported from `helpers.go`.

use crate::debuglog;
use crate::values::Values;
use reqwest_cookie_store::CookieStoreMutex;
use std::borrow::Cow;
use std::sync::Arc;
use std::time::Duration;

/// App can be used to satisfy a context value key.
///
/// It is not used in this library; provided for convenience.
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct App(pub Cow<'static, str>);

impl App {
    /// Emby.
    pub const EMBY: App = App(Cow::Borrowed("Emby"));
    /// Lidarr.
    pub const LIDARR: App = App(Cow::Borrowed("Lidarr"));
    /// Plex.
    pub const PLEX: App = App(Cow::Borrowed("Plex"));
    /// Prowlarr.
    pub const PROWLARR: App = App(Cow::Borrowed("Prowlarr"));
    /// Radarr.
    pub const RADARR: App = App(Cow::Borrowed("Radarr"));
    /// Readarr.
    pub const READARR: App = App(Cow::Borrowed("Readarr"));
    /// Sonarr.
    pub const SONARR: App = App(Cow::Borrowed("Sonarr"));
    /// Whisparr.
    pub const WHISPARR: App = App(Cow::Borrowed("Whisparr"));

    /// Builds an app name from any string.
    pub fn new(name: impl Into<Cow<'static, str>>) -> Self {
        Self(name.into())
    }

    /// Turns an App name into a string.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Turns an App name into a lowercase string.
    pub fn lower(&self) -> String {
        self.0.to_lowercase()
    }
}

impl Default for App {
    fn default() -> Self {
        App(Cow::Borrowed(""))
    }
}

impl std::fmt::Display for App {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

/// Converts numbers and booleans to a string the way Go's `strconv` does.
///
/// This backs [`str_val`], the port of the generic `starr.Str()` function.
pub trait StrVal {
    /// Returns the Go-compatible string form of this value.
    fn str_val(&self) -> String;
}

macro_rules! impl_str_val_display {
    ($($ty:ty),+) => {
        $(impl StrVal for $ty {
            fn str_val(&self) -> String {
                self.to_string()
            }
        })+
    };
}

// Rust's Display for integers and bools matches strconv.Itoa/FormatInt/FormatBool.
// Display for floats emits the shortest round-trip form without an exponent for
// ordinary magnitudes, matching strconv.FormatFloat(v, 'f', -1, 64).
impl_str_val_display!(i8, i16, i32, i64, isize, u8, u16, u32, u64, usize, f32, f64, bool);

impl StrVal for String {
    fn str_val(&self) -> String {
        self.clone()
    }
}

impl StrVal for &str {
    fn str_val(&self) -> String {
        (*self).to_string()
    }
}

/// Converts numbers and booleans to a string. Port of `starr.Str()`.
pub fn str_val<T: StrVal>(val: T) -> String {
    val.str_val()
}

/// Returns `Some(value)`, the Rust spelling of Go's `starr.Ptr()`.
///
/// Optional API fields are `Option<T>` in this crate: use `Some(true)` and
/// `Some(false)` where the Go library used `starr.True()` and `starr.False()`.
pub fn ptr<T>(value: T) -> Option<T> {
    Some(value)
}

/// Returns the query values used by Starr apps to force-save a resource.
pub fn force_save(force: bool) -> Values {
    let mut values = Values::new();
    values.set("forceSave", str_val(force));
    values
}

/// Returns the default client, and is used if one is not passed in.
///
/// A zero `timeout` means no timeout, matching Go's `http.Client`.
/// Redirects are never followed, matching `http.ErrUseLastResponse`.
pub fn client(timeout: Duration, verify_ssl: bool) -> reqwest::Client {
    build_client(timeout, verify_ssl, None).expect("building the default starr HTTP client")
}

/// Returns an HTTP client with a debug logger enabled.
///
/// Unlike the Go version, which installs a `RoundTripper`, the debug config is
/// carried by [`crate::Config`]; this helper exists for API parity and returns
/// the pair you hand to [`crate::Config::set_client_with_debug`].
pub fn client_with_debug(
    timeout: Duration,
    verify_ssl: bool,
    log_config: debuglog::Config,
) -> (reqwest::Client, debuglog::Config) {
    (client(timeout, verify_ssl), log_config)
}

/// Builds a reqwest client with the redirect, TLS and cookie behavior the Go library uses.
pub(crate) fn build_client(
    timeout: Duration,
    verify_ssl: bool,
    jar: Option<Arc<CookieStoreMutex>>,
) -> reqwest::Result<reqwest::Client> {
    let mut builder = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .danger_accept_invalid_certs(!verify_ssl);

    if !timeout.is_zero() {
        builder = builder.timeout(timeout);
    }

    if let Some(jar) = jar {
        builder = builder.cookie_provider(jar);
    }

    builder.build()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn str_val_matches_go_strconv() {
        assert_eq!(str_val(42_i64), "42");
        assert_eq!(str_val(true), "true");
        assert_eq!(str_val(1.0_f64), "1");
        assert_eq!(str_val(1.5_f64), "1.5");
    }

    #[test]
    fn app_lower() {
        assert_eq!(App::SONARR.lower(), "sonarr");
        assert_eq!(App::SONARR.to_string(), "Sonarr");
    }

    #[test]
    fn force_save_encodes() {
        assert_eq!(force_save(true).encode(), "forceSave=true");
    }
}
