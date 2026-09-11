//! Support for Go's `type Foo string` enum pattern.
//!
//! Starr APIs return open string enums: the documented constants cover the
//! common cases, but the apps are free to send anything. A Rust `enum` would
//! reject unknown values, so these become newtypes over a string with
//! associated constants, exactly matching the Go semantics.

/// Declares a newtype over a string with associated constants.
///
/// ```ignore
/// string_enum! {
///     /// Protocol used to download media.
///     pub struct Protocol {
///         /// Unknown protocol.
///         const UNKNOWN = "unknown";
///         /// Usenet.
///         const USENET = "usenet";
///     }
/// }
/// ```
#[macro_export]
macro_rules! string_enum {
    (
        $(#[$meta:meta])*
        $vis:vis struct $name:ident {
            $( $(#[$cmeta:meta])* const $konst:ident = $val:expr; )*
        }
    ) => {
        $(#[$meta])*
        #[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, ::serde::Serialize, ::serde::Deserialize)]
        #[serde(transparent)]
        $vis struct $name(pub ::std::borrow::Cow<'static, str>);

        impl $name {
            $( $(#[$cmeta])* pub const $konst: $name = $name(::std::borrow::Cow::Borrowed($val)); )*

            /// Builds a value from any string, including ones this crate does not know.
            pub fn new(value: impl Into<::std::borrow::Cow<'static, str>>) -> Self {
                $name(value.into())
            }

            /// Returns the wire value.
            pub fn as_str(&self) -> &str {
                &self.0
            }

            /// Reports whether the value is the empty string.
            pub fn is_empty(&self) -> bool {
                self.0.is_empty()
            }
        }

        impl ::std::default::Default for $name {
            fn default() -> Self {
                $name(::std::borrow::Cow::Borrowed(""))
            }
        }

        impl ::std::fmt::Display for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                f.write_str(&self.0)
            }
        }

        impl ::std::convert::From<&'static str> for $name {
            fn from(value: &'static str) -> Self {
                $name(::std::borrow::Cow::Borrowed(value))
            }
        }

        impl ::std::convert::From<String> for $name {
            fn from(value: String) -> Self {
                $name(::std::borrow::Cow::Owned(value))
            }
        }

        impl ::std::cmp::PartialEq<str> for $name {
            fn eq(&self, other: &str) -> bool {
                self.0 == other
            }
        }
    };
}
