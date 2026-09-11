//! A faithful port of Go's `net/url.Values`.
//!
//! Starr APIs are extremely sensitive to query-string shape, so this type
//! reproduces Go's ordering (sorted keys) and escaping rules exactly rather
//! than delegating to a generic form encoder.

use std::collections::BTreeMap;

/// Values maps a string key to a list of values, like Go's `url.Values`.
///
/// Keys are kept sorted so [`Values::encode`] produces byte-identical output
/// to Go's `url.Values.Encode`.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Values(BTreeMap<String, Vec<String>>);

impl Values {
    /// Returns an empty set of values.
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }

    /// Sets the key to value, replacing any existing values.
    pub fn set(&mut self, key: impl Into<String>, value: impl Into<String>) -> &mut Self {
        self.0.insert(key.into(), vec![value.into()]);
        self
    }

    /// Adds the value to key, appending to any existing values.
    pub fn add(&mut self, key: impl Into<String>, value: impl Into<String>) -> &mut Self {
        self.0.entry(key.into()).or_default().push(value.into());
        self
    }

    /// Gets the first value associated with key, or `""` when absent.
    ///
    /// This mirrors Go's `url.Values.Get`, which never fails.
    pub fn get(&self, key: &str) -> &str {
        self.0
            .get(key)
            .and_then(|vals| vals.first())
            .map_or("", String::as_str)
    }

    /// Returns every value associated with key.
    pub fn get_all(&self, key: &str) -> &[String] {
        self.0.get(key).map_or(&[], Vec::as_slice)
    }

    /// Reports whether key is present.
    pub fn has(&self, key: &str) -> bool {
        self.0.contains_key(key)
    }

    /// Deletes the values associated with key.
    pub fn del(&mut self, key: &str) -> &mut Self {
        self.0.remove(key);
        self
    }

    /// Number of distinct keys.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Reports whether there are no keys.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Iterates key/values pairs in sorted key order.
    pub fn iter(&self) -> impl Iterator<Item = (&String, &Vec<String>)> {
        self.0.iter()
    }

    /// Encodes the values into `URL encoded` form sorted by key.
    pub fn encode(&self) -> String {
        let mut out = String::new();

        for (key, vals) in &self.0 {
            let key = query_escape(key);

            for val in vals {
                if !out.is_empty() {
                    out.push('&');
                }

                out.push_str(&key);
                out.push('=');
                out.push_str(&query_escape(val));
            }
        }

        out
    }
}

impl Values {
    /// Builds values from key/multi-value pairs, like a Go `url.Values` literal.
    pub fn from_pairs<I, K>(pairs: I) -> Self
    where
        I: IntoIterator<Item = (K, Vec<String>)>,
        K: Into<String>,
    {
        Self(pairs.into_iter().map(|(k, v)| (k.into(), v)).collect())
    }
}

impl<K: Into<String>, V: Into<String>> FromIterator<(K, V)> for Values {
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        let mut values = Self::new();

        for (key, val) in iter {
            values.add(key, val);
        }

        values
    }
}

/// Escapes a string so it can be safely placed inside a URL query,
/// using the same rules as Go's `url.QueryEscape`.
///
/// Unreserved characters are `A-Z a-z 0-9 - _ . ~`; a space becomes `+`.
pub fn query_escape(input: &str) -> String {
    const HEX: &[u8; 16] = b"0123456789ABCDEF";

    let mut out = String::with_capacity(input.len());

    for &byte in input.as_bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(byte as char);
            }
            b' ' => out.push('+'),
            _ => {
                out.push('%');
                out.push(HEX[(byte >> 4) as usize] as char);
                out.push(HEX[(byte & 0xF) as usize] as char);
            }
        }
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_sorts_keys_and_escapes_like_go() {
        let mut values = Values::new();
        values.set("zebra", "a b");
        values.set("alpha", "x~y*z");
        values.add("alpha", "second");

        assert_eq!(values.encode(), "alpha=x~y%2Az&alpha=second&zebra=a+b");
    }

    #[test]
    fn get_returns_empty_string_when_missing() {
        let values = Values::new();
        assert_eq!(values.get("nope"), "");
    }
}
