//! Data structures for 42 API user-related entities.
//!
//! This module contains data structures that represent user information
//! from the 42 Intra API, including user profiles and related identifiers.

use crate::models::prelude::*;
use rsb_derive::Builder;
use rvstruct::ValueStruct;
use serde::{Deserialize, Serialize};

/// Represents a user from the 42 Intra API.
/// 
/// Contains comprehensive information about a 42 school user including personal details,
/// academic information, achievements, and more.
#[derive(Debug, PartialEq, PartialOrd, Serialize, Deserialize, Builder)]
pub struct FtUser {
    pub achievements: Option<Vec<FtAchievement>>,
    #[serde(rename = "active?")]
    pub active: Option<bool>,
    #[serde(rename = "alumni?")]
    pub alumni: Option<bool>,
    pub alumnized_at: Option<FtDateTimeFixedOffset>,
    pub anonymize_date: Option<FtDateTimeFixedOffset>,
    pub campus_users: Option<Vec<FtCampusUser>>,
    pub campus: Option<Vec<FtCampus>>,
    pub correction_point: Option<FtCorrectionPoint>,
    pub created_at: Option<FtDateTimeUtc>,
    pub cursus_users: Option<Vec<FtCursusUser>>,
    pub data_erasure_date: Option<FtDateTimeUtc>,
    pub displayname: Option<FtDisplayName>,
    pub email: Option<FtEmail>,
    pub first_name: Option<FtFirstName>,
    pub id: Option<FtUserId>,
    pub image: Option<FtImage>,
    pub kind: Option<FtKind>,
    pub languages_users: Option<Vec<FtLanguagesUser>>,
    pub last_name: Option<FtLastName>,
    pub location: Option<FtHost>,
    pub login: Option<FtLoginId>,
    pub phone: Option<FtPhone>,
    pub pool_month: Option<FtPoolMonth>,
    pub pool_year: Option<FtPoolYear>,
    pub projects_users: Option<Vec<FtProjectsUser>>,
    pub roles: Option<Vec<FtRole>>,
    #[serde(rename = "staff?")]
    pub staff: Option<bool>,
    pub titles_users: Option<Vec<FtTitleUser>>,
    pub titles: Option<Vec<FtTitle>>,
    pub updated_at: Option<FtDateTimeUtc>,
    pub url: Option<FtUrl>,
    pub usual_first_name: Option<FtUsualFirstName>,
    pub usual_full_name: Option<FtUsualFullName>,
    pub wallet: Option<FtWallet>,
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

#[derive(
    Debug, Eq, Hash, PartialEq, PartialOrd, Copy, Clone, Serialize, Deserialize, ValueStruct,
)]
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
