use super::Sonarr;
use crate::error::Result;
use crate::interface::ApiClientExt;
use crate::req::Request;

const BP_DISK_SPACE: &str = "v3/diskspace";

/// DiskSpace is the `/api/v3/diskspace` resource.
pub type DiskSpace = crate::starrshared::DiskSpace;

impl Sonarr {
    /// Returns disk space information for Sonarr paths.
    pub async fn get_disk_space(&self) -> Result<Vec<DiskSpace>> {
        self.api.get_into(Request::new(BP_DISK_SPACE)).await
    }
}
