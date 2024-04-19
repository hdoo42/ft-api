use chrono::{DateTime, Utc};
use rvstruct::ValueStruct;
use serde::{Deserialize, Serialize};

pub use locations::*;
mod locations;

#[derive(Serialize, Deserialize, Debug)]
pub struct DateTimeSerde {
    #[serde(with = "chrono::serde::ts_seconds")]
    datetime: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FtUser {
    id: FtUserId,
    email: FtEmail,
    login: FtLoginId,
    first_name: FtFirstName,
    last_name: FtLastName,
    url: FtUrl,
    phone: FtPhone,
    displayname: FtDisplayName,
    kind: FtKind,
}

#[derive(Debug, Eq, Hash, PartialEq, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtEmail(String);

#[derive(Debug, Eq, Hash, PartialEq, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtFirstName(String);

#[derive(Debug, Eq, Hash, PartialEq, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtDisplayName(String);

#[derive(Debug, Eq, Hash, PartialEq, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtLastName(String);

#[derive(Debug, Eq, Hash, PartialEq, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtUserId(i32);

#[derive(Debug, Eq, Hash, PartialEq, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtLoginId(String);

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
}