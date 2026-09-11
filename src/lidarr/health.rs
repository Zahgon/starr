use super::Lidarr;
use crate::error::Result;
use crate::interface::ApiClientExt;
use crate::req::Request;

const BP_HEALTH: &str = "v1/health";

/// Health is the `/api/v1/health` resource.
pub type Health = crate::starrshared::Health;

impl Lidarr {
    /// Returns current health check messages.
    pub async fn get_health(&self) -> Result<Vec<Health>> {
        self.api.get_into(Request::new(BP_HEALTH)).await
    }
}
