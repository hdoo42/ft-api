use chrono::{DateTime, Utc};
use rvstruct::ValueStruct;
use serde::{Deserialize, Serialize};

pub use locations::*;
mod locations;
pub use scale_teams::*;
mod scale_teams;
pub use flag::*;
mod flag;
pub use project_session::*;
mod project_session;
pub use scale::*;
mod scale;
pub use feedback::*;
mod feedback;
pub use team::*;
mod team;
pub use language::*;
mod language;
pub use image::*;
mod image;

#[derive(Serialize, Deserialize, Debug)]
pub struct DateTimeSerde(DateTime<Utc>);

#[derive(Debug, Serialize, Deserialize)]
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
    pub alumnized_at: Option<DateTimeSerde>,
    pub anonymize_date: Option<DateTimeSerde>,
    pub correction_point: Option<FtCorrectionPoint>,
    pub created_at: Option<DateTimeSerde>,
    pub data_erasure_date: Option<DateTimeSerde>,
    pub image: Option<FtImage>,
    pub location: Option<FtHost>,
    pub pool_month: Option<DateTimeSerde>,
    pub pool_year: Option<DateTimeSerde>,
    pub staff: Option<bool>,
    pub updated_at: Option<DateTimeSerde>,
    pub usual_first_name: Option<FtUsualFirstName>,
    pub usual_full_name: Option<FtUsualFullName>,
    pub wallet: Option<FtWallet>,
}

#[derive(Debug, Eq, Hash, PartialEq, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtEmail(String);

#[derive(Debug, Eq, Hash, PartialEq, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtUsualFirstName(String);

#[derive(Debug, Eq, Hash, PartialEq, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtUsualFullName(String);

#[derive(Debug, Eq, Hash, PartialEq, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtCorrectionPoint(i32);

#[derive(Debug, Eq, Hash, PartialEq, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtWallet(i32);

#[derive(Debug, Eq, Hash, PartialEq, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtFirstName(String);

#[derive(Debug, Eq, Hash, PartialEq, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtDisplayName(String);

#[derive(Debug, Eq, Hash, PartialEq, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtLastName(String);

#[derive(Debug, Eq, Hash, PartialEq, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtUserId(i32);

#[derive(Debug, Eq, Hash, PartialEq, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtLoginId(pub String);

#[derive(Debug, Eq, Hash, PartialEq, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtCampusId(i32);

#[derive(Debug, Eq, Hash, PartialEq, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtUrl(String);

#[derive(Debug, Eq, Hash, PartialEq, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtPhone(pub String);

impl FtPhone {
    pub fn is_hidden(&self) -> bool {
        self.0 == "hidden"
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FtKind {
    Student,
    Staff,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::from_str;

    #[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
    struct FtTestUser {
        user: FtLoginId,
    }

    #[test]
    fn test_loginid() {
        let json_user = r#"{ "user": "hdoo"}"#;
        let expected_user = FtTestUser {
            user: FtLoginId("hdoo".to_string()),
        };
        let deserialize_login: FtTestUser = from_str(json_user).unwrap();
        assert_eq!(deserialize_login, expected_user);
    }

    #[test]
    fn partial_user() {
        let raw_partial_user = r#"
      {
        "id": 183812,
        "login": "nkanaan",
        "url": "https://api.intra.42.fr/v2/users/nkanaan"
      }
        "#;

        let res: Result<FtUser, serde_json::Error> = serde_json::from_str(raw_partial_user);
        assert!(res.is_ok(), "{:?}", res);
    }
}
