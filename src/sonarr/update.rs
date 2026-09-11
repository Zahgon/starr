use super::Sonarr;
use crate::error::Result;
use crate::interface::ApiClientExt;
use crate::req::Request;

const BP_UPDATE: &str = "v3/update";

/// UpdateChanges is the change log embedded in [`Update`].
pub type UpdateChanges = crate::starrshared::UpdateChanges;

/// Update is one available or installed update from `/api/v3/update`.
pub type Update = crate::starrshared::Update;

impl Sonarr {
    /// Returns available application updates.
    pub async fn get_updates(&self) -> Result<Vec<Update>> {
        self.api.get_into(Request::new(BP_UPDATE)).await
    }
}
