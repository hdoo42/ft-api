use chrono::{DateTime, FixedOffset, Utc};
use rvstruct::ValueStruct;
use serde::{Deserialize, Serialize};

#[derive(Serialize, PartialEq, PartialOrd, Deserialize, Debug, ValueStruct)]
pub struct FtDateTimeUtc(pub DateTime<Utc>);

#[derive(Serialize, PartialEq, PartialOrd, Deserialize, Debug, ValueStruct)]
pub struct FtDateTimeFixedOffset(DateTime<FixedOffset>);

pub type Seresult<T> = Result<T, serde_json::Error>;

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};
    use serde_json::from_str;

    use crate::models::prelude::*;
    #[derive(Debug, PartialEq, PartialOrd, Clone, Serialize, Deserialize)]
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
