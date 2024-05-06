use rvstruct::ValueStruct;
use serde::{Deserialize, Serialize};

use crate::models::{DateTimeUtc, FtCampusId, FtUser};

#[derive(Debug, Serialize, Deserialize)]
pub struct FtLocation {
    pub id: FtLocationId,
    pub begin_at: DateTimeUtc,
    pub end_at: Option<DateTimeUtc>,
    pub primary: bool,
    pub host: FtHost,
    pub campus_id: FtCampusId,
    pub user: FtUser,
}

#[derive(Debug, Eq, Hash, PartialEq, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtLocationId(i64);

#[derive(Debug, Eq, Hash, PartialEq, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtHost(pub String);