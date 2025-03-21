use rvstruct::ValueStruct;
use serde::{Deserialize, Serialize};

use crate::models::{FtCampusId, FtDateTimeUtc, FtUser};

#[derive(Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct FtLocation {
    pub id: FtLocationId,
    pub begin_at: FtDateTimeUtc,
    pub end_at: Option<FtDateTimeUtc>,
    pub primary: bool,
    pub host: FtHost,
    pub campus_id: FtCampusId,
    pub user: FtUser,
}

#[derive(Debug, Eq, Hash, PartialEq, PartialOrd, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtLocationId(i64);

#[derive(Debug, Eq, Hash, PartialEq, PartialOrd, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtHost(pub String);
