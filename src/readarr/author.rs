use super::{AddBookOptions, Readarr};
use crate::error::Result;
use crate::helpers::str_val;
use crate::interface::ApiClientExt;
use crate::is_default;
use crate::req::{Request, path_join};
use crate::shared::{Image, IsLoaded, Link, Ratings};
use crate::values::Values;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

const BP_AUTHOR: &str = "v1/author";

/// Author is the `/api/v1/author` endpoint.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Author {
    /// Author ID.
    #[serde(default)]
    pub id: i64,
    /// Author status.
    #[serde(default, skip_serializing_if = "is_default")]
    pub status: String,
    /// Author name.
    #[serde(default, skip_serializing_if = "is_default")]
    pub author_name: String,
    /// Goodreads author ID.
    #[serde(default, skip_serializing_if = "is_default")]
    pub foreign_author_id: String,
    /// Slug used in URLs.
    #[serde(default, skip_serializing_if = "is_default")]
    pub title_slug: String,
    /// Author overview.
    #[serde(default, skip_serializing_if = "is_default")]
    pub overview: String,
    /// External links for this author.
    #[serde(default, skip_serializing_if = "is_default")]
    pub links: Vec<Link>,
    /// Author images.
    #[serde(default, skip_serializing_if = "is_default")]
    pub images: Vec<Image>,
    /// Full path to the author folder.
    #[serde(default, skip_serializing_if = "is_default")]
    pub path: String,
    /// Quality profile applied to this author.
    #[serde(default, skip_serializing_if = "is_default")]
    pub quality_profile_id: i32,
    /// Metadata profile applied to this author.
    #[serde(default, skip_serializing_if = "is_default")]
    pub metadata_profile_id: i32,
    /// Genres this author writes.
    #[serde(default, skip_serializing_if = "is_default")]
    pub genres: Vec<String>,
    /// Name with special characters removed.
    #[serde(default, skip_serializing_if = "is_default")]
    pub clean_name: String,
    /// Name used for sorting.
    #[serde(default, skip_serializing_if = "is_default")]
    pub sort_name: String,
    /// Tags applied to this author.
    #[serde(default, skip_serializing_if = "is_default")]
    pub tags: Vec<i32>,
    /// When the author was added.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub added: Option<DateTime<Utc>>,
    /// Author ratings.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ratings: Option<Ratings>,
    /// Library statistics for this author.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub statistics: Option<Statistics>,
    /// The most recent book.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_book: Option<AuthorBook>,
    /// The next upcoming book.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_book: Option<AuthorBook>,
    /// Whether the author is no longer active.
    #[serde(default, skip_serializing_if = "is_default")]
    pub ended: bool,
    /// Whether the author is monitored.
    #[serde(default)]
    pub monitored: bool,
    /// Author metadata ID.
    #[serde(default)]
    pub author_metadata_id: i64,
    /// Author name in `Last, First` order.
    #[serde(default)]
    pub author_name_last_first: String,
    /// How new items are monitored.
    #[serde(default)]
    pub monitor_new_items: String,
    /// Sort name in `Last, First` order.
    #[serde(default)]
    pub sort_name_last_first: String,
}

/// AuthorBook is part of an [`Author`], and is very different from a normal
/// [`super::Book`] type.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthorBook {
    /// Book ID.
    #[serde(default)]
    pub id: i64,
    /// Author metadata ID.
    #[serde(default)]
    pub author_metadata_id: i32,
    /// Goodreads book ID.
    #[serde(default)]
    pub foreign_book_id: String,
    /// Slug used in URLs.
    #[serde(default)]
    pub title_slug: String,
    /// Book title.
    #[serde(default)]
    pub title: String,
    /// When the book was released.
    #[serde(default)]
    pub release_date: Option<DateTime<Utc>>,
    /// External links for this book.
    #[serde(default)]
    pub links: Vec<Link>,
    /// Genres this book belongs to.
    #[serde(default)]
    pub genres: Vec<String>,
    /// Book ratings.
    #[serde(default)]
    pub ratings: Option<Ratings>,
    /// Title with special characters removed.
    #[serde(default)]
    pub clean_title: String,
    /// Whether the book is monitored.
    #[serde(default)]
    pub monitored: bool,
    /// Whether any edition satisfies the monitor.
    #[serde(default)]
    pub any_edition_ok: bool,
    /// When metadata was last synced.
    #[serde(default)]
    pub last_info_sync: Option<DateTime<Utc>>,
    /// When the book was added.
    #[serde(default)]
    pub added: Option<DateTime<Utc>>,
    /// Options applied when adding the book.
    #[serde(default)]
    pub add_options: Option<AddBookOptions>,
    /// Whether the author metadata is loaded.
    #[serde(default)]
    pub author_metadata: Option<IsLoaded>,
    /// Whether the author is loaded.
    #[serde(default)]
    pub author: Option<IsLoaded>,
    /// Whether the editions are loaded.
    #[serde(default)]
    pub editions: Option<IsLoaded>,
    /// Whether the book files are loaded.
    #[serde(default)]
    pub book_files: Option<IsLoaded>,
    /// Whether the series links are loaded.
    #[serde(default)]
    pub series_links: Option<IsLoaded>,
}

/// Statistics for a Book, or maybe an author.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Statistics {
    /// Number of books.
    #[serde(default)]
    pub book_count: i32,
    /// Number of book files on disk.
    #[serde(default)]
    pub book_file_count: i32,
    /// Total number of books.
    #[serde(default)]
    pub total_book_count: i32,
    /// Bytes used on disk.
    #[serde(default)]
    pub size_on_disk: i32,
    /// Percentage of books present on disk.
    #[serde(default)]
    pub percent_of_books: f64,
    /// Number of books available.
    #[serde(default)]
    pub available_book_count: i32,
}

impl Readarr {
    /// Returns all authors in the library.
    pub async fn get_authors(&self) -> Result<Vec<Author>> {
        self.api.get_into(Request::new(BP_AUTHOR)).await
    }

    /// Adds a new author to Readarr.
    pub async fn add_author(&self, author: &Author) -> Result<Author> {
        self.api
            .post_into(Request::new(BP_AUTHOR).with_json(author)?)
            .await
    }

    /// Searches for authors matching the specified search term.
    pub async fn lookup_author(&self, term: &str) -> Result<Vec<Author>> {
        if term.is_empty() {
            return Ok(Vec::new());
        }

        let mut query = Values::new();
        query.set("term", term);

        self.api
            .get_into(Request::new(path_join(&[BP_AUTHOR, "lookup"])).with_query(query))
            .await
    }

    /// Returns an author.
    pub async fn get_author_by_id(&self, author_id: i64) -> Result<Author> {
        self.api
            .get_into(Request::new(path_join(&[BP_AUTHOR, &str_val(author_id)])))
            .await
    }

    /// Updates an author in place.
    pub async fn update_author(&self, author: &Author, move_files: bool) -> Result<Author> {
        let mut query = Values::new();
        query.add("moveFiles", str_val(move_files));

        let req = Request::new(path_join(&[BP_AUTHOR, &str_val(author.id)]))
            .with_json(author)?
            .with_query(query);
        self.api.put_into(req).await
    }

    /// Removes an Author from the database.
    ///
    /// Setting `delete_files` true will delete all content for the Author.
    pub async fn delete_author(
        &self,
        author_id: i64,
        delete_files: bool,
        add_import_exclusion: bool,
    ) -> Result<()> {
        let mut query = Values::new();
        query.set("deleteFiles", str_val(delete_files));
        query.set("addImportListExclusion", str_val(add_import_exclusion));

        self.api
            .delete_any(
                Request::new(path_join(&[BP_AUTHOR, &str_val(author_id)])).with_query(query),
            )
            .await
    }
}
