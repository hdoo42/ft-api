use rvstruct::ValueStruct;
use serde::{Deserialize, Serialize};

use crate::{FtFinalMark, FtProjectId, FtProjectSessionId, FtScaleTeam, FtUrl, FtUser};

use super::FtDateTimeUtc;

#[derive(Debug, Serialize, Deserialize)]
pub struct FtTeam {
    pub id: FtTeamId,
    pub final_mark: Option<FtFinalMark>,
    pub closed: Option<bool>,
    pub closed_at: FtDateTimeUtc,
    pub created_at: FtDateTimeUtc,
    pub locked: Option<bool>,
    pub locked_at: FtDateTimeUtc,
    pub name: FtName,
    pub project_gitlab_path: FtProjectGitlabPath,
    pub project_id: FtProjectId,
    pub project_session_id: FtProjectSessionId,
    pub repo_url: FtUrl,
    pub repo_uuid: FtRepoUuid,
    pub scale_teams: Option<Vec<FtScaleTeam>>,
    pub status: FtStatus,
    pub teams_uploads: Option<Vec<FtTeamUpload>>,
    pub terminating_at: FtDateTimeUtc,
    pub updated_at: FtDateTimeUtc,
    pub url: FtUrl,
    pub users: Vec<FtUser>,
    pub validated: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FtTeamUpload {
    pub id: FtTeamId,
    pub final_mark: FtFinalMark,
    pub comment: String,
    pub created_at: FtDateTimeUtc,
    pub upload_id: FtUploadId,
}

#[derive(Debug, Eq, Hash, PartialEq, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtTeamId(pub i32);

#[derive(Debug, Eq, Hash, PartialEq, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtName(pub String);

#[derive(Debug, Eq, Hash, PartialEq, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtProjectGitlabPath(pub String);

#[derive(Debug, Eq, Hash, PartialEq, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtRepoUuid(pub String);

#[derive(Debug, Eq, Hash, PartialEq, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtStatus(pub String);

#[derive(Debug, Eq, Hash, PartialEq, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtUploadId(i32);

#[test]
fn test_ft_team_deserialization() {
    // Sample JSON data representing an FtTeam instance
    let data = r#"
[
  {
    "id": 5760239,
    "name": "nahyukim's group",
    "url": "https://api.intra.42.fr/v2/teams/5760239",
    "final_mark": 50,
    "project_id": 1255,
    "created_at": "2024-06-27T01:21:15.988Z",
    "updated_at": "2024-07-04T01:13:37.177Z",
    "status": "finished",
    "terminating_at": "2024-07-04T12:25:57.023Z",
    "users": [
      {
        "id": 190824,
        "login": "nahyukim",
        "url": "https://api.intra.42.fr/v2/users/nahyukim",
        "leader": true,
        "occurrence": 0,
        "validated": true,
        "projects_user_id": 3730632
      }
    ],
    "locked?": true,
    "validated?": true,
    "closed?": true,
    "repo_url": "git@vogsphere.42gyeongsan.kr:vogsphere/intra-uuid-02e77d04-fa19-4811-a296-1644b1112720-5760239-nahyukim",
    "repo_uuid": "intra-uuid-02e77d04-fa19-4811-a296-1644b1112720-5760239-nahyukim",
    "locked_at": "2024-06-27T01:21:16.016Z",
    "closed_at": "2024-07-03T12:25:57.023Z",
    "project_session_id": 11193,
    "project_gitlab_path": "pedago_world/c-piscine/shell-00",
    "scale_teams": [
      {
        "id": 6831696,
        "scale_id": 33823,
        "comment": "C 과제를 먼저 진행하신 후, Shell 00에 도전하셔서 필수 구현 항목에 중점을 둬 모두 잘 구현하신 것이 드러나 무난한 통과가 예상됩니다. 혹여 시간이 남으시면 말씀드린 것처럼 Exercise 08도 꼭 한 번 도전해보시길 바라겠습니다. 화이팅입니다~~!!",
        "created_at": "2024-07-03T14:13:03.716Z",
        "updated_at": "2024-07-03T15:24:19.630Z",
        "feedback": "편안한 분위기에서 평가가 이루어질 수 있도록 진행해주셔서 한층 긴장을 놓고 평가 받을 수 있었습니다. 어떤 문제가 앞으로 도움이 될지에 관한 얘기들도 해주셔서 정말 감사드립니다. 꼭 한 번 도전해보겠습니다! 수고 많으셨습니다 :)",
        "final_mark": 50,
        "flag": {
          "id": 1,
          "name": "Ok",
          "positive": true,
          "icon": "check-4",
          "created_at": "2015-09-14T23:06:52.000Z",
          "updated_at": "2015-09-14T23:06:52.000Z"
        },
        "begin_at": "2024-07-03T15:15:00.000Z",
        "correcteds": [
          {
            "id": 190824,
            "login": "nahyukim",
            "url": "https://api.intra.42.fr/v2/users/nahyukim"
          }
        ],
        "corrector": {
          "id": 190913,
          "login": "hyeonwki",
          "url": "https://api.intra.42.fr/v2/users/hyeonwki"
        },
        "truant": {},
        "filled_at": "2024-07-03T15:22:34.062Z",
        "questions_with_answers": []
      },
      {
        "id": 6831081,
        "scale_id": 33823,
        "comment": "명령어의 옵션에 대한 개념과 이해를 잘 숙지하고 계셨습니다. 또한 장난도 잘 받아주시며 평가 분위기를 밝게 만들어주셔서 저도 편안하게 리뷰를 할 수 있었습니다. 나중에 혹시 뒤의 exercise를 풀 마음이 생기시면 충분히 빠른 시간으로 해결할 수 있을 것 같습니다.\r\n감사합니다 !",
        "created_at": "2024-07-03T13:24:24.043Z",
        "updated_at": "2024-07-03T14:10:25.840Z",
        "feedback": "평가 분위기를 편안하게 만들어주셔서 즐겁게 평가 받을 수 있었습니다. 하나하나 친절하게 봐주셔서 감사했습니다. 수고 많으셨습니다 :)",
        "final_mark": 50,
        "flag": {
          "id": 1,
          "name": "Ok",
          "positive": true,
          "icon": "check-4",
          "created_at": "2015-09-14T23:06:52.000Z",
          "updated_at": "2015-09-14T23:06:52.000Z"
        },
        "begin_at": "2024-07-03T14:00:00.000Z",
        "correcteds": [
          {
            "id": 190824,
            "login": "nahyukim",
            "url": "https://api.intra.42.fr/v2/users/nahyukim"
          }
        ],
        "corrector": {
          "id": 190816,
          "login": "jkil",
          "url": "https://api.intra.42.fr/v2/users/jkil"
        },
        "truant": {},
        "filled_at": "2024-07-03T14:08:34.228Z",
        "questions_with_answers": []
      }
    ],
    "teams_uploads": [
      {
        "id": 2202804,
        "final_mark": 50,
        "comment": "ex00: OK | ex01: OK | ex02: OK | ex03: OK | ex04: OK | ex05: OK | ex06: Nothing turned in | ex07: Nothing turned in | ex08: Nothing turned in | ex09: Nothing turned in",
        "created_at": "2024-07-04T01:13:37.175Z",
        "upload_id": 1481
      }
    ]
  },
  {
    "id": 5760239,
    "name": "nahyukim's group",
    "url": "https://api.intra.42.fr/v2/teams/5760239",
    "final_mark": 50,
    "project_id": 1255,
    "created_at": "2024-06-27T01:21:15.988Z",
    "updated_at": "2024-07-04T01:13:37.177Z",
    "status": "finished",
    "terminating_at": "2024-07-04T12:25:57.023Z",
    "users": [
      {
        "id": 190824,
        "login": "nahyukim",
        "url": "https://api.intra.42.fr/v2/users/nahyukim",
        "leader": true,
        "occurrence": 0,
        "validated": true,
        "projects_user_id": 3730632
      }
    ],
    "locked?": true,
    "validated?": true,
    "closed?": true,
    "repo_url": "git@vogsphere.42gyeongsan.kr:vogsphere/intra-uuid-02e77d04-fa19-4811-a296-1644b1112720-5760239-nahyukim",
    "repo_uuid": "intra-uuid-02e77d04-fa19-4811-a296-1644b1112720-5760239-nahyukim",
    "locked_at": "2024-06-27T01:21:16.016Z",
    "closed_at": "2024-07-03T12:25:57.023Z",
    "project_session_id": 11193,
    "project_gitlab_path": "pedago_world/c-piscine/shell-00",
    "scale_teams": [
      {
        "id": 6831696,
        "scale_id": 33823,
        "comment": "C 과제를 먼저 진행하신 후, Shell 00에 도전하셔서 필수 구현 항목에 중점을 둬 모두 잘 구현하신 것이 드러나 무난한 통과가 예상됩니다. 혹여 시간이 남으시면 말씀드린 것처럼 Exercise 08도 꼭 한 번 도전해보시길 바라겠습니다. 화이팅입니다~~!!",
        "created_at": "2024-07-03T14:13:03.716Z",
        "updated_at": "2024-07-03T15:24:19.630Z",
        "feedback": "편안한 분위기에서 평가가 이루어질 수 있도록 진행해주셔서 한층 긴장을 놓고 평가 받을 수 있었습니다. 어떤 문제가 앞으로 도움이 될지에 관한 얘기들도 해주셔서 정말 감사드립니다. 꼭 한 번 도전해보겠습니다! 수고 많으셨습니다 :)",
        "final_mark": 50,
        "flag": {
          "id": 1,
          "name": "Ok",
          "positive": true,
          "icon": "check-4",
          "created_at": "2015-09-14T23:06:52.000Z",
          "updated_at": "2015-09-14T23:06:52.000Z"
        },
        "begin_at": "2024-07-03T15:15:00.000Z",
        "correcteds": [
          {
            "id": 190824,
            "login": "nahyukim",
            "url": "https://api.intra.42.fr/v2/users/nahyukim"
          }
        ],
        "corrector": {
          "id": 190913,
          "login": "hyeonwki",
          "url": "https://api.intra.42.fr/v2/users/hyeonwki"
        },
        "truant": {},
        "filled_at": "2024-07-03T15:22:34.062Z",
        "questions_with_answers": []
      },
      {
        "id": 6831081,
        "scale_id": 33823,
        "comment": "명령어의 옵션에 대한 개념과 이해를 잘 숙지하고 계셨습니다. 또한 장난도 잘 받아주시며 평가 분위기를 밝게 만들어주셔서 저도 편안하게 리뷰를 할 수 있었습니다. 나중에 혹시 뒤의 exercise를 풀 마음이 생기시면 충분히 빠른 시간으로 해결할 수 있을 것 같습니다.\r\n감사합니다 !",
        "created_at": "2024-07-03T13:24:24.043Z",
        "updated_at": "2024-07-03T14:10:25.840Z",
        "feedback": "평가 분위기를 편안하게 만들어주셔서 즐겁게 평가 받을 수 있었습니다. 하나하나 친절하게 봐주셔서 감사했습니다. 수고 많으셨습니다 :)",
        "final_mark": 50,
        "flag": {
          "id": 1,
          "name": "Ok",
          "positive": true,
          "icon": "check-4",
          "created_at": "2015-09-14T23:06:52.000Z",
          "updated_at": "2015-09-14T23:06:52.000Z"
        },
        "begin_at": "2024-07-03T14:00:00.000Z",
        "correcteds": [
          {
            "id": 190824,
            "login": "nahyukim",
            "url": "https://api.intra.42.fr/v2/users/nahyukim"
          }
        ],
        "corrector": {
          "id": 190816,
          "login": "jkil",
          "url": "https://api.intra.42.fr/v2/users/jkil"
        },
        "truant": {},
        "filled_at": "2024-07-03T14:08:34.228Z",
        "questions_with_answers": []
      }
    ],
    "teams_uploads": [
      {
        "id": 2202804,
        "final_mark": 50,
        "comment": "ex00: OK | ex01: OK | ex02: OK | ex03: OK | ex04: OK | ex05: OK | ex06: Nothing turned in | ex07: Nothing turned in | ex08: Nothing turned in | ex09: Nothing turned in",
        "created_at": "2024-07-04T01:13:37.175Z",
        "upload_id": 1481
      }
    ]
  },
  {
    "id": 5760239,
    "name": "nahyukim's group",
    "url": "https://api.intra.42.fr/v2/teams/5760239",
    "final_mark": 50,
    "project_id": 1255,
    "created_at": "2024-06-27T01:21:15.988Z",
    "updated_at": "2024-07-04T01:13:37.177Z",
    "status": "finished",
    "terminating_at": "2024-07-04T12:25:57.023Z",
    "users": [
      {
        "id": 190824,
        "login": "nahyukim",
        "url": "https://api.intra.42.fr/v2/users/nahyukim",
        "leader": true,
        "occurrence": 0,
        "validated": true,
        "projects_user_id": 3730632
      }
    ],
    "locked?": true,
    "validated?": true,
    "closed?": true,
    "repo_url": "git@vogsphere.42gyeongsan.kr:vogsphere/intra-uuid-02e77d04-fa19-4811-a296-1644b1112720-5760239-nahyukim",
    "repo_uuid": "intra-uuid-02e77d04-fa19-4811-a296-1644b1112720-5760239-nahyukim",
    "locked_at": "2024-06-27T01:21:16.016Z",
    "closed_at": "2024-07-03T12:25:57.023Z",
    "project_session_id": 11193,
    "project_gitlab_path": "pedago_world/c-piscine/shell-00",
    "scale_teams": [
      {
        "id": 6831696,
        "scale_id": 33823,
        "comment": "C 과제를 먼저 진행하신 후, Shell 00에 도전하셔서 필수 구현 항목에 중점을 둬 모두 잘 구현하신 것이 드러나 무난한 통과가 예상됩니다. 혹여 시간이 남으시면 말씀드린 것처럼 Exercise 08도 꼭 한 번 도전해보시길 바라겠습니다. 화이팅입니다~~!!",
        "created_at": "2024-07-03T14:13:03.716Z",
        "updated_at": "2024-07-03T15:24:19.630Z",
        "feedback": "편안한 분위기에서 평가가 이루어질 수 있도록 진행해주셔서 한층 긴장을 놓고 평가 받을 수 있었습니다. 어떤 문제가 앞으로 도움이 될지에 관한 얘기들도 해주셔서 정말 감사드립니다. 꼭 한 번 도전해보겠습니다! 수고 많으셨습니다 :)",
        "final_mark": 50,
        "flag": {
          "id": 1,
          "name": "Ok",
          "positive": true,
          "icon": "check-4",
          "created_at": "2015-09-14T23:06:52.000Z",
          "updated_at": "2015-09-14T23:06:52.000Z"
        },
        "begin_at": "2024-07-03T15:15:00.000Z",
        "correcteds": [
          {
            "id": 190824,
            "login": "nahyukim",
            "url": "https://api.intra.42.fr/v2/users/nahyukim"
          }
        ],
        "corrector": {
          "id": 190913,
          "login": "hyeonwki",
          "url": "https://api.intra.42.fr/v2/users/hyeonwki"
        },
        "truant": {},
        "filled_at": "2024-07-03T15:22:34.062Z",
        "questions_with_answers": []
      },
      {
        "id": 6831081,
        "scale_id": 33823,
        "comment": "명령어의 옵션에 대한 개념과 이해를 잘 숙지하고 계셨습니다. 또한 장난도 잘 받아주시며 평가 분위기를 밝게 만들어주셔서 저도 편안하게 리뷰를 할 수 있었습니다. 나중에 혹시 뒤의 exercise를 풀 마음이 생기시면 충분히 빠른 시간으로 해결할 수 있을 것 같습니다.\r\n감사합니다 !",
        "created_at": "2024-07-03T13:24:24.043Z",
        "updated_at": "2024-07-03T14:10:25.840Z",
        "feedback": "평가 분위기를 편안하게 만들어주셔서 즐겁게 평가 받을 수 있었습니다. 하나하나 친절하게 봐주셔서 감사했습니다. 수고 많으셨습니다 :)",
        "final_mark": 50,
        "flag": {
          "id": 1,
          "name": "Ok",
          "positive": true,
          "icon": "check-4",
          "created_at": "2015-09-14T23:06:52.000Z",
          "updated_at": "2015-09-14T23:06:52.000Z"
        },
        "begin_at": "2024-07-03T14:00:00.000Z",
        "correcteds": [
          {
            "id": 190824,
            "login": "nahyukim",
            "url": "https://api.intra.42.fr/v2/users/nahyukim"
          }
        ],
        "corrector": {
          "id": 190816,
          "login": "jkil",
          "url": "https://api.intra.42.fr/v2/users/jkil"
        },
        "truant": {},
        "filled_at": "2024-07-03T14:08:34.228Z",
        "questions_with_answers": []
      }
    ],
    "teams_uploads": [
      {
        "id": 2202804,
        "final_mark": 50,
        "comment": "ex00: OK | ex01: OK | ex02: OK | ex03: OK | ex04: OK | ex05: OK | ex06: Nothing turned in | ex07: Nothing turned in | ex08: Nothing turned in | ex09: Nothing turned in",
        "created_at": "2024-07-04T01:13:37.175Z",
        "upload_id": 1481
      }
    ]
  },
  {
    "id": 5760239,
    "name": "nahyukim's group",
    "url": "https://api.intra.42.fr/v2/teams/5760239",
    "final_mark": 50,
    "project_id": 1255,
    "created_at": "2024-06-27T01:21:15.988Z",
    "updated_at": "2024-07-04T01:13:37.177Z",
    "status": "finished",
    "terminating_at": "2024-07-04T12:25:57.023Z",
    "users": [
      {
        "id": 190824,
        "login": "nahyukim",
        "url": "https://api.intra.42.fr/v2/users/nahyukim",
        "leader": true,
        "occurrence": 0,
        "validated": true,
        "projects_user_id": 3730632
      }
    ],
    "locked?": true,
    "validated?": true,
    "closed?": true,
    "repo_url": "git@vogsphere.42gyeongsan.kr:vogsphere/intra-uuid-02e77d04-fa19-4811-a296-1644b1112720-5760239-nahyukim",
    "repo_uuid": "intra-uuid-02e77d04-fa19-4811-a296-1644b1112720-5760239-nahyukim",
    "locked_at": "2024-06-27T01:21:16.016Z",
    "closed_at": "2024-07-03T12:25:57.023Z",
    "project_session_id": 11193,
    "project_gitlab_path": "pedago_world/c-piscine/shell-00",
    "scale_teams": [
      {
        "id": 6831696,
        "scale_id": 33823,
        "comment": "C 과제를 먼저 진행하신 후, Shell 00에 도전하셔서 필수 구현 항목에 중점을 둬 모두 잘 구현하신 것이 드러나 무난한 통과가 예상됩니다. 혹여 시간이 남으시면 말씀드린 것처럼 Exercise 08도 꼭 한 번 도전해보시길 바라겠습니다. 화이팅입니다~~!!",
        "created_at": "2024-07-03T14:13:03.716Z",
        "updated_at": "2024-07-03T15:24:19.630Z",
        "feedback": "편안한 분위기에서 평가가 이루어질 수 있도록 진행해주셔서 한층 긴장을 놓고 평가 받을 수 있었습니다. 어떤 문제가 앞으로 도움이 될지에 관한 얘기들도 해주셔서 정말 감사드립니다. 꼭 한 번 도전해보겠습니다! 수고 많으셨습니다 :)",
        "final_mark": 50,
        "flag": {
          "id": 1,
          "name": "Ok",
          "positive": true,
          "icon": "check-4",
          "created_at": "2015-09-14T23:06:52.000Z",
          "updated_at": "2015-09-14T23:06:52.000Z"
        },
        "begin_at": "2024-07-03T15:15:00.000Z",
        "correcteds": [
          {
            "id": 190824,
            "login": "nahyukim",
            "url": "https://api.intra.42.fr/v2/users/nahyukim"
          }
        ],
        "corrector": {
          "id": 190913,
          "login": "hyeonwki",
          "url": "https://api.intra.42.fr/v2/users/hyeonwki"
        },
        "truant": {},
        "filled_at": "2024-07-03T15:22:34.062Z",
        "questions_with_answers": []
      },
      {
        "id": 6831081,
        "scale_id": 33823,
        "comment": "명령어의 옵션에 대한 개념과 이해를 잘 숙지하고 계셨습니다. 또한 장난도 잘 받아주시며 평가 분위기를 밝게 만들어주셔서 저도 편안하게 리뷰를 할 수 있었습니다. 나중에 혹시 뒤의 exercise를 풀 마음이 생기시면 충분히 빠른 시간으로 해결할 수 있을 것 같습니다.\r\n감사합니다 !",
        "created_at": "2024-07-03T13:24:24.043Z",
        "updated_at": "2024-07-03T14:10:25.840Z",
        "feedback": "평가 분위기를 편안하게 만들어주셔서 즐겁게 평가 받을 수 있었습니다. 하나하나 친절하게 봐주셔서 감사했습니다. 수고 많으셨습니다 :)",
        "final_mark": 50,
        "flag": {
          "id": 1,
          "name": "Ok",
          "positive": true,
          "icon": "check-4",
          "created_at": "2015-09-14T23:06:52.000Z",
          "updated_at": "2015-09-14T23:06:52.000Z"
        },
        "begin_at": "2024-07-03T14:00:00.000Z",
        "correcteds": [
          {
            "id": 190824,
            "login": "nahyukim",
            "url": "https://api.intra.42.fr/v2/users/nahyukim"
          }
        ],
        "corrector": {
          "id": 190816,
          "login": "jkil",
          "url": "https://api.intra.42.fr/v2/users/jkil"
        },
        "truant": {},
        "filled_at": "2024-07-03T14:08:34.228Z",
        "questions_with_answers": []
      }
    ],
    "teams_uploads": [
      {
        "id": 2202804,
        "final_mark": 50,
        "comment": "ex00: OK | ex01: OK | ex02: OK | ex03: OK | ex04: OK | ex05: OK | ex06: Nothing turned in | ex07: Nothing turned in | ex08: Nothing turned in | ex09: Nothing turned in",
        "created_at": "2024-07-04T01:13:37.175Z",
        "upload_id": 1481
      }
    ]
  },
  {
    "id": 5760239,
    "name": "nahyukim's group",
    "url": "https://api.intra.42.fr/v2/teams/5760239",
    "final_mark": 50,
    "project_id": 1255,
    "created_at": "2024-06-27T01:21:15.988Z",
    "updated_at": "2024-07-04T01:13:37.177Z",
    "status": "finished",
    "terminating_at": "2024-07-04T12:25:57.023Z",
    "users": [
      {
        "id": 190824,
        "login": "nahyukim",
        "url": "https://api.intra.42.fr/v2/users/nahyukim",
        "leader": true,
        "occurrence": 0,
        "validated": true,
        "projects_user_id": 3730632
      }
    ],
    "locked?": true,
    "validated?": true,
    "closed?": true,
    "repo_url": "git@vogsphere.42gyeongsan.kr:vogsphere/intra-uuid-02e77d04-fa19-4811-a296-1644b1112720-5760239-nahyukim",
    "repo_uuid": "intra-uuid-02e77d04-fa19-4811-a296-1644b1112720-5760239-nahyukim",
    "locked_at": "2024-06-27T01:21:16.016Z",
    "closed_at": "2024-07-03T12:25:57.023Z",
    "project_session_id": 11193,
    "project_gitlab_path": "pedago_world/c-piscine/shell-00",
    "scale_teams": [
      {
        "id": 6831696,
        "scale_id": 33823,
        "comment": "C 과제를 먼저 진행하신 후, Shell 00에 도전하셔서 필수 구현 항목에 중점을 둬 모두 잘 구현하신 것이 드러나 무난한 통과가 예상됩니다. 혹여 시간이 남으시면 말씀드린 것처럼 Exercise 08도 꼭 한 번 도전해보시길 바라겠습니다. 화이팅입니다~~!!",
        "created_at": "2024-07-03T14:13:03.716Z",
        "updated_at": "2024-07-03T15:24:19.630Z",
        "feedback": "편안한 분위기에서 평가가 이루어질 수 있도록 진행해주셔서 한층 긴장을 놓고 평가 받을 수 있었습니다. 어떤 문제가 앞으로 도움이 될지에 관한 얘기들도 해주셔서 정말 감사드립니다. 꼭 한 번 도전해보겠습니다! 수고 많으셨습니다 :)",
        "final_mark": 50,
        "flag": {
          "id": 1,
          "name": "Ok",
          "positive": true,
          "icon": "check-4",
          "created_at": "2015-09-14T23:06:52.000Z",
          "updated_at": "2015-09-14T23:06:52.000Z"
        },
        "begin_at": "2024-07-03T15:15:00.000Z",
        "correcteds": [
          {
            "id": 190824,
            "login": "nahyukim",
            "url": "https://api.intra.42.fr/v2/users/nahyukim"
          }
        ],
        "corrector": {
          "id": 190913,
          "login": "hyeonwki",
          "url": "https://api.intra.42.fr/v2/users/hyeonwki"
        },
        "truant": {},
        "filled_at": "2024-07-03T15:22:34.062Z",
        "questions_with_answers": []
      },
      {
        "id": 6831081,
        "scale_id": 33823,
        "comment": "명령어의 옵션에 대한 개념과 이해를 잘 숙지하고 계셨습니다. 또한 장난도 잘 받아주시며 평가 분위기를 밝게 만들어주셔서 저도 편안하게 리뷰를 할 수 있었습니다. 나중에 혹시 뒤의 exercise를 풀 마음이 생기시면 충분히 빠른 시간으로 해결할 수 있을 것 같습니다.\r\n감사합니다 !",
        "created_at": "2024-07-03T13:24:24.043Z",
        "updated_at": "2024-07-03T14:10:25.840Z",
        "feedback": "평가 분위기를 편안하게 만들어주셔서 즐겁게 평가 받을 수 있었습니다. 하나하나 친절하게 봐주셔서 감사했습니다. 수고 많으셨습니다 :)",
        "final_mark": 50,
        "flag": {
          "id": 1,
          "name": "Ok",
          "positive": true,
          "icon": "check-4",
          "created_at": "2015-09-14T23:06:52.000Z",
          "updated_at": "2015-09-14T23:06:52.000Z"
        },
        "begin_at": "2024-07-03T14:00:00.000Z",
        "correcteds": [
          {
            "id": 190824,
            "login": "nahyukim",
            "url": "https://api.intra.42.fr/v2/users/nahyukim"
          }
        ],
        "corrector": {
          "id": 190816,
          "login": "jkil",
          "url": "https://api.intra.42.fr/v2/users/jkil"
        },
        "truant": {},
        "filled_at": "2024-07-03T14:08:34.228Z",
        "questions_with_answers": []
      }
    ],
    "teams_uploads": [
      {
        "id": 2202804,
        "final_mark": 50,
        "comment": "ex00: OK | ex01: OK | ex02: OK | ex03: OK | ex04: OK | ex05: OK | ex06: Nothing turned in | ex07: Nothing turned in | ex08: Nothing turned in | ex09: Nothing turned in",
        "created_at": "2024-07-04T01:13:37.175Z",
        "upload_id": 1481
      }
    ]
  },
  {
    "id": 5760239,
    "name": "nahyukim's group",
    "url": "https://api.intra.42.fr/v2/teams/5760239",
    "final_mark": 50,
    "project_id": 1255,
    "created_at": "2024-06-27T01:21:15.988Z",
    "updated_at": "2024-07-04T01:13:37.177Z",
    "status": "finished",
    "terminating_at": "2024-07-04T12:25:57.023Z",
    "users": [
      {
        "id": 190824,
        "login": "nahyukim",
        "url": "https://api.intra.42.fr/v2/users/nahyukim",
        "leader": true,
        "occurrence": 0,
        "validated": true,
        "projects_user_id": 3730632
      }
    ],
    "locked?": true,
    "validated?": true,
    "closed?": true,
    "repo_url": "git@vogsphere.42gyeongsan.kr:vogsphere/intra-uuid-02e77d04-fa19-4811-a296-1644b1112720-5760239-nahyukim",
    "repo_uuid": "intra-uuid-02e77d04-fa19-4811-a296-1644b1112720-5760239-nahyukim",
    "locked_at": "2024-06-27T01:21:16.016Z",
    "closed_at": "2024-07-03T12:25:57.023Z",
    "project_session_id": 11193,
    "project_gitlab_path": "pedago_world/c-piscine/shell-00",
    "scale_teams": [
      {
        "id": 6831696,
        "scale_id": 33823,
        "comment": "C 과제를 먼저 진행하신 후, Shell 00에 도전하셔서 필수 구현 항목에 중점을 둬 모두 잘 구현하신 것이 드러나 무난한 통과가 예상됩니다. 혹여 시간이 남으시면 말씀드린 것처럼 Exercise 08도 꼭 한 번 도전해보시길 바라겠습니다. 화이팅입니다~~!!",
        "created_at": "2024-07-03T14:13:03.716Z",
        "updated_at": "2024-07-03T15:24:19.630Z",
        "feedback": "편안한 분위기에서 평가가 이루어질 수 있도록 진행해주셔서 한층 긴장을 놓고 평가 받을 수 있었습니다. 어떤 문제가 앞으로 도움이 될지에 관한 얘기들도 해주셔서 정말 감사드립니다. 꼭 한 번 도전해보겠습니다! 수고 많으셨습니다 :)",
        "final_mark": 50,
        "flag": {
          "id": 1,
          "name": "Ok",
          "positive": true,
          "icon": "check-4",
          "created_at": "2015-09-14T23:06:52.000Z",
          "updated_at": "2015-09-14T23:06:52.000Z"
        },
        "begin_at": "2024-07-03T15:15:00.000Z",
        "correcteds": [
          {
            "id": 190824,
            "login": "nahyukim",
            "url": "https://api.intra.42.fr/v2/users/nahyukim"
          }
        ],
        "corrector": {
          "id": 190913,
          "login": "hyeonwki",
          "url": "https://api.intra.42.fr/v2/users/hyeonwki"
        },
        "truant": {},
        "filled_at": "2024-07-03T15:22:34.062Z",
        "questions_with_answers": []
      },
      {
        "id": 6831081,
        "scale_id": 33823,
        "comment": "명령어의 옵션에 대한 개념과 이해를 잘 숙지하고 계셨습니다. 또한 장난도 잘 받아주시며 평가 분위기를 밝게 만들어주셔서 저도 편안하게 리뷰를 할 수 있었습니다. 나중에 혹시 뒤의 exercise를 풀 마음이 생기시면 충분히 빠른 시간으로 해결할 수 있을 것 같습니다.\r\n감사합니다 !",
        "created_at": "2024-07-03T13:24:24.043Z",
        "updated_at": "2024-07-03T14:10:25.840Z",
        "feedback": "평가 분위기를 편안하게 만들어주셔서 즐겁게 평가 받을 수 있었습니다. 하나하나 친절하게 봐주셔서 감사했습니다. 수고 많으셨습니다 :)",
        "final_mark": 50,
        "flag": {
          "id": 1,
          "name": "Ok",
          "positive": true,
          "icon": "check-4",
          "created_at": "2015-09-14T23:06:52.000Z",
          "updated_at": "2015-09-14T23:06:52.000Z"
        },
        "begin_at": "2024-07-03T14:00:00.000Z",
        "correcteds": [
          {
            "id": 190824,
            "login": "nahyukim",
            "url": "https://api.intra.42.fr/v2/users/nahyukim"
          }
        ],
        "corrector": {
          "id": 190816,
          "login": "jkil",
          "url": "https://api.intra.42.fr/v2/users/jkil"
        },
        "truant": {},
        "filled_at": "2024-07-03T14:08:34.228Z",
        "questions_with_answers": []
      }
    ],
    "teams_uploads": [
      {
        "id": 2202804,
        "final_mark": 50,
        "comment": "ex00: OK | ex01: OK | ex02: OK | ex03: OK | ex04: OK | ex05: OK | ex06: Nothing turned in | ex07: Nothing turned in | ex08: Nothing turned in | ex09: Nothing turned in",
        "created_at": "2024-07-04T01:13:37.175Z",
        "upload_id": 1481
      }
    ]
  }
]"#;

    // Deserialize JSON data to FtTeam struct
    let ft_team = serde_json::from_str::<Vec<FtTeam>>(data);

    assert!(ft_team.is_ok())
}
