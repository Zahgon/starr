use super::Readarr;
use crate::error::Result;
use crate::interface::ApiClientExt;
use crate::req::Request;

const BP_DISK_SPACE: &str = "v1/diskspace";

/// DiskSpace is the `/api/v1/diskspace` resource.
pub type DiskSpace = crate::starrshared::DiskSpace;

impl Readarr {
    /// Returns disk space information for Readarr paths.
    pub async fn get_disk_space(&self) -> Result<Vec<DiskSpace>> {
        self.api.get_into(Request::new(BP_DISK_SPACE)).await
    }
}
