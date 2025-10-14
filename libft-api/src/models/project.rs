use crate::models::prelude::*;
use rvstruct::ValueStruct;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct FtProject {
    pub campus: Option<Vec<FtCampus>>,
    pub children: Option<Vec<FtProject>>,
    pub created_at: Option<FtDateTimeUtc>,
    pub cursus: Option<Vec<FtCursus>>,
    pub difficulty: Option<i32>,
    pub exam: Option<bool>,
    pub git_id: Option<i32>,
    pub id: FtProjectId,
    pub name: FtProjectName,
    pub parent: Option<FtParentProject>,
    pub project_sessions: Option<Vec<FtProjectSession>>,
    pub repository: Option<String>,
    pub slug: FtSlug,
    pub updated_at: Option<FtDateTimeUtc>,
    pub videos: Option<Vec<FtVideo>>,
}

#[derive(Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct FtChildProject {
    pub name: String,
    pub id: FtProjectId,
    pub slug: FtSlug,
    pub url: FtUrl,
}

#[derive(Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct FtCursus {
    pub id: FtCursusId,
    pub created_at: FtDateTimeUtc,
    pub name: String,
    pub slug: FtSlug,
    pub kind: String,
}

#[derive(Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct FtParentProject {
    pub name: FtProjectName,
    pub id: FtProjectId,
    pub slug: FtSlug,
    pub url: FtUrl,
}

#[derive(Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct FtVideo {}

// Newtype structs for various IDs and other specific fields
#[derive(Debug, Eq, Hash, PartialEq, PartialOrd, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtProjectId(pub i32);

#[derive(Debug, Eq, Hash, PartialEq, PartialOrd, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtProjectName(pub String);

#[derive(Debug, Eq, Hash, PartialEq, PartialOrd, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtSlug(pub String);

#[derive(Debug, Eq, Hash, PartialEq, PartialOrd, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtCursusId(pub i32);
