use rsb_derive::Builder;
use rvstruct::ValueStruct;
use serde::{Deserialize, Serialize};

use crate::{FtDateTimeFixedOffset, FtDateTimeUtc, FtHost, FtImage, FtUrl};

use super::{
    FtAchievement, FtCampus, FtCampusUser, FtCursusUser, FtLanguagesUser, FtProjectsUser, FtRole,
    FtTitle, FtTitleUser,
};

#[derive(Debug, PartialEq, PartialOrd, Serialize, Deserialize, Builder)]
pub struct FtUser {
    pub id: Option<FtUserId>,
    pub email: Option<FtEmail>,
    pub login: Option<FtLoginId>,
    pub first_name: Option<FtFirstName>,
    pub last_name: Option<FtLastName>,
    pub url: Option<FtUrl>,
    pub phone: Option<FtPhone>,
    pub displayname: Option<FtDisplayName>,
    pub kind: Option<FtKind>,
    pub active: Option<bool>,
    pub alumni: Option<bool>,
    pub alumnized_at: Option<FtDateTimeFixedOffset>,
    pub anonymize_date: Option<FtDateTimeFixedOffset>,
    pub correction_point: Option<FtCorrectionPoint>,
    pub created_at: Option<FtDateTimeUtc>,
    pub data_erasure_date: Option<FtDateTimeUtc>,
    pub image: Option<FtImage>,
    pub location: Option<FtHost>,
    pub pool_month: Option<FtPoolMonth>,
    pub pool_year: Option<FtPoolYear>,
    pub staff: Option<bool>,
    pub updated_at: Option<FtDateTimeUtc>,
    pub usual_first_name: Option<FtUsualFirstName>,
    pub usual_full_name: Option<FtUsualFullName>,
    pub wallet: Option<FtWallet>,
    pub cursus_users: Option<Vec<FtCursusUser>>,
    pub projects_users: Option<Vec<FtProjectsUser>>,
    pub languages_users: Option<Vec<FtLanguagesUser>>,
    pub achievements: Option<Vec<FtAchievement>>,
    pub campus: Option<Vec<FtCampus>>,
    pub campus_users: Option<Vec<FtCampusUser>>,
    pub titles: Option<Vec<FtTitle>>,
    pub titles_users: Option<Vec<FtTitleUser>>,
    pub roles: Option<Vec<FtRole>>,
}

#[derive(Debug, Eq, Hash, PartialEq, PartialOrd, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FtPoolMonth {
    January,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December,
}

#[derive(Debug, Eq, Hash, PartialEq, PartialOrd, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtPoolYear(pub String);

#[derive(Debug, Eq, Hash, PartialEq, PartialOrd, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtEmail(String);

#[derive(Debug, Eq, Hash, PartialEq, PartialOrd, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtUsualFirstName(String);

#[derive(Debug, Eq, Hash, PartialEq, PartialOrd, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtUsualFullName(String);

#[derive(Debug, Eq, Hash, PartialEq, PartialOrd, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtCorrectionPoint(i32);

#[derive(Debug, Eq, Hash, PartialEq, PartialOrd, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtWallet(i32);

#[derive(Debug, Eq, Hash, PartialEq, PartialOrd, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtFirstName(String);

#[derive(Debug, Eq, Hash, PartialEq, PartialOrd, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtDisplayName(String);

#[derive(Debug, Eq, Hash, PartialEq, PartialOrd, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtLastName(String);

#[derive(Debug, Eq, Hash, PartialEq, PartialOrd, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtUserId(i32);

#[derive(Debug, Eq, Hash, PartialEq, PartialOrd, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtLoginId(pub String);

#[derive(Debug, Eq, Hash, PartialEq, PartialOrd, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtPhone(pub String);

impl FtPhone {
    #[must_use]
    pub fn is_hidden(&self) -> bool {
        self.0 == "hidden"
    }
}

#[derive(Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum FtUserIdentifier {
    Login(FtLoginId),
    UserId(FtUserId),
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FtKind {
    Admin,
    Student,
    Staff,
    External,
}
