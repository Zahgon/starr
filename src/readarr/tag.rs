use super::Readarr;
use crate::error::Result;
use crate::helpers::str_val;
use crate::interface::ApiClientExt;
use crate::is_default;
use crate::req::{Request, path_join};
use crate::shared::Tag;
use serde::{Deserialize, Serialize};

const BP_TAG: &str = "v1/tag";

/// TagDetails is the `/api/v1/tag/detail` resource.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TagDetails {
    /// Tag ID.
    #[serde(default)]
    pub id: i32,
    /// Tag label.
    #[serde(default, skip_serializing_if = "is_default")]
    pub label: String,
    /// Delay profiles using this tag.
    #[serde(default, skip_serializing_if = "is_default")]
    pub delay_profile_ids: Vec<i32>,
    /// Import lists using this tag.
    #[serde(default, skip_serializing_if = "is_default")]
    pub import_list_ids: Vec<i32>,
    /// Notifications using this tag.
    #[serde(default, skip_serializing_if = "is_default")]
    pub notification_ids: Vec<i32>,
    /// Indexers using this tag.
    #[serde(default, skip_serializing_if = "is_default")]
    pub indexer_ids: Vec<i32>,
    /// Download clients using this tag.
    #[serde(default, skip_serializing_if = "is_default")]
    pub download_client_ids: Vec<i32>,
    /// Auto tag rules using this tag.
    #[serde(default, skip_serializing_if = "is_default")]
    pub auto_tag_ids: Vec<i32>,
    /// Authors using this tag.
    #[serde(default, skip_serializing_if = "is_default")]
    pub author_ids: Vec<i32>,
    /// Indexer proxies using this tag.
    #[serde(default, skip_serializing_if = "is_default")]
    pub indexer_proxy_ids: Vec<i32>,
}

impl Readarr {
    /// Returns all configured tags.
    pub async fn get_tags(&self) -> Result<Vec<Tag>> {
        self.api.get_into(Request::new(BP_TAG)).await
    }

    /// Returns a single tag.
    pub async fn get_tag(&self, tag_id: i32) -> Result<Tag> {
        self.api
            .get_into(Request::new(path_join(&[BP_TAG, &str_val(tag_id)])))
            .await
    }

    /// Creates a tag.
    pub async fn add_tag(&self, tag: &Tag) -> Result<Tag> {
        self.api
            .post_into(Request::new(BP_TAG).with_json(tag)?)
            .await
    }

    /// Updates a tag.
    pub async fn update_tag(&self, tag: &Tag) -> Result<Tag> {
        let req = Request::new(path_join(&[BP_TAG, &str_val(tag.id)])).with_json(tag)?;
        self.api.put_into(req).await
    }

    /// Removes a single tag.
    pub async fn delete_tag(&self, tag_id: i32) -> Result<()> {
        self.api
            .delete_any(Request::new(path_join(&[BP_TAG, &str_val(tag_id)])))
            .await
    }

    /// Returns tag usage details for all tags.
    pub async fn get_tag_details(&self) -> Result<Vec<TagDetails>> {
        self.api
            .get_into(Request::new(path_join(&[BP_TAG, "detail"])))
            .await
    }

    /// Returns tag usage details for a single tag.
    pub async fn get_tag_detail(&self, tag_id: i32) -> Result<TagDetails> {
        self.api
            .get_into(Request::new(path_join(&[
                BP_TAG,
                "detail",
                &str_val(tag_id),
            ])))
            .await
    }
}
