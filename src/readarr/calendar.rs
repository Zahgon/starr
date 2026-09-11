use super::{Book, Readarr};
use crate::error::Result;
use crate::helpers::str_val;
use crate::interface::ApiClientExt;
use crate::req::{Request, path_join};
use crate::shared::CALENDAR_TIME_FILTER_FORMAT;
use crate::values::Values;
use chrono::{DateTime, Utc};

const BP_CALENDAR: &str = "v1/calendar";

/// Calendar defines the filters for fetching calendar items.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Calendar {
    /// Only return items on or after this time.
    pub start: Option<DateTime<Utc>>,
    /// Only return items on or before this time.
    pub end: Option<DateTime<Utc>>,
    /// Include unmonitored books.
    pub unmonitored: bool,
    /// Include the full author in each book.
    pub include_author: bool,
}

impl Readarr {
    /// Returns calendars based on filters.
    pub async fn get_calendar(&self, filter: &Calendar) -> Result<Vec<Book>> {
        let mut query = Values::new();
        query.add("unmonitored", str_val(filter.unmonitored));
        query.add("includeAuthor", str_val(filter.include_author));

        if let Some(start) = filter.start {
            query.add("start", start.format(CALENDAR_TIME_FILTER_FORMAT).to_string());
        }

        if let Some(end) = filter.end {
            query.add("end", end.format(CALENDAR_TIME_FILTER_FORMAT).to_string());
        }

        self.api
            .get_into(Request::new(BP_CALENDAR).with_query(query))
            .await
    }

    /// Returns a single calendar item by ID.
    pub async fn get_calendar_id(&self, calendar_id: i64) -> Result<Book> {
        self.api
            .get_into(Request::new(path_join(&[
                BP_CALENDAR,
                &str_val(calendar_id),
            ])))
            .await
    }
}
