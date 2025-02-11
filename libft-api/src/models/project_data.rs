use rvstruct::ValueStruct;
use serde::{Deserialize, Serialize};

use crate::FtProjectSessionId;

#[derive(Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct FtProjectData {
    pub by: Vec<Vec<FtProjectDataBy>>,
    pub coordinates: Vec<FtProjectDataCoordinate>,
    pub id: FtProjectDataId,
    pub kind: FtProjectDataKind,
    pub project_session_id: FtProjectSessionId,
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtProjectDataBy(pub f64);

#[derive(Debug, PartialEq, PartialOrd, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtProjectDataCoordinate(pub f64);

#[derive(Debug, PartialEq, PartialOrd, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtProjectDataId(pub i64);

#[derive(Debug, PartialEq, PartialOrd, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtProjectDataKind(pub String);
