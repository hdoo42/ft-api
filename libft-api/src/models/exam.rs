use crate::models::prelude::*;
use rvstruct::ValueStruct;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct FtExamUser {
    pub id: FtExamUserId,
    pub exam_id: FtExamId,
    pub user_id: FtUserId,
    pub created_at: FtDateTimeUtc,
    pub updated_at: FtDateTimeUtc,
    pub user: FtUser,
    pub exam: FtExam,
}

#[derive(Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct FtExam {
    pub id: FtExamId,
    pub ip_range: String,
    pub begin_at: FtDateTimeUtc,
    pub end_at: FtDateTimeUtc,
    pub location: String,
    pub max_people: Option<i32>,
    pub nbr_subscribers: Option<i32>,
    pub name: String,
    pub created_at: FtDateTimeUtc,
    pub updated_at: FtDateTimeUtc,
}

#[derive(Debug, Eq, Hash, PartialEq, PartialOrd, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtExamId(pub i32);

#[derive(Debug, Eq, Hash, PartialEq, PartialOrd, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtExamUserId(pub i32);
