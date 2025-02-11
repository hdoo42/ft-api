use rvstruct::ValueStruct;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct FtGroup {
    pub id: FtGroupId,
    pub name: FtGroupName,
}

// #[derive(Debug, PartialEq, PartialOrd, Clone, Serialize, Deserialize)]
// pub enum FtGroupId {
//     Staff = 1,
//     TestAccount = 119,
// }
//
#[derive(Debug, Eq, Hash, PartialEq, PartialOrd, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtGroupId(i32);

#[derive(Debug, Eq, Hash, PartialEq, PartialOrd, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtGroupName(String);
