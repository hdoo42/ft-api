use rsb_derive::Builder;
use rvstruct::ValueStruct;
use serde::{Deserialize, Serialize};

use crate::models::{FtCampusId, FtDateTimeUtc, FtUser};

#[derive(Debug, Serialize, Deserialize)]
pub struct FtLocation {
    pub id: FtLocationId,
    pub begin_at: FtDateTimeUtc,
    pub end_at: Option<FtDateTimeUtc>,
    pub primary: bool,
    pub host: FtHost,
    pub campus_id: FtCampusId,
    pub user: FtUser,
}

#[derive(Debug, Eq, Hash, PartialEq, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtLocationId(i64);

#[derive(Debug, Eq, Hash, PartialEq, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtHost(pub String);

#[derive(Debug, Eq, Hash, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FtFilterField {
    Id,
    UserId,
    BeginAt,
    EndAt,
    Primary,
    Host,
    CampusId,
    Active,
    Inactive,
    Future,
    End,
}

#[derive(Debug, Eq, Hash, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct FtFilterOption {
    pub field: FtFilterField,
    pub value: Vec<String>,
}

#[derive(Debug, Eq, Hash, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FtLocationSortField {
    Id,
    UserId,
    BeginAt,
    EndAt,
    Primary,
    Host,
    CampusId,
}

#[derive(Debug, Eq, Hash, PartialEq, Clone, Serialize, Deserialize)]
pub struct FtLocationSortOption {
    pub field: FtLocationSortField,
    pub descending: bool,
}
