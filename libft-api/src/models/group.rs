use rvstruct::ValueStruct;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct FtGroup {
    pub id: FtGroupId,
    pub name: FtGroupName,
}

// #[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
// pub enum FtGroupId {
//     Staff = 1,
//     TestAccount = 119,
// }
//
#[derive(Debug, Eq, Hash, PartialEq, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtGroupId(i32);

#[derive(Debug, Eq, Hash, PartialEq, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtGroupName(String);
