use rvstruct::ValueStruct;
use serde::{Deserialize, Serialize};

use crate::{
    FtCursusId, FtDateTimeUtc, FtFinalMark, FtProjects, FtStatus, FtTeam, FtTeamId, FtUser,
}; // Assuming these are already defined

#[derive(Debug, Serialize, Deserialize)]
pub struct FtProjectsUsers {
    pub created_at: FtDateTimeUtc,
    pub current_team_id: Option<FtTeamId>,
    pub cursus_ids: Vec<FtCursusId>,
    pub final_mark: Option<FtFinalMark>,
    pub id: FtProjectUserId,
    pub marked: bool,
    pub marked_at: Option<FtDateTimeUtc>,
    pub occurrence: FtOccurrence,
    pub project: FtProjects,
    pub retriable_at: Option<FtDateTimeUtc>,
    pub status: FtStatus,
    pub teams: Vec<FtTeam>,
    pub updated_at: FtDateTimeUtc,
    pub user: FtUser,
    pub validated: Option<bool>,
}

#[derive(Debug, Eq, Hash, PartialEq, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtProjectUserId(pub i32);

#[derive(Debug, Eq, Hash, PartialEq, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtOccurrence(i32);
