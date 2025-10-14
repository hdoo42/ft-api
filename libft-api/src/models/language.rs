use crate::models::prelude::*;
use rvstruct::ValueStruct;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct FtLanguage {
    pub id: FtLanguageId,
    pub identifier: String,
    pub name: String,
    pub created_at: Option<FtDateTimeUtc>,
    pub updated_at: Option<FtDateTimeUtc>,
}

#[derive(Debug, Eq, Hash, PartialEq, PartialOrd, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtLanguageId(i32);

//
// FtLanguagesUser and its field structs
//

#[derive(Debug, Eq, Hash, PartialEq, PartialOrd, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtLanguagesUserId(pub u64);

#[derive(Debug, Eq, Hash, PartialEq, PartialOrd, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtLanguagesUserLanguageId(pub u64);

#[derive(Debug, Eq, Hash, PartialEq, PartialOrd, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtLanguagesUserUserId(pub u64);

#[derive(Debug, Eq, Hash, PartialEq, PartialOrd, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtLanguagesUserPosition(pub u64);

#[derive(Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct FtLanguagesUser {
    pub id: FtLanguagesUserId,
    pub language_id: FtLanguagesUserLanguageId,
    pub user_id: FtLanguagesUserUserId,
    pub position: FtLanguagesUserPosition,
    pub created_at: FtDateTimeUtc,
}
