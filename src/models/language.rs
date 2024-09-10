use rvstruct::ValueStruct;
use serde::{Deserialize, Serialize};

use crate::FtDateTimeUtc;

#[derive(Debug, Serialize, Deserialize)]
pub struct FtLanguage {
    pub created_at: FtDateTimeUtc,
    pub id: FtLanguageId,
    pub identifier: String,
    pub name: String,
    pub updated_at: FtDateTimeUtc,
}

#[derive(Debug, Eq, Hash, PartialEq, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtLanguageId(i32);
