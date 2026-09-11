use super::Readarr;
use crate::error::Result;
use crate::helpers::str_val;
use crate::interface::ApiClientExt;
use crate::is_default;
use crate::req::{Request, path_join};
use crate::shared::{PlayTime, Quality};
use crate::values::Values;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

const BP_BOOK_FILE: &str = "v1/bookfile";

/// BookFile represents the data from the bookfile endpoint.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BookFile {
    /// Author the file belongs to.
    #[serde(default)]
    pub author_id: i64,
    /// Book the file belongs to.
    #[serde(default)]
    pub book_id: i64,
    /// Path to the file on disk.
    #[serde(default)]
    pub path: String,
    /// File size in bytes.
    #[serde(default, skip_serializing_if = "is_default")]
    pub size: i32,
    /// When the file was added.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date_added: Option<DateTime<Utc>>,
    /// Quality of the file.
    #[serde(default)]
    pub quality: Option<Quality>,
    /// Weight of the quality.
    #[serde(default, skip_serializing_if = "is_default")]
    pub quality_weight: i32,
    /// Whether the quality cutoff was not met.
    #[serde(default)]
    pub quality_cutoff_not_met: bool,
    /// Book file ID.
    #[serde(default)]
    pub id: i64,
    /// Audio tags read from the file.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub audio_tags: Option<AudioTags>,
}

/// AudioTags are part of a [`BookFile`].
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AudioTags {
    /// Book title.
    #[serde(default)]
    pub title: String,
    /// Title with special characters removed.
    #[serde(default)]
    pub clean_title: String,
    /// Authors of the book.
    #[serde(default)]
    pub authors: Vec<String>,
    /// Title of the author.
    #[serde(default)]
    pub author_title: String,
    /// Title of the book.
    #[serde(default)]
    pub book_title: String,
    /// Title of the series.
    #[serde(default)]
    pub series_title: String,
    /// Position in the series.
    #[serde(default)]
    pub series_index: String,
    /// ISBN of the book.
    #[serde(default, rename = "isbn")]
    pub isbn: String,
    /// Amazon ASIN of the book.
    #[serde(default, rename = "asin")]
    pub asin: String,
    /// Goodreads ID of the book.
    #[serde(default, rename = "goodreadsId")]
    pub goodreads_id: String,
    /// MusicBrainz author ID.
    #[serde(default, rename = "authorMBId")]
    pub author_mbid: String,
    /// MusicBrainz book ID.
    #[serde(default, rename = "bookMBId")]
    pub book_mbid: String,
    /// MusicBrainz release ID.
    #[serde(default, rename = "releaseMBId")]
    pub release_mbid: String,
    /// MusicBrainz recording ID.
    #[serde(default, rename = "recordingMBId")]
    pub recording_mbid: String,
    /// MusicBrainz track ID.
    #[serde(default, rename = "trackMBId")]
    pub track_mbid: String,
    /// Disc this file is on.
    #[serde(default)]
    pub disc_number: i32,
    /// Number of discs in the release.
    #[serde(default)]
    pub disc_count: i32,
    /// Country the release came from.
    #[serde(default)]
    pub country: Option<AudioCountry>,
    /// Release year.
    #[serde(default)]
    pub year: i32,
    /// Publisher of the book.
    #[serde(default)]
    pub publisher: String,
    /// Record label.
    #[serde(default)]
    pub label: String,
    /// Source of the file.
    #[serde(default)]
    pub source: String,
    /// Catalog number.
    #[serde(default)]
    pub catalog_number: String,
    /// Text that distinguishes this release from others.
    #[serde(default)]
    pub disambiguation: String,
    /// Duration of the audio.
    #[serde(default)]
    pub duration: PlayTime,
    /// Quality of the file.
    #[serde(default)]
    pub quality: Option<Quality>,
    /// Media information read from the tags.
    #[serde(default)]
    pub media_info: Option<AudioMediaInfo>,
    /// Track numbers in the release.
    #[serde(default)]
    pub track_numbers: Vec<i32>,
    /// Language of the book.
    #[serde(default)]
    pub language: String,
    /// Release group that produced the release.
    #[serde(default)]
    pub release_group: String,
    /// Hash of the release.
    #[serde(default)]
    pub release_hash: String,
}

/// AudioMediaInfo is part of [`AudioTags`].
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AudioMediaInfo {
    /// Audio format.
    #[serde(default)]
    pub audio_format: String,
    /// Audio bitrate.
    #[serde(default)]
    pub audio_bitrate: i32,
    /// Number of audio channels.
    #[serde(default)]
    pub audio_channels: i32,
    /// Audio bit depth.
    #[serde(default)]
    pub audio_bits: i32,
    /// Audio sample rate.
    #[serde(default)]
    pub audio_sample_rate: i32,
}

/// AudioCountry is part of [`AudioTags`].
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AudioCountry {
    /// Two letter country code.
    #[serde(default)]
    pub two_letter_code: String,
    /// Country name.
    #[serde(default)]
    pub name: String,
}

impl Readarr {
    /// Returns the book files for an author.
    pub async fn get_book_files_for_author(&self, author_id: i64) -> Result<Vec<BookFile>> {
        let mut query = Values::new();
        query.add("authorId", str_val(author_id));

        self.api
            .get_into(Request::new(BP_BOOK_FILE).with_query(query))
            .await
    }

    /// Returns the book files for a book or books.
    pub async fn get_book_files_for_book(&self, book_ids: &[i64]) -> Result<Vec<BookFile>> {
        let mut query = Values::new();
        for id in book_ids {
            query.add("bookId", str_val(*id));
        }

        self.api
            .get_into(Request::new(BP_BOOK_FILE).with_query(query))
            .await
    }

    /// Returns the requested book files by their IDs.
    pub async fn get_book_files(&self, book_file_ids: &[i64]) -> Result<Vec<BookFile>> {
        if book_file_ids.is_empty() {
            return Ok(Vec::new());
        }

        let mut query = Values::new();
        for id in book_file_ids {
            query.add("bookFileIds", str_val(*id));
        }

        self.api
            .get_into(Request::new(BP_BOOK_FILE).with_query(query))
            .await
    }

    /// Updates a book file.
    pub async fn update_book_file(&self, book_file: &BookFile) -> Result<BookFile> {
        let req =
            Request::new(path_join(&[BP_BOOK_FILE, &str_val(book_file.id)])).with_json(book_file)?;
        self.api.put_into(req).await
    }

    /// Deletes a book file.
    pub async fn delete_book_file(&self, book_file_id: i64) -> Result<()> {
        self.api
            .delete_any(Request::new(path_join(&[
                BP_BOOK_FILE,
                &str_val(book_file_id),
            ])))
            .await
    }

    /// Bulk deletes book files by their IDs.
    pub async fn delete_book_files(&self, book_file_ids: &[i64]) -> Result<()> {
        #[derive(Serialize)]
        struct PostData<'a> {
            #[serde(rename = "bookFileIds")]
            book_file_ids: &'a [i64],
        }

        let req = Request::new(path_join(&[BP_BOOK_FILE, "bulk"])).with_json(&PostData {
            book_file_ids,
        })?;
        self.api.delete_any(req).await
    }
}
