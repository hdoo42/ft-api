use rvstruct::ValueStruct;
use serde::{Deserialize, Serialize};

use super::FtDateTimeUtc;
//
// FtRole and its field structs
//

#[derive(Debug, Eq, Hash, PartialEq, PartialOrd, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtRoleId(pub u64);

#[derive(Debug, Eq, Hash, PartialEq, PartialOrd, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtRoleName(pub String);

#[derive(Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct FtRole {
    pub id: FtRoleId,
    pub name: FtRoleName,
}
