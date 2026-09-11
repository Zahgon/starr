use super::Sonarr;
use crate::error::Result;
use crate::helpers::str_val;
use crate::interface::ApiClientExt;
use crate::req::{Request, path_join};
use crate::shared::Tag;

const BP_TAG: &str = "v3/tag";

impl Sonarr {
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
}
