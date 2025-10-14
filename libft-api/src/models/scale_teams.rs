use crate::models::prelude::*;
use rvstruct::ValueStruct;
use serde::{Deserialize, Deserializer, Serialize};

#[derive(Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct FtScaleTeam {
    pub id: FtScaleTeamId,
    pub scale_id: FtScaleId,
    pub comment: Option<FtScaleTeamComment>,
    pub created_at: FtDateTimeUtc,
    pub updated_at: FtDateTimeUtc,
    pub final_mark: Option<FtFinalMark>,
    pub feedback: Option<FtScaleTeamFeedback>,
    pub flag: Option<FtScaleFlag>,
    pub begin_at: Option<FtDateTimeUtc>,
    pub corrector: FtCorrector,
    pub correcteds: FtCorrecteds,
    pub filled_at: Option<FtDateTimeUtc>,
    #[serde(deserialize_with = "deserialize_truant")]
    pub truant: Option<FtUser>,
    pub scale: Option<FtScale>,
    pub team: Option<FtTeam>,
    pub feedbacks: Option<Vec<FtFeedback>>,
}

fn deserialize_truant<'de, D>(deserializer: D) -> Result<Option<FtUser>, D::Error>
where
    D: Deserializer<'de>,
{
    let value: FtUser = Deserialize::deserialize(deserializer)?;
    if value == FtUser::new() {
        Ok(None)
    } else {
        Ok(Some(value))
    }
}

#[derive(Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FtCorrector {
    User(Box<FtUser>),
    String(String),
}

#[derive(Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FtCorrecteds {
    String(String),
    Vec(Vec<FtUser>),
}

#[derive(
    Debug, Eq, Hash, PartialEq, PartialOrd, Ord, Clone, Serialize, Deserialize, ValueStruct,
)]
pub struct FtFinalMark(i32);

#[derive(Debug, Eq, Hash, PartialEq, PartialOrd, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtScaleTeamId(i32);

#[derive(Debug, Eq, Hash, PartialEq, PartialOrd, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtScaleTeamComment(String);

#[derive(Debug, Eq, Hash, PartialEq, PartialOrd, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtScaleTeamFeedback(String);

#[test]
fn deserialize_scaleteam() {
    let raw_scaleteam = r#"
     {
    "id": 6657485,
    "scale_id": 31388,
    "comment": null,
    "created_at": "2024-05-04T04:39:27.121Z",
    "updated_at": "2024-05-04T04:39:27.121Z",
    "feedback": null,
    "final_mark": null,
    "flag": {
      "id": 1,
      "name": "Ok",
      "positive": true,
      "icon": "check-4",
      "created_at": "2015-09-14T23:06:52.000Z",
      "updated_at": "2015-09-14T23:06:52.000Z"
    },
    "begin_at": "2024-05-04T05:15:00.000Z",
    "correcteds": [
      {
        "id": 185439,
        "login": "osuliman",
        "url": "https://api.intra.42.fr/v2/users/osuliman"
      }
    ],
    "corrector": {
      "id": 184937,
      "login": "meid",
      "url": "https://api.intra.42.fr/v2/users/meid"
    },
    "truant": {},
    "filled_at": null,
    "questions_with_answers": [],
    "scale": {
      "id": 31388,
      "evaluation_id": 1502,
      "name": "scale 3",
      "is_primary": true,
      "comment": "",
      "introduction_md": "Please respect the following rules:\n\n- Remain polite, courteous, respectful and constructive\n  throughout the evaluation process. The well-being of the community\n  depends on it.\n\n- Identify with the person (or the group) evaluated the eventual\n  dysfunctions of the work. Take the time to discuss\n  and debate the problems you have identified.\n\n- You must consider that there might be some difference in how your\n  peers might have understood the project's instructions and the\n  scope of its functionalities. Always keep an open mind and grade\n  him/her as honestly as possible. The pedagogy is valid only and\n  only if peer-evaluation is conducted seriously.\n",
      "disclaimer_md": "",
      "guidelines_md": "- Only grade the work that is in the student or group's\n  GiT repository.\n\n- Double-check that the GiT repository belongs to the student\n  or the group. Ensure that the work is for the relevant project\n  and also check that \"git clone\" is used in an empty folder.\n\n- Check carefully that no malicious aliases was used to fool you\n  and make you evaluate something other than the content of the\n  official repository.\n\n- To avoid any surprises, carefully check that both the evaluating\n  and the evaluated students have reviewed the possible scripts used\n  to facilitate the grading.\n\n- If the evaluating student has not completed that particular\n  project yet, it is mandatory for this student to read the\n  entire subject prior to starting the defence.\n\n- Use the flags available on this scale to signal an empty repository,\n  non-functioning program, a norm error, cheating etc. In these cases,\n  the grading is over and the final grade is 0 (or -42 in case of\n  cheating). However, with the exception of cheating, you are\n  encouraged to continue to discuss your work (even if you have not\n  finished it) in order to identify any issues that may have caused\n  this failure and avoid repeating the same mistake in the future.\n\n- Remember that for the duration of the defence, no segfault,\n  no other unexpected, premature, uncontrolled or unexpected\n  termination of the program, else the final grade is 0. Use the\n  appropriate flag. \n  You should never have to edit any file except the configuration file if it exists.\n  If you want to edit a file, take the time to explicit the reasons with the \n  evaluated student and make sure both of you are okay with this.\n\n- Check that there are only the requested files available in the git repository. \n  If not, the evaluation stop here.\n",
      "created_at": "2024-01-11T16:25:33.451Z",
      "correction_number": 2,
      "duration": 900,
      "manual_subscription": true,
      "languages": [
        {
          "id": 2,
          "name": "English",
          "identifier": "en",
          "created_at": "2015-04-14T16:07:38.122Z",
          "updated_at": "2024-05-03T15:05:19.408Z"
        },
        {
          "id": 1,
          "name": "Français",
          "identifier": "fr",
          "created_at": "2014-11-02T16:43:38.466Z",
          "updated_at": "2024-05-03T15:05:21.506Z"
        },
        {
          "id": 17,
          "name": "Brazilian Portuguese",
          "identifier": "pt_br",
          "created_at": "2020-12-10T14:15:00.994Z",
          "updated_at": "2024-05-03T12:42:39.741Z"
        },
        {
          "id": 11,
          "name": "Spanish",
          "identifier": "es",
          "created_at": "2019-08-09T15:14:32.544Z",
          "updated_at": "2024-05-03T12:42:34.775Z"
        },
        {
          "id": 15,
          "name": "Armenian",
          "identifier": "hy",
          "created_at": "2020-03-12T09:15:12.038Z",
          "updated_at": "2024-05-02T14:08:31.457Z"
        },
        {
          "id": 13,
          "name": "Japanese",
          "identifier": "ja",
          "created_at": "2019-11-15T13:34:10.581Z",
          "updated_at": "2024-04-30T04:58:01.829Z"
        },
        {
          "id": 18,
          "name": "Turkish",
          "identifier": "tr",
          "created_at": "2021-08-20T10:50:01.782Z",
          "updated_at": "2024-05-02T14:14:49.463Z"
        }
      ],
      "flags": [
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
      ],
      "free": false
    },
    "team": {
      "id": 5667326,
      "name": "osuliman's group",
      "url": "https://api.intra.42.fr/v2/teams/5667326",
      "final_mark": null,
      "project_id": 1259,
      "created_at": "2024-05-04T02:45:52.148Z",
      "updated_at": "2024-05-04T05:21:33.120Z",
      "status": "waiting_for_correction",
      "terminating_at": "2024-05-05T04:09:37.752Z",
      "users": [
        {
          "id": 185439,
          "login": "osuliman",
          "url": "https://api.intra.42.fr/v2/users/osuliman",
          "leader": true,
          "occurrence": 0,
          "validated": true,
          "projects_user_id": 3662215
        }
      ],
      "locked?": true,
      "validated?": null,
      "closed?": true,
      "repo_url": "git@vogsphere.42abudhabi.ae:vogsphere/intra-uuid-34b3f4ee-8013-4523-8a62-1cca1617cfc8-5667326-osuliman",
      "repo_uuid": "intra-uuid-34b3f4ee-8013-4523-8a62-1cca1617cfc8-5667326-osuliman",
      "locked_at": "2024-05-04T02:45:52.194Z",
      "closed_at": "2024-05-04T04:09:37.752Z",
      "project_session_id": 3025,
      "project_gitlab_path": "pedago_world/c-piscine/c-02"
    },
    "feedbacks": []
  }
    "#;
    let res: Result<FtScaleTeam, serde_json::Error> = serde_json::from_str(raw_scaleteam);
    assert!(res.is_ok(), "{:?}", res);
}
