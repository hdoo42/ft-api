use serde::{Deserialize, Serialize};

use crate::FtUrl;

#[derive(Debug, Serialize, Deserialize)]
pub struct FtImage {
    pub link: Option<FtUrl>,
    pub versions: Option<FtVersions>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FtVersions {
    large: Option<FtUrl>,
    medium: Option<FtUrl>,
    small: Option<FtUrl>,
    micro: Option<FtUrl>,
}
