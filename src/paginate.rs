//! Helper types for page-able API calls, ported from `paginate.go`.
//!
//! Used by methods like `get_history()` and `get_queue()`.

use crate::helpers::str_val;
use crate::values::Values;

/// Sorting is used as a request parameter value to sort lists, like History and Queue.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum Sorting {
    /// No sort direction requested; the caller-side default applies.
    #[default]
    Unset,
    /// Sorts lists in ascending order. This is the API default.
    Ascend,
    /// Flips the sort order to descending.
    Descend,
}

impl Sorting {
    /// Returns the wire value, or `""` when unset.
    pub fn as_str(self) -> &'static str {
        match self {
            Sorting::Unset => "",
            Sorting::Ascend => "ascending",
            Sorting::Descend => "descending",
        }
    }

    /// Makes sure the sort direction is valid.
    ///
    /// Anything that is not `descending` becomes `ascending`, matching Go's `Set`.
    pub fn set(&mut self, val: &str) {
        *self = if val.eq_ignore_ascii_case("descending") {
            Sorting::Descend
        } else {
            Sorting::Ascend
        };
    }
}

impl std::fmt::Display for Sorting {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Filtering is used as a request parameter value to filter lists, like History and Queue.
///
/// The filter values are different per-app, so find their values in their
/// respective modules.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Filtering(pub i32);

impl Filtering {
    /// Returns the string value of a Filter eventType.
    pub fn param(self) -> String {
        str_val(self.0)
    }
}

impl From<i32> for Filtering {
    fn from(val: i32) -> Self {
        Filtering(val)
    }
}

/// PageReq is the input to search requests that have page-able responses.
///
/// These are turned into HTTP parameters.
#[derive(Debug, Clone, Default)]
pub struct PageReq {
    /// Additional values that may be set.
    pub values: Values,
    /// 10 is default if not provided.
    pub page_size: i32,
    /// 1 is default if not provided.
    pub page: i32,
    /// date, timeleft, others?
    pub sort_key: String,
    /// ascending, descending
    pub sort_dir: Sorting,
    /// enums for eventTypes. App specific.
    pub filter: Filtering,
}

impl PageReq {
    /// Returns an empty page request; every field falls back to an API default.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the page number.
    pub fn page(mut self, page: i32) -> Self {
        self.page = page;
        self
    }

    /// Sets the number of records per page.
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = page_size;
        self
    }

    /// Sets the sort key.
    pub fn sort_key(mut self, sort_key: impl Into<String>) -> Self {
        self.sort_key = sort_key.into();
        self
    }

    /// Sets the sort direction.
    pub fn sort_dir(mut self, sort_dir: Sorting) -> Self {
        self.sort_dir = sort_dir;
        self
    }

    /// Sets the event type filter.
    pub fn filter(mut self, filter: impl Into<Filtering>) -> Self {
        self.filter = filter.into();
        self
    }

    /// Returns a brand new [`Values`] with all request parameters combined.
    pub fn params(&self) -> Values {
        let mut params = Values::new();

        if self.filter.0 > 0 {
            params.set("eventType", self.filter.param());
        }

        if self.page > 0 {
            params.set("page", str_val(self.page));
        } else {
            params.set("page", "1");
        }

        if self.page_size > 0 {
            params.set("pageSize", str_val(self.page_size));
        } else {
            params.set("pageSize", "10");
        }

        if !self.sort_key.is_empty() {
            params.set("sortKey", self.sort_key.clone());
        } else {
            params.set("sortKey", "date"); // timeleft, title, id
        }

        if self.sort_dir != Sorting::Unset {
            params.set("sortDirection", self.sort_dir.as_str());
        } else {
            params.set("sortDirection", "ascending"); // descending
        }

        for (key, vals) in self.values.iter() {
            for val in vals {
                params.set(key.clone(), val.clone());
            }
        }

        params
    }

    /// Turns our request parameters into a URI string.
    pub fn encode(&self) -> String {
        self.params().encode()
    }

    /// Returns a request parameter, checking the named fields first.
    pub fn get(&self, key: &str) -> String {
        match key.to_lowercase().as_str() {
            "page" => str_val(self.page),
            "pagesize" => str_val(self.page_size),
            "sortkey" => self.sort_key.clone(),
            "sortdirection" => self.sort_dir.as_str().to_string(),
            _ => self.values.get(key).to_string(),
        }
    }

    /// Sets a request parameter if it's not already set.
    pub fn check_set(&mut self, key: &str, value: &str) {
        match key.to_lowercase().as_str() {
            "page" => {
                if self.page == 0 {
                    self.page = value.parse().unwrap_or(0);
                }
            }
            "pagesize" => {
                if self.page_size == 0 {
                    self.page_size = value.parse().unwrap_or(0);
                }
            }
            "sortkey" => {
                if self.sort_key.is_empty() {
                    self.sort_key = value.to_string();
                }
            }
            "sortdirection" => {
                if self.sort_dir == Sorting::Unset {
                    self.sort_dir.set(value);
                }
            }
            _ => {
                if self.values.get(key).is_empty() {
                    self.values.set(key, value);
                }
            }
        }
    }

    /// Sets a request parameter.
    pub fn set(&mut self, key: &str, value: &str) {
        match key.to_lowercase().as_str() {
            "page" => self.page = value.parse().unwrap_or(0),
            "pagesize" => self.page_size = value.parse().unwrap_or(0),
            "sortkey" => self.sort_key = value.to_string(),
            "sortdirection" => self.sort_dir.set(value),
            _ => {
                self.values.set(key, value);
            }
        }
    }
}

/// Returns a proper `per_page` value that is not equal to zero, and not larger
/// than the record count desired.
///
/// If the count is zero, then `per_page` can be anything other than zero.
/// This is used by paginated methods in the app modules.
pub fn set_per_page(records: i32, mut per_page: i32) -> i32 {
    const PER_PAGE_DEFAULT: i32 = 500;

    if per_page <= 1 {
        if records > PER_PAGE_DEFAULT || records == 0 {
            per_page = PER_PAGE_DEFAULT;
        } else {
            per_page = records;
        }
    } else if per_page > records && records != 0 {
        per_page = records;
    }

    per_page
}

/// Adjusts `per_page` to make sure we don't go over, or ask for more records than exist.
///
/// `records` is the number requested, `total` is the number in the app,
/// `collected` is how many we have so far, and `per_page` is the current setting.
pub fn adjust_per_page(records: i32, total: i32, collected: i32, mut per_page: i32) -> i32 {
    // Do not ask for more than was requested.
    let remaining = records - collected;
    if per_page > remaining && remaining > 0 {
        per_page = remaining;
    }

    // Ask for only the known total.
    let known = total - collected;
    if per_page > known {
        per_page = known;
    }

    per_page
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn params_fill_in_defaults() {
        let req = PageReq::new();
        assert_eq!(
            req.encode(),
            "page=1&pageSize=10&sortDirection=ascending&sortKey=date"
        );
    }

    #[test]
    fn params_use_provided_values() {
        let req = PageReq::new()
            .page(3)
            .page_size(25)
            .sort_key("timeleft")
            .sort_dir(Sorting::Descend)
            .filter(2);

        assert_eq!(
            req.encode(),
            "eventType=2&page=3&pageSize=25&sortDirection=descending&sortKey=timeleft"
        );
    }

    #[test]
    fn check_set_does_not_overwrite() {
        let mut req = PageReq::new().page(7);
        req.check_set("page", "9");
        assert_eq!(req.page, 7);

        req.check_set("pageSize", "9");
        assert_eq!(req.page_size, 9);
    }

    #[test]
    fn per_page_helpers() {
        assert_eq!(set_per_page(0, 0), 500);
        assert_eq!(set_per_page(20, 0), 20);
        assert_eq!(set_per_page(20, 50), 20);
        assert_eq!(adjust_per_page(100, 30, 0, 500), 30);
        assert_eq!(adjust_per_page(10, 1000, 5, 500), 5);
    }
}
