use rvstruct::ValueStruct;
use serde::{Deserialize, Serialize};

use crate::FtDateTimeUtc;

#[derive(Debug, Serialize, Deserialize)]
pub struct FtLanguage {
    pub id: FtLanguageId,
    pub identifier: String,
    pub name: String,
    pub created_at: Option<FtDateTimeUtc>,
    pub updated_at: Option<FtDateTimeUtc>,
}

#[derive(Debug, Eq, Hash, PartialEq, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtLanguageId(i32);
