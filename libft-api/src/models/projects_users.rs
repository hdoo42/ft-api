use rvstruct::ValueStruct;
use serde::{Deserialize, Serialize};

use crate::prelude::*;

#[derive(Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct FtProjectsUser {
    pub created_at: FtDateTimeUtc,
    pub current_team_id: Option<FtTeamId>,
    pub cursus_ids: Vec<FtCursusId>,
    pub final_mark: Option<FtFinalMark>,
    pub id: FtProjectUserId,
    pub marked: bool,
    pub marked_at: Option<FtDateTimeUtc>,
    pub occurrence: FtOccurrence,
    pub project: FtProject,
    pub retriable_at: Option<FtDateTimeUtc>,
    pub status: FtStatus,
    pub teams: Option<Vec<FtTeam>>,
    pub updated_at: FtDateTimeUtc,
    pub user: Option<FtUser>,
    #[serde(rename = "validated?")]
    pub validated: Option<bool>,
}

#[derive(Debug, Eq, Hash, PartialEq, PartialOrd, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtProjectUserId(pub i32);

#[derive(Debug, Eq, Hash, PartialEq, PartialOrd, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtOccurrence(i32);

#[test]
fn deser_projects_users() {
    let raw_text = r#"
[
  {
    "id": 1895658,
    "occurrence": 0,
    "final_mark": 0,
    "status": "in_progress",
    "validated?": false,
    "current_team_id": 3191965,
    "project": {
      "id": 1314,
      "name": "Libft",
      "slug": "42cursus-libft",
      "parent_id": null
    },
    "cursus_ids": [21],
    "marked_at": "2023-07-11T15:03:43.393Z",
    "marked": true,
    "retriable_at": "2023-07-12T15:03:43.528Z",
    "created_at": "2020-03-23T12:49:56.171Z",
    "updated_at": "2023-07-11T17:46:39.704Z",
    "user": {
      "id": 53022,
      "email": "thomas.nogueira@42.fr",
      "login": "thomas",
      "first_name": "Thomas",
      "last_name": "Nogueira",
      "usual_full_name": "Thomas Nogueira",
      "usual_first_name": null,
      "url": "https://api.intra.42.fr/v2/users/thomas",
      "phone": "hidden",
      "displayname": "Thomas Nogueira",
      "kind": "admin",
      "image": {
        "link": "https://cdn.intra.42.fr/users/846b31565c82ec4f14aabc1d3e77bf05/thomas.gif",
        "versions": {
          "large": "https://cdn.intra.42.fr/users/bdcc02f0424ab7de390dadeaabc627c4/large_thomas.gif",
          "medium": "https://cdn.intra.42.fr/users/484363fdc2b63140c1bbeb276d803397/medium_thomas.gif",
          "small": "https://cdn.intra.42.fr/users/e2949f9087b6adbf43c1195f990c7267/small_thomas.gif",
          "micro": "https://cdn.intra.42.fr/users/e5dabf7afd6be5ea9c18350adb623b65/micro_thomas.gif"
        }
      },
      "staff?": true,
      "correction_point": 3,
      "pool_month": null,
      "pool_year": null,
      "location": null,
      "wallet": 0,
      "anonymize_date": "2027-08-02T00:00:00.000+09:00",
      "data_erasure_date": "2027-08-02T00:00:00.000+09:00",
      "created_at": "2019-02-26T13:04:43.926Z",
      "updated_at": "2024-06-25T09:11:17.363Z",
      "alumnized_at": null,
      "alumni?": false,
      "active?": true
    },
    "teams": [
      {
        "id": 3191965,
        "name": "thomas's group",
        "url": "https://api.intra.42.fr/v2/teams/3191965",
        "final_mark": 0,
        "project_id": 1314,
        "created_at": "2020-03-23T12:49:56.197Z",
        "updated_at": "2023-05-24T13:55:00.658Z",
        "status": "finished",
        "terminating_at": null,
        "users": [
          {
            "id": 53022,
            "login": "thomas",
            "url": "https://api.intra.42.fr/v2/users/thomas",
            "leader": true,
            "occurrence": 0,
            "validated": true,
            "projects_user_id": 1895658
          }
        ],
        "locked?": true,
        "validated?": false,
        "closed?": true,
        "repo_url": "git@vogsphere-v2.codam.nl:vogsphere/intra-uuid-6b38da20-4b92-48f5-891b-d59e5caf7b5a-3191965",
        "repo_uuid": "intra-uuid-6b38da20-4b92-48f5-891b-d59e5caf7b5a-3191965",
        "locked_at": "2020-03-23T12:49:56.241Z",
        "closed_at": "2020-03-23T13:18:11.894Z",
        "project_session_id": 3300,
        "project_gitlab_path": "pedago_world/42-cursus/inner-circle/libft"
      },
      {
        "id": 4937656,
        "name": "thomas's team",
        "url": "https://api.intra.42.fr/v2/teams/4937656",
        "final_mark": null,
        "project_id": 1314,
        "created_at": "2023-07-11T15:04:00.236Z",
        "updated_at": "2023-07-11T17:09:51.617Z",
        "status": "in_progress",
        "terminating_at": null,
        "users": [
          {
            "id": 53022,
            "login": "thomas",
            "url": "https://api.intra.42.fr/v2/users/thomas",
            "leader": true,
            "occurrence": 0,
            "validated": true,
            "projects_user_id": 1895658
          }
        ],
        "locked?": true,
        "validated?": null,
        "closed?": false,
        "repo_url": "git@vogsphere.42luxembourg.lu:vogsphere/intra-uuid-cfb143fd-917f-4390-b21e-6bb3456664a1-4937656-thomas",
        "repo_uuid": "intra-uuid-cfb143fd-917f-4390-b21e-6bb3456664a1-4937656-thomas",
        "locked_at": "2023-07-11T15:04:00.281Z",
        "closed_at": null,
        "project_session_id": 3300,
        "project_gitlab_path": "pedago_world/42-cursus/inner-circle/libft"
      }
    ]
  },
  {
    "id": 3426506,
    "occurrence": 1,
    "final_mark": 100,
    "status": "finished",
    "validated?": true,
    "current_team_id": 5342219,
    "project": {
      "id": 1255,
      "name": "C Piscine Shell 00",
      "slug": "c-piscine-shell-00",
      "parent_id": null
    },
    "cursus_ids": [9],
    "marked_at": "2023-11-29T13:31:21.947Z",
    "marked": true,
    "retriable_at": "2023-11-29T14:11:22.013Z",
    "created_at": "2023-11-27T00:58:22.093Z",
    "updated_at": "2023-11-30T07:10:21.941Z",
    "user": {
      "id": 172316,
      "email": "yoskwon@student.42gyeongsan.kr",
      "login": "yoskwon",
      "first_name": "Yosepina",
      "last_name": "Kwon",
      "usual_full_name": "Yosepina Kwon",
      "usual_first_name": null,
      "url": "https://api.intra.42.fr/v2/users/yoskwon",
      "phone": "hidden",
      "displayname": "Yosepina Kwon",
      "kind": "student",
      "image": {
        "link": "https://cdn.intra.42.fr/users/91a86517b1448e40bd024b69356cadb5/yoskwon.jpg",
        "versions": {
          "large": "https://cdn.intra.42.fr/users/4d1cd4bf4cd4124100a3b68775c07133/large_yoskwon.jpg",
          "medium": "https://cdn.intra.42.fr/users/86155882b4804bc2796530139b9d998f/medium_yoskwon.jpg",
          "small": "https://cdn.intra.42.fr/users/b798522b95e61e9c237fc8723601cf1e/small_yoskwon.jpg",
          "micro": "https://cdn.intra.42.fr/users/97690421d5ff2f4405cc0c8c9b1b48ea/micro_yoskwon.jpg"
        }
      },
      "staff?": false,
      "correction_point": 1,
      "pool_month": "december",
      "pool_year": "2023",
      "location": null,
      "wallet": 0,
      "anonymize_date": "2026-12-29T00:00:00.000+09:00",
      "data_erasure_date": "2026-12-29T00:00:00.000+09:00",
      "created_at": "2023-11-26T04:09:50.042Z",
      "updated_at": "2023-12-22T00:52:38.917Z",
      "alumnized_at": null,
      "alumni?": false,
      "active?": false
    },
    "teams": [
      {
        "id": 5336562,
        "name": "yoskwon's group",
        "url": "https://api.intra.42.fr/v2/teams/5336562",
        "final_mark": 5,
        "project_id": 1255,
        "created_at": "2023-11-27T00:58:22.104Z",
        "updated_at": "2023-11-29T06:49:24.015Z",
        "status": "finished",
        "terminating_at": "2023-11-29T06:49:15.217Z",
        "users": [
          {
            "id": 172316,
            "login": "yoskwon",
            "url": "https://api.intra.42.fr/v2/users/yoskwon",
            "leader": true,
            "occurrence": 0,
            "validated": true,
            "projects_user_id": 3426506
          }
        ],
        "locked?": true,
        "validated?": false,
        "closed?": true,
        "repo_url": "git@vogsphere.42gyeongsan.kr:vogsphere/intra-uuid-8294c9d3-325c-4856-8d6a-2b34882272c5-5336562-yoskwon",
        "repo_uuid": "intra-uuid-8294c9d3-325c-4856-8d6a-2b34882272c5-5336562-yoskwon",
        "locked_at": "2023-11-27T00:58:22.132Z",
        "closed_at": "2023-11-28T06:49:15.217Z",
        "project_session_id": 11193,
        "project_gitlab_path": "pedago_world/c-piscine/shell-00"
      },
      {
        "id": 5342219,
        "name": "yoskwon's group-1",
        "url": "https://api.intra.42.fr/v2/teams/5342219",
        "final_mark": 100,
        "project_id": 1255,
        "created_at": "2023-11-29T01:37:24.510Z",
        "updated_at": "2023-11-30T07:10:21.899Z",
        "status": "finished",
        "terminating_at": "2023-11-30T07:09:58.797Z",
        "users": [
          {
            "id": 172316,
            "login": "yoskwon",
            "url": "https://api.intra.42.fr/v2/users/yoskwon",
            "leader": true,
            "occurrence": 1,
            "validated": true,
            "projects_user_id": 3426506
          }
        ],
        "locked?": true,
        "validated?": true,
        "closed?": true,
        "repo_url": "git@vogsphere.42gyeongsan.kr:vogsphere/intra-uuid-613e8142-b532-4dce-85f9-6e9a4ccbdd8d-5342219-yoskwon",
        "repo_uuid": "intra-uuid-613e8142-b532-4dce-85f9-6e9a4ccbdd8d-5342219-yoskwon",
        "locked_at": "2023-11-29T01:37:24.543Z",
        "closed_at": "2023-11-29T07:09:58.797Z",
        "project_session_id": 11193,
        "project_gitlab_path": "pedago_world/c-piscine/shell-00"
      }
    ]
  }
]
"#;

    let result = serde_json::from_str::<Vec<FtProjectsUser>>(raw_text);

    assert!(result.is_ok());
}
