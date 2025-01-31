use chrono::{DateTime, FixedOffset, Utc};
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
pub use project::*;
mod project;
pub use project_data::*;
mod project_data;
pub use projects_users::*;
mod projects_users;
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
pub use user::*;
mod user;
pub use campus::*;
mod campus;
pub use correction_point_history::*;
mod correction_point_history;
mod cursus_user;
pub use cursus_user::*;
mod campus_user;
pub use campus_user::*;
mod journals;
pub use journals::*;
mod group;
pub use group::*;
mod exam;
pub use exam::*;

mod common;

#[derive(Serialize, Deserialize, Debug, PartialEq, ValueStruct)]
pub struct FtDateTimeUtc(pub DateTime<Utc>);

#[derive(Serialize, PartialEq, Deserialize, Debug, ValueStruct)]
pub struct FtDateTimeFixedOffset(DateTime<FixedOffset>);

pub type Seresult<T> = Result<T, serde_json::Error>;

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
