use super::{Author, Readarr, Statistics};
use crate::error::Result;
use crate::helpers::str_val;
use crate::interface::ApiClientExt;
use crate::is_default;
use crate::req::{Request, path_join};
use crate::shared::{Image, Link, Ratings};
use crate::values::Values;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

const BP_BOOK: &str = "v1/book";

/// Book is the `/api/v1/book` endpoint among others, and gets used across this module.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Book {
    /// When the book was added.
    #[serde(default)]
    pub added: Option<DateTime<Utc>>,
    /// Whether any edition satisfies the monitor.
    #[serde(default)]
    pub any_edition_ok: bool,
    /// Author the book belongs to.
    #[serde(default)]
    pub author_id: i64,
    /// Title of the author.
    #[serde(default)]
    pub author_title: String,
    /// Text that distinguishes this book from others with the same title.
    #[serde(default, skip_serializing_if = "is_default")]
    pub disambiguation: String,
    /// Editions of this book.
    #[serde(default)]
    pub editions: Vec<Edition>,
    /// Goodreads book ID.
    #[serde(default)]
    pub foreign_book_id: String,
    /// Genres this book belongs to.
    #[serde(default)]
    pub genres: Vec<String>,
    /// Book ID.
    #[serde(default)]
    pub id: i64,
    /// Book images.
    #[serde(default)]
    pub images: Vec<Image>,
    /// External links for this book.
    #[serde(default)]
    pub links: Vec<Link>,
    /// Whether the book is monitored.
    #[serde(default)]
    pub monitored: bool,
    /// Whether the book was grabbed.
    #[serde(default)]
    pub grabbed: bool,
    /// Book overview.
    #[serde(default)]
    pub overview: String,
    /// Number of pages.
    #[serde(default)]
    pub page_count: i32,
    /// Book ratings.
    #[serde(default)]
    pub ratings: Option<Ratings>,
    /// When the book was released.
    #[serde(default)]
    pub release_date: Option<DateTime<Utc>>,
    /// URL of a remote cover image.
    #[serde(default, skip_serializing_if = "is_default")]
    pub remote_cover: String,
    /// Title of the series this book belongs to.
    #[serde(default)]
    pub series_title: String,
    /// Library statistics for this book.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub statistics: Option<Statistics>,
    /// Book title.
    #[serde(default)]
    pub title: String,
    /// Slug used in URLs.
    #[serde(default)]
    pub title_slug: String,
    /// The author of this book.
    #[serde(default)]
    pub author: Option<Author>,
}

/// Edition is more Book meta data.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Edition {
    /// Edition ID.
    #[serde(default)]
    pub id: i64,
    /// Book this edition belongs to.
    #[serde(default)]
    pub book_id: i64,
    /// Goodreads edition ID.
    #[serde(default)]
    pub foreign_edition_id: String,
    /// Slug used in URLs.
    #[serde(default)]
    pub title_slug: String,
    /// ISBN-13 of this edition.
    #[serde(default, rename = "isbn13")]
    pub isbn13: String,
    /// Amazon ASIN of this edition.
    #[serde(default)]
    pub asin: String,
    /// Edition title.
    #[serde(default)]
    pub title: String,
    /// Edition overview.
    #[serde(default)]
    pub overview: String,
    /// Format of this edition.
    #[serde(default)]
    pub format: String,
    /// Publisher of this edition.
    #[serde(default)]
    pub publisher: String,
    /// Number of pages.
    #[serde(default)]
    pub page_count: i32,
    /// When the edition was released.
    #[serde(default)]
    pub release_date: Option<DateTime<Utc>>,
    /// Edition images.
    #[serde(default)]
    pub images: Vec<Image>,
    /// External links for this edition.
    #[serde(default)]
    pub links: Vec<Link>,
    /// Edition ratings.
    #[serde(default)]
    pub ratings: Option<Ratings>,
    /// Whether the edition is monitored.
    #[serde(default)]
    pub monitored: bool,
    /// Whether the edition was added manually.
    #[serde(default)]
    pub manual_add: bool,
    /// Whether the edition is an ebook.
    #[serde(default)]
    pub is_ebook: bool,
}

/// AddBookInput is the input to add a book.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddBookInput {
    /// Whether the book is monitored.
    #[serde(default)]
    pub monitored: bool,
    /// Tags applied to this book.
    #[serde(default)]
    pub tags: Vec<i32>,
    /// Contains Search.
    #[serde(default)]
    pub add_options: Option<AddBookOptions>,
    /// Contains Author ID.
    #[serde(default)]
    pub author: Option<AddBookAuthor>,
    /// Contains GRID Edition ID.
    #[serde(default)]
    pub editions: Vec<AddBookEdition>,
    /// GRID Book ID.
    #[serde(default)]
    pub foreign_book_id: String,
}

/// AddBookOptions is part of [`AddBookInput`].
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddBookOptions {
    /// How the book is added.
    #[serde(default, skip_serializing_if = "is_default")]
    pub add_type: String,
    /// Whether to search for the book after adding it.
    #[serde(default)]
    pub search_for_new_book: bool,
}

/// AddBookAuthor is part of [`AddBookInput`].
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddBookAuthor {
    /// Whether the author is monitored.
    #[serde(default)]
    pub monitored: bool,
    /// Quality profile applied to this author. Required.
    #[serde(default)]
    pub quality_profile_id: i64,
    /// Metadata profile applied to this author. Required.
    #[serde(default)]
    pub metadata_profile_id: i64,
    /// Goodreads author ID. Required.
    #[serde(default)]
    pub foreign_author_id: String,
    /// Root folder the author is placed in. Required.
    #[serde(default)]
    pub root_folder_path: String,
    /// Tags applied to this author.
    #[serde(default)]
    pub tags: Vec<i32>,
    /// Options applied when adding the author.
    #[serde(default)]
    pub add_options: Option<AddAuthorOptions>,
}

/// AddAuthorOptions is part of [`AddBookAuthor`].
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddAuthorOptions {
    /// Whether missing books are searched for.
    #[serde(default)]
    pub search_for_missing_books: bool,
    /// Whether the author is monitored.
    #[serde(default)]
    pub monitored: bool,
    /// What gets monitored on the author.
    #[serde(default)]
    pub monitor: String,
    /// Books that get monitored.
    #[serde(default)]
    pub books_to_monitor: Vec<String>,
}

/// AddBookEdition is part of [`AddBookInput`].
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddBookEdition {
    /// Edition title.
    #[serde(default)]
    pub title: String,
    /// Slug used in URLs.
    #[serde(default)]
    pub title_slug: String,
    /// Edition images.
    #[serde(default)]
    pub images: Vec<Image>,
    /// GRID edition ID.
    #[serde(default)]
    pub foreign_edition_id: String,
    /// Whether the edition is monitored.
    #[serde(default)]
    pub monitored: bool,
    /// Whether the edition was added manually.
    #[serde(default)]
    pub manual_add: bool,
}

/// BooksMonitoredInput is the body for `PUT /book/monitor`.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BooksMonitoredInput {
    /// Books to change.
    #[serde(default)]
    pub book_ids: Vec<i64>,
    /// Monitored state to apply.
    #[serde(default)]
    pub monitored: bool,
}

impl Readarr {
    /// Returns books. All books are returned if `grid_id` is empty.
    pub async fn get_book(&self, grid_id: &str) -> Result<Vec<Book>> {
        let mut query = Values::new();
        if !grid_id.is_empty() {
            query.add("titleSlug", grid_id); // this may change, but works for now.
        }

        self.api
            .get_into(Request::new(BP_BOOK).with_query(query))
            .await
    }

    /// Returns a book.
    pub async fn get_book_by_id(&self, book_id: i64) -> Result<Book> {
        self.api
            .get_into(Request::new(path_join(&[BP_BOOK, &str_val(book_id)])))
            .await
    }

    /// Updates a book in place.
    pub async fn update_book(&self, book_id: i64, book: &Book, move_files: bool) -> Result<()> {
        let mut query = Values::new();
        query.add("moveFiles", str_val(move_files));

        let req = Request::new(path_join(&[BP_BOOK, &str_val(book_id)]))
            .with_json(book)?
            .with_query(query);
        self.api.put_any(req).await
    }

    /// Adds a new book to the library.
    pub async fn add_book(&self, book: &AddBookInput) -> Result<Book> {
        self.api
            .post_into(Request::new(BP_BOOK).with_json(book)?)
            .await
    }

    /// Searches for books matching the specified search term.
    pub async fn lookup(&self, term: &str) -> Result<Vec<Book>> {
        if term.is_empty() {
            return Ok(Vec::new());
        }

        let mut query = Values::new();
        query.set("term", term);

        self.api
            .get_into(Request::new(path_join(&[BP_BOOK, "lookup"])).with_query(query))
            .await
    }

    /// Removes a Book from the database.
    ///
    /// Setting `delete_files` true will delete all content for the Book.
    pub async fn delete_book(
        &self,
        book_id: i64,
        delete_files: bool,
        add_import_exclusion: bool,
    ) -> Result<()> {
        let mut query = Values::new();
        query.set("deleteFiles", str_val(delete_files));
        query.set("addImportListExclusion", str_val(add_import_exclusion));

        self.api
            .delete_any(Request::new(path_join(&[BP_BOOK, &str_val(book_id)])).with_query(query))
            .await
    }

    /// Sets monitored state for the given book IDs.
    pub async fn monitor_books(&self, book_ids: &[i64], monitored: bool) -> Result<Vec<Book>> {
        let input = BooksMonitoredInput {
            book_ids: book_ids.to_vec(),
            monitored,
        };

        let req = Request::new(path_join(&[BP_BOOK, "monitor"])).with_json(&input)?;
        self.api.put_into(req).await
    }
}
