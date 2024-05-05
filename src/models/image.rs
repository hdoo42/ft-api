use serde::{Deserialize, Serialize};

use crate::FtUrl;

#[derive(Debug, Serialize, Deserialize)]
pub struct FtImage {
    pub link: FtUrl,
    pub versions: Option<FtVersions>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FtVersions {
    large: FtUrl,
    medium: FtUrl,
    small: FtUrl,
    micro: FtUrl,
}
