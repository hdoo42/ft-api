use std::str::FromStr;

use serde::{de::Error, Deserialize, Deserializer, Serialize};

use crate::DateTimeUtc;

#[derive(Debug, Serialize, Deserialize)]
pub struct FtFlag {
    pub id: i8,
    pub name: FtFlagName,
    pub positive: bool,
    pub icon: String,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}

#[derive(PartialEq, Serialize, Debug)]
pub enum FtFlagName {
    Ok,
    EmptyWork,
    IncompleteWork,
    InvalidCompilation,
    Norme,
    Cheat,
    Crash,
    OutstandingProject,
    ConcerningSituation,
    Leaks,
    ForbiddenFunction,
    CantSupportExplainCode,
}

impl FromStr for FtFlagName {
    type Err = serde_json::error::Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "Ok" => Ok(FtFlagName::Ok),
            "Empty work" => Ok(FtFlagName::EmptyWork),
            "Incomplete work" => Ok(FtFlagName::IncompleteWork),
            "Invalid compilation" => Ok(FtFlagName::InvalidCompilation),
            "Norme" => Ok(FtFlagName::Norme),
            "Cheat" => Ok(FtFlagName::Cheat),
            "Crash" => Ok(FtFlagName::Crash),
            "Outstanding project" => Ok(FtFlagName::OutstandingProject),
            "Concerning situation" => Ok(FtFlagName::ConcerningSituation),
            "Leaks" => Ok(FtFlagName::Leaks),
            "Forbidden Function" => Ok(FtFlagName::ForbiddenFunction),
            "Can’t support / explain code" => Ok(FtFlagName::CantSupportExplainCode),
            _ => Err(serde_json::error::Error::custom("Flag does not exist")),
        }
    }
}

impl<'de> Deserialize<'de> for FtFlagName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FtFlagVisitor;

        impl<'de> serde::de::Visitor<'de> for FtFlagVisitor {
            type Value = FtFlagName;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a string matching a 42 flags")
            }

            fn visit_str<E>(self, value: &str) -> Result<FtFlagName, E>
            where
                E: serde::de::Error,
            {
                FtFlagName::from_str(value).map_err(serde::de::Error::custom)
            }
        }
        deserializer.deserialize_str(FtFlagVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn deserialize_flag() {
        let raw_string = r#"[
        {
          "id": 1,
          "name": "Ok",
          "positive": true,
          "icon": "check-4",
          "created_at": "2015-09-14T23:06:52.000Z",
          "updated_at": "2015-09-14T23:06:52.000Z"
        },
        {
          "id": 2,
          "name": "Empty work",
          "positive": false,
          "icon": "iconf-folder-1",
          "created_at": "2015-09-14T23:06:52.000Z",
          "updated_at": "2015-09-14T23:06:52.000Z"
        },
        {
          "id": 3,
          "name": "Incomplete work",
          "positive": false,
          "icon": "file-attention",
          "created_at": "2015-09-14T23:06:52.000Z",
          "updated_at": "2015-09-14T23:06:52.000Z"
        },
        {
          "id": 5,
          "name": "Invalid compilation",
          "positive": false,
          "icon": "skull-2",
          "created_at": "2015-09-14T23:06:52.000Z",
          "updated_at": "2015-09-14T23:06:52.000Z"
        },
        {
          "id": 6,
          "name": "Norme",
          "positive": false,
          "icon": "receipt-1",
          "created_at": "2015-09-14T23:06:52.000Z",
          "updated_at": "2015-09-14T23:06:52.000Z"
        },
        {
          "id": 7,
          "name": "Cheat",
          "positive": false,
          "icon": "layers",
          "created_at": "2015-09-14T23:06:52.000Z",
          "updated_at": "2015-09-14T23:06:52.000Z"
        },
        {
          "id": 8,
          "name": "Crash",
          "positive": false,
          "icon": "bomb",
          "created_at": "2015-09-14T23:06:52.000Z",
          "updated_at": "2015-09-14T23:06:52.000Z"
        },
        {
          "id": 9,
          "name": "Outstanding project",
          "positive": true,
          "icon": "star-1",
          "created_at": "2017-05-18T14:07:37.380Z",
          "updated_at": "2017-05-18T14:12:07.415Z"
        },
        {
          "id": 11,
          "name": "Concerning situation",
          "positive": false,
          "icon": "alert-2",
          "created_at": "2017-11-03T12:27:44.876Z",
          "updated_at": "2017-11-03T12:27:44.876Z"
        },
        {
          "id": 12,
          "name": "Leaks",
          "positive": false,
          "icon": "blood",
          "created_at": "2018-02-09T15:50:28.558Z",
          "updated_at": "2018-02-09T15:50:28.558Z"
        },
        {
          "id": 13,
          "name": "Forbidden Function",
          "positive": false,
          "icon": "delete-2",
          "created_at": "2018-05-15T12:44:59.600Z",
          "updated_at": "2018-05-15T12:44:59.600Z"
        },
        {
          "id": 14,
          "name": "Can’t support / explain code",
          "positive": false,
          "icon": "bubble-attention-4",
          "created_at": "2023-06-15T13:50:25.655Z",
          "updated_at": "2023-06-15T13:50:25.655Z"
        }
      ] "#;
        let mut flags: Vec<FtFlag> = serde_json::from_str(raw_string).unwrap();

        assert_eq!(
            flags.pop().unwrap().name,
            FtFlagName::CantSupportExplainCode
        );
        assert_eq!(flags.pop().unwrap().name, FtFlagName::ForbiddenFunction);
        assert_eq!(flags.pop().unwrap().name, FtFlagName::Leaks);
        assert_eq!(flags.pop().unwrap().name, FtFlagName::ConcerningSituation);
        assert_eq!(flags.pop().unwrap().name, FtFlagName::OutstandingProject);
        assert_eq!(flags.pop().unwrap().name, FtFlagName::Crash);
        assert_eq!(flags.pop().unwrap().name, FtFlagName::Cheat);
        assert_eq!(flags.pop().unwrap().name, FtFlagName::Norme);
        assert_eq!(flags.pop().unwrap().name, FtFlagName::InvalidCompilation);
        assert_eq!(flags.pop().unwrap().name, FtFlagName::IncompleteWork);
        assert_eq!(flags.pop().unwrap().name, FtFlagName::EmptyWork);
        assert_eq!(flags.pop().unwrap().name, FtFlagName::Ok);
    }
}
