use rvstruct::ValueStruct;
use serde::{Deserialize, Serialize};

use crate::{FtCampus, FtDateTimeUtc, FtUrl};

#[derive(Debug, Serialize, Deserialize)]
pub struct FtProjects {
    pub campus: Vec<FtCampus>,
    pub children: Vec<FtChildProject>,
    pub created_at: FtDateTimeUtc,
    pub cursus: Vec<FtCursus>,
    pub difficulty: Option<i32>,
    pub exam: bool,
    pub git_id: Option<i32>,
    pub id: FtProjectId,
    pub name: FtProjectName,
    pub parent: Option<FtParentProject>,
    pub project_sessions: Vec<FtProjectSession>,
    pub repository: Option<String>,
    pub slug: FtSlug,
    pub updated_at: FtDateTimeUtc,
    pub videos: Vec<FtVideo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FtChildProject {
    pub name: String,
    pub id: FtProjectId,
    pub slug: FtSlug,
    pub url: FtUrl,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FtCursus {
    pub id: FtCursusId,
    pub created_at: FtDateTimeUtc,
    pub name: String,
    pub slug: FtSlug,
    pub kind: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FtParentProject {
    pub name: String,
    pub id: FtProjectId,
    pub slug: FtSlug,
    pub url: FtUrl,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FtProjectSession {
    pub id: i32,
    pub solo: bool,
    pub begin_at: Option<FtDateTimeUtc>,
    pub end_at: Option<FtDateTimeUtc>,
    pub estimate_time: String,
    pub difficulty: Option<i32>,
    pub objectives: Vec<String>,
    pub description: String,
    pub duration_days: Option<i32>,
    pub terminating_after: Option<String>,
    pub project_id: FtProjectId,
    pub campus_id: Option<i32>,
    pub cursus_id: Option<i32>,
    pub created_at: FtDateTimeUtc,
    pub updated_at: FtDateTimeUtc,
    pub max_people: Option<i32>,
    pub is_subscriptable: bool,
    pub scales: Vec<String>,
    pub uploads: Vec<String>,
    pub team_behaviour: String,
    pub commit: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FtVideo {}

// Newtype structs for various IDs and other specific fields
#[derive(Debug, Eq, Hash, PartialEq, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtProjectId(pub i32);

#[derive(Debug, Eq, Hash, PartialEq, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtProjectName(pub String);

#[derive(Debug, Eq, Hash, PartialEq, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtSlug(pub String);

#[derive(Debug, Eq, Hash, PartialEq, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtCursusId(pub i32);
