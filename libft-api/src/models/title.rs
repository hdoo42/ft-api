use crate::models::prelude::*;
use rvstruct::ValueStruct;
use serde::{Deserialize, Serialize};

//
// FtTitle and its field structs
//

#[derive(Debug, Eq, Hash, PartialEq, PartialOrd, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtTitleId(pub u64);

#[derive(Debug, Eq, Hash, PartialEq, PartialOrd, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtTitleName(pub String);

#[derive(Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct FtTitle {
    pub id: FtTitleId,
    pub name: FtTitleName,
}

//
// FtTitleUser and its field structs
//

#[derive(Debug, Eq, Hash, PartialEq, PartialOrd, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtTitleUserId(pub u64);

#[derive(Debug, Eq, Hash, PartialEq, PartialOrd, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtTitleUserUserId(pub u64);

#[derive(Debug, Eq, Hash, PartialEq, PartialOrd, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtTitleUserTitleId(pub u64);

#[derive(Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct FtTitleUser {
    pub id: FtTitleUserId,
    pub user_id: FtTitleUserUserId,
    pub title_id: FtTitleUserTitleId,
    pub selected: bool,
    pub created_at: FtDateTimeUtc,
    pub updated_at: FtDateTimeUtc,
}
