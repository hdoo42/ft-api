use rvstruct::ValueStruct;
use serde::{Deserialize, Serialize};

use crate::{FtFinalMark, FtProjectId, FtProjectSessionId, FtScaleTeam, FtUrl, FtUser};

use super::FtDateTimeUtc;

#[derive(Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct FtTeam {
    pub id: FtTeamId,
    pub created_at: Option<FtDateTimeUtc>,
    pub name: Option<FtTeamName>,
    pub project_id: Option<FtProjectId>,
    pub project_session_id: Option<FtProjectSessionId>,
    pub repo_uuid: Option<FtRepoUuid>,
    pub status: Option<FtStatus>,
    pub updated_at: Option<FtDateTimeUtc>,
    pub url: Option<FtUrl>,
    pub users: Option<Vec<FtUser>>,
    pub final_mark: Option<FtFinalMark>,
    pub closed: Option<bool>,
    pub closed_at: Option<FtDateTimeUtc>,
    pub locked: Option<bool>,
    pub locked_at: Option<FtDateTimeUtc>,
    pub project_gitlab_path: Option<FtProjectGitlabPath>,
    pub repo_url: Option<FtUrl>,
    pub scale_teams: Option<Vec<FtScaleTeam>>,
    pub teams_uploads: Option<Vec<FtTeamUpload>>,
    pub terminating_at: Option<FtDateTimeUtc>,
    pub validated: Option<bool>,
}

#[derive(Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct FtTeamUpload {
    pub id: FtTeamId,
    pub final_mark: FtFinalMark,
    pub comment: String,
    pub created_at: FtDateTimeUtc,
    pub upload_id: FtTeamUploadId,
}

#[derive(Debug, Eq, Hash, PartialEq, PartialOrd, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtTeamId(pub i32);

#[derive(Debug, Eq, Hash, PartialEq, PartialOrd, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtTeamName(pub String);

#[derive(Debug, Eq, Hash, PartialEq, PartialOrd, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtProjectGitlabPath(pub String);

#[derive(Debug, Eq, Hash, PartialEq, PartialOrd, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtRepoUuid(pub String);

#[derive(Debug, Eq, Hash, PartialEq, PartialOrd, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtStatus(pub String);

#[derive(Debug, Eq, Hash, PartialEq, PartialOrd, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtTeamUploadId(i32);

#[test]
fn test_ft_team_deserialization() {
    // Sample JSON data representing an FtTeam instance
    let data = r#"[
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
    "project_gitlab_path": "pedago_world/42-cursus/inner-circle/libft",
    "scale_teams": [
      {
        "id": 2553328,
        "scale_id": 2295,
        "comment": "IT'S AMAZING !! ",
        "created_at": "2020-03-23T13:58:04.299Z",
        "updated_at": "2023-05-24T13:55:00.635Z",
        "feedback": "good peer eval , karthur is a nice guy , elle est ou la poulette ? J'suis chef de guerre, moi. J'suis pas là pour secouer des drapeaux et jouer de la trompette ! ",
        "final_mark": 0,
        "flag": {
          "id": 9,
          "name": "Outstanding project",
          "positive": true,
          "icon": "star-1",
          "created_at": "2017-05-18T14:07:37.380Z",
          "updated_at": "2017-05-18T14:12:07.415Z"
        },
        "begin_at": "2020-03-23T15:00:00.000Z",
        "correcteds": [
          {
            "id": 53022,
            "login": "thomas",
            "url": "https://api.intra.42.fr/v2/users/thomas"
          }
        ],
        "corrector": {
          "id": 45757,
          "login": "karthur",
          "url": "https://api.intra.42.fr/v2/users/karthur"
        },
        "truant": {},
        "filled_at": "2020-03-23T15:05:41.073Z",
        "questions_with_answers": []
      }
    ],
    "teams_uploads": []
  },
  {
    "id": 3563202,
    "name": "taeng's group",
    "url": "https://api.intra.42.fr/v2/teams/3563202",
    "final_mark": 100,
    "project_id": 1318,
    "created_at": "2021-05-12T10:53:21.441Z",
    "updated_at": "2021-05-12T10:55:17.524Z",
    "status": "finished",
    "terminating_at": null,
    "users": [
      {
        "id": 82562,
        "login": "taeng",
        "url": "https://api.intra.42.fr/v2/users/taeng",
        "leader": true,
        "occurrence": 0,
        "validated": true,
        "projects_user_id": 2171359
      }
    ],
    "locked?": true,
    "validated?": true,
    "closed?": true,
    "repo_url": "git@vogsphere.42seoul.kr:vogsphere/intra-uuid-66316ba5-d6fc-4469-b365-72f3bd89912a-3563202",
    "repo_uuid": "intra-uuid-66316ba5-d6fc-4469-b365-72f3bd89912a-3563202",
    "locked_at": "2021-05-12T10:53:21.497Z",
    "closed_at": "2021-05-12T10:53:34.463Z",
    "project_session_id": 5652,
    "project_gitlab_path": "pedago_world/42-cursus/inner-circle/netwhat",
    "scale_teams": [],
    "teams_uploads": []
  },
  {
    "id": 3563205,
    "name": "taeng's group",
    "url": "https://api.intra.42.fr/v2/teams/3563205",
    "final_mark": 100,
    "project_id": 1328,
    "created_at": "2021-05-12T10:54:05.734Z",
    "updated_at": "2021-05-12T10:55:42.308Z",
    "status": "finished",
    "terminating_at": null,
    "users": [
      {
        "id": 82562,
        "login": "taeng",
        "url": "https://api.intra.42.fr/v2/users/taeng",
        "leader": true,
        "occurrence": 0,
        "validated": true,
        "projects_user_id": 2171362
      }
    ],
    "locked?": true,
    "validated?": true,
    "closed?": true,
    "repo_url": "git@vogsphere.42seoul.kr:vogsphere/intra-uuid-83e442ed-af19-478e-981a-2b069eb785db-3563205",
    "repo_uuid": "intra-uuid-83e442ed-af19-478e-981a-2b069eb785db-3563205",
    "locked_at": "2021-05-12T10:54:05.783Z",
    "closed_at": "2021-05-12T10:54:21.783Z",
    "project_session_id": 3314,
    "project_gitlab_path": "pedago_world/42-cursus/inner-circle/ft_server",
    "scale_teams": [],
    "teams_uploads": []
  },
  {
    "id": 3563225,
    "name": "taeng's group",
    "url": "https://api.intra.42.fr/v2/teams/3563225",
    "final_mark": 99,
    "project_id": 1329,
    "created_at": "2021-05-12T10:57:30.550Z",
    "updated_at": "2021-05-12T11:04:49.657Z",
    "status": "finished",
    "terminating_at": null,
    "users": [
      {
        "id": 82562,
        "login": "taeng",
        "url": "https://api.intra.42.fr/v2/users/taeng",
        "leader": true,
        "occurrence": 0,
        "validated": true,
        "projects_user_id": 2171381
      }
    ],
    "locked?": true,
    "validated?": false,
    "closed?": true,
    "repo_url": "git@vogsphere.42seoul.kr:vogsphere/intra-uuid-4f2c6450-40eb-4d3b-8f9d-b45ee385aa9a-3563225",
    "repo_uuid": "intra-uuid-4f2c6450-40eb-4d3b-8f9d-b45ee385aa9a-3563225",
    "locked_at": "2021-05-12T10:57:30.602Z",
    "closed_at": "2021-05-12T10:57:38.113Z",
    "project_session_id": 5130,
    "project_gitlab_path": "pedago_world/42-cursus/inner-circle/ft_services",
    "scale_teams": [],
    "teams_uploads": []
  },
  {
    "id": 3821516,
    "name": "dorian's group",
    "url": "https://api.intra.42.fr/v2/teams/3821516",
    "final_mark": 100,
    "project_id": 1316,
    "created_at": "2021-10-04T08:21:44.490Z",
    "updated_at": "2021-10-21T16:28:00.334Z",
    "status": "finished",
    "terminating_at": null,
    "users": [
      {
        "id": 98539,
        "login": "dorian",
        "url": "https://api.intra.42.fr/v2/users/dorian",
        "leader": true,
        "occurrence": 0,
        "validated": true,
        "projects_user_id": 2364782
      }
    ],
    "locked?": true,
    "validated?": true,
    "closed?": true,
    "repo_url": "git@vogsphere.42paris.fr:vogsphere/intra-uuid-33b90e13-aacd-4899-9cc3-4a96ae465006-3821516",
    "repo_uuid": "intra-uuid-33b90e13-aacd-4899-9cc3-4a96ae465006-3821516",
    "locked_at": "2021-10-04T08:21:44.540Z",
    "closed_at": "2021-10-21T16:28:00.319Z",
    "project_session_id": 4472,
    "project_gitlab_path": "pedago_world/42-cursus/inner-circle/ft_printf",
    "scale_teams": [],
    "teams_uploads": []
  },
  {
    "id": 3874607,
    "name": "dorian's team",
    "url": "https://api.intra.42.fr/v2/teams/3874607",
    "final_mark": 55,
    "project_id": 1316,
    "created_at": "2021-11-09T09:24:41.844Z",
    "updated_at": "2022-06-30T15:47:55.119Z",
    "status": "finished",
    "terminating_at": null,
    "users": [
      {
        "id": 98539,
        "login": "dorian",
        "url": "https://api.intra.42.fr/v2/users/dorian",
        "leader": true,
        "occurrence": 0,
        "validated": true,
        "projects_user_id": 2364782
      }
    ],
    "locked?": true,
    "validated?": false,
    "closed?": true,
    "repo_url": "git@vogsphere.42paris.fr:vogsphere/intra-uuid-79bb233e-a9df-4ee1-a686-ea9722a879c1-3874607",
    "repo_uuid": "intra-uuid-79bb233e-a9df-4ee1-a686-ea9722a879c1-3874607",
    "locked_at": "2021-11-09T09:24:41.905Z",
    "closed_at": "2021-11-17T08:28:49.549Z",
    "project_session_id": 4472,
    "project_gitlab_path": "pedago_world/42-cursus/inner-circle/ft_printf",
    "scale_teams": [],
    "teams_uploads": []
  },
  {
    "id": 3886627,
    "name": "dorian's group",
    "url": "https://api.intra.42.fr/v2/teams/3886627",
    "final_mark": 0,
    "project_id": 1327,
    "created_at": "2021-11-18T13:39:17.723Z",
    "updated_at": "2022-05-11T09:11:14.008Z",
    "status": "finished",
    "terminating_at": null,
    "users": [
      {
        "id": 98539,
        "login": "dorian",
        "url": "https://api.intra.42.fr/v2/users/dorian",
        "leader": true,
        "occurrence": 0,
        "validated": true,
        "projects_user_id": 2411315
      }
    ],
    "locked?": true,
    "validated?": false,
    "closed?": true,
    "repo_url": "git@vogsphere.42paris.fr:vogsphere/intra-uuid-b420e7f8-3569-4abb-ac04-d60fe6336fc7-3886627",
    "repo_uuid": "intra-uuid-b420e7f8-3569-4abb-ac04-d60fe6336fc7-3886627",
    "locked_at": "2021-11-18T13:39:17.768Z",
    "closed_at": "2022-05-11T08:46:00.112Z",
    "project_session_id": 6601,
    "project_gitlab_path": "pedago_world/42-cursus/inner-circle/get_next_line",
    "scale_teams": [],
    "teams_uploads": []
  },
  {
    "id": 3927158,
    "name": "dorian's group",
    "url": "https://api.intra.42.fr/v2/teams/3927158",
    "final_mark": 0,
    "project_id": 1483,
    "created_at": "2021-12-16T08:50:35.078Z",
    "updated_at": "2022-01-31T12:39:12.030Z",
    "status": "finished",
    "terminating_at": null,
    "users": [
      {
        "id": 98539,
        "login": "dorian",
        "url": "https://api.intra.42.fr/v2/users/dorian",
        "leader": true,
        "occurrence": 0,
        "validated": true,
        "projects_user_id": 2441921
      }
    ],
    "locked?": true,
    "validated?": false,
    "closed?": true,
    "repo_url": null,
    "repo_uuid": "intra-uuid-b340cfe3-a05a-44da-bf7b-a278caf4d9e6-3927158",
    "locked_at": "2021-12-16T08:50:35.124Z",
    "closed_at": "2021-12-16T08:50:35.143Z",
    "project_session_id": 3623,
    "project_gitlab_path": "pedago_world/42-cursus/outer-circle/piscine-python-django/main-project",
    "scale_teams": [],
    "teams_uploads": []
  },
  {
    "id": 4042624,
    "name": "dorian's group",
    "url": "https://api.intra.42.fr/v2/teams/4042624",
    "final_mark": null,
    "project_id": 1944,
    "created_at": "2022-03-07T14:15:42.125Z",
    "updated_at": "2022-03-14T14:02:22.402Z",
    "status": "in_progress",
    "terminating_at": null,
    "users": [
      {
        "id": 98539,
        "login": "dorian",
        "url": "https://api.intra.42.fr/v2/users/dorian",
        "leader": true,
        "occurrence": 0,
        "validated": true,
        "projects_user_id": 2524041
      }
    ],
    "locked?": true,
    "validated?": null,
    "closed?": false,
    "repo_url": "git@vogsphere.42paris.fr:vogsphere/intra-uuid-06613c1d-54ea-4cf0-87c4-4d58411c39a1-4042624-dorian",
    "repo_uuid": "intra-uuid-06613c1d-54ea-4cf0-87c4-4d58411c39a1-4042624-dorian",
    "locked_at": "2022-03-07T14:15:42.197Z",
    "closed_at": null,
    "project_session_id": 5528,
    "project_gitlab_path": "pedago_world/42-cursus/outer-circle/ladder",
    "scale_teams": [],
    "teams_uploads": []
  },
  {
    "id": 4093901,
    "name": "dorian's team",
    "url": "https://api.intra.42.fr/v2/teams/4093901",
    "final_mark": 100,
    "project_id": 2070,
    "created_at": "2022-04-13T13:01:10.624Z",
    "updated_at": "2022-04-13T13:01:20.116Z",
    "status": "finished",
    "terminating_at": null,
    "users": [
      {
        "id": 98539,
        "login": "dorian",
        "url": "https://api.intra.42.fr/v2/users/dorian",
        "leader": true,
        "occurrence": 0,
        "validated": true,
        "projects_user_id": 2560578
      }
    ],
    "locked?": false,
    "validated?": true,
    "closed?": true,
    "repo_url": null,
    "repo_uuid": "intra-uuid-af4a1a82-1812-4a93-8c3a-a1aef40913ac-4093901-dorian",
    "locked_at": null,
    "closed_at": "2022-04-13T13:01:20.111Z",
    "project_session_id": 6563,
    "project_gitlab_path": "pedago_world/42-cursus/rushes/hotrace",
    "scale_teams": [],
    "teams_uploads": []
  },
  {
    "id": 4094270,
    "name": "dorian's team",
    "url": "https://api.intra.42.fr/v2/teams/4094270",
    "final_mark": 100,
    "project_id": 2102,
    "created_at": "2022-04-13T15:56:26.954Z",
    "updated_at": "2022-06-07T13:27:21.657Z",
    "status": "finished",
    "terminating_at": "2022-05-04T15:56:45.193Z",
    "users": [
      {
        "id": 98539,
        "login": "dorian",
        "url": "https://api.intra.42.fr/v2/users/dorian",
        "leader": true,
        "occurrence": 0,
        "validated": true,
        "projects_user_id": 2560908
      }
    ],
    "locked?": false,
    "validated?": true,
    "closed?": true,
    "repo_url": null,
    "repo_uuid": "intra-uuid-758238bf-2e05-40a8-9a28-1a7b07209b49-4094270-dorian",
    "locked_at": null,
    "closed_at": "2022-04-13T15:56:45.193Z",
    "project_session_id": 7058,
    "project_gitlab_path": "pedago_world/42-cursus/rushes/alcu",
    "scale_teams": [
      {
        "id": 4123701,
        "scale_id": 11417,
        "comment": "Missing peer-evaluation replaced by a 0",
        "created_at": "2022-05-04T15:57:24.911Z",
        "updated_at": "2022-05-04T15:57:25.108Z",
        "feedback": "The Intranet is right ! I deserve this 0 !",
        "final_mark": 0,
        "flag": {
          "id": 1,
          "name": "Ok",
          "positive": true,
          "icon": "check-4",
          "created_at": "2015-09-14T23:06:52.000Z",
          "updated_at": "2015-09-14T23:06:52.000Z"
        },
        "begin_at": null,
        "correcteds": "invisible",
        "corrector": "invisible",
        "truant": {},
        "filled_at": "2022-05-04T15:57:25.056Z",
        "questions_with_answers": []
      },
      {
        "id": 4123700,
        "scale_id": 11417,
        "comment": "Missing peer-evaluation replaced by a 0",
        "created_at": "2022-05-04T15:57:24.296Z",
        "updated_at": "2022-05-04T15:57:24.551Z",
        "feedback": "The Intranet is right ! I deserve this 0 !",
        "final_mark": 0,
        "flag": {
          "id": 1,
          "name": "Ok",
          "positive": true,
          "icon": "check-4",
          "created_at": "2015-09-14T23:06:52.000Z",
          "updated_at": "2015-09-14T23:06:52.000Z"
        },
        "begin_at": null,
        "correcteds": "invisible",
        "corrector": "invisible",
        "truant": {},
        "filled_at": "2022-05-04T15:57:24.504Z",
        "questions_with_answers": []
      }
    ],
    "teams_uploads": []
  },
  {
    "id": 4094296,
    "name": "dorian's team",
    "url": "https://api.intra.42.fr/v2/teams/4094296",
    "final_mark": 100,
    "project_id": 2122,
    "created_at": "2022-04-13T16:21:01.471Z",
    "updated_at": "2022-06-07T13:27:37.877Z",
    "status": "finished",
    "terminating_at": "2022-05-04T16:21:15.412Z",
    "users": [
      {
        "id": 98539,
        "login": "dorian",
        "url": "https://api.intra.42.fr/v2/users/dorian",
        "leader": true,
        "occurrence": 0,
        "validated": true,
        "projects_user_id": 2560929
      }
    ],
    "locked?": false,
    "validated?": true,
    "closed?": true,
    "repo_url": null,
    "repo_uuid": "intra-uuid-bb5ce7ec-10bc-4b3c-a92f-be888add1145-4094296-dorian",
    "locked_at": null,
    "closed_at": "2022-04-13T16:21:15.412Z",
    "project_session_id": 7418,
    "project_gitlab_path": "pedago_world/42-cursus/rushes/wong_kar_wai",
    "scale_teams": [
      {
        "id": 4123748,
        "scale_id": 12751,
        "comment": "Missing peer-evaluation replaced by a 0",
        "created_at": "2022-05-04T16:21:42.977Z",
        "updated_at": "2022-05-04T16:21:43.309Z",
        "feedback": "The Intranet is right ! I deserve this 0 !",
        "final_mark": 0,
        "flag": {
          "id": 1,
          "name": "Ok",
          "positive": true,
          "icon": "check-4",
          "created_at": "2015-09-14T23:06:52.000Z",
          "updated_at": "2015-09-14T23:06:52.000Z"
        },
        "begin_at": null,
        "correcteds": "invisible",
        "corrector": "invisible",
        "truant": {},
        "filled_at": "2022-05-04T16:21:43.237Z",
        "questions_with_answers": []
      },
      {
        "id": 4123747,
        "scale_id": 12751,
        "comment": "Missing peer-evaluation replaced by a 0",
        "created_at": "2022-05-04T16:21:42.668Z",
        "updated_at": "2022-05-04T16:21:42.915Z",
        "feedback": "The Intranet is right ! I deserve this 0 !",
        "final_mark": 0,
        "flag": {
          "id": 1,
          "name": "Ok",
          "positive": true,
          "icon": "check-4",
          "created_at": "2015-09-14T23:06:52.000Z",
          "updated_at": "2015-09-14T23:06:52.000Z"
        },
        "begin_at": null,
        "correcteds": "invisible",
        "corrector": "invisible",
        "truant": {},
        "filled_at": "2022-05-04T16:21:42.847Z",
        "questions_with_answers": []
      }
    ],
    "teams_uploads": []
  },
  {
    "id": 4094298,
    "name": "dorian's team",
    "url": "https://api.intra.42.fr/v2/teams/4094298",
    "final_mark": 0,
    "project_id": 2136,
    "created_at": "2022-04-13T16:21:36.276Z",
    "updated_at": "2022-06-13T08:31:44.319Z",
    "status": "finished",
    "terminating_at": "2022-05-04T16:21:47.780Z",
    "users": [
      {
        "id": 98539,
        "login": "dorian",
        "url": "https://api.intra.42.fr/v2/users/dorian",
        "leader": true,
        "occurrence": 0,
        "validated": true,
        "projects_user_id": 2560931
      }
    ],
    "locked?": false,
    "validated?": false,
    "closed?": true,
    "repo_url": null,
    "repo_uuid": "intra-uuid-3d47a93f-2b9f-4741-a837-76a1aa54c19d-4094298-dorian",
    "locked_at": null,
    "closed_at": "2022-04-13T16:21:47.780Z",
    "project_session_id": 7658,
    "project_gitlab_path": "pedago_world/42-cursus/rushes/yasl",
    "scale_teams": [
      {
        "id": 4123751,
        "scale_id": 13416,
        "comment": "Missing peer-evaluation replaced by a 0",
        "created_at": "2022-05-04T16:21:57.758Z",
        "updated_at": "2022-05-04T16:21:58.060Z",
        "feedback": "The Intranet is right ! I deserve this 0 !",
        "final_mark": 0,
        "flag": {
          "id": 1,
          "name": "Ok",
          "positive": true,
          "icon": "check-4",
          "created_at": "2015-09-14T23:06:52.000Z",
          "updated_at": "2015-09-14T23:06:52.000Z"
        },
        "begin_at": null,
        "correcteds": "invisible",
        "corrector": "invisible",
        "truant": {},
        "filled_at": "2022-05-04T16:21:57.981Z",
        "questions_with_answers": []
      },
      {
        "id": 4123750,
        "scale_id": 13416,
        "comment": "Missing peer-evaluation replaced by a 0",
        "created_at": "2022-05-04T16:21:57.417Z",
        "updated_at": "2022-05-04T16:21:57.695Z",
        "feedback": "The Intranet is right ! I deserve this 0 !",
        "final_mark": 0,
        "flag": {
          "id": 1,
          "name": "Ok",
          "positive": true,
          "icon": "check-4",
          "created_at": "2015-09-14T23:06:52.000Z",
          "updated_at": "2015-09-14T23:06:52.000Z"
        },
        "begin_at": null,
        "correcteds": "invisible",
        "corrector": "invisible",
        "truant": {},
        "filled_at": "2022-05-04T16:21:57.628Z",
        "questions_with_answers": []
      }
    ],
    "teams_uploads": []
  },
  {
    "id": 4128452,
    "name": "dorian's group",
    "url": "https://api.intra.42.fr/v2/teams/4128452",
    "final_mark": null,
    "project_id": 1320,
    "created_at": "2022-05-11T08:55:58.976Z",
    "updated_at": "2022-05-11T08:55:59.088Z",
    "status": "in_progress",
    "terminating_at": null,
    "users": [
      {
        "id": 98539,
        "login": "dorian",
        "url": "https://api.intra.42.fr/v2/users/dorian",
        "leader": true,
        "occurrence": 0,
        "validated": true,
        "projects_user_id": 2586432
      }
    ],
    "locked?": true,
    "validated?": null,
    "closed?": false,
    "repo_url": null,
    "repo_uuid": "intra-uuid-f256e686-88f8-4eab-a938-347c38d932b5-4128452-dorian",
    "locked_at": "2022-05-11T08:55:59.014Z",
    "closed_at": null,
    "project_session_id": 6076,
    "project_gitlab_path": null,
    "scale_teams": [],
    "teams_uploads": []
  },
  {
    "id": 4160560,
    "name": "dorian's group-1",
    "url": "https://api.intra.42.fr/v2/teams/4160560",
    "final_mark": 80,
    "project_id": 1316,
    "created_at": "2022-06-03T14:04:30.528Z",
    "updated_at": "2022-06-30T14:57:16.879Z",
    "status": "in_progress",
    "terminating_at": null,
    "users": [
      {
        "id": 98539,
        "login": "dorian",
        "url": "https://api.intra.42.fr/v2/users/dorian",
        "leader": true,
        "occurrence": 1,
        "validated": true,
        "projects_user_id": 2364782
      }
    ],
    "locked?": true,
    "validated?": false,
    "closed?": false,
    "repo_url": "git@vogsphere.42paris.fr:vogsphere/intra-uuid-330cfc34-44c6-4885-be4e-026f5352a1fd-4160560-dorian",
    "repo_uuid": "intra-uuid-330cfc34-44c6-4885-be4e-026f5352a1fd-4160560-dorian",
    "locked_at": "2022-06-03T14:04:30.556Z",
    "closed_at": null,
    "project_session_id": 4472,
    "project_gitlab_path": "pedago_world/42-cursus/inner-circle/ft_printf",
    "scale_teams": [],
    "teams_uploads": []
  },
  {
    "id": 4170236,
    "name": "taeng's team",
    "url": "https://api.intra.42.fr/v2/teams/4170236",
    "final_mark": 100,
    "project_id": 1316,
    "created_at": "2022-06-09T01:04:02.231Z",
    "updated_at": "2023-01-29T09:28:30.607Z",
    "status": "finished",
    "terminating_at": null,
    "users": [
      {
        "id": 82562,
        "login": "taeng",
        "url": "https://api.intra.42.fr/v2/users/taeng",
        "leader": true,
        "occurrence": 1,
        "validated": true,
        "projects_user_id": 2171360
      }
    ],
    "locked?": true,
    "validated?": true,
    "closed?": true,
    "repo_url": "git@vogsphere.42seoul.kr:vogsphere/intra-uuid-9ea562ed-014f-451e-9a74-ba524d0fd9f1-4170236-taeng",
    "repo_uuid": "intra-uuid-9ea562ed-014f-451e-9a74-ba524d0fd9f1-4170236-taeng",
    "locked_at": "2022-06-09T01:04:02.298Z",
    "closed_at": "2023-01-29T09:28:21.650Z",
    "project_session_id": 6959,
    "project_gitlab_path": "pedago_world/42-cursus/inner-circle/ft_printf",
    "scale_teams": [],
    "teams_uploads": []
  },
  {
    "id": 4175658,
    "name": "dorian's team",
    "url": "https://api.intra.42.fr/v2/teams/4175658",
    "final_mark": null,
    "project_id": 2065,
    "created_at": "2022-06-13T08:31:45.576Z",
    "updated_at": "2022-06-13T08:31:45.714Z",
    "status": "in_progress",
    "terminating_at": null,
    "users": [
      {
        "id": 98539,
        "login": "dorian",
        "url": "https://api.intra.42.fr/v2/users/dorian",
        "leader": true,
        "occurrence": 0,
        "validated": true,
        "projects_user_id": 2620002
      }
    ],
    "locked?": true,
    "validated?": null,
    "closed?": true,
    "repo_url": null,
    "repo_uuid": "intra-uuid-c200e171-00c3-4b92-9255-4b983608bd7d-4175658-dorian",
    "locked_at": "2022-06-13T08:31:45.662Z",
    "closed_at": "2022-06-13T08:31:45.711Z",
    "project_session_id": 6951,
    "project_gitlab_path": null,
    "scale_teams": [],
    "teams_uploads": []
  },
  {
    "id": 4221000,
    "name": "taeng's group",
    "url": "https://api.intra.42.fr/v2/teams/4221000",
    "final_mark": 100,
    "project_id": 1994,
    "created_at": "2022-07-07T10:37:18.282Z",
    "updated_at": "2023-01-29T09:27:42.989Z",
    "status": "finished",
    "terminating_at": null,
    "users": [
      {
        "id": 82562,
        "login": "taeng",
        "url": "https://api.intra.42.fr/v2/users/taeng",
        "leader": true,
        "occurrence": 0,
        "validated": true,
        "projects_user_id": 2654653
      }
    ],
    "locked?": true,
    "validated?": true,
    "closed?": true,
    "repo_url": "git@vogsphere.42seoul.kr:vogsphere/intra-uuid-6cb34f36-b32f-42fc-b9ca-76c9b87d044f-4221000-taeng",
    "repo_uuid": "intra-uuid-6cb34f36-b32f-42fc-b9ca-76c9b87d044f-4221000-taeng",
    "locked_at": "2022-07-07T10:37:18.328Z",
    "closed_at": "2023-01-29T09:26:48.945Z",
    "project_session_id": 6958,
    "project_gitlab_path": "pedago_world/42-cursus/inner-circle/born2beroot",
    "scale_teams": [],
    "teams_uploads": []
  },
  {
    "id": 4310854,
    "name": "taeng's group",
    "url": "https://api.intra.42.fr/v2/teams/4310854",
    "final_mark": 100,
    "project_id": 1314,
    "created_at": "2022-08-10T05:52:08.209Z",
    "updated_at": "2023-01-29T09:28:16.995Z",
    "status": "finished",
    "terminating_at": null,
    "users": [
      {
        "id": 82562,
        "login": "taeng",
        "url": "https://api.intra.42.fr/v2/users/taeng",
        "leader": true,
        "occurrence": 0,
        "validated": true,
        "projects_user_id": 2715119
      }
    ],
    "locked?": true,
    "validated?": true,
    "closed?": true,
    "repo_url": "git@vogsphere.42seoul.kr:vogsphere/intra-uuid-703de7af-306a-4968-a25a-e63f87a8a90d-4310854-taeng",
    "repo_uuid": "intra-uuid-703de7af-306a-4968-a25a-e63f87a8a90d-4310854-taeng",
    "locked_at": "2022-08-10T05:52:08.241Z",
    "closed_at": "2023-01-29T09:28:04.354Z",
    "project_session_id": 6957,
    "project_gitlab_path": "pedago_world/42-cursus/inner-circle/libft",
    "scale_teams": [],
    "teams_uploads": []
  },
  {
    "id": 4647359,
    "name": "taeng's group",
    "url": "https://api.intra.42.fr/v2/teams/4647359",
    "final_mark": 100,
    "project_id": 1327,
    "created_at": "2023-01-29T09:29:38.375Z",
    "updated_at": "2023-01-29T09:31:32.553Z",
    "status": "finished",
    "terminating_at": null,
    "users": [
      {
        "id": 82562,
        "login": "taeng",
        "url": "https://api.intra.42.fr/v2/users/taeng",
        "leader": true,
        "occurrence": 0,
        "validated": true,
        "projects_user_id": 2953141
      }
    ],
    "locked?": true,
    "validated?": true,
    "closed?": true,
    "repo_url": "git@vogsphere.42seoul.kr:vogsphere/intra-uuid-f2eb020e-cc17-4700-8f41-263ad5462d6a-4647359-taeng",
    "repo_uuid": "intra-uuid-f2eb020e-cc17-4700-8f41-263ad5462d6a-4647359-taeng",
    "locked_at": "2023-01-29T09:29:38.398Z",
    "closed_at": "2023-01-29T09:31:24.908Z",
    "project_session_id": 6960,
    "project_gitlab_path": "pedago_world/42-cursus/inner-circle/get_next_line",
    "scale_teams": [],
    "teams_uploads": []
  },
  {
    "id": 4647360,
    "name": "taeng's team",
    "url": "https://api.intra.42.fr/v2/teams/4647360",
    "final_mark": 0,
    "project_id": 1327,
    "created_at": "2023-01-29T09:31:42.559Z",
    "updated_at": "2023-01-29T09:34:03.349Z",
    "status": "finished",
    "terminating_at": null,
    "users": [
      {
        "id": 82562,
        "login": "taeng",
        "url": "https://api.intra.42.fr/v2/users/taeng",
        "leader": true,
        "occurrence": 0,
        "validated": true,
        "projects_user_id": 2953141
      }
    ],
    "locked?": true,
    "validated?": false,
    "closed?": true,
    "repo_url": "git@vogsphere.42seoul.kr:vogsphere/intra-uuid-e95a6448-36a2-4b76-8e43-d083748fc847-4647360-taeng",
    "repo_uuid": "intra-uuid-e95a6448-36a2-4b76-8e43-d083748fc847-4647360-taeng",
    "locked_at": "2023-01-29T09:31:42.607Z",
    "closed_at": "2023-01-29T09:34:00.596Z",
    "project_session_id": 6960,
    "project_gitlab_path": "pedago_world/42-cursus/inner-circle/get_next_line",
    "scale_teams": [],
    "teams_uploads": []
  },
  {
    "id": 4647366,
    "name": "taeng's group",
    "url": "https://api.intra.42.fr/v2/teams/4647366",
    "final_mark": 100,
    "project_id": 1320,
    "created_at": "2023-01-29T09:34:29.520Z",
    "updated_at": "2023-01-29T09:35:36.143Z",
    "status": "finished",
    "terminating_at": null,
    "users": [
      {
        "id": 82562,
        "login": "taeng",
        "url": "https://api.intra.42.fr/v2/users/taeng",
        "leader": true,
        "occurrence": 0,
        "validated": true,
        "projects_user_id": 2953147
      }
    ],
    "locked?": true,
    "validated?": true,
    "closed?": true,
    "repo_url": null,
    "repo_uuid": "intra-uuid-97ea3a62-d59c-4676-9b72-05349191e6ab-4647366-taeng",
    "locked_at": "2023-01-29T09:34:29.548Z",
    "closed_at": "2023-01-29T09:35:36.133Z",
    "project_session_id": 4459,
    "project_gitlab_path": null,
    "scale_teams": [],
    "teams_uploads": []
  },
  {
    "id": 4647367,
    "name": "taeng's group",
    "url": "https://api.intra.42.fr/v2/teams/4647367",
    "final_mark": 100,
    "project_id": 1476,
    "created_at": "2023-01-29T09:34:30.673Z",
    "updated_at": "2023-01-29T09:36:10.666Z",
    "status": "finished",
    "terminating_at": null,
    "users": [
      {
        "id": 82562,
        "login": "taeng",
        "url": "https://api.intra.42.fr/v2/users/taeng",
        "leader": true,
        "occurrence": 0,
        "validated": true,
        "projects_user_id": 2953148
      }
    ],
    "locked?": true,
    "validated?": true,
    "closed?": true,
    "repo_url": "git@vogsphere.42seoul.kr:vogsphere/intra-uuid-51596b3e-acff-43e2-823d-ff7b0c3fc359-4647367-taeng",
    "repo_uuid": "intra-uuid-51596b3e-acff-43e2-823d-ff7b0c3fc359-4647367-taeng",
    "locked_at": "2023-01-29T09:34:30.702Z",
    "closed_at": "2023-01-29T09:35:40.603Z",
    "project_session_id": 6965,
    "project_gitlab_path": "pedago_world/42-cursus/inner-circle/fract-ol",
    "scale_teams": [],
    "teams_uploads": []
  },
  {
    "id": 4647368,
    "name": "taeng's group",
    "url": "https://api.intra.42.fr/v2/teams/4647368",
    "final_mark": 100,
    "project_id": 2004,
    "created_at": "2023-01-29T09:34:31.412Z",
    "updated_at": "2023-01-29T09:36:10.658Z",
    "status": "finished",
    "terminating_at": null,
    "users": [
      {
        "id": 82562,
        "login": "taeng",
        "url": "https://api.intra.42.fr/v2/users/taeng",
        "leader": true,
        "occurrence": 0,
        "validated": true,
        "projects_user_id": 2953149
      }
    ],
    "locked?": true,
    "validated?": true,
    "closed?": true,
    "repo_url": "git@vogsphere.42seoul.kr:vogsphere/intra-uuid-4dd1847e-5609-4d8b-9023-4dd2f647a38b-4647368-taeng",
    "repo_uuid": "intra-uuid-4dd1847e-5609-4d8b-9023-4dd2f647a38b-4647368-taeng",
    "locked_at": "2023-01-29T09:34:31.447Z",
    "closed_at": "2023-01-29T09:35:43.324Z",
    "project_session_id": 6962,
    "project_gitlab_path": "pedago_world/42-cursus/inner-circle/pipex",
    "scale_teams": [],
    "teams_uploads": []
  },
  {
    "id": 4647369,
    "name": "taeng's group",
    "url": "https://api.intra.42.fr/v2/teams/4647369",
    "final_mark": 100,
    "project_id": 1471,
    "created_at": "2023-01-29T09:34:32.821Z",
    "updated_at": "2023-01-29T09:36:10.740Z",
    "status": "finished",
    "terminating_at": null,
    "users": [
      {
        "id": 82562,
        "login": "taeng",
        "url": "https://api.intra.42.fr/v2/users/taeng",
        "leader": true,
        "occurrence": 0,
        "validated": true,
        "projects_user_id": 2953150
      }
    ],
    "locked?": true,
    "validated?": true,
    "closed?": true,
    "repo_url": "git@vogsphere.42seoul.kr:vogsphere/intra-uuid-f891aeb1-24b0-4901-9e29-19f27620feb8-4647369-taeng",
    "repo_uuid": "intra-uuid-f891aeb1-24b0-4901-9e29-19f27620feb8-4647369-taeng",
    "locked_at": "2023-01-29T09:34:32.853Z",
    "closed_at": "2023-01-29T09:35:45.865Z",
    "project_session_id": 5698,
    "project_gitlab_path": "pedago_world/42-cursus/inner-circle/push_swap",
    "scale_teams": [],
    "teams_uploads": []
  },
  {
    "id": 4647370,
    "name": "taeng's group",
    "url": "https://api.intra.42.fr/v2/teams/4647370",
    "final_mark": 100,
    "project_id": 1334,
    "created_at": "2023-01-29T09:36:04.456Z",
    "updated_at": "2023-01-29T09:37:40.019Z",
    "status": "finished",
    "terminating_at": null,
    "users": [
      {
        "id": 82562,
        "login": "taeng",
        "url": "https://api.intra.42.fr/v2/users/taeng",
        "leader": true,
        "occurrence": 0,
        "validated": true,
        "projects_user_id": 2953151
      }
    ],
    "locked?": true,
    "validated?": true,
    "closed?": true,
    "repo_url": "git@vogsphere.42seoul.kr:vogsphere/intra-uuid-ff6d01c3-eae4-4d79-9698-e9a10ae62bf7-4647370-taeng",
    "repo_uuid": "intra-uuid-ff6d01c3-eae4-4d79-9698-e9a10ae62bf7-4647370-taeng",
    "locked_at": "2023-01-29T09:36:04.480Z",
    "closed_at": "2023-01-29T09:36:15.274Z",
    "project_session_id": 5132,
    "project_gitlab_path": "pedago_world/42-cursus/inner-circle/philosophers",
    "scale_teams": [],
    "teams_uploads": []
  },
  {
    "id": 4647371,
    "name": "taeng's group",
    "url": "https://api.intra.42.fr/v2/teams/4647371",
    "final_mark": 100,
    "project_id": 1321,
    "created_at": "2023-01-29T09:36:06.293Z",
    "updated_at": "2023-01-29T09:36:24.023Z",
    "status": "finished",
    "terminating_at": null,
    "users": [
      {
        "id": 82562,
        "login": "taeng",
        "url": "https://api.intra.42.fr/v2/users/taeng",
        "leader": true,
        "occurrence": 0,
        "validated": true,
        "projects_user_id": 2953153
      }
    ],
    "locked?": true,
    "validated?": true,
    "closed?": true,
    "repo_url": null,
    "repo_uuid": "intra-uuid-36a8b3c7-b897-4edf-a834-ed6ab7b5876f-4647371-taeng",
    "locked_at": "2023-01-29T09:36:06.332Z",
    "closed_at": "2023-01-29T09:36:24.017Z",
    "project_session_id": 6967,
    "project_gitlab_path": null,
    "scale_teams": [],
    "teams_uploads": []
  },
  {
    "id": 4647373,
    "name": "taeng's team",
    "url": "https://api.intra.42.fr/v2/teams/4647373",
    "final_mark": 100,
    "project_id": 1331,
    "created_at": "2023-01-29T09:36:53.102Z",
    "updated_at": "2023-01-29T09:40:05.385Z",
    "status": "finished",
    "terminating_at": null,
    "users": [
      {
        "id": 82562,
        "login": "taeng",
        "url": "https://api.intra.42.fr/v2/users/taeng",
        "leader": true,
        "occurrence": 0,
        "validated": true,
        "projects_user_id": 2953152
      },
      {
        "id": 142051,
        "login": "agu-test",
        "url": "https://api.intra.42.fr/v2/users/agu-test",
        "leader": false,
        "occurrence": 0,
        "validated": true,
        "projects_user_id": 2953155
      }
    ],
    "locked?": true,
    "validated?": true,
    "closed?": true,
    "repo_url": "git@vogsphere.42seoul.kr:vogsphere/intra-uuid-00460d89-7060-4311-9e5b-1809cac94cc6-4647373-taeng",
    "repo_uuid": "intra-uuid-00460d89-7060-4311-9e5b-1809cac94cc6-4647373-taeng",
    "locked_at": "2023-01-29T09:38:22.219Z",
    "closed_at": "2023-01-29T09:38:31.904Z",
    "project_session_id": 5129,
    "project_gitlab_path": "pedago_world/42-cursus/inner-circle/minishell",
    "scale_teams": [],
    "teams_uploads": []
  },
  {
    "id": 4647375,
    "name": "taeng's group",
    "url": "https://api.intra.42.fr/v2/teams/4647375",
    "final_mark": 100,
    "project_id": 1322,
    "created_at": "2023-01-29T09:38:55.925Z",
    "updated_at": "2023-01-29T09:39:07.500Z",
    "status": "finished",
    "terminating_at": null,
    "users": [
      {
        "id": 82562,
        "login": "taeng",
        "url": "https://api.intra.42.fr/v2/users/taeng",
        "leader": true,
        "occurrence": 0,
        "validated": true,
        "projects_user_id": 2953156
      }
    ],
    "locked?": true,
    "validated?": true,
    "closed?": true,
    "repo_url": null,
    "repo_uuid": "intra-uuid-c0a5f4c0-e019-457d-9129-2b42cc18276c-4647375-taeng",
    "locked_at": "2023-01-29T09:38:55.951Z",
    "closed_at": "2023-01-29T09:39:07.497Z",
    "project_session_id": 6986,
    "project_gitlab_path": null,
    "scale_teams": [],
    "teams_uploads": []
  },
  {
    "id": 4647376,
    "name": "taeng's group",
    "url": "https://api.intra.42.fr/v2/teams/4647376",
    "final_mark": 100,
    "project_id": 1338,
    "created_at": "2023-01-29T09:38:57.850Z",
    "updated_at": "2023-01-29T09:40:27.032Z",
    "status": "finished",
    "terminating_at": null,
    "users": [
      {
        "id": 82562,
        "login": "taeng",
        "url": "https://api.intra.42.fr/v2/users/taeng",
        "leader": true,
        "occurrence": 0,
        "validated": true,
        "projects_user_id": 2953158
      }
    ],
    "locked?": true,
    "validated?": true,
    "closed?": true,
    "repo_url": "git@vogsphere.42seoul.kr:vogsphere/intra-uuid-9f466cfa-69fc-4c6e-a925-349d6a7ff68c-4647376-taeng",
    "repo_uuid": "intra-uuid-9f466cfa-69fc-4c6e-a925-349d6a7ff68c-4647376-taeng",
    "locked_at": "2023-01-29T09:38:57.880Z",
    "closed_at": "2023-01-29T09:39:18.709Z",
    "project_session_id": 5133,
    "project_gitlab_path": "pedago_world/42-cursus/inner-circle/cpp-00",
    "scale_teams": [],
    "teams_uploads": []
  },
  {
    "id": 4647378,
    "name": "taeng's group",
    "url": "https://api.intra.42.fr/v2/teams/4647378",
    "final_mark": 100,
    "project_id": 1339,
    "created_at": "2023-01-29T09:39:46.591Z",
    "updated_at": "2023-01-29T09:42:10.415Z",
    "status": "finished",
    "terminating_at": null,
    "users": [
      {
        "id": 82562,
        "login": "taeng",
        "url": "https://api.intra.42.fr/v2/users/taeng",
        "leader": true,
        "occurrence": 0,
        "validated": true,
        "projects_user_id": 2953159
      }
    ],
    "locked?": true,
    "validated?": true,
    "closed?": true,
    "repo_url": "git@vogsphere.42seoul.kr:vogsphere/intra-uuid-24df671f-f1b7-4e3f-acaf-b13743b62bc7-4647378-taeng",
    "repo_uuid": "intra-uuid-24df671f-f1b7-4e3f-acaf-b13743b62bc7-4647378-taeng",
    "locked_at": "2023-01-29T09:39:46.621Z",
    "closed_at": "2023-01-29T09:39:51.827Z",
    "project_session_id": 5134,
    "project_gitlab_path": "pedago_world/42-cursus/inner-circle/cpp-01",
    "scale_teams": [],
    "teams_uploads": []
  },
  {
    "id": 4647379,
    "name": "taeng's group",
    "url": "https://api.intra.42.fr/v2/teams/4647379",
    "final_mark": 100,
    "project_id": 1340,
    "created_at": "2023-01-29T09:39:54.921Z",
    "updated_at": "2023-01-29T09:42:10.346Z",
    "status": "finished",
    "terminating_at": null,
    "users": [
      {
        "id": 82562,
        "login": "taeng",
        "url": "https://api.intra.42.fr/v2/users/taeng",
        "leader": true,
        "occurrence": 0,
        "validated": true,
        "projects_user_id": 2953160
      }
    ],
    "locked?": true,
    "validated?": true,
    "closed?": true,
    "repo_url": "git@vogsphere.42seoul.kr:vogsphere/intra-uuid-b9c23501-b977-4c83-921e-3b4207c3872d-4647379-taeng",
    "repo_uuid": "intra-uuid-b9c23501-b977-4c83-921e-3b4207c3872d-4647379-taeng",
    "locked_at": "2023-01-29T09:39:54.956Z",
    "closed_at": "2023-01-29T09:39:59.294Z",
    "project_session_id": 5139,
    "project_gitlab_path": "pedago_world/42-cursus/inner-circle/cpp-02",
    "scale_teams": [],
    "teams_uploads": []
  },
  {
    "id": 4647380,
    "name": "taeng's group",
    "url": "https://api.intra.42.fr/v2/teams/4647380",
    "final_mark": 100,
    "project_id": 1341,
    "created_at": "2023-01-29T09:40:06.945Z",
    "updated_at": "2023-01-29T09:42:10.347Z",
    "status": "finished",
    "terminating_at": null,
    "users": [
      {
        "id": 82562,
        "login": "taeng",
        "url": "https://api.intra.42.fr/v2/users/taeng",
        "leader": true,
        "occurrence": 0,
        "validated": true,
        "projects_user_id": 2953161
      }
    ],
    "locked?": true,
    "validated?": true,
    "closed?": true,
    "repo_url": "git@vogsphere.42seoul.kr:vogsphere/intra-uuid-bc606da6-8682-441f-b21a-e51517ac26fe-4647380-taeng",
    "repo_uuid": "intra-uuid-bc606da6-8682-441f-b21a-e51517ac26fe-4647380-taeng",
    "locked_at": "2023-01-29T09:40:06.976Z",
    "closed_at": "2023-01-29T09:40:11.446Z",
    "project_session_id": 5140,
    "project_gitlab_path": "pedago_world/42-cursus/inner-circle/cpp-03",
    "scale_teams": [],
    "teams_uploads": []
  },
  {
    "id": 4647381,
    "name": "taeng's group",
    "url": "https://api.intra.42.fr/v2/teams/4647381",
    "final_mark": 100,
    "project_id": 1342,
    "created_at": "2023-01-29T09:40:15.022Z",
    "updated_at": "2023-01-29T09:42:10.311Z",
    "status": "finished",
    "terminating_at": null,
    "users": [
      {
        "id": 82562,
        "login": "taeng",
        "url": "https://api.intra.42.fr/v2/users/taeng",
        "leader": true,
        "occurrence": 0,
        "validated": true,
        "projects_user_id": 2953162
      }
    ],
    "locked?": true,
    "validated?": true,
    "closed?": true,
    "repo_url": "git@vogsphere.42seoul.kr:vogsphere/intra-uuid-502a3375-32f3-4aba-8567-0406122d74dd-4647381-taeng",
    "repo_uuid": "intra-uuid-502a3375-32f3-4aba-8567-0406122d74dd-4647381-taeng",
    "locked_at": "2023-01-29T09:40:15.048Z",
    "closed_at": "2023-01-29T09:40:19.941Z",
    "project_session_id": 5141,
    "project_gitlab_path": "pedago_world/42-cursus/inner-circle/cpp-04",
    "scale_teams": [],
    "teams_uploads": []
  },
  {
    "id": 4647382,
    "name": "taeng's group",
    "url": "https://api.intra.42.fr/v2/teams/4647382",
    "final_mark": 100,
    "project_id": 1343,
    "created_at": "2023-01-29T09:40:23.261Z",
    "updated_at": "2023-01-29T09:42:10.381Z",
    "status": "finished",
    "terminating_at": null,
    "users": [
      {
        "id": 82562,
        "login": "taeng",
        "url": "https://api.intra.42.fr/v2/users/taeng",
        "leader": true,
        "occurrence": 0,
        "validated": true,
        "projects_user_id": 2953163
      }
    ],
    "locked?": true,
    "validated?": true,
    "closed?": true,
    "repo_url": "git@vogsphere.42seoul.kr:vogsphere/intra-uuid-ea8397a6-3797-4a50-861f-2a39acdbd7c6-4647382-taeng",
    "repo_uuid": "intra-uuid-ea8397a6-3797-4a50-861f-2a39acdbd7c6-4647382-taeng",
    "locked_at": "2023-01-29T09:40:23.288Z",
    "closed_at": "2023-01-29T09:40:28.115Z",
    "project_session_id": 5142,
    "project_gitlab_path": "pedago_world/42-cursus/inner-circle/cpp-05",
    "scale_teams": [],
    "teams_uploads": []
  },
  {
    "id": 4647383,
    "name": "taeng's group",
    "url": "https://api.intra.42.fr/v2/teams/4647383",
    "final_mark": 100,
    "project_id": 1344,
    "created_at": "2023-01-29T09:40:32.772Z",
    "updated_at": "2023-01-29T09:42:33.578Z",
    "status": "finished",
    "terminating_at": null,
    "users": [
      {
        "id": 82562,
        "login": "taeng",
        "url": "https://api.intra.42.fr/v2/users/taeng",
        "leader": true,
        "occurrence": 0,
        "validated": true,
        "projects_user_id": 2953164
      }
    ],
    "locked?": true,
    "validated?": true,
    "closed?": true,
    "repo_url": "git@vogsphere.42seoul.kr:vogsphere/intra-uuid-1c63da3d-592d-4ef4-8a25-a9320b26057f-4647383-taeng",
    "repo_uuid": "intra-uuid-1c63da3d-592d-4ef4-8a25-a9320b26057f-4647383-taeng",
    "locked_at": "2023-01-29T09:40:32.805Z",
    "closed_at": "2023-01-29T09:40:37.795Z",
    "project_session_id": 5143,
    "project_gitlab_path": "pedago_world/42-cursus/inner-circle/cpp-06",
    "scale_teams": [],
    "teams_uploads": []
  },
  {
    "id": 4647384,
    "name": "taeng's group",
    "url": "https://api.intra.42.fr/v2/teams/4647384",
    "final_mark": 100,
    "project_id": 1345,
    "created_at": "2023-01-29T09:40:41.922Z",
    "updated_at": "2023-01-29T09:42:33.588Z",
    "status": "finished",
    "terminating_at": null,
    "users": [
      {
        "id": 82562,
        "login": "taeng",
        "url": "https://api.intra.42.fr/v2/users/taeng",
        "leader": true,
        "occurrence": 0,
        "validated": true,
        "projects_user_id": 2953165
      }
    ],
    "locked?": true,
    "validated?": true,
    "closed?": true,
    "repo_url": "git@vogsphere.42seoul.kr:vogsphere/intra-uuid-282ee912-56f5-4079-9077-ec65d580a072-4647384-taeng",
    "repo_uuid": "intra-uuid-282ee912-56f5-4079-9077-ec65d580a072-4647384-taeng",
    "locked_at": "2023-01-29T09:40:41.961Z",
    "closed_at": "2023-01-29T09:40:48.435Z",
    "project_session_id": 5144,
    "project_gitlab_path": "pedago_world/42-cursus/inner-circle/cpp-07",
    "scale_teams": [],
    "teams_uploads": []
  },
  {
    "id": 4647385,
    "name": "taeng's group",
    "url": "https://api.intra.42.fr/v2/teams/4647385",
    "final_mark": 100,
    "project_id": 1346,
    "created_at": "2023-01-29T09:40:56.999Z",
    "updated_at": "2023-01-29T09:42:33.583Z",
    "status": "finished",
    "terminating_at": null,
    "users": [
      {
        "id": 82562,
        "login": "taeng",
        "url": "https://api.intra.42.fr/v2/users/taeng",
        "leader": true,
        "occurrence": 0,
        "validated": true,
        "projects_user_id": 2953166
      }
    ],
    "locked?": true,
    "validated?": true,
    "closed?": true,
    "repo_url": "git@vogsphere.42seoul.kr:vogsphere/intra-uuid-0aae8a29-ff4c-44b8-8fa1-d67ccf9e56bc-4647385-taeng",
    "repo_uuid": "intra-uuid-0aae8a29-ff4c-44b8-8fa1-d67ccf9e56bc-4647385-taeng",
    "locked_at": "2023-01-29T09:40:57.037Z",
    "closed_at": "2023-01-29T09:41:02.540Z",
    "project_session_id": 5145,
    "project_gitlab_path": "pedago_world/42-cursus/inner-circle/cpp-08",
    "scale_teams": [],
    "teams_uploads": []
  },
  {
    "id": 4647386,
    "name": "taeng's team",
    "url": "https://api.intra.42.fr/v2/teams/4647386",
    "final_mark": 100,
    "project_id": 1315,
    "created_at": "2023-01-29T09:41:31.965Z",
    "updated_at": "2023-01-29T09:43:44.593Z",
    "status": "finished",
    "terminating_at": null,
    "users": [
      {
        "id": 82562,
        "login": "taeng",
        "url": "https://api.intra.42.fr/v2/users/taeng",
        "leader": true,
        "occurrence": 0,
        "validated": true,
        "projects_user_id": 2953157
      },
      {
        "id": 142051,
        "login": "agu-test",
        "url": "https://api.intra.42.fr/v2/users/agu-test",
        "leader": false,
        "occurrence": 0,
        "validated": true,
        "projects_user_id": 2953167
      }
    ],
    "locked?": true,
    "validated?": true,
    "closed?": true,
    "repo_url": "git@vogsphere.42seoul.kr:vogsphere/intra-uuid-c550110a-1c8f-4f07-b9b4-da5fbf781aae-4647386-taeng",
    "repo_uuid": "intra-uuid-c550110a-1c8f-4f07-b9b4-da5fbf781aae-4647386-taeng",
    "locked_at": "2023-01-29T09:42:00.679Z",
    "closed_at": "2023-01-29T09:42:07.938Z",
    "project_session_id": 5656,
    "project_gitlab_path": "pedago_world/42-cursus/inner-circle/minirt",
    "scale_teams": [],
    "teams_uploads": []
  },
  {
    "id": 4647388,
    "name": "taeng's group",
    "url": "https://api.intra.42.fr/v2/teams/4647388",
    "final_mark": 100,
    "project_id": 2007,
    "created_at": "2023-01-29T09:42:40.524Z",
    "updated_at": "2023-01-29T09:44:17.264Z",
    "status": "finished",
    "terminating_at": null,
    "users": [
      {
        "id": 82562,
        "login": "taeng",
        "url": "https://api.intra.42.fr/v2/users/taeng",
        "leader": true,
        "occurrence": 0,
        "validated": true,
        "projects_user_id": 2953170
      }
    ],
    "locked?": true,
    "validated?": true,
    "closed?": true,
    "repo_url": "git@vogsphere.42seoul.kr:vogsphere/intra-uuid-1d2a5876-94df-4e4b-a0cb-60f104f3b871-4647388-taeng",
    "repo_uuid": "intra-uuid-1d2a5876-94df-4e4b-a0cb-60f104f3b871-4647388-taeng",
    "locked_at": "2023-01-29T09:42:40.563Z",
    "closed_at": "2023-01-29T09:42:54.479Z",
    "project_session_id": 6966,
    "project_gitlab_path": "pedago_world/42-cursus/inner-circle/net_practice",
    "scale_teams": [],
    "teams_uploads": []
  },
  {
    "id": 4647389,
    "name": "taeng's group",
    "url": "https://api.intra.42.fr/v2/teams/4647389",
    "final_mark": 100,
    "project_id": 1335,
    "created_at": "2023-01-29T09:43:13.009Z",
    "updated_at": "2023-01-29T09:44:38.503Z",
    "status": "finished",
    "terminating_at": null,
    "users": [
      {
        "id": 82562,
        "login": "taeng",
        "url": "https://api.intra.42.fr/v2/users/taeng",
        "leader": true,
        "occurrence": 0,
        "validated": true,
        "projects_user_id": 2953171
      }
    ],
    "locked?": true,
    "validated?": true,
    "closed?": true,
    "repo_url": "git@vogsphere.42seoul.kr:vogsphere/intra-uuid-cd9d59a8-93e3-4811-bf94-579b1e88f288-4647389-taeng",
    "repo_uuid": "intra-uuid-cd9d59a8-93e3-4811-bf94-579b1e88f288-4647389-taeng",
    "locked_at": "2023-01-29T09:43:13.057Z",
    "closed_at": "2023-01-29T09:43:30.926Z",
    "project_session_id": 5135,
    "project_gitlab_path": "pedago_world/42-cursus/inner-circle/ft_containers",
    "scale_teams": [],
    "teams_uploads": []
  },
  {
    "id": 4647390,
    "name": "taeng's group",
    "url": "https://api.intra.42.fr/v2/teams/4647390",
    "final_mark": 100,
    "project_id": 1983,
    "created_at": "2023-01-29T09:43:13.669Z",
    "updated_at": "2023-01-29T09:44:38.522Z",
    "status": "finished",
    "terminating_at": null,
    "users": [
      {
        "id": 82562,
        "login": "taeng",
        "url": "https://api.intra.42.fr/v2/users/taeng",
        "leader": true,
        "occurrence": 0,
        "validated": true,
        "projects_user_id": 2953172
      }
    ],
    "locked?": true,
    "validated?": true,
    "closed?": true,
    "repo_url": "git@vogsphere.42seoul.kr:vogsphere/intra-uuid-148bb57d-b36e-466e-8f54-988badcc250c-4647390-taeng",
    "repo_uuid": "intra-uuid-148bb57d-b36e-466e-8f54-988badcc250c-4647390-taeng",
    "locked_at": "2023-01-29T09:43:13.697Z",
    "closed_at": "2023-01-29T09:43:24.947Z",
    "project_session_id": 6011,
    "project_gitlab_path": "pedago_world/42-cursus/inner-circle/inception",
    "scale_teams": [],
    "teams_uploads": []
  },
  {
    "id": 4647391,
    "name": "taeng's group",
    "url": "https://api.intra.42.fr/v2/teams/4647391",
    "final_mark": 100,
    "project_id": 1323,
    "created_at": "2023-01-29T09:43:14.700Z",
    "updated_at": "2023-01-29T09:43:20.168Z",
    "status": "finished",
    "terminating_at": null,
    "users": [
      {
        "id": 82562,
        "login": "taeng",
        "url": "https://api.intra.42.fr/v2/users/taeng",
        "leader": true,
        "occurrence": 0,
        "validated": true,
        "projects_user_id": 2953173
      }
    ],
    "locked?": true,
    "validated?": true,
    "closed?": true,
    "repo_url": null,
    "repo_uuid": "intra-uuid-59a52279-3b8e-4aa1-b8d3-ac116ab94f05-4647391-taeng",
    "locked_at": "2023-01-29T09:43:14.747Z",
    "closed_at": "2023-01-29T09:43:20.165Z",
    "project_session_id": 6987,
    "project_gitlab_path": null,
    "scale_teams": [],
    "teams_uploads": []
  },
  {
    "id": 4647392,
    "name": "taeng's team",
    "url": "https://api.intra.42.fr/v2/teams/4647392",
    "final_mark": 100,
    "project_id": 1332,
    "created_at": "2023-01-29T09:43:39.191Z",
    "updated_at": "2023-01-29T09:45:45.184Z",
    "status": "finished",
    "terminating_at": null,
    "users": [
      {
        "id": 82562,
        "login": "taeng",
        "url": "https://api.intra.42.fr/v2/users/taeng",
        "leader": true,
        "occurrence": 0,
        "validated": true,
        "projects_user_id": 2953174
      },
      {
        "id": 142051,
        "login": "agu-test",
        "url": "https://api.intra.42.fr/v2/users/agu-test",
        "leader": false,
        "occurrence": 0,
        "validated": true,
        "projects_user_id": 2953175
      }
    ],
    "locked?": true,
    "validated?": true,
    "closed?": true,
    "repo_url": "git@vogsphere.42seoul.kr:vogsphere/intra-uuid-085d3ef5-9eaf-4df9-afaf-da0e9fcb3529-4647392-taeng",
    "repo_uuid": "intra-uuid-085d3ef5-9eaf-4df9-afaf-da0e9fcb3529-4647392-taeng",
    "locked_at": "2023-01-29T09:44:24.742Z",
    "closed_at": "2023-01-29T09:44:33.373Z",
    "project_session_id": 5136,
    "project_gitlab_path": "pedago_world/42-cursus/inner-circle/webserv",
    "scale_teams": [],
    "teams_uploads": []
  },
  {
    "id": 4647394,
    "name": "taeng's group",
    "url": "https://api.intra.42.fr/v2/teams/4647394",
    "final_mark": 100,
    "project_id": 1324,
    "created_at": "2023-01-29T09:45:08.407Z",
    "updated_at": "2023-01-29T09:45:15.680Z",
    "status": "finished",
    "terminating_at": null,
    "users": [
      {
        "id": 82562,
        "login": "taeng",
        "url": "https://api.intra.42.fr/v2/users/taeng",
        "leader": true,
        "occurrence": 0,
        "validated": true,
        "projects_user_id": 2953179
      }
    ],
    "locked?": true,
    "validated?": true,
    "closed?": true,
    "repo_url": null,
    "repo_uuid": "intra-uuid-3b0d230e-3527-459a-8576-3a239b6bbfb0-4647394-taeng",
    "locked_at": "2023-01-29T09:45:08.445Z",
    "closed_at": "2023-01-29T09:45:15.673Z",
    "project_session_id": 6988,
    "project_gitlab_path": null,
    "scale_teams": [],
    "teams_uploads": []
  },
  {
    "id": 4647395,
    "name": "taeng's team",
    "url": "https://api.intra.42.fr/v2/teams/4647395",
    "final_mark": 100,
    "project_id": 1337,
    "created_at": "2023-01-29T09:45:20.596Z",
    "updated_at": "2023-01-29T09:45:50.988Z",
    "status": "finished",
    "terminating_at": null,
    "users": [
      {
        "id": 82562,
        "login": "taeng",
        "url": "https://api.intra.42.fr/v2/users/taeng",
        "leader": true,
        "occurrence": 0,
        "validated": true,
        "projects_user_id": 2953178
      },
      {
        "id": 142051,
        "login": "agu-test",
        "url": "https://api.intra.42.fr/v2/users/agu-test",
        "leader": false,
        "occurrence": 0,
        "validated": true,
        "projects_user_id": 2953177
      }
    ],
    "locked?": false,
    "validated?": true,
    "closed?": true,
    "repo_url": null,
    "repo_uuid": "intra-uuid-f1767c8b-4170-43f3-81a6-71ed6f4e566a-4647395-taeng",
    "locked_at": null,
    "closed_at": "2023-01-29T09:45:50.984Z",
    "project_session_id": 5138,
    "project_gitlab_path": "pedago_world/42-cursus/inner-circle/ft_transcendence",
    "scale_teams": [],
    "teams_uploads": []
  },
  {
    "id": 4651747,
    "name": "taeng's group",
    "url": "https://api.intra.42.fr/v2/teams/4651747",
    "final_mark": null,
    "project_id": 1984,
    "created_at": "2023-01-31T06:20:35.376Z",
    "updated_at": "2023-01-31T06:22:32.105Z",
    "status": "in_progress",
    "terminating_at": null,
    "users": [
      {
        "id": 82562,
        "login": "taeng",
        "url": "https://api.intra.42.fr/v2/users/taeng",
        "leader": true,
        "occurrence": 0,
        "validated": true,
        "projects_user_id": 2956368
      }
    ],
    "locked?": true,
    "validated?": null,
    "closed?": false,
    "repo_url": "git@vogsphere.42seoul.kr:vogsphere/intra-uuid-58a31574-11f3-4c7b-b954-1d61fc5b64ba-4651747-taeng",
    "repo_uuid": "intra-uuid-58a31574-11f3-4c7b-b954-1d61fc5b64ba-4651747-taeng",
    "locked_at": "2023-01-31T06:20:35.404Z",
    "closed_at": null,
    "project_session_id": 5822,
    "project_gitlab_path": "pedago_world/42-cursus/outer-circle/python-module-00",
    "scale_teams": [],
    "teams_uploads": []
  },
  {
    "id": 4757091,
    "name": "taeng's group",
    "url": "https://api.intra.42.fr/v2/teams/4757091",
    "final_mark": 0,
    "project_id": 1330,
    "created_at": "2023-03-15T08:19:06.888Z",
    "updated_at": "2024-01-14T14:08:59.311Z",
    "status": "finished",
    "terminating_at": null,
    "users": [
      {
        "id": 82562,
        "login": "taeng",
        "url": "https://api.intra.42.fr/v2/users/taeng",
        "leader": true,
        "occurrence": 0,
        "validated": true,
        "projects_user_id": 3028842
      }
    ],
    "locked?": true,
    "validated?": false,
    "closed?": true,
    "repo_url": "git@vogsphere.42seoul.kr:vogsphere/intra-uuid-77a1f3e9-29fe-4905-a21c-52ded73b5c01-4757091-taeng",
    "repo_uuid": "intra-uuid-77a1f3e9-29fe-4905-a21c-52ded73b5c01-4757091-taeng",
    "locked_at": "2023-03-15T08:19:06.927Z",
    "closed_at": "2024-01-14T14:04:53.611Z",
    "project_session_id": 5131,
    "project_gitlab_path": "pedago_world/42-cursus/inner-circle/libasm",
    "scale_teams": [],
    "teams_uploads": []
  },
  {
    "id": 4816920,
    "name": "taeng's group",
    "url": "https://api.intra.42.fr/v2/teams/4816920",
    "final_mark": 0,
    "project_id": 1415,
    "created_at": "2023-04-21T01:53:11.504Z",
    "updated_at": "2024-01-14T14:03:10.745Z",
    "status": "finished",
    "terminating_at": null,
    "users": [
      {
        "id": 82562,
        "login": "taeng",
        "url": "https://api.intra.42.fr/v2/users/taeng",
        "leader": true,
        "occurrence": 0,
        "validated": true,
        "projects_user_id": 3072403
      }
    ],
    "locked?": true,
    "validated?": false,
    "closed?": true,
    "repo_url": "git@vogsphere.42seoul.kr:vogsphere/intra-uuid-78c507ab-d2d1-46f5-bb67-04823ff477eb-4816920-taeng",
    "repo_uuid": "intra-uuid-78c507ab-d2d1-46f5-bb67-04823ff477eb-4816920-taeng",
    "locked_at": "2023-04-21T01:53:11.543Z",
    "closed_at": "2023-04-21T01:55:34.881Z",
    "project_session_id": 8249,
    "project_gitlab_path": "pedago_world/42-cursus/outer-circle/ft_linux",
    "scale_teams": [],
    "teams_uploads": []
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
    "project_gitlab_path": "pedago_world/42-cursus/inner-circle/libft",
    "scale_teams": [],
    "teams_uploads": []
  },
  {
    "id": 5217798,
    "name": "lchauvin's team",
    "url": "https://api.intra.42.fr/v2/teams/5217798",
    "final_mark": 0,
    "project_id": 1314,
    "created_at": "2023-09-29T09:19:21.475Z",
    "updated_at": "2023-12-16T09:02:42.666Z",
    "status": "finished",
    "terminating_at": null,
    "users": [
      {
        "id": 167614,
        "login": "lchauvin",
        "url": "https://api.intra.42.fr/v2/users/lchauvin",
        "leader": true,
        "occurrence": 0,
        "validated": true,
        "projects_user_id": 3341687
      }
    ],
    "locked?": true,
    "validated?": false,
    "closed?": true,
    "repo_url": "git@vogsphere.42campus.org:vogsphere/intra-uuid-ecf33709-90d0-47f2-b281-af7af0e16ab5-5217798-lchauvin",
    "repo_uuid": "intra-uuid-ecf33709-90d0-47f2-b281-af7af0e16ab5-5217798-lchauvin",
    "locked_at": "2023-09-29T09:19:21.518Z",
    "closed_at": "2023-12-16T09:02:42.152Z",
    "project_session_id": 3300,
    "project_gitlab_path": "pedago_world/42-cursus/inner-circle/libft",
    "scale_teams": [],
    "teams_uploads": []
  },
  {
    "id": 5248765,
    "name": "lchauvin's group",
    "url": "https://api.intra.42.fr/v2/teams/5248765",
    "final_mark": 0,
    "project_id": 2520,
    "created_at": "2023-10-16T13:17:13.916Z",
    "updated_at": "2023-11-01T10:17:26.051Z",
    "status": "finished",
    "terminating_at": "2023-11-01T10:17:21.664Z",
    "users": [
      {
        "id": 167614,
        "login": "lchauvin",
        "url": "https://api.intra.42.fr/v2/users/lchauvin",
        "leader": true,
        "occurrence": 3,
        "validated": true,
        "projects_user_id": 3361927
      },
      {
        "id": 168353,
        "login": "krandom5",
        "url": "https://api.intra.42.fr/v2/users/krandom5",
        "leader": false,
        "occurrence": 7,
        "validated": true,
        "projects_user_id": 3356127
      }
    ],
    "locked?": true,
    "validated?": false,
    "closed?": true,
    "repo_url": "git@vogsphere.42campus.org:vogsphere/intra-uuid-64aca373-1ada-4c3e-88b4-8eccf7486235-5248765-lchauvin",
    "repo_uuid": "intra-uuid-64aca373-1ada-4c3e-88b4-8eccf7486235-5248765-lchauvin",
    "locked_at": "2023-10-16T13:17:14.048Z",
    "closed_at": "2023-10-17T09:17:21.664Z",
    "project_session_id": 10973,
    "project_gitlab_path": "pedago_world/42-cursus/outer-circle/microsoft_projects/42_active_discovery",
    "scale_teams": [
      {
        "id": 6031642,
        "scale_id": 28874,
        "comment": "Missing peer-evaluation replaced by a 0",
        "created_at": "2023-11-01T10:17:25.640Z",
        "updated_at": "2023-11-01T10:17:25.844Z",
        "feedback": "The Intranet is right ! I deserve this 0 !",
        "final_mark": 0,
        "flag": {
          "id": 1,
          "name": "Ok",
          "positive": true,
          "icon": "check-4",
          "created_at": "2015-09-14T23:06:52.000Z",
          "updated_at": "2015-09-14T23:06:52.000Z"
        },
        "begin_at": null,
        "correcteds": "invisible",
        "corrector": "invisible",
        "truant": {},
        "filled_at": "2023-11-01T10:17:25.778Z",
        "questions_with_answers": []
      },
      {
        "id": 6031641,
        "scale_id": 28874,
        "comment": "Missing peer-evaluation replaced by a 0",
        "created_at": "2023-11-01T10:17:25.369Z",
        "updated_at": "2023-11-01T10:17:25.597Z",
        "feedback": "The Intranet is right ! I deserve this 0 !",
        "final_mark": 0,
        "flag": {
          "id": 1,
          "name": "Ok",
          "positive": true,
          "icon": "check-4",
          "created_at": "2015-09-14T23:06:52.000Z",
          "updated_at": "2015-09-14T23:06:52.000Z"
        },
        "begin_at": null,
        "correcteds": "invisible",
        "corrector": "invisible",
        "truant": {},
        "filled_at": "2023-11-01T10:17:25.514Z",
        "questions_with_answers": []
      },
      {
        "id": 6031640,
        "scale_id": 28874,
        "comment": "Missing peer-evaluation replaced by a 0",
        "created_at": "2023-11-01T10:17:25.086Z",
        "updated_at": "2023-11-01T10:17:25.326Z",
        "feedback": "The Intranet is right ! I deserve this 0 !",
        "final_mark": 0,
        "flag": {
          "id": 1,
          "name": "Ok",
          "positive": true,
          "icon": "check-4",
          "created_at": "2015-09-14T23:06:52.000Z",
          "updated_at": "2015-09-14T23:06:52.000Z"
        },
        "begin_at": null,
        "correcteds": "invisible",
        "corrector": "invisible",
        "truant": {},
        "filled_at": "2023-11-01T10:17:25.258Z",
        "questions_with_answers": []
      }
    ],
    "teams_uploads": []
  },
  {
    "id": 5250082,
    "name": "taeng's group",
    "url": "https://api.intra.42.fr/v2/teams/5250082",
    "final_mark": null,
    "project_id": 1638,
    "created_at": "2023-10-17T03:54:49.719Z",
    "updated_at": "2023-10-17T03:54:49.792Z",
    "status": "in_progress",
    "terminating_at": null,
    "users": [
      {
        "id": 82562,
        "login": "taeng",
        "url": "https://api.intra.42.fr/v2/users/taeng",
        "leader": true,
        "occurrence": 0,
        "validated": true,
        "projects_user_id": 3364088
      }
    ],
    "locked?": true,
    "validated?": null,
    "closed?": false,
    "repo_url": null,
    "repo_uuid": "intra-uuid-79a0cbbc-0f15-4642-8a55-501170e9a944-5250082-taeng",
    "locked_at": "2023-10-17T03:54:49.751Z",
    "closed_at": null,
    "project_session_id": 5127,
    "project_gitlab_path": "pedago_world/42-cursus/outer-circle/internship-i/internship-i-main-project",
    "scale_teams": [],
    "teams_uploads": []
  },
  {
    "id": 5250085,
    "name": "taeng's group",
    "url": "https://api.intra.42.fr/v2/teams/5250085",
    "final_mark": 100,
    "project_id": 1640,
    "created_at": "2023-10-17T03:58:55.136Z",
    "updated_at": "2023-10-17T03:58:55.418Z",
    "status": "finished",
    "terminating_at": null,
    "users": [
      {
        "id": 82562,
        "login": "taeng",
        "url": "https://api.intra.42.fr/v2/users/taeng",
        "leader": true,
        "occurrence": 0,
        "validated": true,
        "projects_user_id": 3364090
      }
    ],
    "locked?": true,
    "validated?": true,
    "closed?": true,
    "repo_url": null,
    "repo_uuid": "intra-uuid-88d864ff-f4d7-43c2-871a-617522b262c4-5250085-taeng",
    "locked_at": "2023-10-17T03:58:55.240Z",
    "closed_at": "2023-10-17T03:58:55.304Z",
    "project_session_id": 3951,
    "project_gitlab_path": "pedago_world/42-cursus/outer-circle/internship-i/internship-i-contract-upload",
    "scale_teams": [],
    "teams_uploads": []
  },
  {
    "id": 5250086,
    "name": "taeng's group",
    "url": "https://api.intra.42.fr/v2/teams/5250086",
    "final_mark": 107,
    "project_id": 1639,
    "created_at": "2023-10-17T03:58:55.678Z",
    "updated_at": "2023-10-17T03:58:55.957Z",
    "status": "finished",
    "terminating_at": null,
    "users": [
      {
        "id": 82562,
        "login": "taeng",
        "url": "https://api.intra.42.fr/v2/users/taeng",
        "leader": true,
        "occurrence": 0,
        "validated": true,
        "projects_user_id": 3364091
      }
    ],
    "locked?": true,
    "validated?": true,
    "closed?": true,
    "repo_url": null,
    "repo_uuid": "intra-uuid-ecef577c-c880-4e46-9e65-ede26f5901b5-5250086-taeng",
    "locked_at": "2023-10-17T03:58:55.781Z",
    "closed_at": "2023-10-17T03:58:55.843Z",
    "project_session_id": 3950,
    "project_gitlab_path": "pedago_world/42-cursus/outer-circle/internship-i/internship-i-duration",
    "scale_teams": [],
    "teams_uploads": []
  },
  {
    "id": 5250087,
    "name": "taeng's group",
    "url": "https://api.intra.42.fr/v2/teams/5250087",
    "final_mark": null,
    "project_id": 1641,
    "created_at": "2023-10-17T03:58:56.170Z",
    "updated_at": "2024-02-07T21:34:05.299Z",
    "status": "waiting_for_correction",
    "terminating_at": null,
    "users": [
      {
        "id": 82562,
        "login": "taeng",
        "url": "https://api.intra.42.fr/v2/users/taeng",
        "leader": true,
        "occurrence": 0,
        "validated": true,
        "projects_user_id": 3364092
      }
    ],
    "locked?": true,
    "validated?": null,
    "closed?": true,
    "repo_url": null,
    "repo_uuid": "intra-uuid-a05c9f06-0943-4756-aab3-4e21156afb81-5250087-taeng",
    "locked_at": "2023-10-17T03:58:56.264Z",
    "closed_at": "2023-10-17T03:58:56.331Z",
    "project_session_id": 3952,
    "project_gitlab_path": "pedago_world/42-cursus/outer-circle/internship-i/internship-i-company-mid-evaluation",
    "scale_teams": [
      {
        "id": 5984514,
        "scale_id": 23194,
        "comment": null,
        "created_at": "2023-10-17T03:58:56.528Z",
        "updated_at": "2023-10-17T03:58:56.528Z",
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
        "begin_at": "2023-12-25T09:00:00.000Z",
        "correcteds": [
          {
            "id": 82562,
            "login": "taeng",
            "url": "https://api.intra.42.fr/v2/users/taeng"
          }
        ],
        "corrector": {
          "id": 51965,
          "login": "supervisor",
          "url": "https://api.intra.42.fr/v2/users/supervisor"
        },
        "truant": {},
        "filled_at": null,
        "questions_with_answers": []
      }
    ],
    "teams_uploads": []
  },
  {
    "id": 5250088,
    "name": "taeng's group",
    "url": "https://api.intra.42.fr/v2/teams/5250088",
    "final_mark": null,
    "project_id": 1642,
    "created_at": "2023-10-17T03:58:56.658Z",
    "updated_at": "2023-10-17T03:58:56.974Z",
    "status": "waiting_for_correction",
    "terminating_at": null,
    "users": [
      {
        "id": 82562,
        "login": "taeng",
        "url": "https://api.intra.42.fr/v2/users/taeng",
        "leader": true,
        "occurrence": 0,
        "validated": true,
        "projects_user_id": 3364093
      }
    ],
    "locked?": true,
    "validated?": null,
    "closed?": true,
    "repo_url": null,
    "repo_uuid": "intra-uuid-3f1680db-7dd4-4dd3-adeb-c96bf7880c3e-5250088-taeng",
    "locked_at": "2023-10-17T03:58:56.739Z",
    "closed_at": "2023-10-17T03:58:56.809Z",
    "project_session_id": 3953,
    "project_gitlab_path": "pedago_world/42-cursus/outer-circle/internship-i/internship-i-company-final-evaluation",
    "scale_teams": [
      {
        "id": 5984515,
        "scale_id": 23195,
        "comment": null,
        "created_at": "2023-10-17T03:58:56.952Z",
        "updated_at": "2023-10-17T03:58:56.952Z",
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
        "begin_at": "2024-03-04T09:00:00.000Z",
        "correcteds": [
          {
            "id": 82562,
            "login": "taeng",
            "url": "https://api.intra.42.fr/v2/users/taeng"
          }
        ],
        "corrector": {
          "id": 51965,
          "login": "supervisor",
          "url": "https://api.intra.42.fr/v2/users/supervisor"
        },
        "truant": {},
        "filled_at": null,
        "questions_with_answers": []
      }
    ],
    "teams_uploads": []
  },
  {
    "id": 5250686,
    "name": "lchauvin's group-1",
    "url": "https://api.intra.42.fr/v2/teams/5250686",
    "final_mark": 0,
    "project_id": 2520,
    "created_at": "2023-10-17T09:22:29.221Z",
    "updated_at": "2023-11-01T12:52:20.362Z",
    "status": "finished",
    "terminating_at": "2023-11-01T12:51:34.614Z",
    "users": [
      {
        "id": 167614,
        "login": "lchauvin",
        "url": "https://api.intra.42.fr/v2/users/lchauvin",
        "leader": true,
        "occurrence": 4,
        "validated": true,
        "projects_user_id": 3361927
      }
    ],
    "locked?": true,
    "validated?": false,
    "closed?": true,
    "repo_url": "git@vogsphere.42campus.org:vogsphere/intra-uuid-c94a1012-3fc2-4e50-b9d4-032cad19da6f-5250686-lchauvin",
    "repo_uuid": "intra-uuid-c94a1012-3fc2-4e50-b9d4-032cad19da6f-5250686-lchauvin",
    "locked_at": "2023-10-17T09:22:29.325Z",
    "closed_at": "2023-10-17T11:51:34.614Z",
    "project_session_id": 10973,
    "project_gitlab_path": "pedago_world/42-cursus/outer-circle/microsoft_projects/42_active_discovery",
    "scale_teams": [
      {
        "id": 6032486,
        "scale_id": 28874,
        "comment": "Missing peer-evaluation replaced by a 0",
        "created_at": "2023-11-01T12:52:19.937Z",
        "updated_at": "2023-11-01T12:52:20.153Z",
        "feedback": "The Intranet is right ! I deserve this 0 !",
        "final_mark": 0,
        "flag": {
          "id": 1,
          "name": "Ok",
          "positive": true,
          "icon": "check-4",
          "created_at": "2015-09-14T23:06:52.000Z",
          "updated_at": "2015-09-14T23:06:52.000Z"
        },
        "begin_at": null,
        "correcteds": "invisible",
        "corrector": "invisible",
        "truant": {},
        "filled_at": "2023-11-01T12:52:20.085Z",
        "questions_with_answers": []
      },
      {
        "id": 6032485,
        "scale_id": 28874,
        "comment": "Missing peer-evaluation replaced by a 0",
        "created_at": "2023-11-01T12:52:19.611Z",
        "updated_at": "2023-11-01T12:52:19.840Z",
        "feedback": "The Intranet is right ! I deserve this 0 !",
        "final_mark": 0,
        "flag": {
          "id": 1,
          "name": "Ok",
          "positive": true,
          "icon": "check-4",
          "created_at": "2015-09-14T23:06:52.000Z",
          "updated_at": "2015-09-14T23:06:52.000Z"
        },
        "begin_at": null,
        "correcteds": "invisible",
        "corrector": "invisible",
        "truant": {},
        "filled_at": "2023-11-01T12:52:19.778Z",
        "questions_with_answers": []
      },
      {
        "id": 6032484,
        "scale_id": 28874,
        "comment": "Missing peer-evaluation replaced by a 0",
        "created_at": "2023-11-01T12:52:19.384Z",
        "updated_at": "2023-11-01T12:52:19.562Z",
        "feedback": "The Intranet is right ! I deserve this 0 !",
        "final_mark": 0,
        "flag": {
          "id": 1,
          "name": "Ok",
          "positive": true,
          "icon": "check-4",
          "created_at": "2015-09-14T23:06:52.000Z",
          "updated_at": "2015-09-14T23:06:52.000Z"
        },
        "begin_at": null,
        "correcteds": "invisible",
        "corrector": "invisible",
        "truant": {},
        "filled_at": "2023-11-01T12:52:19.501Z",
        "questions_with_answers": []
      }
    ],
    "teams_uploads": []
  },
  {
    "id": 5251155,
    "name": "lchauvin's group-2",
    "url": "https://api.intra.42.fr/v2/teams/5251155",
    "final_mark": 0,
    "project_id": 2520,
    "created_at": "2023-10-17T11:54:23.452Z",
    "updated_at": "2023-11-01T14:33:47.589Z",
    "status": "finished",
    "terminating_at": "2023-11-01T14:33:43.578Z",
    "users": [
      {
        "id": 167614,
        "login": "lchauvin",
        "url": "https://api.intra.42.fr/v2/users/lchauvin",
        "leader": true,
        "occurrence": 5,
        "validated": true,
        "projects_user_id": 3361927
      }
    ],
    "locked?": true,
    "validated?": false,
    "closed?": true,
    "repo_url": "git@vogsphere.42campus.org:vogsphere/intra-uuid-dd6514db-260c-4b22-869a-74d027e241cf-5251155-lchauvin",
    "repo_uuid": "intra-uuid-dd6514db-260c-4b22-869a-74d027e241cf-5251155-lchauvin",
    "locked_at": "2023-10-17T11:54:23.545Z",
    "closed_at": "2023-10-17T13:33:43.578Z",
    "project_session_id": 10973,
    "project_gitlab_path": "pedago_world/42-cursus/outer-circle/microsoft_projects/42_active_discovery",
    "scale_teams": [
      {
        "id": 6033035,
        "scale_id": 28874,
        "comment": "Missing peer-evaluation replaced by a 0",
        "created_at": "2023-11-01T14:33:47.130Z",
        "updated_at": "2023-11-01T14:33:47.350Z",
        "feedback": "The Intranet is right ! I deserve this 0 !",
        "final_mark": 0,
        "flag": {
          "id": 1,
          "name": "Ok",
          "positive": true,
          "icon": "check-4",
          "created_at": "2015-09-14T23:06:52.000Z",
          "updated_at": "2015-09-14T23:06:52.000Z"
        },
        "begin_at": null,
        "correcteds": "invisible",
        "corrector": "invisible",
        "truant": {},
        "filled_at": "2023-11-01T14:33:47.285Z",
        "questions_with_answers": []
      },
      {
        "id": 6033034,
        "scale_id": 28874,
        "comment": "Missing peer-evaluation replaced by a 0",
        "created_at": "2023-11-01T14:33:46.841Z",
        "updated_at": "2023-11-01T14:33:47.066Z",
        "feedback": "The Intranet is right ! I deserve this 0 !",
        "final_mark": 0,
        "flag": {
          "id": 1,
          "name": "Ok",
          "positive": true,
          "icon": "check-4",
          "created_at": "2015-09-14T23:06:52.000Z",
          "updated_at": "2015-09-14T23:06:52.000Z"
        },
        "begin_at": null,
        "correcteds": "invisible",
        "corrector": "invisible",
        "truant": {},
        "filled_at": "2023-11-01T14:33:46.998Z",
        "questions_with_answers": []
      },
      {
        "id": 6033033,
        "scale_id": 28874,
        "comment": "Missing peer-evaluation replaced by a 0",
        "created_at": "2023-11-01T14:33:46.555Z",
        "updated_at": "2023-11-01T14:33:46.775Z",
        "feedback": "The Intranet is right ! I deserve this 0 !",
        "final_mark": 0,
        "flag": {
          "id": 1,
          "name": "Ok",
          "positive": true,
          "icon": "check-4",
          "created_at": "2015-09-14T23:06:52.000Z",
          "updated_at": "2015-09-14T23:06:52.000Z"
        },
        "begin_at": null,
        "correcteds": "invisible",
        "corrector": "invisible",
        "truant": {},
        "filled_at": "2023-11-01T14:33:46.707Z",
        "questions_with_answers": []
      }
    ],
    "teams_uploads": []
  },
  {
    "id": 5251500,
    "name": "lchauvin's group-3",
    "url": "https://api.intra.42.fr/v2/teams/5251500",
    "final_mark": 0,
    "project_id": 2520,
    "created_at": "2023-10-17T13:37:23.333Z",
    "updated_at": "2023-11-01T18:06:50.399Z",
    "status": "finished",
    "terminating_at": "2023-11-01T18:06:26.719Z",
    "users": [
      {
        "id": 167614,
        "login": "lchauvin",
        "url": "https://api.intra.42.fr/v2/users/lchauvin",
        "leader": true,
        "occurrence": 6,
        "validated": true,
        "projects_user_id": 3361927
      },
      {
        "id": 168353,
        "login": "krandom5",
        "url": "https://api.intra.42.fr/v2/users/krandom5",
        "leader": false,
        "occurrence": 8,
        "validated": true,
        "projects_user_id": 3356127
      }
    ],
    "locked?": true,
    "validated?": false,
    "closed?": true,
    "repo_url": "git@vogsphere.42campus.org:vogsphere/intra-uuid-afd86e2a-c711-4800-a76d-ff836e1a6965-5251500-lchauvin",
    "repo_uuid": "intra-uuid-afd86e2a-c711-4800-a76d-ff836e1a6965-5251500-lchauvin",
    "locked_at": "2023-10-17T13:37:23.443Z",
    "closed_at": "2023-10-17T17:06:26.719Z",
    "project_session_id": 10973,
    "project_gitlab_path": "pedago_world/42-cursus/outer-circle/microsoft_projects/42_active_discovery",
    "scale_teams": [
      {
        "id": 6033950,
        "scale_id": 28874,
        "comment": "Missing peer-evaluation replaced by a 0",
        "created_at": "2023-11-01T18:06:50.006Z",
        "updated_at": "2023-11-01T18:06:50.195Z",
        "feedback": "The Intranet is right ! I deserve this 0 !",
        "final_mark": 0,
        "flag": {
          "id": 1,
          "name": "Ok",
          "positive": true,
          "icon": "check-4",
          "created_at": "2015-09-14T23:06:52.000Z",
          "updated_at": "2015-09-14T23:06:52.000Z"
        },
        "begin_at": null,
        "correcteds": "invisible",
        "corrector": "invisible",
        "truant": {},
        "filled_at": "2023-11-01T18:06:50.134Z",
        "questions_with_answers": []
      },
      {
        "id": 6033949,
        "scale_id": 28874,
        "comment": "Missing peer-evaluation replaced by a 0",
        "created_at": "2023-11-01T18:06:49.728Z",
        "updated_at": "2023-11-01T18:06:49.947Z",
        "feedback": "The Intranet is right ! I deserve this 0 !",
        "final_mark": 0,
        "flag": {
          "id": 1,
          "name": "Ok",
          "positive": true,
          "icon": "check-4",
          "created_at": "2015-09-14T23:06:52.000Z",
          "updated_at": "2015-09-14T23:06:52.000Z"
        },
        "begin_at": null,
        "correcteds": "invisible",
        "corrector": "invisible",
        "truant": {},
        "filled_at": "2023-11-01T18:06:49.877Z",
        "questions_with_answers": []
      },
      {
        "id": 6033948,
        "scale_id": 28874,
        "comment": "Missing peer-evaluation replaced by a 0",
        "created_at": "2023-11-01T18:06:49.425Z",
        "updated_at": "2023-11-01T18:06:49.644Z",
        "feedback": "The Intranet is right ! I deserve this 0 !",
        "final_mark": 0,
        "flag": {
          "id": 1,
          "name": "Ok",
          "positive": true,
          "icon": "check-4",
          "created_at": "2015-09-14T23:06:52.000Z",
          "updated_at": "2015-09-14T23:06:52.000Z"
        },
        "begin_at": null,
        "correcteds": "invisible",
        "corrector": "invisible",
        "truant": {},
        "filled_at": "2023-11-01T18:06:49.575Z",
        "questions_with_answers": []
      }
    ],
    "teams_uploads": []
  },
  {
    "id": 5406938,
    "name": "taeng's team",
    "url": "https://api.intra.42.fr/v2/teams/5406938",
    "final_mark": 0,
    "project_id": 1415,
    "created_at": "2024-01-14T14:03:15.235Z",
    "updated_at": "2024-01-14T14:24:18.007Z",
    "status": "finished",
    "terminating_at": null,
    "users": [
      {
        "id": 82562,
        "login": "taeng",
        "url": "https://api.intra.42.fr/v2/users/taeng",
        "leader": true,
        "occurrence": 0,
        "validated": true,
        "projects_user_id": 3072403
      }
    ],
    "locked?": true,
    "validated?": false,
    "closed?": true,
    "repo_url": "git@vogsphere.42seoul.kr:vogsphere/intra-uuid-2ba9fcb4-7b49-4ab3-8c6a-de96a03e2a56-5406938-taeng",
    "repo_uuid": "intra-uuid-2ba9fcb4-7b49-4ab3-8c6a-de96a03e2a56-5406938-taeng",
    "locked_at": "2024-01-14T14:03:15.282Z",
    "closed_at": "2024-01-14T14:23:52.926Z",
    "project_session_id": 8249,
    "project_gitlab_path": "pedago_world/42-cursus/outer-circle/ft_linux",
    "scale_teams": [],
    "teams_uploads": []
  },
  {
    "id": 5406945,
    "name": "taeng's team",
    "url": "https://api.intra.42.fr/v2/teams/5406945",
    "final_mark": 0,
    "project_id": 1330,
    "created_at": "2024-01-14T14:09:03.769Z",
    "updated_at": "2024-01-14T14:23:24.784Z",
    "status": "finished",
    "terminating_at": null,
    "users": [
      {
        "id": 82562,
        "login": "taeng",
        "url": "https://api.intra.42.fr/v2/users/taeng",
        "leader": true,
        "occurrence": 0,
        "validated": true,
        "projects_user_id": 3028842
      }
    ],
    "locked?": true,
    "validated?": false,
    "closed?": true,
    "repo_url": "git@vogsphere.42seoul.kr:vogsphere/intra-uuid-628b155f-3ec9-4dc2-b617-f373d97365e3-5406945-taeng",
    "repo_uuid": "intra-uuid-628b155f-3ec9-4dc2-b617-f373d97365e3-5406945-taeng",
    "locked_at": "2024-01-14T14:09:03.809Z",
    "closed_at": "2024-01-14T14:21:41.545Z",
    "project_session_id": 5131,
    "project_gitlab_path": "pedago_world/42-cursus/inner-circle/libasm",
    "scale_teams": [],
    "teams_uploads": []
  },
  {
    "id": 5406970,
    "name": "taeng's team",
    "url": "https://api.intra.42.fr/v2/teams/5406970",
    "final_mark": 0,
    "project_id": 1330,
    "created_at": "2024-01-14T14:23:29.378Z",
    "updated_at": "2024-01-14T14:35:00.657Z",
    "status": "finished",
    "terminating_at": null,
    "users": [
      {
        "id": 82562,
        "login": "taeng",
        "url": "https://api.intra.42.fr/v2/users/taeng",
        "leader": true,
        "occurrence": 0,
        "validated": true,
        "projects_user_id": 3028842
      }
    ],
    "locked?": true,
    "validated?": false,
    "closed?": true,
    "repo_url": "git@vogsphere.42seoul.kr:vogsphere/intra-uuid-dc677337-3fc9-4a73-bd0d-d48cc352bc25-5406970-taeng",
    "repo_uuid": "intra-uuid-dc677337-3fc9-4a73-bd0d-d48cc352bc25-5406970-taeng",
    "locked_at": "2024-01-14T14:23:29.426Z",
    "closed_at": "2024-01-14T14:30:01.969Z",
    "project_session_id": 5131,
    "project_gitlab_path": "pedago_world/42-cursus/inner-circle/libasm",
    "scale_teams": [],
    "teams_uploads": []
  },
  {
    "id": 5406983,
    "name": "taeng's team",
    "url": "https://api.intra.42.fr/v2/teams/5406983",
    "final_mark": 0,
    "project_id": 1415,
    "created_at": "2024-01-14T14:28:33.100Z",
    "updated_at": "2024-01-14T14:35:06.743Z",
    "status": "finished",
    "terminating_at": null,
    "users": [
      {
        "id": 82562,
        "login": "taeng",
        "url": "https://api.intra.42.fr/v2/users/taeng",
        "leader": true,
        "occurrence": 0,
        "validated": true,
        "projects_user_id": 3072403
      }
    ],
    "locked?": true,
    "validated?": false,
    "closed?": true,
    "repo_url": "git@vogsphere.42seoul.kr:vogsphere/intra-uuid-e28115eb-affa-458c-be18-a30add008e22-5406983-taeng",
    "repo_uuid": "intra-uuid-e28115eb-affa-458c-be18-a30add008e22-5406983-taeng",
    "locked_at": "2024-01-14T14:28:33.137Z",
    "closed_at": "2024-01-14T14:33:30.348Z",
    "project_session_id": 8249,
    "project_gitlab_path": "pedago_world/42-cursus/outer-circle/ft_linux",
    "scale_teams": [],
    "teams_uploads": []
  },
  {
    "id": 5406994,
    "name": "taeng's team",
    "url": "https://api.intra.42.fr/v2/teams/5406994",
    "final_mark": 0,
    "project_id": 1330,
    "created_at": "2024-01-14T14:35:10.291Z",
    "updated_at": "2024-01-14T14:47:17.583Z",
    "status": "finished",
    "terminating_at": null,
    "users": [
      {
        "id": 82562,
        "login": "taeng",
        "url": "https://api.intra.42.fr/v2/users/taeng",
        "leader": true,
        "occurrence": 0,
        "validated": true,
        "projects_user_id": 3028842
      }
    ],
    "locked?": true,
    "validated?": false,
    "closed?": true,
    "repo_url": "git@vogsphere.42seoul.kr:vogsphere/intra-uuid-16f6553b-08ad-403e-8d02-51d6fed00088-5406994-taeng",
    "repo_uuid": "intra-uuid-16f6553b-08ad-403e-8d02-51d6fed00088-5406994-taeng",
    "locked_at": "2024-01-14T14:35:10.338Z",
    "closed_at": "2024-01-14T14:46:02.035Z",
    "project_session_id": 5131,
    "project_gitlab_path": "pedago_world/42-cursus/inner-circle/libasm",
    "scale_teams": [],
    "teams_uploads": []
  },
  {
    "id": 5406995,
    "name": "taeng's team",
    "url": "https://api.intra.42.fr/v2/teams/5406995",
    "final_mark": 0,
    "project_id": 1415,
    "created_at": "2024-01-14T14:35:33.729Z",
    "updated_at": "2024-01-14T14:45:59.450Z",
    "status": "finished",
    "terminating_at": null,
    "users": [
      {
        "id": 82562,
        "login": "taeng",
        "url": "https://api.intra.42.fr/v2/users/taeng",
        "leader": true,
        "occurrence": 0,
        "validated": true,
        "projects_user_id": 3072403
      }
    ],
    "locked?": true,
    "validated?": false,
    "closed?": true,
    "repo_url": "git@vogsphere.42seoul.kr:vogsphere/intra-uuid-f10bc6b8-3d27-4c99-8a7b-b69e54db8f2e-5406995-taeng",
    "repo_uuid": "intra-uuid-f10bc6b8-3d27-4c99-8a7b-b69e54db8f2e-5406995-taeng",
    "locked_at": "2024-01-14T14:35:33.792Z",
    "closed_at": "2024-01-14T14:41:35.176Z",
    "project_session_id": 8249,
    "project_gitlab_path": "pedago_world/42-cursus/outer-circle/ft_linux",
    "scale_teams": [],
    "teams_uploads": []
  },
  {
    "id": 5407011,
    "name": "taeng's team",
    "url": "https://api.intra.42.fr/v2/teams/5407011",
    "final_mark": 0,
    "project_id": 1415,
    "created_at": "2024-01-14T14:47:19.888Z",
    "updated_at": "2024-01-14T14:50:33.559Z",
    "status": "finished",
    "terminating_at": null,
    "users": [
      {
        "id": 82562,
        "login": "taeng",
        "url": "https://api.intra.42.fr/v2/users/taeng",
        "leader": true,
        "occurrence": 0,
        "validated": true,
        "projects_user_id": 3072403
      }
    ],
    "locked?": true,
    "validated?": false,
    "closed?": true,
    "repo_url": "git@vogsphere.42seoul.kr:vogsphere/intra-uuid-bff51290-d41e-48d6-8289-00f941584fcd-5407011-taeng",
    "repo_uuid": "intra-uuid-bff51290-d41e-48d6-8289-00f941584fcd-5407011-taeng",
    "locked_at": "2024-01-14T14:47:19.926Z",
    "closed_at": "2024-01-14T14:49:43.578Z",
    "project_session_id": 8249,
    "project_gitlab_path": "pedago_world/42-cursus/outer-circle/ft_linux",
    "scale_teams": [],
    "teams_uploads": []
  },
  {
    "id": 5407014,
    "name": "taeng's team",
    "url": "https://api.intra.42.fr/v2/teams/5407014",
    "final_mark": 0,
    "project_id": 1330,
    "created_at": "2024-01-14T14:47:48.925Z",
    "updated_at": "2024-01-14T14:50:35.896Z",
    "status": "finished",
    "terminating_at": null,
    "users": [
      {
        "id": 82562,
        "login": "taeng",
        "url": "https://api.intra.42.fr/v2/users/taeng",
        "leader": true,
        "occurrence": 0,
        "validated": true,
        "projects_user_id": 3028842
      }
    ],
    "locked?": true,
    "validated?": false,
    "closed?": true,
    "repo_url": "git@vogsphere.42seoul.kr:vogsphere/intra-uuid-6a9c6495-9391-4b72-89c8-dbf23677e51d-5407014-taeng",
    "repo_uuid": "intra-uuid-6a9c6495-9391-4b72-89c8-dbf23677e51d-5407014-taeng",
    "locked_at": "2024-01-14T14:47:48.980Z",
    "closed_at": "2024-01-14T14:50:25.126Z",
    "project_session_id": 5131,
    "project_gitlab_path": "pedago_world/42-cursus/inner-circle/libasm",
    "scale_teams": [],
    "teams_uploads": []
  },
  {
    "id": 5407021,
    "name": "taeng's team",
    "url": "https://api.intra.42.fr/v2/teams/5407021",
    "final_mark": 0,
    "project_id": 1330,
    "created_at": "2024-01-14T14:53:43.035Z",
    "updated_at": "2024-01-14T15:01:29.792Z",
    "status": "finished",
    "terminating_at": null,
    "users": [
      {
        "id": 82562,
        "login": "taeng",
        "url": "https://api.intra.42.fr/v2/users/taeng",
        "leader": true,
        "occurrence": 0,
        "validated": true,
        "projects_user_id": 3028842
      }
    ],
    "locked?": true,
    "validated?": false,
    "closed?": true,
    "repo_url": "git@vogsphere.42seoul.kr:vogsphere/intra-uuid-31b329ec-df18-4293-bb78-e168173411fe-5407021-taeng",
    "repo_uuid": "intra-uuid-31b329ec-df18-4293-bb78-e168173411fe-5407021-taeng",
    "locked_at": "2024-01-14T14:53:43.078Z",
    "closed_at": "2024-01-14T15:00:10.244Z",
    "project_session_id": 5131,
    "project_gitlab_path": "pedago_world/42-cursus/inner-circle/libasm",
    "scale_teams": [],
    "teams_uploads": []
  },
  {
    "id": 5407027,
    "name": "taeng's team",
    "url": "https://api.intra.42.fr/v2/teams/5407027",
    "final_mark": 0,
    "project_id": 1415,
    "created_at": "2024-01-14T14:56:36.669Z",
    "updated_at": "2024-01-14T15:02:58.444Z",
    "status": "finished",
    "terminating_at": null,
    "users": [
      {
        "id": 82562,
        "login": "taeng",
        "url": "https://api.intra.42.fr/v2/users/taeng",
        "leader": true,
        "occurrence": 0,
        "validated": true,
        "projects_user_id": 3072403
      }
    ],
    "locked?": true,
    "validated?": false,
    "closed?": true,
    "repo_url": "git@vogsphere.42seoul.kr:vogsphere/intra-uuid-808408d8-129f-4ad7-86da-e5aee6e09ddc-5407027-taeng",
    "repo_uuid": "intra-uuid-808408d8-129f-4ad7-86da-e5aee6e09ddc-5407027-taeng",
    "locked_at": "2024-01-14T14:56:36.699Z",
    "closed_at": "2024-01-14T15:01:34.650Z",
    "project_session_id": 8249,
    "project_gitlab_path": "pedago_world/42-cursus/outer-circle/ft_linux",
    "scale_teams": [],
    "teams_uploads": []
  },
  {
    "id": 5407038,
    "name": "taeng's team",
    "url": "https://api.intra.42.fr/v2/teams/5407038",
    "final_mark": 0,
    "project_id": 1415,
    "created_at": "2024-01-14T15:08:18.800Z",
    "updated_at": "2024-01-14T15:10:50.811Z",
    "status": "finished",
    "terminating_at": null,
    "users": [
      {
        "id": 82562,
        "login": "taeng",
        "url": "https://api.intra.42.fr/v2/users/taeng",
        "leader": true,
        "occurrence": 0,
        "validated": true,
        "projects_user_id": 3072403
      }
    ],
    "locked?": true,
    "validated?": false,
    "closed?": true,
    "repo_url": "git@vogsphere.42seoul.kr:vogsphere/intra-uuid-b0b0f36f-a9ec-4f44-b5e0-34cb5847a594-5407038-taeng",
    "repo_uuid": "intra-uuid-b0b0f36f-a9ec-4f44-b5e0-34cb5847a594-5407038-taeng",
    "locked_at": "2024-01-14T15:08:18.853Z",
    "closed_at": "2024-01-14T15:10:21.832Z",
    "project_session_id": 8249,
    "project_gitlab_path": "pedago_world/42-cursus/outer-circle/ft_linux",
    "scale_teams": [],
    "teams_uploads": []
  },
  {
    "id": 5407040,
    "name": "taeng's team",
    "url": "https://api.intra.42.fr/v2/teams/5407040",
    "final_mark": null,
    "project_id": 1330,
    "created_at": "2024-01-14T15:08:50.195Z",
    "updated_at": "2024-02-02T14:18:50.984Z",
    "status": "waiting_for_correction",
    "terminating_at": null,
    "users": [
      {
        "id": 82562,
        "login": "taeng",
        "url": "https://api.intra.42.fr/v2/users/taeng",
        "leader": true,
        "occurrence": 0,
        "validated": true,
        "projects_user_id": 3028842
      }
    ],
    "locked?": true,
    "validated?": null,
    "closed?": true,
    "repo_url": "git@vogsphere.42seoul.kr:vogsphere/intra-uuid-1cf2e442-61c3-4d05-933a-65e252720ebe-5407040-taeng",
    "repo_uuid": "intra-uuid-1cf2e442-61c3-4d05-933a-65e252720ebe-5407040-taeng",
    "locked_at": "2024-01-14T15:08:50.267Z",
    "closed_at": "2024-01-14T15:10:59.055Z",
    "project_session_id": 5131,
    "project_gitlab_path": "pedago_world/42-cursus/inner-circle/libasm",
    "scale_teams": [
      {
        "id": 6332609,
        "scale_id": 20936,
        "comment": null,
        "created_at": "2024-02-02T14:18:50.358Z",
        "updated_at": "2024-02-02T14:18:50.975Z",
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
        "begin_at": null,
        "correcteds": "invisible",
        "corrector": "invisible",
        "truant": {},
        "filled_at": null,
        "questions_with_answers": []
      },
      {
        "id": 6332573,
        "scale_id": 20936,
        "comment": null,
        "created_at": "2024-02-02T14:10:21.736Z",
        "updated_at": "2024-02-02T14:11:53.068Z",
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
        "begin_at": null,
        "correcteds": "invisible",
        "corrector": "invisible",
        "truant": {},
        "filled_at": null,
        "questions_with_answers": []
      },
      {
        "id": 6332287,
        "scale_id": 20936,
        "comment": "test eval woo-hyot!",
        "created_at": "2024-02-02T12:46:22.582Z",
        "updated_at": "2024-02-02T13:46:16.306Z",
        "feedback": null,
        "final_mark": 125,
        "flag": {
          "id": 9,
          "name": "Outstanding project",
          "positive": true,
          "icon": "star-1",
          "created_at": "2017-05-18T14:07:37.380Z",
          "updated_at": "2017-05-18T14:12:07.415Z"
        },
        "begin_at": "2024-02-02T13:00:00.000Z",
        "correcteds": [
          {
            "id": 82562,
            "login": "taeng",
            "url": "https://api.intra.42.fr/v2/users/taeng"
          }
        ],
        "corrector": {
          "id": 82562,
          "login": "taeng",
          "url": "https://api.intra.42.fr/v2/users/taeng"
        },
        "truant": {},
        "filled_at": "2024-02-02T12:48:45.308Z",
        "questions_with_answers": []
      },
      {
        "id": 6332296,
        "scale_id": 20936,
        "comment": null,
        "created_at": "2024-02-02T12:48:11.535Z",
        "updated_at": "2024-02-02T12:48:11.535Z",
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
        "begin_at": "2024-02-02T15:00:00.000Z",
        "correcteds": [
          {
            "id": 82562,
            "login": "taeng",
            "url": "https://api.intra.42.fr/v2/users/taeng"
          }
        ],
        "corrector": {
          "id": 82562,
          "login": "taeng",
          "url": "https://api.intra.42.fr/v2/users/taeng"
        },
        "truant": {},
        "filled_at": null,
        "questions_with_answers": []
      },
      {
        "id": 6332293,
        "scale_id": 20936,
        "comment": null,
        "created_at": "2024-02-02T12:47:21.245Z",
        "updated_at": "2024-02-02T12:47:21.245Z",
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
        "begin_at": "2024-02-02T13:45:00.000Z",
        "correcteds": [
          {
            "id": 82562,
            "login": "taeng",
            "url": "https://api.intra.42.fr/v2/users/taeng"
          }
        ],
        "corrector": {
          "id": 82562,
          "login": "taeng",
          "url": "https://api.intra.42.fr/v2/users/taeng"
        },
        "truant": {},
        "filled_at": null,
        "questions_with_answers": []
      }
    ],
    "teams_uploads": []
  },
  {
    "id": 5520400,
    "name": "dino42's group",
    "url": "https://api.intra.42.fr/v2/teams/5520400",
    "final_mark": 0,
    "project_id": 1314,
    "created_at": "2024-02-21T03:43:46.344Z",
    "updated_at": "2024-02-21T04:26:25.674Z",
    "status": "finished",
    "terminating_at": null,
    "users": [
      {
        "id": 178691,
        "login": "dino42",
        "url": "https://api.intra.42.fr/v2/users/dino42",
        "leader": true,
        "occurrence": 0,
        "validated": true,
        "projects_user_id": 3556490
      }
    ],
    "locked?": true,
    "validated?": false,
    "closed?": true,
    "repo_url": "git@vogsphere.42gyeongsan.kr:vogsphere/intra-uuid-0c777eb9-1ff3-411a-b364-c24b71159696-5520400-dino42",
    "repo_uuid": "intra-uuid-0c777eb9-1ff3-411a-b364-c24b71159696-5520400-dino42",
    "locked_at": "2024-02-21T03:43:46.386Z",
    "closed_at": "2024-02-21T04:23:58.122Z",
    "project_session_id": 11805,
    "project_gitlab_path": "pedago_world/42-cursus/inner-circle/libft",
    "scale_teams": [],
    "teams_uploads": []
  },
  {
    "id": 5533738,
    "name": "junhoh's group",
    "url": "https://api.intra.42.fr/v2/teams/5533738",
    "final_mark": 0,
    "project_id": 1314,
    "created_at": "2024-02-26T00:43:00.899Z",
    "updated_at": "2024-03-09T11:00:44.699Z",
    "status": "finished",
    "terminating_at": null,
    "users": [
      {
        "id": 174103,
        "login": "junhoh",
        "url": "https://api.intra.42.fr/v2/users/junhoh",
        "leader": true,
        "occurrence": 0,
        "validated": true,
        "projects_user_id": 3565864
      }
    ],
    "locked?": true,
    "validated?": false,
    "closed?": true,
    "repo_url": "git@vogsphere.42gyeongsan.kr:vogsphere/intra-uuid-4d186e59-701d-4519-a5e4-6dbc43d8463e-5533738-junhoh",
    "repo_uuid": "intra-uuid-4d186e59-701d-4519-a5e4-6dbc43d8463e-5533738-junhoh",
    "locked_at": "2024-02-26T00:43:00.934Z",
    "closed_at": "2024-03-05T10:49:08.372Z",
    "project_session_id": 11805,
    "project_gitlab_path": "pedago_world/42-cursus/inner-circle/libft",
    "scale_teams": [],
    "teams_uploads": []
  },
  {
    "id": 5533741,
    "name": "wooslee's group",
    "url": "https://api.intra.42.fr/v2/teams/5533741",
    "final_mark": 31,
    "project_id": 1314,
    "created_at": "2024-02-26T00:45:05.103Z",
    "updated_at": "2024-02-27T05:32:12.418Z",
    "status": "finished",
    "terminating_at": null,
    "users": [
      {
        "id": 174087,
        "login": "wooslee",
        "url": "https://api.intra.42.fr/v2/users/wooslee",
        "leader": true,
        "occurrence": 0,
        "validated": true,
        "projects_user_id": 3565865
      }
    ],
    "locked?": true,
    "validated?": false,
    "closed?": true,
    "repo_url": "git@vogsphere.42gyeongsan.kr:vogsphere/intra-uuid-17d3b0ed-40d8-4b26-ab79-3ff196008edb-5533741-wooslee",
    "repo_uuid": "intra-uuid-17d3b0ed-40d8-4b26-ab79-3ff196008edb-5533741-wooslee",
    "locked_at": "2024-02-26T00:45:05.144Z",
    "closed_at": "2024-02-27T02:11:46.335Z",
    "project_session_id": 11805,
    "project_gitlab_path": "pedago_world/42-cursus/inner-circle/libft",
    "scale_teams": [
      {
        "id": 6446873,
        "scale_id": 32445,
        "comment": "안녕하세요. wooslee님의 평가를 진행한 myeochoi입니다. 아직 libft과제의 첫번째 섹션 밖에 진행하지 못해서 정확한 평가를 진행하지 못한 부분이 개인적으로 아쉬웠지만 보너스과제와 여러 부분들에 대해서 많이 배울 수 있는 기회였습니다. 다음번 시도 때에는 만점 받으시길 바랍니다. 화이팅입니다.",
        "created_at": "2024-02-27T02:13:29.841Z",
        "updated_at": "2024-02-27T05:31:52.034Z",
        "feedback": "아직 진행을 하시지 않은 부분임에도 불구하고 코드를 하나하나 살펴봐주시고 이해하려고 노력하시는 모습에서 배울 수 있는 점이 많았습니다. 고생 많으셨습니다.",
        "final_mark": 63,
        "flag": {
          "id": 1,
          "name": "Ok",
          "positive": true,
          "icon": "check-4",
          "created_at": "2015-09-14T23:06:52.000Z",
          "updated_at": "2015-09-14T23:06:52.000Z"
        },
        "begin_at": "2024-02-27T04:45:00.000Z",
        "correcteds": [
          {
            "id": 174087,
            "login": "wooslee",
            "url": "https://api.intra.42.fr/v2/users/wooslee"
          }
        ],
        "corrector": {
          "id": 172342,
          "login": "myeochoi",
          "url": "https://api.intra.42.fr/v2/users/myeochoi"
        },
        "truant": {},
        "filled_at": "2024-02-27T05:28:11.007Z",
        "questions_with_answers": []
      },
      {
        "id": 6446870,
        "scale_id": 32445,
        "comment": "안녕하세요! 평가자 ksuh입니다. 시간약속을 지키지 못했는데 이해해주셔서 감사합니다. 코드를 일관성 있게 잘 짜셨는데 몇몇 사소한 실수들을 하신 것 같습니다. ft_memcmp, ft_calloc, ft_strncmp, ft_isprint를 다시 고쳐보시면 좋을 것 같네요. 저도 풀다가 모르는 거 있으면 질문하러 오겠습니다! 화이팅하세요",
        "created_at": "2024-02-27T02:12:00.265Z",
        "updated_at": "2024-02-27T05:29:59.490Z",
        "feedback": "함수가 40개가 넘는 데도 불구하고 하나하나 꼼꼼하게 확인해주셔서 감사합니다. 고생 많으셨고, 과제에서 좋은 결과 얻으시길 바라겠습니다.",
        "final_mark": 0,
        "flag": {
          "id": 1,
          "name": "Ok",
          "positive": true,
          "icon": "check-4",
          "created_at": "2015-09-14T23:06:52.000Z",
          "updated_at": "2015-09-14T23:06:52.000Z"
        },
        "begin_at": "2024-02-27T02:45:00.000Z",
        "correcteds": [
          {
            "id": 174087,
            "login": "wooslee",
            "url": "https://api.intra.42.fr/v2/users/wooslee"
          }
        ],
        "corrector": {
          "id": 172367,
          "login": "ksuh",
          "url": "https://api.intra.42.fr/v2/users/ksuh"
        },
        "truant": {},
        "filled_at": "2024-02-27T04:30:54.309Z",
        "questions_with_answers": []
      },
      {
        "id": 6446872,
        "scale_id": 32445,
        "comment": "안녕하세요. 평가를 맡게된 김승현입니다. 해당 과제 저도 진행 중인 과제인데 보너스까지 끝내놓으셔서 평가하면서 배울 점이 많았습니다. 노트 해둔 점을 제 코드에 반영하면 코드의 완성도를 높일 수 있을 것 같아 고맙게 생각합니다. 설명을 천천히 잘해주셨고 과제에 대한 이해도가 높으셔서 제 질문에 대한 답을 함에 있어 막힘이 없으셔서 평가가 한결 수월했습니다. 점심시간이 막 지난 시점이고 서로 배가 고픈 상황이라 평가를 이만 줄입니다. 전체적으로 만족스럽고 배울 점이 많아 좋았습니다. 감사합니다.",
        "created_at": "2024-02-27T02:13:18.580Z",
        "updated_at": "2024-02-27T05:31:22.127Z",
        "feedback": "40개가 넘는 함수를 꼼꼼하게 살펴봐주시고 오류를 찾아내주셔서 다음 트라이 때에 큰 도움이 될 것 같습니다. 고생 많으셨습니다.",
        "final_mark": 125,
        "flag": {
          "id": 1,
          "name": "Ok",
          "positive": true,
          "icon": "check-4",
          "created_at": "2015-09-14T23:06:52.000Z",
          "updated_at": "2015-09-14T23:06:52.000Z"
        },
        "begin_at": "2024-02-27T03:30:00.000Z",
        "correcteds": [
          {
            "id": 174087,
            "login": "wooslee",
            "url": "https://api.intra.42.fr/v2/users/wooslee"
          }
        ],
        "corrector": {
          "id": 172385,
          "login": "sunghyki",
          "url": "https://api.intra.42.fr/v2/users/sunghyki"
        },
        "truant": {},
        "filled_at": "2024-02-27T03:56:07.979Z",
        "questions_with_answers": []
      }
    ],
    "teams_uploads": [
      {
        "id": 2109047,
        "final_mark": 1,
        "comment": "initial_errors:  | test_ft_isalpha: KO (Does not compile) | test_ft_isdigit: KO (Does not compile) | test_ft_isalnum: KO (Does not compile) | test_ft_isascii: KO (Does not compile) | test_ft_isprint: KO (Does not compile) | test_ft_strlen: OK | test_ft_memset: OK | test_ft_bzero: OK | test_ft_memcpy: OK | test_ft_memmove: Error encountered while testing | test_ft_strlcpy: OK | test_ft_strlcat: Error encountered while testing | test_ft_toupper: KO (Does not compile) | test_ft_tolower: KO (Does not compile) | test_ft_strchr: OK | test_ft_strrchr: Error encountered while testing | test_ft_strncmp: Error encountered while testing | test_ft_memchr: Error encountered while testing | test_ft_memcmp: Error encountered while testing | test_ft_strnstr: Error encountered while testing | test_ft_atoi: Error encountered while testing | test_ft_calloc: Error encountered while testing | test_ft_strdup: Error encountered while testing | test_ft_substr: Error encountered while testing | test_ft_strjoin: Error encountered while testing | test_ft_strtrim: Error encountered while testing | test_ft_split: Error encountered while testing | test_ft_itoa: Error encountered while testing | test_ft_strmapi: OK | test_ft_striteri: OK | test_ft_putchar_fd: OK | test_ft_putstr_fd: OK | test_ft_putendl_fd: OK | test_ft_putnbr_fd: OK | bonus: 8/9 functions correct",
        "created_at": "2024-02-27T05:32:12.417Z",
        "upload_id": 1558
      }
    ]
  },
  {
    "id": 5533742,
    "name": "jishin's group",
    "url": "https://api.intra.42.fr/v2/teams/5533742",
    "final_mark": null,
    "project_id": 1314,
    "created_at": "2024-02-26T00:45:34.618Z",
    "updated_at": "2024-02-26T00:47:30.807Z",
    "status": "in_progress",
    "terminating_at": null,
    "users": [
      {
        "id": 172335,
        "login": "jishin",
        "url": "https://api.intra.42.fr/v2/users/jishin",
        "leader": true,
        "occurrence": 0,
        "validated": true,
        "projects_user_id": 3565866
      }
    ],
    "locked?": true,
    "validated?": null,
    "closed?": false,
    "repo_url": "git@vogsphere.42gyeongsan.kr:vogsphere/intra-uuid-7bac6452-5717-4b33-93ab-14b96b7197b8-5533742-jishin",
    "repo_uuid": "intra-uuid-7bac6452-5717-4b33-93ab-14b96b7197b8-5533742-jishin",
    "locked_at": "2024-02-26T00:45:34.652Z",
    "closed_at": null,
    "project_session_id": 11805,
    "project_gitlab_path": "pedago_world/42-cursus/inner-circle/libft",
    "scale_teams": [],
    "teams_uploads": []
  },
  {
    "id": 5533743,
    "name": "seonhwan's group",
    "url": "https://api.intra.42.fr/v2/teams/5533743",
    "final_mark": 93,
    "project_id": 1314,
    "created_at": "2024-02-26T00:45:42.025Z",
    "updated_at": "2024-03-14T20:06:09.092Z",
    "status": "finished",
    "terminating_at": null,
    "users": [
      {
        "id": 174157,
        "login": "seonhwan",
        "url": "https://api.intra.42.fr/v2/users/seonhwan",
        "leader": true,
        "occurrence": 0,
        "validated": true,
        "projects_user_id": 3565867
      }
    ],
    "locked?": true,
    "validated?": false,
    "closed?": true,
    "repo_url": "git@vogsphere.42gyeongsan.kr:vogsphere/intra-uuid-4ea1979d-09d4-41f6-abf3-f48e4e42b272-5533743-seonhwan",
    "repo_uuid": "intra-uuid-4ea1979d-09d4-41f6-abf3-f48e4e42b272-5533743-seonhwan",
    "locked_at": "2024-02-26T00:45:42.059Z",
    "closed_at": "2024-03-14T14:56:01.381Z",
    "project_session_id": 11805,
    "project_gitlab_path": "pedago_world/42-cursus/inner-circle/libft",
    "scale_teams": [
      {
        "id": 6516023,
        "scale_id": 32445,
        "comment": "안녕하세요 yokoh입니다.일단 성능을 생각하시는 자세가 존경스럽습니다. memset에서 비트연산을 이용해서 시간단축하는 코드가 인상적이였습니다.",
        "created_at": "2024-03-14T15:01:53.224Z",
        "updated_at": "2024-03-14T20:04:09.177Z",
        "feedback": "새벽 밤샘으로 매우 피곤하셨을 텐데도 밝은 미소로 평가에 임해주셔서 너무 감사했습니다. 40분동안의 긴 평가였지만, 제가 작성한 코드에 대해 미처 생각하지 못했던 시간복잡도, 연산 횟수 등에 대해 많은 피드백을 해주셔서 정말 도움이 많이 되었고, 코드 작성 노하우도 습득할 수 있었던 좋은 기회였습니다. 특히 memset()의 구현이나 bit 연산 등에 대해 자세하게 피드백해주셔서 감사했습니다. 수고 많으셨습니다. 화이팅!",
        "final_mark": 125,
        "flag": {
          "id": 9,
          "name": "Outstanding project",
          "positive": true,
          "icon": "star-1",
          "created_at": "2017-05-18T14:07:37.380Z",
          "updated_at": "2017-05-18T14:12:07.415Z"
        },
        "begin_at": "2024-03-14T15:45:00.000Z",
        "correcteds": [
          {
            "id": 174157,
            "login": "seonhwan",
            "url": "https://api.intra.42.fr/v2/users/seonhwan"
          }
        ],
        "corrector": {
          "id": 172330,
          "login": "yokoh",
          "url": "https://api.intra.42.fr/v2/users/yokoh"
        },
        "truant": {},
        "filled_at": "2024-03-14T16:25:13.220Z",
        "questions_with_answers": []
      },
      {
        "id": 6516035,
        "scale_id": 32445,
        "comment": "설명해주신 부분들 모두 다 이해가 잘 되었고, 직접 구현을 고민해본 분임을 느꼈습니다. 아마 첫트에 125로 통과하실 것 같네요. 본과정 클리어까지 함께 화이팅입니다! :) LGTM!",
        "created_at": "2024-03-14T15:04:05.328Z",
        "updated_at": "2024-03-14T20:05:52.804Z",
        "feedback": "이틀간 밤샘 공부를 하시느라 심신이 많이 힘드셨을 텐데도 평가에 친절하게 임해주시고, 제 설명도 최대한 들어주시려 노력하셔서 너무 감동받았습니다. 밝은 미소로 평가를 해주셔서 평가 내내 긴장하지 않고 제가 작성했던 코드를 수월하게 설명드릴 수 있었습니다. 그리고 제 구현에 대한 피드백도 너무 도움 많이 되었고, 코드 작성 노하우도 많이 알아가는 좋은 기회였습니다. 평가 고생많으셨습니다. 감사합니다!",
        "final_mark": 125,
        "flag": {
          "id": 9,
          "name": "Outstanding project",
          "positive": true,
          "icon": "star-1",
          "created_at": "2017-05-18T14:07:37.380Z",
          "updated_at": "2017-05-18T14:12:07.415Z"
        },
        "begin_at": "2024-03-14T17:30:00.000Z",
        "correcteds": [
          {
            "id": 174157,
            "login": "seonhwan",
            "url": "https://api.intra.42.fr/v2/users/seonhwan"
          }
        ],
        "corrector": {
          "id": 172319,
          "login": "wonhseo",
          "url": "https://api.intra.42.fr/v2/users/wonhseo"
        },
        "truant": {},
        "filled_at": "2024-03-14T18:41:51.031Z",
        "questions_with_answers": []
      },
      {
        "id": 6516021,
        "scale_id": 32445,
        "comment": "안녕하세요! keunykim 입니다. 자신만의 신념에 따라서 코드를 짜신 것이 느껴졌습니다. 유닛테스트는 오래전에 구현된 c라이브러리가 발생시키는 오류까지도 똑같이 발생시켜야만이 OK로 통과시켜주었는데, seonhwan님께서는 그런 부분에서 CRASH를 받더라도 '함수가 마땅히 가져야하는 성질이라면', 혹은 '정의되지 않은 동작이라서 구현자의 재량에 맡기는 부분이라면', 앞으로의 본과정을 진행하는데 있어서 유리하고 논리적인 방식으로 구현하신 것을 볼 수 있었습니다. 그밖에 memset 같은 경우 대부분 1바이트씩 하나하나 복사하는 구현에 그치는데, 여러바이트를 하나의 청크로만들어 복사하므로써 속도를 최적화시킨 부분을 알 수 있었습니다. 자신만의 신념과 CS지식을 적극 활용하여 코드를 최적화한 부분이 많이 보였습니다. 더 쓸 말이 많지만 시간 관계상 이만 줄이겠습니다. 많이 배우고싶습니다. 감사합니다!",
        "created_at": "2024-03-14T15:01:45.919Z",
        "updated_at": "2024-03-14T20:01:38.804Z",
        "feedback": "새벽에 매우 피곤하셨을 텐데도 친절히 평가 해주셔서 너무 감사했습니다. 2시간동안의 긴 평가였지만, 평가 동안 제 코드 상에서의 시간 복잡도 문제, 연산 횟수 등을 최적화 하는 방안으로 피드백을 매우 자세하게 해주셔서 코드 개선에 정말 많은 도움이 되었고, 공부가 많이 되었습니다. 평가 내내 제 코드를 자세히 설명해드리면서 제가 코드 작성시에는 미처 생각하지 못했던 부분들도 캐치해낼 수 있었고, 코드 작성 노하우를 많이 체득할 수 있었던 좋은 기회였습니다. 평가 감사했습니다. 화이팅!",
        "final_mark": 125,
        "flag": {
          "id": 9,
          "name": "Outstanding project",
          "positive": true,
          "icon": "star-1",
          "created_at": "2017-05-18T14:07:37.380Z",
          "updated_at": "2017-05-18T14:12:07.415Z"
        },
        "begin_at": "2024-03-14T16:15:00.000Z",
        "correcteds": [
          {
            "id": 174157,
            "login": "seonhwan",
            "url": "https://api.intra.42.fr/v2/users/seonhwan"
          }
        ],
        "corrector": {
          "id": 172402,
          "login": "keunykim",
          "url": "https://api.intra.42.fr/v2/users/keunykim"
        },
        "truant": {},
        "filled_at": "2024-03-14T18:11:25.994Z",
        "questions_with_answers": []
      }
    ],
    "teams_uploads": [
      {
        "id": 2128952,
        "final_mark": 61,
        "comment": "initial_errors:  | test_ft_isalpha: OK | test_ft_isdigit: OK | test_ft_isalnum: OK | test_ft_isascii: OK | test_ft_isprint: OK | test_ft_strlen: OK | test_ft_memset: OK | test_ft_bzero: OK | test_ft_memcpy: OK | test_ft_memmove: OK | test_ft_strlcpy: OK | test_ft_strlcat: OK | test_ft_toupper: OK | test_ft_tolower: OK | test_ft_strchr: OK | test_ft_strrchr: OK | test_ft_strncmp: OK | test_ft_memchr: OK | test_ft_memcmp: OK | test_ft_strnstr: OK | test_ft_atoi: OK | test_ft_calloc: Error encountered while testing | test_ft_strdup: OK | test_ft_substr: OK | test_ft_strjoin: OK | test_ft_strtrim: OK | test_ft_split: OK | test_ft_itoa: OK | test_ft_strmapi: OK | test_ft_striteri: OK | test_ft_putchar_fd: OK | test_ft_putstr_fd: OK | test_ft_putendl_fd: OK | test_ft_putnbr_fd: OK | bonus: 9/9 functions correct",
        "created_at": "2024-03-14T20:06:09.090Z",
        "upload_id": 1558
      }
    ]
  },
  {
    "id": 5533746,
    "name": "doji's group",
    "url": "https://api.intra.42.fr/v2/teams/5533746",
    "final_mark": 95,
    "project_id": 1314,
    "created_at": "2024-02-26T00:48:21.250Z",
    "updated_at": "2024-03-09T10:01:52.060Z",
    "status": "finished",
    "terminating_at": null,
    "users": [
      {
        "id": 174127,
        "login": "doji",
        "url": "https://api.intra.42.fr/v2/users/doji",
        "leader": true,
        "occurrence": 0,
        "validated": true,
        "projects_user_id": 3565869
      }
    ],
    "locked?": true,
    "validated?": false,
    "closed?": true,
    "repo_url": "git@vogsphere.42gyeongsan.kr:vogsphere/intra-uuid-54435719-b4f2-4914-85fd-c492e946a0f7-5533746-doji",
    "repo_uuid": "intra-uuid-54435719-b4f2-4914-85fd-c492e946a0f7-5533746-doji",
    "locked_at": "2024-02-26T00:48:21.316Z",
    "closed_at": "2024-03-09T06:22:14.278Z",
    "project_session_id": 11805,
    "project_gitlab_path": "pedago_world/42-cursus/inner-circle/libft",
    "scale_teams": [
      {
        "id": 6494471,
        "scale_id": 32445,
        "comment": "피신 때부터 많은 배움을 받은 동료입니다.\r\n모든 코드들 함께 꼼꼼히 살펴보았고, 마지막의 마지막 부분에서 사소한 실수를 발견하였지만, 그 부분에 대한 피드백 후 아주 나중에는 고치시는 것으로 합의!\r\n코딩 스타일에 대해 첨언하였지만 사실 저의 피드백이 필요치 않은 분이라 걱정하지 않습니다!\r\n앞으로 장거리 마라톤이니 템포 조절하셔서 화이팅 하시길 바랍니다!",
        "created_at": "2024-03-09T07:50:56.001Z",
        "updated_at": "2024-03-09T10:01:29.237Z",
        "feedback": "채점을 많이 하셔서 능숙하게 문제가 될 부분들을 찾아주셨습니다. 보너스파트에서 프리하지 않은 함수를 발견해주셨고 메이크파일이나 다른 함수들에서도 몇몇 문제가 될 부분들을 발견해주셨습니다. 감사합니다.",
        "final_mark": 124,
        "flag": {
          "id": 9,
          "name": "Outstanding project",
          "positive": true,
          "icon": "star-1",
          "created_at": "2017-05-18T14:07:37.380Z",
          "updated_at": "2017-05-18T14:12:07.415Z"
        },
        "begin_at": "2024-03-09T09:15:00.000Z",
        "correcteds": [
          {
            "id": 174127,
            "login": "doji",
            "url": "https://api.intra.42.fr/v2/users/doji"
          }
        ],
        "corrector": {
          "id": 174133,
          "login": "kyouhele",
          "url": "https://api.intra.42.fr/v2/users/kyouhele"
        },
        "truant": {},
        "filled_at": "2024-03-09T09:58:11.293Z",
        "questions_with_answers": []
      },
      {
        "id": 6494470,
        "scale_id": 32445,
        "comment": "제가 libft 진도를 아직 파트1까지 밖에 안나갔는데 doji님께서 함수를 하나하나 천천히 설명해주셔서 제가 작성하면서 몰랐던 개념 포인터들을 확인할 수 있었습니다. 이전에 작성한 함수들을 libft.h에 넣어서 재활용 하는 부분이 감명깊었습니다 어떤 함수들은 이전 함수 재활용을 통해 보다 손쉽게 코드를 짤 수 있는 것을 배웠습니다",
        "created_at": "2024-03-09T07:50:38.049Z",
        "updated_at": "2024-03-09T10:00:48.417Z",
        "feedback": "아직 문제를 풀고 계셔서 저에게 코드를 보고 모르는부분에 대하여 질문을 주시면 그것에 대해 설명을 해드렸습니다. 하나하나 꼼꼼히 봐주셨습니다. 감사합니다.",
        "final_mark": 125,
        "flag": {
          "id": 1,
          "name": "Ok",
          "positive": true,
          "icon": "check-4",
          "created_at": "2015-09-14T23:06:52.000Z",
          "updated_at": "2015-09-14T23:06:52.000Z"
        },
        "begin_at": "2024-03-09T08:30:00.000Z",
        "correcteds": [
          {
            "id": 174127,
            "login": "doji",
            "url": "https://api.intra.42.fr/v2/users/doji"
          }
        ],
        "corrector": {
          "id": 174112,
          "login": "dojkoo",
          "url": "https://api.intra.42.fr/v2/users/dojkoo"
        },
        "truant": {},
        "filled_at": "2024-03-09T09:13:19.158Z",
        "questions_with_answers": []
      },
      {
        "id": 6494373,
        "scale_id": 32445,
        "comment": "파트 2 strtrim을 하는 중이라 뒷부분에 대해서는 평가하기가 어려웠습니다. 그래서 모르는 부분에 대해 질문을 했고 답변을 잘 들었습니다. 많은 질문에도 잘 대답해주셔서 좋았고 평가를 하면서 부족한 개념에 대해서 더 학습해야할 필요성을 느꼈습니다. 좋은 결과 있으시길 바라며 계속 화이팅하시길 바라겠습니다!",
        "created_at": "2024-03-09T06:22:24.061Z",
        "updated_at": "2024-03-09T10:00:38.962Z",
        "feedback": "아직 문제를 풀고 계셔서 저에게 코드를 보고 모르는부분에 대하여 질문을 주시면 그것에 대해 설명을 해드렸습니다. 하나하나 꼼꼼히 봐주셨습니다. 감사합니다.",
        "final_mark": 125,
        "flag": {
          "id": 1,
          "name": "Ok",
          "positive": true,
          "icon": "check-4",
          "created_at": "2015-09-14T23:06:52.000Z",
          "updated_at": "2015-09-14T23:06:52.000Z"
        },
        "begin_at": "2024-03-09T07:00:00.000Z",
        "correcteds": [
          {
            "id": 174127,
            "login": "doji",
            "url": "https://api.intra.42.fr/v2/users/doji"
          }
        ],
        "corrector": {
          "id": 174168,
          "login": "jehbae",
          "url": "https://api.intra.42.fr/v2/users/jehbae"
        },
        "truant": {},
        "filled_at": "2024-03-09T07:49:47.228Z",
        "questions_with_answers": []
      }
    ],
    "teams_uploads": [
      {
        "id": 2122865,
        "final_mark": 67,
        "comment": "initial_errors:  | test_ft_isalpha: OK | test_ft_isdigit: OK | test_ft_isalnum: OK | test_ft_isascii: OK | test_ft_isprint: OK | test_ft_strlen: OK | test_ft_memset: OK | test_ft_bzero: OK | test_ft_memcpy: OK | test_ft_memmove: OK | test_ft_strlcpy: OK | test_ft_strlcat: OK | test_ft_toupper: OK | test_ft_tolower: OK | test_ft_strchr: OK | test_ft_strrchr: OK | test_ft_strncmp: OK | test_ft_memchr: OK | test_ft_memcmp: OK | test_ft_strnstr: OK | test_ft_atoi: OK | test_ft_calloc: OK | test_ft_strdup: OK | test_ft_substr: Error encountered while testing | test_ft_strjoin: OK | test_ft_strtrim: OK | test_ft_split: OK | test_ft_itoa: OK | test_ft_strmapi: OK | test_ft_striteri: OK | test_ft_putchar_fd: OK | test_ft_putstr_fd: OK | test_ft_putendl_fd: OK | test_ft_putnbr_fd: OK | bonus: 9/9 functions correct",
        "created_at": "2024-03-09T10:01:52.058Z",
        "upload_id": 1558
      }
    ]
  },
  {
    "id": 5533747,
    "name": "junmin's group",
    "url": "https://api.intra.42.fr/v2/teams/5533747",
    "final_mark": 0,
    "project_id": 1314,
    "created_at": "2024-02-26T00:48:35.542Z",
    "updated_at": "2024-03-01T15:43:44.880Z",
    "status": "finished",
    "terminating_at": null,
    "users": [
      {
        "id": 174096,
        "login": "junmin",
        "url": "https://api.intra.42.fr/v2/users/junmin",
        "leader": true,
        "occurrence": 0,
        "validated": true,
        "projects_user_id": 3565870
      }
    ],
    "locked?": true,
    "validated?": false,
    "closed?": true,
    "repo_url": "git@vogsphere.42gyeongsan.kr:vogsphere/intra-uuid-3a0a0eee-0aa5-49c6-9db9-add9c9f7cdab-5533747-junmin",
    "repo_uuid": "intra-uuid-3a0a0eee-0aa5-49c6-9db9-add9c9f7cdab-5533747-junmin",
    "locked_at": "2024-02-26T00:48:35.589Z",
    "closed_at": "2024-03-01T12:03:51.550Z",
    "project_session_id": 11805,
    "project_gitlab_path": "pedago_world/42-cursus/inner-circle/libft",
    "scale_teams": [
      {
        "id": 6466544,
        "scale_id": 32445,
        "comment": "보너스 파트까지 완벽하게 구현하시고 개념에 헷갈렸던 링크드 리스트에 대해 자세히 설명해주셨습니다.\r\n오랜시간 고민하며 문제를 해결하신 흔적을 볼 수 있었습니다.\r\n소스파일과 헤더파일에 대해 따로 디렉토리를 생성하셔서 기계채점이 어떻게 될 진 모르겠지만 부디 통과가 되길 바라겠습니다.\r\n수고많으셨습니다!",
        "created_at": "2024-03-01T12:04:22.287Z",
        "updated_at": "2024-03-01T15:43:33.881Z",
        "feedback": "제 시간에 와주셔서 평가를 바로 시작할 수 있었습니다. 모든 코드를 아주 꼼꼼하게 평가 해주셨습니다. 또 제가 미쳐 보지못했던 여러 요소들을 알려주셔서 다음에 어떤 부분을 수정해야 할지 알려주셨습니다. 감사합니다. :) 추가로 t_list를 테스트 케이스를 만드는 방법을 여쭤보셨는데 제대로 답변을 못드려서 죄송합니다.ㅠㅠ",
        "final_mark": 125,
        "flag": {
          "id": 1,
          "name": "Ok",
          "positive": true,
          "icon": "check-4",
          "created_at": "2015-09-14T23:06:52.000Z",
          "updated_at": "2015-09-14T23:06:52.000Z"
        },
        "begin_at": "2024-03-01T12:45:00.000Z",
        "correcteds": [
          {
            "id": 174096,
            "login": "junmin",
            "url": "https://api.intra.42.fr/v2/users/junmin"
          }
        ],
        "corrector": {
          "id": 174081,
          "login": "hyebinle",
          "url": "https://api.intra.42.fr/v2/users/hyebinle"
        },
        "truant": {},
        "filled_at": "2024-03-01T13:13:40.140Z",
        "questions_with_answers": []
      },
      {
        "id": 6466552,
        "scale_id": 32445,
        "comment": "안녕하세요 평가를 맡은 dae-lee 입니다. 꼼꼼하게 잘 작성하시고 테스트 해보신 부분이 느껴지셨습니다.  아쉽게도 다른 테스트 케이스에서 오류가 다수 발생해서, 그부분에 아쉬움이 있었지만, 금방 수정하실것 같습니다. make 파일도 높은이해도로 작성하신것 같습니다. make all 시 CFLAGS = -Wall -Wextra -Werror 구문으로 작은 에러가 발생하는 부분이 있어서 수정하시면 될것 같고, 테스트케이스를 잘 확인하셔서 빠르게 통과하시길 바랍니다 친절한 평가 진행 감사합니다!\r\n",
        "created_at": "2024-03-01T12:05:08.337Z",
        "updated_at": "2024-03-01T15:34:54.718Z",
        "feedback": "이전 평가가 너무 늦게 끝나서 거의 두시간이상 늦게 평가가 지연되었습니다. 그런데도 아주 친절하게 평가를 진행해주셨고 평가프로그램을 돌리는 것을 도와주셔서 덕분에 과제어서 어떤 점이 문제인지 또 어떻게 수정해야 하는지 알게되었고 사용하는 방법도 익힐 수 있었습니다 감사합니다:)",
        "final_mark": 0,
        "flag": {
          "id": 8,
          "name": "Crash",
          "positive": false,
          "icon": "bomb",
          "created_at": "2015-09-14T23:06:52.000Z",
          "updated_at": "2015-09-14T23:06:52.000Z"
        },
        "begin_at": "2024-03-01T13:15:00.000Z",
        "correcteds": [
          {
            "id": 174096,
            "login": "junmin",
            "url": "https://api.intra.42.fr/v2/users/junmin"
          }
        ],
        "corrector": {
          "id": 172352,
          "login": "dae-lee",
          "url": "https://api.intra.42.fr/v2/users/dae-lee"
        },
        "truant": {},
        "filled_at": "2024-03-01T15:32:27.252Z",
        "questions_with_answers": []
      },
      {
        "id": 6466553,
        "scale_id": 32445,
        "comment": "제가 함수마다 설명을 부탁드렸을때 아는 만큼 최대한 저를 이해시킬 수 있도록 설명을 해주셨습니다. 조금 더 공부하셨으면 더 깔끔한 설명이 되지 않았을까 싶었습니다. 고생하셨습니다.",
        "created_at": "2024-03-01T12:05:49.513Z",
        "updated_at": "2024-03-01T15:32:53.832Z",
        "feedback": "제가 평가를 너무 이른시간에 잡아서 이전 평가가 늦어 30분가량 늦게 시작하게 되었는데 편의를 봐주셔서 평가를 잘 진행할 수 있었습니다. 질문을 아주 날카롭게 해주셔서 제가 어떤 부분을 잘 모르는지 알게되는 평가였습니다. 덕분에 많이 배웠습니다. 감사합니다. :)",
        "final_mark": 125,
        "flag": {
          "id": 9,
          "name": "Outstanding project",
          "positive": true,
          "icon": "star-1",
          "created_at": "2017-05-18T14:07:37.380Z",
          "updated_at": "2017-05-18T14:12:07.415Z"
        },
        "begin_at": "2024-03-01T13:00:00.000Z",
        "correcteds": [
          {
            "id": 174096,
            "login": "junmin",
            "url": "https://api.intra.42.fr/v2/users/junmin"
          }
        ],
        "corrector": {
          "id": 174160,
          "login": "yeojukim",
          "url": "https://api.intra.42.fr/v2/users/yeojukim"
        },
        "truant": {},
        "filled_at": "2024-03-01T14:47:00.539Z",
        "questions_with_answers": []
      }
    ],
    "teams_uploads": [
      {
        "id": 2115561,
        "final_mark": 0,
        "comment": "initial_errors: KO (Does not compile) | test_ft_isalpha: KO (Does not compile), KO (Does not compile) | test_ft_isdigit: KO (Does not compile), KO (Does not compile) | test_ft_isalnum: KO (Does not compile), KO (Does not compile) | test_ft_isascii: KO (Does not compile), KO (Does not compile) | test_ft_isprint: KO (Does not compile), KO (Does not compile) | test_ft_strlen: KO (Does not compile), KO (Does not compile) | test_ft_memset: KO (Does not compile), KO (Does not compile) | test_ft_bzero: KO (Does not compile), KO (Does not compile) | test_ft_memcpy: KO (Does not compile), KO (Does not compile) | test_ft_memmove: KO (Does not compile), KO (Does not compile) | test_ft_strlcpy: KO (Does not compile), KO (Does not compile) | test_ft_strlcat: KO (Does not compile), KO (Does not compile) | test_ft_toupper: KO (Does not compile), KO (Does not compile) | test_ft_tolower: KO (Does not compile), KO (Does not compile) | test_ft_strchr: KO (Does not compile), KO (Does not compile) | test_ft_strrchr: KO (Does not compile), KO (Does not compile) | test_ft_strncmp: KO (Does not compile), KO (Does not compile) | test_ft_memchr: KO (Does not compile), KO (Does not compile) | test_ft_memcmp: KO (Does not compile), KO (Does not compile) | test_ft_strnstr: KO (Does not compile), KO (Does not compile) | test_ft_atoi: KO (Does not compile), KO (Does not compile) | test_ft_calloc: KO (Does not compile), KO (Does not compile) | test_ft_strdup: KO (Does not compile), KO (Does not compile) | test_ft_substr: KO (Does not compile), KO (Does not compile) | test_ft_strjoin: KO (Does not compile), KO (Does not compile) | test_ft_strtrim: KO (Does not compile), KO (Does not compile) | test_ft_split: KO (Does not compile), KO (Does not compile) | test_ft_itoa: KO (Does not compile), KO (Does not compile) | test_ft_strmapi: KO (Does not compile), KO (Does not compile) | test_ft_striteri: KO (Does not compile), KO (Does not compile) | test_ft_putchar_fd: KO (Does not compile), KO (Does not compile) | test_ft_putstr_fd: KO (Does not compile), KO (Does not compile) | test_ft_putendl_fd: KO (Does not compile), KO (Does not compile) | test_ft_putnbr_fd: KO (Does not compile), KO (Does not compile) | bonus: KO (Does not compile)",
        "created_at": "2024-03-01T15:43:44.879Z",
        "upload_id": 1558
      }
    ]
  },
  {
    "id": 5533748,
    "name": "taejikim's group",
    "url": "https://api.intra.42.fr/v2/teams/5533748",
    "final_mark": 72,
    "project_id": 1314,
    "created_at": "2024-02-26T00:48:55.012Z",
    "updated_at": "2024-02-29T02:17:36.165Z",
    "status": "finished",
    "terminating_at": null,
    "users": [
      {
        "id": 174083,
        "login": "taejikim",
        "url": "https://api.intra.42.fr/v2/users/taejikim",
        "leader": true,
        "occurrence": 0,
        "validated": true,
        "projects_user_id": 3565871
      }
    ],
    "locked?": true,
    "validated?": false,
    "closed?": true,
    "repo_url": "git@vogsphere.42gyeongsan.kr:vogsphere/intra-uuid-ed61f68c-50b0-47f5-95b7-5c64878cd8c9-5533748-taejikim",
    "repo_uuid": "intra-uuid-ed61f68c-50b0-47f5-95b7-5c64878cd8c9-5533748-taejikim",
    "locked_at": "2024-02-26T00:48:55.040Z",
    "closed_at": "2024-02-28T10:47:50.416Z",
    "project_session_id": 11805,
    "project_gitlab_path": "pedago_world/42-cursus/inner-circle/libft",
    "scale_teams": [
      {
        "id": 6455365,
        "scale_id": 32445,
        "comment": "안녕하세요. taejikim님의 libft평가를 맡은 myeochoi입니다. 평가를 진행하면서 비트 연산, malloc 0 할당 등 여러가지 개념들에 대해서 친절하게 설명해주셨습니다. 또한 첫번째 섹션에서는 man페이지에 있는 개념들에 대해서 정확히 이야기해주신 것을 보니 매우 꼼꼼하게 코드를 작성한 것을 알 수 있었고 앞으로 과제를 하는데 큰 도움이 될것같습니다. 이번 평가에서 원하시는 점수 받기를 바라겠습니다.",
        "created_at": "2024-02-28T12:50:35.765Z",
        "updated_at": "2024-02-29T02:17:19.119Z",
        "feedback": "안녕하세요 뵙게되어 반가웠습니다 각 함수마다 개괄적인 설명과 함께 변수의 초기값과 타입에 대한 이유설명을 적절하게 요구해주셨고 과정에서 제 코드의 취약점을 새로이 발견할 수 있었습니다 더욱 코드를 안전하게 만들어보겠습니다 감사합니다 평가 수고 많으셨습니다.",
        "final_mark": 125,
        "flag": {
          "id": 1,
          "name": "Ok",
          "positive": true,
          "icon": "check-4",
          "created_at": "2015-09-14T23:06:52.000Z",
          "updated_at": "2015-09-14T23:06:52.000Z"
        },
        "begin_at": "2024-02-29T01:00:00.000Z",
        "correcteds": [
          {
            "id": 174083,
            "login": "taejikim",
            "url": "https://api.intra.42.fr/v2/users/taejikim"
          }
        ],
        "corrector": {
          "id": 172342,
          "login": "myeochoi",
          "url": "https://api.intra.42.fr/v2/users/myeochoi"
        },
        "truant": {},
        "filled_at": "2024-02-29T02:15:33.985Z",
        "questions_with_answers": []
      },
      {
        "id": 6454453,
        "scale_id": 32445,
        "comment": "안녕하세요 taejikim님, libft 평가를 맡게된 wonhseo입니다. 앞선 평가자께서 말씀하셨듯이 ft_bzero에서 비트연산으로 효율을 높인 것이 아주 인상 깊었습니다. 각 함수들과 까다로운 질문들에 대한 설명도 완벽했고요. 쓰이지 않는 코드만 정리해주시면 더욱 좋을 것 같아요 (ft_split: stdio.h, ft_substr: s_len, ft_putstr_fd: comments)! 너무 유익한 시간이었습니다. 다음에도 꼭 뵙고 싶어요, 서로 왕래해요! 본과정 클리어까지 화이팅입니다 :D",
        "created_at": "2024-02-28T10:48:05.563Z",
        "updated_at": "2024-02-29T00:48:10.154Z",
        "feedback": "정말 인상깊은 평가였습니다 개괄적인 설명만 들으시고도 unused var를 잡아내시는 것을 보고 감탄했습니다 컴파일러가 지나치는 부분을 꼼꼼하게 봐주시는 걸 보고 진심으로 평가해주시는구나 생각했습니다 말씀해주신 부분 리트라이를 하게 된다면 수정하겠습니다 변수의 타입 선언의 이유도 체크해주시니 왜 그렇게 선언했는지 제 자신에게 질문하면서 코드의 개선 가능성을 발견할 수 있었습니다 동료학습의 표본이십니다 정말 감사드립니다 남은 본과정도 잘 부탁드립니다",
        "final_mark": 125,
        "flag": {
          "id": 9,
          "name": "Outstanding project",
          "positive": true,
          "icon": "star-1",
          "created_at": "2017-05-18T14:07:37.380Z",
          "updated_at": "2017-05-18T14:12:07.415Z"
        },
        "begin_at": "2024-02-28T23:45:00.000Z",
        "correcteds": [
          {
            "id": 174083,
            "login": "taejikim",
            "url": "https://api.intra.42.fr/v2/users/taejikim"
          }
        ],
        "corrector": {
          "id": 172319,
          "login": "wonhseo",
          "url": "https://api.intra.42.fr/v2/users/wonhseo"
        },
        "truant": {},
        "filled_at": "2024-02-29T00:44:37.472Z",
        "questions_with_answers": []
      },
      {
        "id": 6454456,
        "scale_id": 32445,
        "comment": "처음으로 libft 평가를 해보게 되어서 영광이었습니다. 문제를 풀면서 개인적으로 드는 의문점들이 많았는데 덕분에 많이 배워갈 수 있었습니다!! 그리고 동시에 컴퓨터 지식에 엄청 빠삭하셔서 정말 많이 놀라고 약간의 두려움을 가지고 돌아갑니다... 시간복잡도까지 고려하셔서 비트연산을 하려고 시도하신 부분도 멋졌고, 노드를 활용해야 하는 보너스 문제들까지 완벽하게 해결하신데다가 과제에서 요구하는 부분들을 꼼꼼하게 체크해두셔서 outstanding을 드릴 수 밖에 없었습니다..! 많이 배우고 갑니다 그리고 긴 평가시간 고생하셨어요!! 다음에 또 뵐 수 있으면 좋겠습니다. 고생하셨습니다!",
        "created_at": "2024-02-28T10:48:31.993Z",
        "updated_at": "2024-02-28T12:48:39.327Z",
        "feedback": "많은 함수들이 있었음에도 불구하고 모든 함수들에 대한 설명을 요구해주셨고 적절한 질문을 해주셨습니다 그 과정에서 제 코드의 취약점과 개선점을 발견하며 복기할 수 있었습니다 하지만 피평가자의 준비 미숙으로 인해서 설명이 매끄럽지 못했으나 잘 이해해주셔서 다행이었습니다 더욱 평가과정을 개선하겠습니다 평가 정말 수고 많으셨습니다 감사드립니다  (저도 도울 수 있는 일이 있다면 언제든지 질문해주세요! :D)",
        "final_mark": 125,
        "flag": {
          "id": 9,
          "name": "Outstanding project",
          "positive": true,
          "icon": "star-1",
          "created_at": "2017-05-18T14:07:37.380Z",
          "updated_at": "2017-05-18T14:12:07.415Z"
        },
        "begin_at": "2024-02-28T11:30:00.000Z",
        "correcteds": [
          {
            "id": 174083,
            "login": "taejikim",
            "url": "https://api.intra.42.fr/v2/users/taejikim"
          }
        ],
        "corrector": {
          "id": 172396,
          "login": "yeoju",
          "url": "https://api.intra.42.fr/v2/users/yeoju"
        },
        "truant": {},
        "filled_at": "2024-02-28T12:34:43.031Z",
        "questions_with_answers": []
      },
      {
        "id": 6454460,
        "scale_id": 32445,
        "comment": null,
        "created_at": "2024-02-28T10:49:28.409Z",
        "updated_at": "2024-02-28T12:44:38.950Z",
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
        "begin_at": "2024-02-28T12:15:00.000Z",
        "correcteds": [
          {
            "id": 174083,
            "login": "taejikim",
            "url": "https://api.intra.42.fr/v2/users/taejikim"
          }
        ],
        "corrector": {
          "id": 174084,
          "login": "changwpa",
          "url": "https://api.intra.42.fr/v2/users/changwpa"
        },
        "truant": {
          "id": 174083,
          "login": "taejikim",
          "url": "https://api.intra.42.fr/v2/users/taejikim"
        },
        "filled_at": null,
        "questions_with_answers": []
      }
    ],
    "teams_uploads": [
      {
        "id": 2112965,
        "final_mark": 19,
        "comment": "initial_errors:  | test_ft_isalpha: OK | test_ft_isdigit: OK | test_ft_isalnum: OK | test_ft_isascii: OK | test_ft_isprint: OK | test_ft_strlen: OK | test_ft_memset: OK | test_ft_bzero: Error encountered while testing | test_ft_memcpy: OK | test_ft_memmove: OK | test_ft_strlcpy: OK | test_ft_strlcat: OK | test_ft_toupper: OK | test_ft_tolower: OK | test_ft_strchr: OK | test_ft_strrchr: OK | test_ft_strncmp: OK | test_ft_memchr: OK | test_ft_memcmp: OK | test_ft_strnstr: OK | test_ft_atoi: OK | test_ft_calloc: OK | test_ft_strdup: OK | test_ft_substr: OK | test_ft_strjoin: OK | test_ft_strtrim: OK | test_ft_split: OK | test_ft_itoa: OK | test_ft_strmapi: OK | test_ft_striteri: OK | test_ft_putchar_fd: OK | test_ft_putstr_fd: OK | test_ft_putendl_fd: OK | test_ft_putnbr_fd: OK | bonus: 9/9 functions correct",
        "created_at": "2024-02-29T02:17:36.163Z",
        "upload_id": 1558
      }
    ]
  },
  {
    "id": 5533749,
    "name": "seungjo's group",
    "url": "https://api.intra.42.fr/v2/teams/5533749",
    "final_mark": 62,
    "project_id": 1314,
    "created_at": "2024-02-26T00:50:51.225Z",
    "updated_at": "2024-03-27T06:46:21.386Z",
    "status": "finished",
    "terminating_at": null,
    "users": [
      {
        "id": 174183,
        "login": "seungjo",
        "url": "https://api.intra.42.fr/v2/users/seungjo",
        "leader": true,
        "occurrence": 0,
        "validated": true,
        "projects_user_id": 3565872
      }
    ],
    "locked?": true,
    "validated?": false,
    "closed?": true,
    "repo_url": "git@vogsphere.42gyeongsan.kr:vogsphere/intra-uuid-1933aa19-1620-4855-93bb-3a5fc23609ec-5533749-seungjo",
    "repo_uuid": "intra-uuid-1933aa19-1620-4855-93bb-3a5fc23609ec-5533749-seungjo",
    "locked_at": "2024-02-26T00:50:51.267Z",
    "closed_at": "2024-03-26T22:33:39.250Z",
    "project_session_id": 11805,
    "project_gitlab_path": "pedago_world/42-cursus/inner-circle/libft",
    "scale_teams": [
      {
        "id": 6566963,
        "scale_id": 32445,
        "comment": "libft평가를 맡은 hisong입니다. 코드를 꼼꼼히 살펴봤는데 다른 함수를 불러서 사용할 수 있는 부분도 직접 설계하여 사용하여 그 함수에 맞는 설계를 볼 수 있었습니다. 보너스도 진행하셔서 제가 하지 않은 보너스에 대한 내용도 배울 수 있었습니다. 또 isalpha나 innum등 is함수에서 어떤 결과가 리턴되어야 하는지에 대해서 실제로 작동시켜보고 그 값을 그대로 넣은 점에 있어서 대단하다는 생각이 들었습니다. 많은 생각을 해볼 수 있는 평가 시간이었습니다. 좋은 결과 있으시길 바랍니다. 감사합니다.",
        "created_at": "2024-03-27T04:48:53.298Z",
        "updated_at": "2024-03-27T06:46:09.876Z",
        "feedback": "평가가 전체적으로 편안하고 유쾌한 분위기로 진행돼서 코드에 대해 설명드릴 때도 긴장하지 않고 막힘없이 설명할 수 있게되어 좋았습니다. 그리고 제가 42헤더에서 이메일 설정을 안하고 있었는데 하는 방법까지 알려주시고 직접 설정해주셔서 정말 감사했습니다. 추가로 함수 구현에서 다른 함수를 호출해서 코드를 단순화시키는 방법을 여러가지로 알려주셔서 정말 유익한 시간이었습니다. 특히 trim에서 굳이 static함수를 만들지 않고 strchr을 호출해도 가능하다는 걸 알려주셔서 정말 놀랐습니다. 정말 배우는 게 많은 시간이었습니다. 고생하셨습니다.",
        "final_mark": 125,
        "flag": {
          "id": 1,
          "name": "Ok",
          "positive": true,
          "icon": "check-4",
          "created_at": "2015-09-14T23:06:52.000Z",
          "updated_at": "2015-09-14T23:06:52.000Z"
        },
        "begin_at": "2024-03-27T05:30:00.000Z",
        "correcteds": [
          {
            "id": 174183,
            "login": "seungjo",
            "url": "https://api.intra.42.fr/v2/users/seungjo"
          }
        ],
        "corrector": {
          "id": 172362,
          "login": "hisong",
          "url": "https://api.intra.42.fr/v2/users/hisong"
        },
        "truant": {},
        "filled_at": "2024-03-27T06:42:53.537Z",
        "questions_with_answers": []
      },
      {
        "id": 6566656,
        "scale_id": 32445,
        "comment": "연결리스트에 대한 개념이 부족했는데 설명을 듣고 나니 어느정도 감이 잘 잡힙니다. 오늘 libft 평가가 ko가 나서 이참에 연결리스트까지 해서 평가하는 것도 좋을 것 같습니다. 좋은 설명에 감사드리며 좋은 결과 있기를 기대하겠습니다!",
        "created_at": "2024-03-27T01:52:48.754Z",
        "updated_at": "2024-03-27T04:47:59.936Z",
        "feedback": "연결리스트에 대해 설명을 요청해주셔서 최대한 설명해드렸는데 제가 아직 설명하는 능력이 조금 부족한 것 같습니다. 저도 배운지 며칠 안됐기 때문에 더 그런것 같습니다. 배운 내용을 설명하면서 좀 더 개념이 명확해진 것 같습니다. 설명 요청해주셔서 감사합니다.",
        "final_mark": 125,
        "flag": {
          "id": 9,
          "name": "Outstanding project",
          "positive": true,
          "icon": "star-1",
          "created_at": "2017-05-18T14:07:37.380Z",
          "updated_at": "2017-05-18T14:12:07.415Z"
        },
        "begin_at": "2024-03-27T04:00:00.000Z",
        "correcteds": [
          {
            "id": 174183,
            "login": "seungjo",
            "url": "https://api.intra.42.fr/v2/users/seungjo"
          }
        ],
        "corrector": {
          "id": 174168,
          "login": "jehbae",
          "url": "https://api.intra.42.fr/v2/users/jehbae"
        },
        "truant": {},
        "filled_at": "2024-03-27T04:45:27.753Z",
        "questions_with_answers": []
      },
      {
        "id": 6566512,
        "scale_id": 32445,
        "comment": "안녕하세요. 평가자 yuhyoon입니다. strlcat에서 dst가 널값일 때와 널일때를 질문드렸는데 실제 strlcat이 어떻게 작동하는지 일일히 다 체크를 해보신 분이여서 그런지 man을 다시 읽지 않아도 답변을 해주신게 놀라웠습니다. 제가 libft 최초평가라니... 오랜시간동안 과제하느라 수고하셨습니다!!! 다음 평가에서도 통과하시길 빌게요!!!! ",
        "created_at": "2024-03-26T22:34:00.649Z",
        "updated_at": "2024-03-27T01:52:03.593Z",
        "feedback": "strlcat 인자로 널과 널값이 들어왔을 때 각각 어떤 동작을 수행하냐고 질문해주셨을때 답변을 늦게드려 죄송합니다. 저는 strlcat을 string.h헤더에 있는 라이브러리 함수의 출력값을 직접 다 확인해본 후 그 값을 토대로 리턴 조건을 설정하여 코드를 짰습니다. 질문해주신 조건에 의하면 모두 같은 리턴 조건으로 빠져 나가는데 리턴값이 다르다고 말씀해주셔서 제가 잘 못 작성한 것인가 고민하느라 시간이 지체됐던 것 같습니다. 그리고 다른 함수들 모두 꼼꼼히 테스트해주셔서 문제 없이 통과할 것 같습니다. 꼼꼼히 테스트해주셔서 감사합니다.",
        "final_mark": 125,
        "flag": {
          "id": 1,
          "name": "Ok",
          "positive": true,
          "icon": "check-4",
          "created_at": "2015-09-14T23:06:52.000Z",
          "updated_at": "2015-09-14T23:06:52.000Z"
        },
        "begin_at": "2024-03-27T01:00:00.000Z",
        "correcteds": [
          {
            "id": 174183,
            "login": "seungjo",
            "url": "https://api.intra.42.fr/v2/users/seungjo"
          }
        ],
        "corrector": {
          "id": 172322,
          "login": "yuhyoon",
          "url": "https://api.intra.42.fr/v2/users/yuhyoon"
        },
        "truant": {},
        "filled_at": "2024-03-27T01:45:44.574Z",
        "questions_with_answers": []
      }
    ],
    "teams_uploads": [
      {
        "id": 2142462,
        "final_mark": 0,
        "comment": "initial_errors: Files besides the allowed ones were found on repository. | test_ft_isalpha: KO (Does not compile), KO (Does not compile) | test_ft_isdigit: KO (Does not compile), KO (Does not compile) | test_ft_isalnum: KO (Does not compile), KO (Does not compile) | test_ft_isascii: KO (Does not compile), KO (Does not compile) | test_ft_isprint: KO (Does not compile), KO (Does not compile) | test_ft_strlen: KO (Does not compile), KO (Does not compile) | test_ft_memset: KO (Does not compile), KO (Does not compile) | test_ft_bzero: KO (Does not compile), KO (Does not compile) | test_ft_memcpy: KO (Does not compile), KO (Does not compile) | test_ft_memmove: KO (Does not compile), KO (Does not compile) | test_ft_strlcpy: KO (Does not compile), KO (Does not compile) | test_ft_strlcat: KO (Does not compile), KO (Does not compile) | test_ft_toupper: KO (Does not compile), KO (Does not compile) | test_ft_tolower: KO (Does not compile), KO (Does not compile) | test_ft_strchr: KO (Does not compile), KO (Does not compile) | test_ft_strrchr: KO (Does not compile), KO (Does not compile) | test_ft_strncmp: KO (Does not compile), KO (Does not compile) | test_ft_memchr: KO (Does not compile), KO (Does not compile) | test_ft_memcmp: KO (Does not compile), KO (Does not compile) | test_ft_strnstr: KO (Does not compile), KO (Does not compile) | test_ft_atoi: KO (Does not compile), KO (Does not compile) | test_ft_calloc: KO (Does not compile), KO (Does not compile) | test_ft_strdup: KO (Does not compile), KO (Does not compile) | test_ft_substr: KO (Does not compile), KO (Does not compile) | test_ft_strjoin: KO (Does not compile), KO (Does not compile) | test_ft_strtrim: KO (Does not compile), KO (Does not compile) | test_ft_split: KO (Does not compile), KO (Does not compile) | test_ft_itoa: KO (Does not compile), KO (Does not compile) | test_ft_strmapi: KO (Does not compile), KO (Does not compile) | test_ft_striteri: KO (Does not compile), KO (Does not compile) | test_ft_putchar_fd: KO (Does not compile), KO (Does not compile) | test_ft_putstr_fd: KO (Does not compile), KO (Does not compile) | test_ft_putendl_fd: KO (Does not compile), KO (Does not compile) | test_ft_putnbr_fd: KO (Does not compile), KO (Does not compile) | bonus: KO (Does not compile)",
        "created_at": "2024-03-27T06:46:21.385Z",
        "upload_id": 1558
      }
    ]
  },
  {
    "id": 5533751,
    "name": "younhbae's group",
    "url": "https://api.intra.42.fr/v2/teams/5533751",
    "final_mark": 87,
    "project_id": 1314,
    "created_at": "2024-02-26T00:52:34.366Z",
    "updated_at": "2024-03-05T06:27:03.744Z",
    "status": "finished",
    "terminating_at": null,
    "users": [
      {
        "id": 172366,
        "login": "younhbae",
        "url": "https://api.intra.42.fr/v2/users/younhbae",
        "leader": true,
        "occurrence": 0,
        "validated": true,
        "projects_user_id": 3565873
      }
    ],
    "locked?": true,
    "validated?": false,
    "closed?": true,
    "repo_url": "git@vogsphere.42gyeongsan.kr:vogsphere/intra-uuid-0bcf10db-c542-4677-b480-bebd26f00097-5533751-younhbae",
    "repo_uuid": "intra-uuid-0bcf10db-c542-4677-b480-bebd26f00097-5533751-younhbae",
    "locked_at": "2024-02-26T00:52:34.401Z",
    "closed_at": "2024-03-05T01:18:15.917Z",
    "project_session_id": 11805,
    "project_gitlab_path": "pedago_world/42-cursus/inner-circle/libft",
    "scale_teams": [
      {
        "id": 6478728,
        "scale_id": 32445,
        "comment": "자신이 만든 함수를 자유롭게 사용하시는 모습이 정말 멋있는 것 같습니다. 나무를 보지 않고 숲을 보라는 말이 딱 맞는 분이신 것 같습니다. 코드도 깔끔하고 이해하기 쉽게 작성하셨고 저의 질분에도 친절하게 답해주셔서 감사했습니다.",
        "created_at": "2024-03-05T05:18:57.312Z",
        "updated_at": "2024-03-05T06:26:43.587Z",
        "feedback": "평가를 받게된 younhbae 입니다. 침착하게 함수 하나하나 매뉴얼 대로 평가해주시면서 제가 무심코 지나간 부분에 대해서 다시 확인할 수 있었습니다. 가끔 질문해주셨을 때 날카롭게 질문해주셔서 그 부분에 대해서도 다심 생각할 수 있었습니다. 앞으로도 많은 교류 했으면 좋겠습니다. 감사합니다",
        "final_mark": 125,
        "flag": {
          "id": 1,
          "name": "Ok",
          "positive": true,
          "icon": "check-4",
          "created_at": "2015-09-14T23:06:52.000Z",
          "updated_at": "2015-09-14T23:06:52.000Z"
        },
        "begin_at": "2024-03-05T06:00:00.000Z",
        "correcteds": [
          {
            "id": 172366,
            "login": "younhbae",
            "url": "https://api.intra.42.fr/v2/users/younhbae"
          }
        ],
        "corrector": {
          "id": 174095,
          "login": "jisopark",
          "url": "https://api.intra.42.fr/v2/users/jisopark"
        },
        "truant": {},
        "filled_at": "2024-03-05T06:25:21.345Z",
        "questions_with_answers": []
      },
      {
        "id": 6478527,
        "scale_id": 32445,
        "comment": "라이브러리에 들어가는 모든 함수들을 정확하게 파악하고 계시며, 함수에 대한 질문이 있었을 때, 설명이 막히지 않고 잘 설명해주었습니다. \r\nmalloc 함수를 이용하는 문제들은 calloc 함수를 이용하여, null 방어를 하는 부분이 인상적이었으며,\r\n작성했던 함수들 이용해 문제를 풀었던 부분에서, 미리 만들어 놓은 함수드를 이용해봐야겠다는 생각도 들었습니다.\r\n해당 make 파일을 실행했을 때 오류가 없었으며, 가이드에 맞게 작성해주셨습니다. 보너스 문제들 또한 오류없이 잘 만들어 주었습니다. 현재 들은 설명과 결과로 보아 pass 하실 것 같습니다.\r\n평가 하시느라 고생 많으셨습니다.",
        "created_at": "2024-03-05T02:17:14.639Z",
        "updated_at": "2024-03-05T05:20:18.878Z",
        "feedback": "평가를 받게된 younhbae 입니다. 각 함수에 대해서 주신 질문에 대해서 설명해드리면서 함수의 작동방식에 대해서 다시한번 리마인드할 수 있게 되었습니다. libft 구현을 거의 완료하셔서 그런지 함수에 대한 이해도 빠르셔서 설명을 무난히 마무리 할 수 있었습니다. 앞으로도 자주 교류했으면 좋겠습니다. 감사합니다.",
        "final_mark": 125,
        "flag": {
          "id": 1,
          "name": "Ok",
          "positive": true,
          "icon": "check-4",
          "created_at": "2015-09-14T23:06:52.000Z",
          "updated_at": "2015-09-14T23:06:52.000Z"
        },
        "begin_at": "2024-03-05T04:30:00.000Z",
        "correcteds": [
          {
            "id": 172366,
            "login": "younhbae",
            "url": "https://api.intra.42.fr/v2/users/younhbae"
          }
        ],
        "corrector": {
          "id": 174137,
          "login": "jemoon",
          "url": "https://api.intra.42.fr/v2/users/jemoon"
        },
        "truant": {},
        "filled_at": "2024-03-05T05:18:04.557Z",
        "questions_with_answers": []
      },
      {
        "id": 6478472,
        "scale_id": 32445,
        "comment": "안녕하세요. 평가를 맡게된 김승현입니다. 해당 과제 꼼꼼하게 잘 이해하고 계셨고 그를 바탕으로 구현도 잘해두셨습니다.  만들어 둔 함수를 적극적으로 잘 활용하시는 모습이 인상적이었고 많이들 ko를 당하는 calloc의 예외처리에 대해서 배울 수 있어 제 코드에도 활용하면 좋을 것 같습니다. 오랜기간 수고하신만큼 꼭 좋은 결과 얻으시길 바랍니다. 감사합니다.",
        "created_at": "2024-03-05T01:18:25.786Z",
        "updated_at": "2024-03-05T02:15:45.576Z",
        "feedback": "안녕하세요 평가를 받게된 younhbae입니다. 진도를 빠르게 진행하셔서 그런지 각 함수의 검토를 매우 빠르게 진행해주셔서 좋았습니다. 벌써 printf와 get next line까지 나가셨는데, 궁금한점있으면 많이 불어볼께요!! 평가 감사합니다.!",
        "final_mark": 125,
        "flag": {
          "id": 1,
          "name": "Ok",
          "positive": true,
          "icon": "check-4",
          "created_at": "2015-09-14T23:06:52.000Z",
          "updated_at": "2015-09-14T23:06:52.000Z"
        },
        "begin_at": "2024-03-05T02:00:00.000Z",
        "correcteds": [
          {
            "id": 172366,
            "login": "younhbae",
            "url": "https://api.intra.42.fr/v2/users/younhbae"
          }
        ],
        "corrector": {
          "id": 172385,
          "login": "sunghyki",
          "url": "https://api.intra.42.fr/v2/users/sunghyki"
        },
        "truant": {},
        "filled_at": "2024-03-05T02:14:22.322Z",
        "questions_with_answers": []
      }
    ],
    "teams_uploads": [
      {
        "id": 2118414,
        "final_mark": 49,
        "comment": "initial_errors:  | test_ft_isalpha: OK | test_ft_isdigit: OK | test_ft_isalnum: OK | test_ft_isascii: OK | test_ft_isprint: OK | test_ft_strlen: OK | test_ft_memset: OK | test_ft_bzero: OK | test_ft_memcpy: OK | test_ft_memmove: OK | test_ft_strlcpy: OK | test_ft_strlcat: OK | test_ft_toupper: OK | test_ft_tolower: OK | test_ft_strchr: OK | test_ft_strrchr: OK | test_ft_strncmp: OK | test_ft_memchr: Error encountered while testing | test_ft_memcmp: OK | test_ft_strnstr: OK | test_ft_atoi: OK | test_ft_calloc: OK | test_ft_strdup: OK | test_ft_substr: OK | test_ft_strjoin: OK | test_ft_strtrim: OK | test_ft_split: OK | test_ft_itoa: OK | test_ft_strmapi: OK | test_ft_striteri: OK | test_ft_putchar_fd: OK | test_ft_putstr_fd: OK | test_ft_putendl_fd: OK | test_ft_putnbr_fd: OK | bonus: 9/9 functions correct",
        "created_at": "2024-03-05T06:27:03.742Z",
        "upload_id": 1558
      }
    ]
  },
  {
    "id": 5533752,
    "name": "yunseo's group",
    "url": "https://api.intra.42.fr/v2/teams/5533752",
    "final_mark": 59,
    "project_id": 1314,
    "created_at": "2024-02-26T00:53:13.454Z",
    "updated_at": "2024-03-07T07:40:12.957Z",
    "status": "finished",
    "terminating_at": null,
    "users": [
      {
        "id": 172360,
        "login": "yunseo",
        "url": "https://api.intra.42.fr/v2/users/yunseo",
        "leader": true,
        "occurrence": 0,
        "validated": true,
        "projects_user_id": 3565874
      }
    ],
    "locked?": true,
    "validated?": false,
    "closed?": true,
    "repo_url": "git@vogsphere.42gyeongsan.kr:vogsphere/intra-uuid-c2397ad0-2343-4bce-bffe-a355fe126a2e-5533752-yunseo",
    "repo_uuid": "intra-uuid-c2397ad0-2343-4bce-bffe-a355fe126a2e-5533752-yunseo",
    "locked_at": "2024-02-26T00:53:13.496Z",
    "closed_at": "2024-03-06T12:51:16.883Z",
    "project_session_id": 11805,
    "project_gitlab_path": "pedago_world/42-cursus/inner-circle/libft",
    "scale_teams": [
      {
        "id": 6487984,
        "scale_id": 32445,
        "comment": "아직 part1밖에 해결하지 못해서 part2와 bonus문제에 대한 설명을 들었는데 한문제 한문제 제가 모르는 개념에 대해 친절히 설명해주셨습니다. 모든 문제를 가독성 좋게 코드를 짜셔서 제가 코드를 이해하는데 수월했습니다. 마지막 평가라하셨는데 좋은 결과 있길 바랍니다 :)",
        "created_at": "2024-03-07T05:52:10.362Z",
        "updated_at": "2024-03-07T07:39:56.820Z",
        "feedback": "평가시간 내내 제 말에 귀기울여서 들어주시고, 빈약한 설명 이였지만 들어주셔서 너무 감사했습니다. 해결하지 못한 과제 금방 해결 하셨으면 좋겠습니다! 평가 고생 많으셨습니다!",
        "final_mark": 125,
        "flag": {
          "id": 1,
          "name": "Ok",
          "positive": true,
          "icon": "check-4",
          "created_at": "2015-09-14T23:06:52.000Z",
          "updated_at": "2015-09-14T23:06:52.000Z"
        },
        "begin_at": "2024-03-07T07:00:00.000Z",
        "correcteds": [
          {
            "id": 172360,
            "login": "yunseo",
            "url": "https://api.intra.42.fr/v2/users/yunseo"
          }
        ],
        "corrector": {
          "id": 174129,
          "login": "hwasong",
          "url": "https://api.intra.42.fr/v2/users/hwasong"
        },
        "truant": {},
        "filled_at": "2024-03-07T07:38:58.086Z",
        "questions_with_answers": []
      },
      {
        "id": 6487945,
        "scale_id": 32445,
        "comment": "안녕하십니까! bonikoo 입니다. 저번에 평가해주셨던 대로, 꼼꼼하게 작성을 잘 하신것 같습니다. 보너스 파트 처리 또한 깔끔하게 잘 해주셨고, 전체적으로 코드의 간결함이 돋보였습니다. 보너스 파트의 마지막 부분을 간략하게 평가하였습니다만, 다음에 제가 따로 여쭤보겠습니다. 전체적으로 잘 하셨습니다. 감사합니다!",
        "created_at": "2024-03-07T05:22:42.289Z",
        "updated_at": "2024-03-07T06:46:39.862Z",
        "feedback": "제가 작성한 코드에 대해서 하나하나 설명 잘 들어주시고, 궁금한 점에 대해서 바로바로 질문 해주셔서 좋았습니다. 서로 함수에 대해서 깊게 이야기 나눌 수 있는 좋은 시간이였던 것 같습니다. 평가 고생 많으셨습니다!\r\n",
        "final_mark": 125,
        "flag": {
          "id": 1,
          "name": "Ok",
          "positive": true,
          "icon": "check-4",
          "created_at": "2015-09-14T23:06:52.000Z",
          "updated_at": "2015-09-14T23:06:52.000Z"
        },
        "begin_at": "2024-03-07T06:00:00.000Z",
        "correcteds": [
          {
            "id": 172360,
            "login": "yunseo",
            "url": "https://api.intra.42.fr/v2/users/yunseo"
          }
        ],
        "corrector": {
          "id": 172353,
          "login": "bonikoo",
          "url": "https://api.intra.42.fr/v2/users/bonikoo"
        },
        "truant": {},
        "filled_at": "2024-03-07T06:45:00.930Z",
        "questions_with_answers": []
      },
      {
        "id": 6485303,
        "scale_id": 32445,
        "comment": "아쉽게도 strlcat의 dest가 null, size가 0이고 src가 null이 아닐 시에 src의 문자열 길이가 아닌 0을 반환하게 되는 오류가 있어서 해당 부분에서 오답을 드리게 되었습니다. 코드가 간결하여 틀린 부분의 수정이 용이하여 다음 평가에서는 만점을 받으실 수 있을 것으로 생각됩니다.",
        "created_at": "2024-03-06T12:51:25.848Z",
        "updated_at": "2024-03-06T14:39:37.460Z",
        "feedback": "코드 한줄 한줄 깊게 생각하시면서 코드 분석해주시는게 너무 좋았습니다.\r\n특히 strlcat 함수에서 BSD/string.h 헤더파일을 이용해서 strlcat 원본과 비교해주시는 디테일이 인상 깊었습니다.\r\nNULL 가드 관련해서 많은 이야기 나눌 수 있어서 좋았습니다.\r\n평가 고생 많으셨습니다!",
        "final_mark": 109,
        "flag": {
          "id": 1,
          "name": "Ok",
          "positive": true,
          "icon": "check-4",
          "created_at": "2015-09-14T23:06:52.000Z",
          "updated_at": "2015-09-14T23:06:52.000Z"
        },
        "begin_at": "2024-03-06T13:30:00.000Z",
        "correcteds": [
          {
            "id": 172360,
            "login": "yunseo",
            "url": "https://api.intra.42.fr/v2/users/yunseo"
          }
        ],
        "corrector": {
          "id": 174094,
          "login": "younam",
          "url": "https://api.intra.42.fr/v2/users/younam"
        },
        "truant": {},
        "filled_at": "2024-03-06T14:37:27.473Z",
        "questions_with_answers": []
      }
    ],
    "teams_uploads": [
      {
        "id": 2121121,
        "final_mark": 0,
        "comment": "initial_errors: KO (Does not compile) | test_ft_isalpha: KO (Does not compile), KO (Does not compile) | test_ft_isdigit: KO (Does not compile), KO (Does not compile) | test_ft_isalnum: KO (Does not compile), KO (Does not compile) | test_ft_isascii: KO (Does not compile), KO (Does not compile) | test_ft_isprint: KO (Does not compile), KO (Does not compile) | test_ft_strlen: KO (Does not compile), KO (Does not compile) | test_ft_memset: KO (Does not compile), KO (Does not compile) | test_ft_bzero: KO (Does not compile), KO (Does not compile) | test_ft_memcpy: KO (Does not compile), KO (Does not compile) | test_ft_memmove: KO (Does not compile), KO (Does not compile) | test_ft_strlcpy: KO (Does not compile), KO (Does not compile) | test_ft_strlcat: KO (Does not compile), KO (Does not compile) | test_ft_toupper: KO (Does not compile), KO (Does not compile) | test_ft_tolower: KO (Does not compile), KO (Does not compile) | test_ft_strchr: KO (Does not compile), KO (Does not compile) | test_ft_strrchr: KO (Does not compile), KO (Does not compile) | test_ft_strncmp: KO (Does not compile), KO (Does not compile) | test_ft_memchr: KO (Does not compile), KO (Does not compile) | test_ft_memcmp: KO (Does not compile), KO (Does not compile) | test_ft_strnstr: KO (Does not compile), KO (Does not compile) | test_ft_atoi: KO (Does not compile), KO (Does not compile) | test_ft_calloc: KO (Does not compile), KO (Does not compile) | test_ft_strdup: KO (Does not compile), KO (Does not compile) | test_ft_substr: KO (Does not compile), KO (Does not compile) | test_ft_strjoin: KO (Does not compile), KO (Does not compile) | test_ft_strtrim: KO (Does not compile), KO (Does not compile) | test_ft_split: KO (Does not compile), KO (Does not compile) | test_ft_itoa: KO (Does not compile), KO (Does not compile) | test_ft_strmapi: KO (Does not compile), KO (Does not compile) | test_ft_striteri: KO (Does not compile), KO (Does not compile) | test_ft_putchar_fd: KO (Does not compile), KO (Does not compile) | test_ft_putstr_fd: KO (Does not compile), KO (Does not compile) | test_ft_putendl_fd: KO (Does not compile), KO (Does not compile) | test_ft_putnbr_fd: KO (Does not compile), KO (Does not compile) | bonus: KO (Does not compile)",
        "created_at": "2024-03-07T07:40:12.956Z",
        "upload_id": 1558
      }
    ]
  },
  {
    "id": 5533753,
    "name": "jehbae's group",
    "url": "https://api.intra.42.fr/v2/teams/5533753",
    "final_mark": 50,
    "project_id": 1314,
    "created_at": "2024-02-26T00:54:07.650Z",
    "updated_at": "2024-03-26T00:31:33.980Z",
    "status": "finished",
    "terminating_at": null,
    "users": [
      {
        "id": 174168,
        "login": "jehbae",
        "url": "https://api.intra.42.fr/v2/users/jehbae",
        "leader": true,
        "occurrence": 0,
        "validated": true,
        "projects_user_id": 3565875
      }
    ],
    "locked?": true,
    "validated?": false,
    "closed?": true,
    "repo_url": "git@vogsphere.42gyeongsan.kr:vogsphere/intra-uuid-f609f655-2271-4388-8dc6-c89111dc3110-5533753-jehbae",
    "repo_uuid": "intra-uuid-f609f655-2271-4388-8dc6-c89111dc3110-5533753-jehbae",
    "locked_at": "2024-02-26T00:54:07.682Z",
    "closed_at": "2024-03-25T11:56:04.285Z",
    "project_session_id": 11805,
    "project_gitlab_path": "pedago_world/42-cursus/inner-circle/libft",
    "scale_teams": [
      {
        "id": 6559998,
        "scale_id": 32445,
        "comment": "안녕하세요. younhbae 입니다. libft 평가를 하면서 코드가 매우 깔끔하여 평가하는데 좋았습니다. 다만 a.out 파일이 git에 업로드되어 있었는데, 이 부분은 기계평가를 통해서 채점을 해봐야 할 것 같습니다. 시간이 되시면 보너스도 해보시면 좋을 것 같아요 감사합니다!",
        "created_at": "2024-03-25T11:56:12.369Z",
        "updated_at": "2024-03-26T00:31:21.400Z",
        "feedback": "우선 친절한 평가에 감사드립니다. 코드 하나하나를 깊게 분석하시면서 궁금한 점에 대해 질문해주셔서 좋았습니다. 저 역시 질문에 답변하며 다시 학습할 수 있게 되었고 어떤 의미를 가지고 코드를 만들었는지 생각해보게 되었습니다. 코드에 대한 추가 설명도 곁들어서 평가를 진행해주셔서 몰랐던 점을 알 수 있었고 앞으로 코드 작성에 있어서 큰 도움이 될 것 같습니다.",
        "final_mark": 100,
        "flag": {
          "id": 1,
          "name": "Ok",
          "positive": true,
          "icon": "check-4",
          "created_at": "2015-09-14T23:06:52.000Z",
          "updated_at": "2015-09-14T23:06:52.000Z"
        },
        "begin_at": "2024-03-26T00:15:00.000Z",
        "correcteds": [
          {
            "id": 174168,
            "login": "jehbae",
            "url": "https://api.intra.42.fr/v2/users/jehbae"
          }
        ],
        "corrector": {
          "id": 172366,
          "login": "younhbae",
          "url": "https://api.intra.42.fr/v2/users/younhbae"
        },
        "truant": {},
        "filled_at": "2024-03-26T00:29:19.376Z",
        "questions_with_answers": []
      },
      {
        "id": 6560569,
        "scale_id": 32445,
        "comment": "오랜만에 전체 libft를 살펴봤습니다. 여러가지 오류가 발생 될 수도 있는 함수의 예외값들을 위주로 봤습니다. 제가 생각했던 특수값들의 방어는잘 되어있었고 코드 진행도 무리 없어 보입니다.\r\n평가 진행하면서 여러 파일들의 나열을 봤을때 작성하시는 부분이 발전하는 것이 눈에 보였습니다.\r\n고생많으셨습니다~!!!",
        "created_at": "2024-03-25T13:29:15.162Z",
        "updated_at": "2024-03-25T14:54:23.935Z",
        "feedback": "코드를 꼼꼼히 분석하시면서 예외처리와 오류 가능성에 대한 이야기를 주로 나눴습니다. 그중에서도 ft_substr의 경우 malloc 함수를 사용하여 len만큼의 메모리 할당이 발생하게 코드를 구성했는데 len만큼의 공간을 다 채우지 못할 경우 남는 값들에 대해서 어떤 값으로 남겨두어야 하느냐에 대한 이야기를 했습니다. 그 결과 안정적인 측면에서 malloc 함수가 아닌 calloc 함수를 사용하여 쓰레기 값이 아닌 0의 값으로 채워주는 것이 오류 발생 가능성을 낮추고 더 좋을 것 같다고 느꼈습니다. 평가로 인하여 코드의 보완점을 확인할 수 있었고 다음 시도 때 보다 나은 코드로 시도할 수 있을 것 같습니다. 친절하게 평가 잘해주셔서 감사드리며 계속해서 좋은 평가 많이 해주시길 바라겠습니다~!",
        "final_mark": 100,
        "flag": {
          "id": 1,
          "name": "Ok",
          "positive": true,
          "icon": "check-4",
          "created_at": "2015-09-14T23:06:52.000Z",
          "updated_at": "2015-09-14T23:06:52.000Z"
        },
        "begin_at": "2024-03-25T14:00:00.000Z",
        "correcteds": [
          {
            "id": 174168,
            "login": "jehbae",
            "url": "https://api.intra.42.fr/v2/users/jehbae"
          }
        ],
        "corrector": {
          "id": 174133,
          "login": "kyouhele",
          "url": "https://api.intra.42.fr/v2/users/kyouhele"
        },
        "truant": {},
        "filled_at": "2024-03-25T14:43:06.060Z",
        "questions_with_answers": []
      },
      {
        "id": 6559999,
        "scale_id": 32445,
        "comment": "libft중 기억나지 않는 함수가 많았는데 설명을 잘해주셔서 이해하는데 무리가 없었습니다. 궁금한 점들도 많이 해결이 되어서 많이 배우고 가는 평가였습니다. 앞으로도 고생하세요!",
        "created_at": "2024-03-25T11:56:32.743Z",
        "updated_at": "2024-03-25T13:50:18.777Z",
        "feedback": "정시에 오셔서 평가 잘해주셨습니다. 궁금하신 부분에 대해서 질문해주셔서 답변을 드리며 코드에 대해 복기할 수 있어서 좋았습니다. 코드에 대한 개념뿐만 아니라 어떻게 활용이 될까라는 질문들도 던져주셔서 서로 이야기하며 지식을 확장해 나갈 수 있었습니다. 좋은 평가 감사합니다!",
        "final_mark": 100,
        "flag": {
          "id": 9,
          "name": "Outstanding project",
          "positive": true,
          "icon": "star-1",
          "created_at": "2017-05-18T14:07:37.380Z",
          "updated_at": "2017-05-18T14:12:07.415Z"
        },
        "begin_at": "2024-03-25T12:30:00.000Z",
        "correcteds": [
          {
            "id": 174168,
            "login": "jehbae",
            "url": "https://api.intra.42.fr/v2/users/jehbae"
          }
        ],
        "corrector": {
          "id": 172364,
          "login": "hchin",
          "url": "https://api.intra.42.fr/v2/users/hchin"
        },
        "truant": {},
        "filled_at": "2024-03-25T13:28:24.817Z",
        "questions_with_answers": []
      }
    ],
    "teams_uploads": [
      {
        "id": 2141349,
        "final_mark": 0,
        "comment": "initial_errors: Files besides the allowed ones were found on repository. | test_ft_isalpha: KO (Does not compile), KO (Does not compile) | test_ft_isdigit: KO (Does not compile), KO (Does not compile) | test_ft_isalnum: KO (Does not compile), KO (Does not compile) | test_ft_isascii: KO (Does not compile), KO (Does not compile) | test_ft_isprint: KO (Does not compile), KO (Does not compile) | test_ft_strlen: KO (Does not compile), KO (Does not compile) | test_ft_memset: KO (Does not compile), KO (Does not compile) | test_ft_bzero: KO (Does not compile), KO (Does not compile) | test_ft_memcpy: KO (Does not compile), KO (Does not compile) | test_ft_memmove: KO (Does not compile), KO (Does not compile) | test_ft_strlcpy: KO (Does not compile), KO (Does not compile) | test_ft_strlcat: KO (Does not compile), KO (Does not compile) | test_ft_toupper: KO (Does not compile), KO (Does not compile) | test_ft_tolower: KO (Does not compile), KO (Does not compile) | test_ft_strchr: KO (Does not compile), KO (Does not compile) | test_ft_strrchr: KO (Does not compile), KO (Does not compile) | test_ft_strncmp: KO (Does not compile), KO (Does not compile) | test_ft_memchr: KO (Does not compile), KO (Does not compile) | test_ft_memcmp: KO (Does not compile), KO (Does not compile) | test_ft_strnstr: KO (Does not compile), KO (Does not compile) | test_ft_atoi: KO (Does not compile), KO (Does not compile) | test_ft_calloc: KO (Does not compile), KO (Does not compile) | test_ft_strdup: KO (Does not compile), KO (Does not compile) | test_ft_substr: KO (Does not compile), KO (Does not compile) | test_ft_strjoin: KO (Does not compile), KO (Does not compile) | test_ft_strtrim: KO (Does not compile), KO (Does not compile) | test_ft_split: KO (Does not compile), KO (Does not compile) | test_ft_itoa: KO (Does not compile), KO (Does not compile) | test_ft_strmapi: KO (Does not compile), KO (Does not compile) | test_ft_striteri: KO (Does not compile), KO (Does not compile) | test_ft_putchar_fd: KO (Does not compile), KO (Does not compile) | test_ft_putstr_fd: KO (Does not compile), KO (Does not compile) | test_ft_putendl_fd: KO (Does not compile), KO (Does not compile) | test_ft_putnbr_fd: KO (Does not compile), KO (Does not compile) | bonus: KO (Does not compile)",
        "created_at": "2024-03-26T00:31:33.978Z",
        "upload_id": 1558
      }
    ]
  },
  {
    "id": 5533754,
    "name": "kyouhele's group",
    "url": "https://api.intra.42.fr/v2/teams/5533754",
    "final_mark": 62,
    "project_id": 1314,
    "created_at": "2024-02-26T00:55:40.262Z",
    "updated_at": "2024-03-06T11:20:29.193Z",
    "status": "finished",
    "terminating_at": null,
    "users": [
      {
        "id": 174133,
        "login": "kyouhele",
        "url": "https://api.intra.42.fr/v2/users/kyouhele",
        "leader": true,
        "occurrence": 0,
        "validated": true,
        "projects_user_id": 3565876
      }
    ],
    "locked?": true,
    "validated?": false,
    "closed?": true,
    "repo_url": "git@vogsphere.42gyeongsan.kr:vogsphere/intra-uuid-6758df01-e8c2-458a-8fa0-d68ce0f8fae8-5533754-kyouhele",
    "repo_uuid": "intra-uuid-6758df01-e8c2-458a-8fa0-d68ce0f8fae8-5533754-kyouhele",
    "locked_at": "2024-02-26T00:55:40.300Z",
    "closed_at": "2024-03-06T07:35:59.356Z",
    "project_session_id": 11805,
    "project_gitlab_path": "pedago_world/42-cursus/inner-circle/libft",
    "scale_teams": [
      {
        "id": 6483605,
        "scale_id": 32445,
        "comment": "전체적으로 코드를 고민을 많이 하시고 구성하신게 눈에 보였습니다. 특히 다른 분들을 평가할 때 보지 못했던 시도들과 참신한 로직을 볼 수 있어서 다른 방식으로 접근할 수 있는 것을 확인할 수 있었습니다. 고생하셨습니다. ",
        "created_at": "2024-03-06T07:36:46.822Z",
        "updated_at": "2024-03-06T11:18:08.512Z",
        "feedback": "이미 풀이하신 문제들이라 굉장히 스무스하게 평가를 진행했습니다.\r\n또한 제가 방어하지 못한 부분에 대해서 한 줄 한 줄 유의미한 피드백을 주셨습니다.\r\n많이 배웠습니다! 처음 받는 평가이기 때문에 소소하게 많은 팁을 주셔서 감사히 날먹 하겠습니다! 고생 많으셨습니다!",
        "final_mark": 109,
        "flag": {
          "id": 1,
          "name": "Ok",
          "positive": true,
          "icon": "check-4",
          "created_at": "2015-09-14T23:06:52.000Z",
          "updated_at": "2015-09-14T23:06:52.000Z"
        },
        "begin_at": "2024-03-06T09:15:00.000Z",
        "correcteds": [
          {
            "id": 174133,
            "login": "kyouhele",
            "url": "https://api.intra.42.fr/v2/users/kyouhele"
          }
        ],
        "corrector": {
          "id": 174188,
          "login": "hocjeong",
          "url": "https://api.intra.42.fr/v2/users/hocjeong"
        },
        "truant": {},
        "filled_at": "2024-03-06T10:04:40.981Z",
        "questions_with_answers": []
      },
      {
        "id": 6483610,
        "scale_id": 32445,
        "comment": "평가자 ksuh입니다. 함수들이 전체적으로 깔끔하고 재사용성이 높게 작성되었습니다. ft_strnstr의 경우 ft_strncmp함수와 포인터 위치의 비교를 하신 방법은 정말 새로웠습니다. 미처 생각하지 못한 간결한 방법으로 구현하셨지만, 아쉽게도 로직에 약간의 오류가 있어서 no를 드렸습니다. 다음번에는 말씀하셨던 재귀함수라던지 while문으로 다시 도전해 보시면 좋은 결과 있으실겁니다. 수고하셨습니다!",
        "created_at": "2024-03-06T07:38:36.733Z",
        "updated_at": "2024-03-06T11:20:09.688Z",
        "feedback": "이미 풀이하신 문제들이라 굉장히 스무스하게 평가를 진행했습니다.\r\n평이하게 풀이한 것들 조차도 한 줄 한줄 세심히 봐주셨고 기계평가 진행 전이라 걱정되는 부분들에 대해 많은 피드백을 주셨습니다!\r\n한시간이 넘는 시간동안 평가를 진행했는데 엄청난 실력자임을 느꼈습니다! 많이 배웠습니다!",
        "final_mark": 34,
        "flag": {
          "id": 1,
          "name": "Ok",
          "positive": true,
          "icon": "check-4",
          "created_at": "2015-09-14T23:06:52.000Z",
          "updated_at": "2015-09-14T23:06:52.000Z"
        },
        "begin_at": "2024-03-06T10:15:00.000Z",
        "correcteds": [
          {
            "id": 174133,
            "login": "kyouhele",
            "url": "https://api.intra.42.fr/v2/users/kyouhele"
          }
        ],
        "corrector": {
          "id": 172367,
          "login": "ksuh",
          "url": "https://api.intra.42.fr/v2/users/ksuh"
        },
        "truant": {},
        "filled_at": "2024-03-06T11:09:03.470Z",
        "questions_with_answers": []
      },
      {
        "id": 6483604,
        "scale_id": 32445,
        "comment": "모든 문제를 다 해결 하셨습니다. 평균적으로 해결하는 방법과는 다르게 문제를 해결 하셔서 한문제 한문제 보면서 많이 배우는 기회가 되었습니다. 여러가지 다양한 방법으로 문제를 풀 수 있는 능력을 이번 평가를 통해서 배울 수 있었던 것 같습니다. 중간에 획기적인 방법으로 문제를 해결 하셨었는데 한 군데 생각을 못하시고 푸셔서 NO드렸습니다. 그 문제도 처음 보는 방법으로 풀이하셔서 많이 배웠습니다. 전반적으로 많은 테스트 케이스와, 예외처리를 생각 하시면서 문제를 푸신 것 같은 모습이 잘 보였습니다. 평가를 하면서 배운 예외처리나 방어등 많은 것들을 제 과제를 해결 할 때 사용한다면 많은 도움이 될 것 같습니다. 화이팅 하시고 건강하세요!",
        "created_at": "2024-03-06T07:36:27.058Z",
        "updated_at": "2024-03-06T11:14:41.453Z",
        "feedback": "안녕하세요! 본과정 첫번째 평가를 gseok님께 받게 되어 영광입니다!\r\n모든 문제를 세세하게 살펴보고, 성실히 평가에 임해주셔서 고마워요!\r\n특히 스스로 너무 잘짰다면서 신나 있던 코드에 대해서 매의 눈으로 눈버깅 하시더니 딱! 동작하지 못한 부분을 짚어주셔서 오답이란 것에 속상한 것보다 gseok님의 눈썰미에 감탄하게 되었습니다\r\n평가 고생많으셨습니다!.",
        "final_mark": 84,
        "flag": {
          "id": 1,
          "name": "Ok",
          "positive": true,
          "icon": "check-4",
          "created_at": "2015-09-14T23:06:52.000Z",
          "updated_at": "2015-09-14T23:06:52.000Z"
        },
        "begin_at": "2024-03-06T08:15:00.000Z",
        "correcteds": [
          {
            "id": 174133,
            "login": "kyouhele",
            "url": "https://api.intra.42.fr/v2/users/kyouhele"
          }
        ],
        "corrector": {
          "id": 174131,
          "login": "gseok",
          "url": "https://api.intra.42.fr/v2/users/gseok"
        },
        "truant": {},
        "filled_at": "2024-03-06T09:24:56.977Z",
        "questions_with_answers": []
      }
    ],
    "teams_uploads": [
      {
        "id": 2119962,
        "final_mark": 49,
        "comment": "initial_errors:  | test_ft_isalpha: OK | test_ft_isdigit: OK | test_ft_isalnum: OK | test_ft_isascii: OK | test_ft_isprint: OK | test_ft_strlen: OK | test_ft_memset: OK | test_ft_bzero: OK | test_ft_memcpy: OK | test_ft_memmove: OK | test_ft_strlcpy: OK | test_ft_strlcat: OK | test_ft_toupper: OK | test_ft_tolower: OK | test_ft_strchr: OK | test_ft_strrchr: OK | test_ft_strncmp: OK | test_ft_memchr: Error encountered while testing | test_ft_memcmp: OK | test_ft_strnstr: Error encountered while testing | test_ft_atoi: OK | test_ft_calloc: OK | test_ft_strdup: OK | test_ft_substr: OK | test_ft_strjoin: OK | test_ft_strtrim: OK | test_ft_split: OK | test_ft_itoa: OK | test_ft_strmapi: OK | test_ft_striteri: OK | test_ft_putchar_fd: OK | test_ft_putstr_fd: OK | test_ft_putendl_fd: OK | test_ft_putnbr_fd: OK | bonus: 9/9 functions correct",
        "created_at": "2024-03-06T11:20:29.191Z",
        "upload_id": 1558
      }
    ]
  },
  {
    "id": 5533755,
    "name": "wonhseo's group",
    "url": "https://api.intra.42.fr/v2/teams/5533755",
    "final_mark": 0,
    "project_id": 1314,
    "created_at": "2024-02-26T00:55:51.220Z",
    "updated_at": "2024-02-27T04:26:52.160Z",
    "status": "finished",
    "terminating_at": null,
    "users": [
      {
        "id": 172319,
        "login": "wonhseo",
        "url": "https://api.intra.42.fr/v2/users/wonhseo",
        "leader": true,
        "occurrence": 0,
        "validated": true,
        "projects_user_id": 3565877
      }
    ],
    "locked?": true,
    "validated?": false,
    "closed?": true,
    "repo_url": "git@vogsphere.42gyeongsan.kr:vogsphere/intra-uuid-278f19cf-f6b0-4dfb-b0d1-3d692d9d05bf-5533755-wonhseo",
    "repo_uuid": "intra-uuid-278f19cf-f6b0-4dfb-b0d1-3d692d9d05bf-5533755-wonhseo",
    "locked_at": "2024-02-26T00:55:51.290Z",
    "closed_at": "2024-02-26T12:23:06.635Z",
    "project_session_id": 11805,
    "project_gitlab_path": "pedago_world/42-cursus/inner-circle/libft",
    "scale_teams": [
      {
        "id": 6446823,
        "scale_id": 32445,
        "comment": "정리를 깔끔하게 하셔서 준비해 놓으신 자료를 통해 많은 공부를 할 수 있었습니다. 키워드 단위로 중요한 개념들을 콕 찝어 주셨고, 코드같은 경우도 재사용이 잘 되게끔 깔끔하게 작성하셨습니다. 제가 모르는 부분까지도 친절하게 테스트 해주셔서 감사했고요. 화이팅입니다!!",
        "created_at": "2024-02-27T01:14:32.822Z",
        "updated_at": "2024-02-27T04:19:19.594Z",
        "feedback": "오랜만이에요 ksuh님! 첫 본과정 평가를 꼼꼼하게 봐주셔서 정말 감사합니다! 평가를 마친뒤, Makefile을 비롯한 함수들을 즉석에서 바꿔보며 재밌는 실험들을 할 수 있어서 좋았어요. 함께 성장해가며 앞으로 더 멋진 프로젝트도 같이 해요! 잘 부탁드립니다!!",
        "final_mark": 0,
        "flag": {
          "id": 9,
          "name": "Outstanding project",
          "positive": true,
          "icon": "star-1",
          "created_at": "2017-05-18T14:07:37.380Z",
          "updated_at": "2017-05-18T14:12:07.415Z"
        },
        "begin_at": "2024-02-27T01:45:00.000Z",
        "correcteds": [
          {
            "id": 172319,
            "login": "wonhseo",
            "url": "https://api.intra.42.fr/v2/users/wonhseo"
          }
        ],
        "corrector": {
          "id": 172367,
          "login": "ksuh",
          "url": "https://api.intra.42.fr/v2/users/ksuh"
        },
        "truant": {},
        "filled_at": "2024-02-27T02:41:25.051Z",
        "questions_with_answers": []
      },
      {
        "id": 6446919,
        "scale_id": 32445,
        "comment": "저는 평가를 하기 전에 Milestone 1까지만 끝내고 온 상태라 온전한 평가를 내릴 수 없는 상황이었는데, 외부에서 가져온 자료를 가지고 테스트를 통과하는 것을 직접 보여주시면서 본인의 코드가 정상적으로 작동하는지를 확인할 수 있도록 도와주셨고, Main 파일은 직접 추가로 만드셔서 절차에 도움을 주셨습니다. 또한, 단순히 본인 코드가 정상 작동하는 것 만 보여주시는 게 아니라, 본인이 헷갈렸던 부분만 추려서 설명해주셔서 제가 이해할 수 있는 영역 내에서 올바른 평가를 내릴 수 있었습니다. 실제로 단 한 개의 오류나 크래시도 없었고, 보너스 문제까지 완벽하게 클리어하셔서 1차 라피신때 보여주셨던 모습 그대로 유지하시는 거 같아 좋았습니다. 앞으로 이런 모습 충분히 유지하실 거라 생각해서, 제가 굳이 동기부여성의 말을 하지 않아도 충분히 원활하게 다음 과제, 이후 과제 잘 풀어나가실 수 있을 거라 확신합니다. 오늘도 내일도 파이팅입니다 :D",
        "created_at": "2024-02-27T02:55:40.532Z",
        "updated_at": "2024-02-27T04:26:12.871Z",
        "feedback": "안녕하세요 inam님! 평가 내내 즐거웠고 정리를 한 보람이 있었습니다. 코드를 보시며 단박에 이해하셔서 역시 고수셨음을 실감했네요 ㅎㅎ 함께 배우고 성장할 수 있어서 정말 기쁘고, 앞으로도 좋은 소통을 이어가길 바랍니다! 오늘도 내일도 파이팅입니다 :D",
        "final_mark": 0,
        "flag": {
          "id": 9,
          "name": "Outstanding project",
          "positive": true,
          "icon": "star-1",
          "created_at": "2017-05-18T14:07:37.380Z",
          "updated_at": "2017-05-18T14:12:07.415Z"
        },
        "begin_at": "2024-02-27T03:30:00.000Z",
        "correcteds": [
          {
            "id": 172319,
            "login": "wonhseo",
            "url": "https://api.intra.42.fr/v2/users/wonhseo"
          }
        ],
        "corrector": {
          "id": 172329,
          "login": "inam",
          "url": "https://api.intra.42.fr/v2/users/inam"
        },
        "truant": {},
        "filled_at": "2024-02-27T04:02:27.925Z",
        "questions_with_answers": []
      }
    ],
    "teams_uploads": []
  },
  {
    "id": 5533756,
    "name": "seoyokim's group",
    "url": "https://api.intra.42.fr/v2/teams/5533756",
    "final_mark": 62,
    "project_id": 1314,
    "created_at": "2024-02-26T00:55:59.796Z",
    "updated_at": "2024-02-29T08:31:11.804Z",
    "status": "finished",
    "terminating_at": null,
    "users": [
      {
        "id": 174105,
        "login": "seoyokim",
        "url": "https://api.intra.42.fr/v2/users/seoyokim",
        "leader": true,
        "occurrence": 0,
        "validated": true,
        "projects_user_id": 3565878
      }
    ],
    "locked?": true,
    "validated?": false,
    "closed?": true,
    "repo_url": "git@vogsphere.42gyeongsan.kr:vogsphere/intra-uuid-c8286523-088a-4f25-b9b9-f5ea6aceda70-5533756-seoyokim",
    "repo_uuid": "intra-uuid-c8286523-088a-4f25-b9b9-f5ea6aceda70-5533756-seoyokim",
    "locked_at": "2024-02-26T00:55:59.830Z",
    "closed_at": "2024-02-29T05:20:43.903Z",
    "project_session_id": 11805,
    "project_gitlab_path": "pedago_world/42-cursus/inner-circle/libft",
    "scale_teams": [
      {
        "id": 6459604,
        "scale_id": 32445,
        "comment": "모든 함수를 하나하나 검사하였습니다. 문자열에 관련된 일관된 반환 로직을 설명해주셨습니다. ft_strtrim.c 함수에서 set이 없을때 free할수있는 문자열을 리턴하기 위해 strdup함수를재사용하는 테크닉을 보고 제가 모르던 사실을 배울 수 있었습니다.\r\nsubstr에서 입력받은 문자열이 start 보다 작을때 start를 이용해 malloc하는 것 또한 예상치 못했는데 설명을 듣고 이해할 수 있었습니다.\r\nstrlcpy, strlcat의 반환값에 대해서는 조금 설명이 부족한듯하여 따로 더 공부를 해보는 것이 좋을 것 같습니다.\r\n이번 평가를 통해 많은 것을배울수 있었습니다. 감사합니다.",
        "created_at": "2024-02-29T06:28:11.966Z",
        "updated_at": "2024-02-29T08:30:57.900Z",
        "feedback": "가이드 라인에 맞춰서 꼼꼼히 해주셔서 감사합니다",
        "final_mark": 125,
        "flag": {
          "id": 1,
          "name": "Ok",
          "positive": true,
          "icon": "check-4",
          "created_at": "2015-09-14T23:06:52.000Z",
          "updated_at": "2015-09-14T23:06:52.000Z"
        },
        "begin_at": "2024-02-29T07:45:00.000Z",
        "correcteds": [
          {
            "id": 174105,
            "login": "seoyokim",
            "url": "https://api.intra.42.fr/v2/users/seoyokim"
          }
        ],
        "corrector": {
          "id": 174127,
          "login": "doji",
          "url": "https://api.intra.42.fr/v2/users/doji"
        },
        "truant": {},
        "filled_at": "2024-02-29T08:28:13.173Z",
        "questions_with_answers": []
      },
      {
        "id": 6459601,
        "scale_id": 32445,
        "comment": "함수를 설명함에 있어 성실하게 설명해주셨습니다. 덕분에 많은 것을 배울 수 있었습니다. 재밌었어용 그리고 함수 기능별로 구현을 깔끔하게 해주셔서 좋았어요 감사합니다.",
        "created_at": "2024-02-29T06:27:59.586Z",
        "updated_at": "2024-02-29T08:30:36.745Z",
        "feedback": "너무 뭔가 제가 빠르게 한거 같습니다 감사합니다",
        "final_mark": 125,
        "flag": {
          "id": 1,
          "name": "Ok",
          "positive": true,
          "icon": "check-4",
          "created_at": "2015-09-14T23:06:52.000Z",
          "updated_at": "2015-09-14T23:06:52.000Z"
        },
        "begin_at": "2024-02-29T07:00:00.000Z",
        "correcteds": [
          {
            "id": 174105,
            "login": "seoyokim",
            "url": "https://api.intra.42.fr/v2/users/seoyokim"
          }
        ],
        "corrector": {
          "id": 174165,
          "login": "ssohn",
          "url": "https://api.intra.42.fr/v2/users/ssohn"
        },
        "truant": {},
        "filled_at": "2024-02-29T07:22:49.617Z",
        "questions_with_answers": []
      },
      {
        "id": 6459479,
        "scale_id": 32445,
        "comment": "모든 문제가 함수가 원하는 값을 반환하는것 같아 모두 yes하였습니다.\r\n고생하셨습니다.",
        "created_at": "2024-02-29T05:21:05.977Z",
        "updated_at": "2024-02-29T08:29:23.033Z",
        "feedback": "감사합니다 빠르게 봐주셔서 편했고 질문하신것들은 최선을 다해 답해드렸습니다",
        "final_mark": 125,
        "flag": {
          "id": 1,
          "name": "Ok",
          "positive": true,
          "icon": "check-4",
          "created_at": "2015-09-14T23:06:52.000Z",
          "updated_at": "2015-09-14T23:06:52.000Z"
        },
        "begin_at": "2024-02-29T06:00:00.000Z",
        "correcteds": [
          {
            "id": 174105,
            "login": "seoyokim",
            "url": "https://api.intra.42.fr/v2/users/seoyokim"
          }
        ],
        "corrector": {
          "id": 174189,
          "login": "changhyu",
          "url": "https://api.intra.42.fr/v2/users/changhyu"
        },
        "truant": {},
        "filled_at": "2024-02-29T06:25:25.792Z",
        "questions_with_answers": []
      }
    ],
    "teams_uploads": [
      {
        "id": 2113214,
        "final_mark": 0,
        "comment": "initial_errors: KO (Does not compile) | test_ft_isalpha: KO (Does not compile), KO (Does not compile) | test_ft_isdigit: KO (Does not compile), KO (Does not compile) | test_ft_isalnum: KO (Does not compile), KO (Does not compile) | test_ft_isascii: KO (Does not compile), KO (Does not compile) | test_ft_isprint: KO (Does not compile), KO (Does not compile) | test_ft_strlen: KO (Does not compile), KO (Does not compile) | test_ft_memset: KO (Does not compile), KO (Does not compile) | test_ft_bzero: KO (Does not compile), KO (Does not compile) | test_ft_memcpy: KO (Does not compile), KO (Does not compile) | test_ft_memmove: KO (Does not compile), KO (Does not compile) | test_ft_strlcpy: KO (Does not compile), KO (Does not compile) | test_ft_strlcat: KO (Does not compile), KO (Does not compile) | test_ft_toupper: KO (Does not compile), KO (Does not compile) | test_ft_tolower: KO (Does not compile), KO (Does not compile) | test_ft_strchr: KO (Does not compile), KO (Does not compile) | test_ft_strrchr: KO (Does not compile), KO (Does not compile) | test_ft_strncmp: KO (Does not compile), KO (Does not compile) | test_ft_memchr: KO (Does not compile), KO (Does not compile) | test_ft_memcmp: KO (Does not compile), KO (Does not compile) | test_ft_strnstr: KO (Does not compile), KO (Does not compile) | test_ft_atoi: KO (Does not compile), KO (Does not compile) | test_ft_calloc: KO (Does not compile), KO (Does not compile) | test_ft_strdup: KO (Does not compile), KO (Does not compile) | test_ft_substr: KO (Does not compile), KO (Does not compile) | test_ft_strjoin: KO (Does not compile), KO (Does not compile) | test_ft_strtrim: KO (Does not compile), KO (Does not compile) | test_ft_split: KO (Does not compile), KO (Does not compile) | test_ft_itoa: KO (Does not compile), KO (Does not compile) | test_ft_strmapi: KO (Does not compile), KO (Does not compile) | test_ft_striteri: KO (Does not compile), KO (Does not compile) | test_ft_putchar_fd: KO (Does not compile), KO (Does not compile) | test_ft_putstr_fd: KO (Does not compile), KO (Does not compile) | test_ft_putendl_fd: KO (Does not compile), KO (Does not compile) | test_ft_putnbr_fd: KO (Does not compile), KO (Does not compile) | bonus: 0/9 functions correct",
        "created_at": "2024-02-29T08:31:11.802Z",
        "upload_id": 1558
      }
    ]
  },
  {
    "id": 5533757,
    "name": "sunghyki's group",
    "url": "https://api.intra.42.fr/v2/teams/5533757",
    "final_mark": 0,
    "project_id": 1314,
    "created_at": "2024-02-26T00:56:13.877Z",
    "updated_at": "2024-02-27T06:03:11.674Z",
    "status": "finished",
    "terminating_at": null,
    "users": [
      {
        "id": 172385,
        "login": "sunghyki",
        "url": "https://api.intra.42.fr/v2/users/sunghyki",
        "leader": true,
        "occurrence": 0,
        "validated": true,
        "projects_user_id": 3565879
      }
    ],
    "locked?": true,
    "validated?": false,
    "closed?": true,
    "repo_url": "git@vogsphere.42gyeongsan.kr:vogsphere/intra-uuid-10980e41-78ee-4d30-a88f-882ab1bfaef9-5533757-sunghyki",
    "repo_uuid": "intra-uuid-10980e41-78ee-4d30-a88f-882ab1bfaef9-5533757-sunghyki",
    "locked_at": "2024-02-26T00:56:13.905Z",
    "closed_at": "2024-02-26T08:26:30.593Z",
    "project_session_id": 11805,
    "project_gitlab_path": "pedago_world/42-cursus/inner-circle/libft",
    "scale_teams": [
      {
        "id": 6447016,
        "scale_id": 32445,
        "comment": "makefile 에서 wildcard를 사용하여 아마 컴파일이 안 될 것이라 생각합니다. calloc에 오버플로우 처리, 여러 함수에서 마지막 널문자 처리, unsigned int 는 size_t로 변경하여야 할 것 같습니다. 또한 is함수들은 라이브러리에서 반환하는 값으로 변경하시는 게 정신적으로 편하실 거라 생각합니다.... 1시간 동안 평가를 하였는데, 고생 많으셨습니다. 다음 트라이 땐 꼭 통과하시길 바랍니다. 화이팅! ",
        "created_at": "2024-02-27T04:25:34.793Z",
        "updated_at": "2024-02-27T06:03:01.626Z",
        "feedback": "안녕하세요. 이번 피평가를 받게된 김승현입니다. 평가자분이 평가를 와주셨을 때 코드 검토를 해주시며 제가 못보던 오류를 잡아주시면 상당히 편한데 그런 면에서 최고의 평가였습니다. 고쳐야할 곳들을 많이 찾게 되어서 좋았고 평가 와중에 의견 교류도 활발하여 많은 도움이 되었습니다. 다음 리뷰에서 다시 뵙게 되길 기대합니다. 감사합니다.",
        "final_mark": 0,
        "flag": {
          "id": 5,
          "name": "Invalid compilation",
          "positive": false,
          "icon": "skull-2",
          "created_at": "2015-09-14T23:06:52.000Z",
          "updated_at": "2015-09-14T23:06:52.000Z"
        },
        "begin_at": "2024-02-27T05:00:00.000Z",
        "correcteds": [
          {
            "id": 172385,
            "login": "sunghyki",
            "url": "https://api.intra.42.fr/v2/users/sunghyki"
          }
        ],
        "corrector": {
          "id": 174181,
          "login": "sejjeong",
          "url": "https://api.intra.42.fr/v2/users/sejjeong"
        },
        "truant": {},
        "filled_at": "2024-02-27T06:00:44.351Z",
        "questions_with_answers": []
      },
      {
        "id": 6442514,
        "scale_id": 32445,
        "comment": "각 문제를 연결하여 헤더파일을 활용하며 문제를 해결 해나갔다. 그렇게 문제를 해결하며 나에게는 창의적인 문제해결 방법이라고 느껴 졌다. 각 문제의 요구사항을 이해하고 어떻게 해결해나가야하는 지를 이해하고 질문에 대한 대답을 보인이 이해한 대로 매우 자세히해주었다. 나의 피드백에 대한 태도 역시 매우 동료학습이라는 것이 느껴질 정도로 정보 교류를 하며 활발히 입하였다. 자신의 실수 또한 합리적으로 수긍하고 나의 실수도 바른 방향으로 잡아주는 인상 깊은 모습을 보여 주었다.",
        "created_at": "2024-02-26T08:40:54.327Z",
        "updated_at": "2024-02-27T00:51:54.416Z",
        "feedback": "안녕하세요. 이번 과제 피평가를 받게된 김승현입니다. 해당 과제에 대한 제 설명을 차분하게 잘 들어주셨으며 함께 코드를 검토하며 코딩 중에는 미처 발견하지 못했던 오류를 많이 발견할 수 있어 좋았습니다. 코드를 하나하나 검토해주시는 열정과 이해하려는 열정이 보기 좋았으며 다음 평가에 또 뵐 수 있으면 좋겠습니다. 감사합니다.",
        "final_mark": 51,
        "flag": {
          "id": 1,
          "name": "Ok",
          "positive": true,
          "icon": "check-4",
          "created_at": "2015-09-14T23:06:52.000Z",
          "updated_at": "2015-09-14T23:06:52.000Z"
        },
        "begin_at": "2024-02-26T10:00:00.000Z",
        "correcteds": [
          {
            "id": 172385,
            "login": "sunghyki",
            "url": "https://api.intra.42.fr/v2/users/sunghyki"
          }
        ],
        "corrector": {
          "id": 174085,
          "login": "jacha",
          "url": "https://api.intra.42.fr/v2/users/jacha"
        },
        "truant": {},
        "filled_at": "2024-02-26T11:19:09.487Z",
        "questions_with_answers": []
      },
      {
        "id": 6442471,
        "scale_id": 32445,
        "comment": "안녕하세요 평가를 맡은 dae-lee 입니다. 과제를 하나하나 자세히 설명해 주셔서 이해가 쏙쏙 되었습니다. 궁금증을 질문드린부분들을 상세하게 설명해주시고 man 페이지를 참조해서 설명해 주셔서 너무 좋았고, 코드도 가독성이 좋아서 이해가 빨를수 있어서 좋았습니다. 너무 감사합니다. 향후에도 많은 부분 배우고 싶습니다. 첫번째로 libft 를 통과하셨으면 좋겠습니다 감사하고 본과정 파이팅이십니다.!!",
        "created_at": "2024-02-26T08:27:18.241Z",
        "updated_at": "2024-02-27T00:50:32.066Z",
        "feedback": "안녕하세요. 이번 과제 피평가를 받게된 김승현입니다. 과제 평가를 와주셔서 제 설명을 차분하게 경청해주셨으며 코드를 함께 검토하며 부족한 부분도 발견해주셔서 고마웠습니다. 설명이 길어져 마지막에 시간이 촉박하게 느껴졌던 점 죄송하게 생각합니다. 본과정 내용이 방대해져 평가 시간이 길었던 점을 미처 생각하지 못했는데 다음부터는 이런 점을 반영하여 평가 간의 텀을 넉넉하게 잡도록 하겠습니다. 감사합니다.",
        "final_mark": 75,
        "flag": {
          "id": 1,
          "name": "Ok",
          "positive": true,
          "icon": "check-4",
          "created_at": "2015-09-14T23:06:52.000Z",
          "updated_at": "2015-09-14T23:06:52.000Z"
        },
        "begin_at": "2024-02-26T09:30:00.000Z",
        "correcteds": [
          {
            "id": 172385,
            "login": "sunghyki",
            "url": "https://api.intra.42.fr/v2/users/sunghyki"
          }
        ],
        "corrector": {
          "id": 172352,
          "login": "dae-lee",
          "url": "https://api.intra.42.fr/v2/users/dae-lee"
        },
        "truant": {},
        "filled_at": "2024-02-26T10:10:50.165Z",
        "questions_with_answers": []
      }
    ],
    "teams_uploads": [
      {
        "id": 2109060,
        "final_mark": 0,
        "comment": "initial_errors: KO (Does not compile) | test_ft_isalpha: KO (Does not compile), KO (Does not compile) | test_ft_isdigit: KO (Does not compile), KO (Does not compile) | test_ft_isalnum: KO (Does not compile), KO (Does not compile) | test_ft_isascii: KO (Does not compile), KO (Does not compile) | test_ft_isprint: KO (Does not compile), KO (Does not compile) | test_ft_strlen: KO (Does not compile), KO (Does not compile) | test_ft_memset: KO (Does not compile), KO (Does not compile) | test_ft_bzero: KO (Does not compile), KO (Does not compile) | test_ft_memcpy: KO (Does not compile), KO (Does not compile) | test_ft_memmove: KO (Does not compile), KO (Does not compile) | test_ft_strlcpy: KO (Does not compile), KO (Does not compile) | test_ft_strlcat: KO (Does not compile), KO (Does not compile) | test_ft_toupper: KO (Does not compile), KO (Does not compile) | test_ft_tolower: KO (Does not compile), KO (Does not compile) | test_ft_strchr: KO (Does not compile), KO (Does not compile) | test_ft_strrchr: KO (Does not compile), KO (Does not compile) | test_ft_strncmp: KO (Does not compile), KO (Does not compile) | test_ft_memchr: KO (Does not compile), KO (Does not compile) | test_ft_memcmp: KO (Does not compile), KO (Does not compile) | test_ft_strnstr: KO (Does not compile), KO (Does not compile) | test_ft_atoi: KO (Does not compile), KO (Does not compile) | test_ft_calloc: KO (Does not compile), KO (Does not compile) | test_ft_strdup: KO (Does not compile), KO (Does not compile) | test_ft_substr: KO (Does not compile), KO (Does not compile) | test_ft_strjoin: KO (Does not compile), KO (Does not compile) | test_ft_strtrim: KO (Does not compile), KO (Does not compile) | test_ft_split: KO (Does not compile), KO (Does not compile) | test_ft_itoa: KO (Does not compile), KO (Does not compile) | test_ft_strmapi: KO (Does not compile), KO (Does not compile) | test_ft_striteri: KO (Does not compile), KO (Does not compile) | test_ft_putchar_fd: KO (Does not compile), KO (Does not compile) | test_ft_putstr_fd: KO (Does not compile), KO (Does not compile) | test_ft_putendl_fd: KO (Does not compile), KO (Does not compile) | test_ft_putnbr_fd: KO (Does not compile), KO (Does not compile) | bonus: KO (Does not compile)",
        "created_at": "2024-02-27T06:03:11.672Z",
        "upload_id": 1558
      }
    ]
  },
  {
    "id": 5533761,
    "name": "junkwak's group",
    "url": "https://api.intra.42.fr/v2/teams/5533761",
    "final_mark": 0,
    "project_id": 1314,
    "created_at": "2024-02-26T01:02:54.747Z",
    "updated_at": "2024-03-19T15:23:31.907Z",
    "status": "finished",
    "terminating_at": null,
    "users": [
      {
        "id": 174123,
        "login": "junkwak",
        "url": "https://api.intra.42.fr/v2/users/junkwak",
        "leader": true,
        "occurrence": 0,
        "validated": true,
        "projects_user_id": 3565880
      }
    ],
    "locked?": true,
    "validated?": false,
    "closed?": true,
    "repo_url": "git@vogsphere.42gyeongsan.kr:vogsphere/intra-uuid-44570075-18a3-48c1-9415-f03858cf1639-5533761-junkwak",
    "repo_uuid": "intra-uuid-44570075-18a3-48c1-9415-f03858cf1639-5533761-junkwak",
    "locked_at": "2024-02-26T01:02:54.794Z",
    "closed_at": "2024-03-19T08:35:48.835Z",
    "project_session_id": 11805,
    "project_gitlab_path": "pedago_world/42-cursus/inner-circle/libft",
    "scale_teams": [
      {
        "id": 6533441,
        "scale_id": 32445,
        "comment": "문제를 발견하시면 오랫동안 그 문제가 무엇이 문제였는지에 대하여 깊게 고민하시는게 코드에서 눈에 띕니다. 그래서 더욱더 저 또한 반성하고 코드를 다시금 확인하게 되는 것 같습니다. 아직 미약하게 코드에 대해 볼 수 있어 평가 왔음에도 제대로 된 조언을 드리지 못해 죄송하지만 그래도 짜신 코드들은 모두 열심히 고민한 흔적이 보입니다. 남은 문제들도 화이팅!",
        "created_at": "2024-03-19T10:19:56.725Z",
        "updated_at": "2024-03-19T15:22:39.525Z",
        "feedback": "안녕하세요 hyunahn님! 좋은 평가와 좋은 가르침을 주셔서 감사합니다!!수고많으셨습니다 !!",
        "final_mark": 0,
        "flag": {
          "id": 9,
          "name": "Outstanding project",
          "positive": true,
          "icon": "star-1",
          "created_at": "2017-05-18T14:07:37.380Z",
          "updated_at": "2017-05-18T14:12:07.415Z"
        },
        "begin_at": "2024-03-19T11:15:00.000Z",
        "correcteds": [
          {
            "id": 174123,
            "login": "junkwak",
            "url": "https://api.intra.42.fr/v2/users/junkwak"
          }
        ],
        "corrector": {
          "id": 174161,
          "login": "hyunahn",
          "url": "https://api.intra.42.fr/v2/users/hyunahn"
        },
        "truant": {},
        "filled_at": "2024-03-19T12:18:58.952Z",
        "questions_with_answers": []
      },
      {
        "id": 6534951,
        "scale_id": 32445,
        "comment": "안녕하세요 yokoh입니다. makefile 에서 relink가 발생하시는것같습니다. .pony에 bonus도 추가해주시면 좋을것같습니다.",
        "created_at": "2024-03-19T13:42:34.286Z",
        "updated_at": "2024-03-19T15:23:28.611Z",
        "feedback": "안녕하세요 yokoh 님! relink 를 놓쳐 0점을 맞게되었지만 비트연산자 및 여러가지 가르침을 주셔서 감사했습니다!!수고많으셨습니다 !!!",
        "final_mark": 0,
        "flag": {
          "id": 1,
          "name": "Ok",
          "positive": true,
          "icon": "check-4",
          "created_at": "2015-09-14T23:06:52.000Z",
          "updated_at": "2015-09-14T23:06:52.000Z"
        },
        "begin_at": "2024-03-19T14:15:00.000Z",
        "correcteds": [
          {
            "id": 174123,
            "login": "junkwak",
            "url": "https://api.intra.42.fr/v2/users/junkwak"
          }
        ],
        "corrector": {
          "id": 172330,
          "login": "yokoh",
          "url": "https://api.intra.42.fr/v2/users/yokoh"
        },
        "truant": {},
        "filled_at": "2024-03-19T15:17:24.539Z",
        "questions_with_answers": []
      }
    ],
    "teams_uploads": []
  },
  {
    "id": 5533763,
    "name": "ksuh's group",
    "url": "https://api.intra.42.fr/v2/teams/5533763",
    "final_mark": 93,
    "project_id": 1314,
    "created_at": "2024-02-26T01:03:54.318Z",
    "updated_at": "2024-03-01T09:19:44.817Z",
    "status": "finished",
    "terminating_at": null,
    "users": [
      {
        "id": 172367,
        "login": "ksuh",
        "url": "https://api.intra.42.fr/v2/users/ksuh",
        "leader": true,
        "occurrence": 0,
        "validated": true,
        "projects_user_id": 3565881
      }
    ],
    "locked?": true,
    "validated?": false,
    "closed?": true,
    "repo_url": "git@vogsphere.42gyeongsan.kr:vogsphere/intra-uuid-64fd6b3a-baa4-4276-b822-97a871a87236-5533763-ksuh",
    "repo_uuid": "intra-uuid-64fd6b3a-baa4-4276-b822-97a871a87236-5533763-ksuh",
    "locked_at": "2024-02-26T01:03:54.346Z",
    "closed_at": "2024-03-01T05:45:47.771Z",
    "project_session_id": 11805,
    "project_gitlab_path": "pedago_world/42-cursus/inner-circle/libft",
    "scale_teams": [
      {
        "id": 6465809,
        "scale_id": 32445,
        "comment": "평가가 잡힌줄 모르고 뒤늦게 도착했는데 잘 알려주셨습니다. 보너스 문제 같은경우 제가 이해하지 못한 문제(ft_lstmap)가 있었는데 설명해주신 덕분에 해당 문제를 어떤식으로 구현해야 할지 힌트가 되었습니다. 감사합니다. :)",
        "created_at": "2024-03-01T07:51:23.857Z",
        "updated_at": "2024-03-01T09:19:27.737Z",
        "feedback": "안녕하세요. 침착하게 평가를 맡아주셔서 감사합니다. 메모리 공간 할당시 정확한 길이를 구하지 않아서 불편했지만 기계채점은 어떻게 될 지 모르겠네요. 감사합니다!",
        "final_mark": 125,
        "flag": {
          "id": 1,
          "name": "Ok",
          "positive": true,
          "icon": "check-4",
          "created_at": "2015-09-14T23:06:52.000Z",
          "updated_at": "2015-09-14T23:06:52.000Z"
        },
        "begin_at": "2024-03-01T08:30:00.000Z",
        "correcteds": [
          {
            "id": 172367,
            "login": "ksuh",
            "url": "https://api.intra.42.fr/v2/users/ksuh"
          }
        ],
        "corrector": {
          "id": 174096,
          "login": "junmin",
          "url": "https://api.intra.42.fr/v2/users/junmin"
        },
        "truant": {},
        "filled_at": "2024-03-01T09:14:14.679Z",
        "questions_with_answers": []
      },
      {
        "id": 6465640,
        "scale_id": 32445,
        "comment": "코드가 문제 없이 작동합니다. 변수의 네이밍을 조금 더 신경 써주신다면 가독성이 더욱 좋아질 거라 생각합니다! 좋은 코드 잘봤습니다. 고생하셨습니다. ",
        "created_at": "2024-03-01T05:45:59.987Z",
        "updated_at": "2024-03-01T07:58:37.415Z",
        "feedback": "sejjeong님 안녕하세요. 통일성 있게끔 변수를 사용하는 내용과 메모리 할당시 누수의 부분 등에서 의미있는 피드백 감사했습니다. 오늘 평가때 처음 인사 나눴는데 코드적으로 아쉬웠던 점을 친절하게 알려주시고 여러방면으로 공유해주셔서 좋은 시간이었네요. 감사합니다!",
        "final_mark": 125,
        "flag": {
          "id": 1,
          "name": "Ok",
          "positive": true,
          "icon": "check-4",
          "created_at": "2015-09-14T23:06:52.000Z",
          "updated_at": "2015-09-14T23:06:52.000Z"
        },
        "begin_at": "2024-03-01T07:00:00.000Z",
        "correcteds": [
          {
            "id": 172367,
            "login": "ksuh",
            "url": "https://api.intra.42.fr/v2/users/ksuh"
          }
        ],
        "corrector": {
          "id": 174181,
          "login": "sejjeong",
          "url": "https://api.intra.42.fr/v2/users/sejjeong"
        },
        "truant": {},
        "filled_at": "2024-03-01T07:50:01.658Z",
        "questions_with_answers": []
      },
      {
        "id": 6465643,
        "scale_id": 32445,
        "comment": "안녕하세요. ksuh님의 평가를 맡은 myeochoi입니다. 헤더 파일 내의 여러가지 함수를 활용하여 재사용한 코드가 인상적이었습니다. 모든 요구사항을 충족하였기 때문에 목표한 점수를 받을 수 있을 것이라고 생각합니다. 또한 친절하게 설명을 해주셔서 좋은 평가를 진행할 수 있었습니다. 앞으로도 화이팅입니다!",
        "created_at": "2024-03-01T05:46:38.469Z",
        "updated_at": "2024-03-01T07:53:43.572Z",
        "feedback": "코드를 세심하게 봐주셨습니다. 틈틈히 메뉴얼도 활용하시는 모습이 인상깊었습니다. 부족한 코드에도 찬사를 보내주셔서 감사합니다. 다음 평가때 뵙겠습니다.",
        "final_mark": 125,
        "flag": {
          "id": 1,
          "name": "Ok",
          "positive": true,
          "icon": "check-4",
          "created_at": "2015-09-14T23:06:52.000Z",
          "updated_at": "2015-09-14T23:06:52.000Z"
        },
        "begin_at": "2024-03-01T06:30:00.000Z",
        "correcteds": [
          {
            "id": 172367,
            "login": "ksuh",
            "url": "https://api.intra.42.fr/v2/users/ksuh"
          }
        ],
        "corrector": {
          "id": 172342,
          "login": "myeochoi",
          "url": "https://api.intra.42.fr/v2/users/myeochoi"
        },
        "truant": {},
        "filled_at": "2024-03-01T07:13:49.070Z",
        "questions_with_answers": []
      }
    ],
    "teams_uploads": [
      {
        "id": 2115352,
        "final_mark": 61,
        "comment": "initial_errors:  | test_ft_isalpha: OK | test_ft_isdigit: OK | test_ft_isalnum: OK | test_ft_isascii: OK | test_ft_isprint: OK | test_ft_strlen: OK | test_ft_memset: OK | test_ft_bzero: OK | test_ft_memcpy: OK | test_ft_memmove: OK | test_ft_strlcpy: OK | test_ft_strlcat: OK | test_ft_toupper: OK | test_ft_tolower: OK | test_ft_strchr: OK | test_ft_strrchr: OK | test_ft_strncmp: OK | test_ft_memchr: OK | test_ft_memcmp: OK | test_ft_strnstr: OK | test_ft_atoi: OK | test_ft_calloc: Error encountered while testing | test_ft_strdup: OK | test_ft_substr: Error encountered while testing | test_ft_strjoin: OK | test_ft_strtrim: OK | test_ft_split: OK | test_ft_itoa: OK | test_ft_strmapi: OK | test_ft_striteri: OK | test_ft_putchar_fd: OK | test_ft_putstr_fd: OK | test_ft_putendl_fd: OK | test_ft_putnbr_fd: OK | bonus: 9/9 functions correct",
        "created_at": "2024-03-01T09:19:44.815Z",
        "upload_id": 1558
      }
    ]
  },
  {
    "id": 5533764,
    "name": "woonhan's group",
    "url": "https://api.intra.42.fr/v2/teams/5533764",
    "final_mark": 0,
    "project_id": 1314,
    "created_at": "2024-02-26T01:05:14.333Z",
    "updated_at": "2024-03-06T05:08:55.955Z",
    "status": "finished",
    "terminating_at": null,
    "users": [
      {
        "id": 174141,
        "login": "woonhan",
        "url": "https://api.intra.42.fr/v2/users/woonhan",
        "leader": true,
        "occurrence": 0,
        "validated": true,
        "projects_user_id": 3565882
      }
    ],
    "locked?": true,
    "validated?": false,
    "closed?": true,
    "repo_url": "git@vogsphere.42gyeongsan.kr:vogsphere/intra-uuid-5cb6685d-554e-4996-b2b4-63b22f89e6c2-5533764-woonhan",
    "repo_uuid": "intra-uuid-5cb6685d-554e-4996-b2b4-63b22f89e6c2-5533764-woonhan",
    "locked_at": "2024-02-26T01:05:14.368Z",
    "closed_at": "2024-03-06T03:34:38.894Z",
    "project_session_id": 11805,
    "project_gitlab_path": "pedago_world/42-cursus/inner-circle/libft",
    "scale_teams": [
      {
        "id": 6483255,
        "scale_id": 32445,
        "comment": "모든 과제를 잘 작성하였고 원하는 값을 반환하고 있지만, calloc 의 경우 옳바른 할당이 되지않고 null이 return 되었을때 free를 시도하고있어 no를 드렸습니다. 그 부분을 수정하시면 모든 과제가 올바르게 작성이 되었다고 생각이 듭니다.",
        "created_at": "2024-03-06T03:35:14.189Z",
        "updated_at": "2024-03-06T05:03:14.778Z",
        "feedback": "libft를 이미 몇차례 평가를 하셨고 실력이 있으시기 때문에 평가를 수월하게 진행 할 수 있었습니다. 코드에 대해서 하나씩 꼼꼼하게 봐주신 덕분에 저도 다시 복기 해볼 수 있었고, 문제가 되는 부분들에 대해서 체크해야할 사항들을 확인할 수 있었습니다. 다음 평가시엔 바로 통과하시길 바라겠습니다. 화이팅!",
        "final_mark": 0,
        "flag": {
          "id": 1,
          "name": "Ok",
          "positive": true,
          "icon": "check-4",
          "created_at": "2015-09-14T23:06:52.000Z",
          "updated_at": "2015-09-14T23:06:52.000Z"
        },
        "begin_at": "2024-03-06T04:15:00.000Z",
        "correcteds": [
          {
            "id": 174141,
            "login": "woonhan",
            "url": "https://api.intra.42.fr/v2/users/woonhan"
          }
        ],
        "corrector": {
          "id": 174189,
          "login": "changhyu",
          "url": "https://api.intra.42.fr/v2/users/changhyu"
        },
        "truant": {},
        "filled_at": "2024-03-06T05:00:07.833Z",
        "questions_with_answers": []
      }
    ],
    "teams_uploads": []
  },
  {
    "id": 5533765,
    "name": "esong's group",
    "url": "https://api.intra.42.fr/v2/teams/5533765",
    "final_mark": 72,
    "project_id": 1314,
    "created_at": "2024-02-26T01:05:29.938Z",
    "updated_at": "2024-03-12T08:28:28.972Z",
    "status": "finished",
    "terminating_at": null,
    "users": [
      {
        "id": 172356,
        "login": "esong",
        "url": "https://api.intra.42.fr/v2/users/esong",
        "leader": true,
        "occurrence": 0,
        "validated": true,
        "projects_user_id": 3565883
      }
    ],
    "locked?": true,
    "validated?": false,
    "closed?": true,
    "repo_url": "git@vogsphere.42gyeongsan.kr:vogsphere/intra-uuid-28b6cb55-c5fc-4439-98f2-85f7314ec321-5533765-esong",
    "repo_uuid": "intra-uuid-28b6cb55-c5fc-4439-98f2-85f7314ec321-5533765-esong",
    "locked_at": "2024-02-26T01:05:29.976Z",
    "closed_at": "2024-03-12T01:21:17.313Z",
    "project_session_id": 11805,
    "project_gitlab_path": "pedago_world/42-cursus/inner-circle/libft",
    "scale_teams": [
      {
        "id": 6502860,
        "scale_id": 32445,
        "comment": "시작하면서 추가적인 테스트 툴과 valgrind를 사용한 메모리 누수 확인 방법을 알려드렸습니다. makefile 리링크 문제가 있다고 하셨는데 phony에 bonus를 등록하기를 권해드렸습니다. ft_strncmp, ft_substr의 경우 잘못된 결과가 나오고 있으므로 수정이 필요할 것 같습니다. ft_split의 경우 메모리 누수가 계속 발생하였는데 조금 더 연구하시면 좋을 것 같습니다. ft_strtrim의 경우 구현 내용을 상세하게 잘 설명해 주셨습니다. 보너스 파트인 링크드 리스트에서는 파일이름에 _bonus를 붙여주셔야 합니다. 인자로 구조체를 가리키는 이중 포인터가 들어오는 경우에 대해서 많은 이야기를 해 볼 수 있었습니다. 저도 평가를 하면서 제가 모르는 부분에 대해 알게 된 것 같아서 많은 도움이 되었습니다.",
        "created_at": "2024-03-12T01:23:59.333Z",
        "updated_at": "2024-03-12T08:28:08.723Z",
        "feedback": "안녕하세요 kchoo님!\r\n메모리 누수 체크하는 방법을 알려주신 덕분에 스플릿 메모리 누수 관련하여 체크를 해볼 수 있을 것 같습니다! 감사합니다!\r\nmakefile에서 리링크 부분 관련하여 phony에 bonus가 빠져서 그런 것 같다고 알려주셨는데 테스트해보도록 하겠습니다.\r\n보너스 문제 관련하여 리링크 아는만큼 최대한 설명해드렸는데... 도움이 되었다면 좋겠습니다!\r\nlibft 꼭 한번에 통과하시길 바랍니다!! 여름 파이팅!!!!!!!!!!!!!!!!!!!!!!!!!!",
        "final_mark": 34,
        "flag": {
          "id": 1,
          "name": "Ok",
          "positive": true,
          "icon": "check-4",
          "created_at": "2015-09-14T23:06:52.000Z",
          "updated_at": "2015-09-14T23:06:52.000Z"
        },
        "begin_at": "2024-03-12T07:00:00.000Z",
        "correcteds": [
          {
            "id": 172356,
            "login": "esong",
            "url": "https://api.intra.42.fr/v2/users/esong"
          }
        ],
        "corrector": {
          "id": 174117,
          "login": "kchoo",
          "url": "https://api.intra.42.fr/v2/users/kchoo"
        },
        "truant": {},
        "filled_at": "2024-03-12T08:20:11.665Z",
        "questions_with_answers": []
      },
      {
        "id": 6502856,
        "scale_id": 32445,
        "comment": "Makefile에서 relink를 방지하지 않으셨고 substr에서 s의 길이보다 start가 클 경우에 예외처리를 해주지 않으셨습니다. 하지만 나머지 코드들은 다 깔끔하게 잘 작성해주셨고 설명도 잘 해주셨습니다. 코드를 작성할 때 함수의 길이를 줄일 수 있는 자잘한 팁들을 알려드렸는데 다음에 작성하실 때 도움이 됐으면 좋겠습니다. 틀린 부분을 고치시면 완벽하실 것 같습니다! 감사합니다!",
        "created_at": "2024-03-12T01:22:40.435Z",
        "updated_at": "2024-03-12T05:44:40.459Z",
        "feedback": "jisopark님 안녕하세요!!! 우선 좋은 평가 해주셔서 감사합니다.\r\n평가하시면서 코드 길이를 줄일 수 있는 여러 꿀팁 공유해주셔서 너무너무 감사해요! 많이 유용하게 쓸 것 같아요.\r\nft_split에서 연산자 자체에 대한 함수를 만드셨다했는데 아까 들으면서 신기하더라구요! 기회가 된다면 다음에 보고싶네요..!\r\n저희 꼭 libft 빨리 통과해보아요.... 파이팅.......!",
        "final_mark": 75,
        "flag": {
          "id": 1,
          "name": "Ok",
          "positive": true,
          "icon": "check-4",
          "created_at": "2015-09-14T23:06:52.000Z",
          "updated_at": "2015-09-14T23:06:52.000Z"
        },
        "begin_at": "2024-03-12T05:00:00.000Z",
        "correcteds": [
          {
            "id": 172356,
            "login": "esong",
            "url": "https://api.intra.42.fr/v2/users/esong"
          }
        ],
        "corrector": {
          "id": 174095,
          "login": "jisopark",
          "url": "https://api.intra.42.fr/v2/users/jisopark"
        },
        "truant": {},
        "filled_at": "2024-03-12T05:32:46.572Z",
        "questions_with_answers": []
      },
      {
        "id": 6502868,
        "scale_id": 32445,
        "comment": "평가를 바로 할 수 있게끔 세팅해주셔서 덕분에 편하게 바로 평가를 할 수 있었습니다. norm를 돌려보았을때 잘 통과하였습니다. 다만 makefile에서 make bonus 할 경우 리링크가 나는 것으로 확인 됩니다. calloc 부분에서 nmemb*size가 overflow나오는 경우, ft_split와  ft_lstclear에서 free **를 해주어야 하는 것, ft_memcmp에서 반복을 i + 1 \u003c n만큼하는 부분이 문제가 될 수도 있어 보입니다. 그 밖에 다른 과제들은 잘 구현된 것으로 확인됩니다. 화이팅:)",
        "created_at": "2024-03-12T01:28:20.060Z",
        "updated_at": "2024-03-12T02:46:38.923Z",
        "feedback": "안녕하세요 junmin님! 우선 꼼꼼한 평가 감사드립니다!!\r\nmakefile에서 bonus 리링크 부분을 생각하지 못했었는데 알려주신 덕분에 알게되었어요. AR플래그나 if문을 사용해서 수정해봐야겠어요.\r\nmemcmp에서  i + 1이어야 하는 이유를 알았어요! 제 코드에서는 while문을 다 돈 후에 return 에서 두 문자의 차를 반환하는데, 만약에 인덱스 2까지 비교를 해야하는데 i + 1이 없는 경우에는 i가 2일 때 while문을 돌게 되고, 이후에 i가 2가 아닌 3이 된 후 return에서 문자의 차를 해주기 때문에 잘못된 결과가 나오게 되더라구요!\r\n이번에 평가 해주시면서 free 부분을 잘 짚어주셔서 감사했어요! 덕분에 ft_lstclear와 ft_split에서 첫번째로 할당한 부분에 대한 free를 놓치고 있었음을 알게되었어요! 감사합니다!\r\nft_calloc에서 man에 오버플로우에 대한 얘기가 있음을 알려주셔서 감사합니다. 다시 제대로 man을 봐야겠어요.... \r\n오늘 꼼꼼한 평가 정말 감사했습니다. printf 하고 계신다 하셨는데 잘 마무리 하시길 바랍니다!!!!",
        "final_mark": 125,
        "flag": {
          "id": 1,
          "name": "Ok",
          "positive": true,
          "icon": "check-4",
          "created_at": "2015-09-14T23:06:52.000Z",
          "updated_at": "2015-09-14T23:06:52.000Z"
        },
        "begin_at": "2024-03-12T02:00:00.000Z",
        "correcteds": [
          {
            "id": 172356,
            "login": "esong",
            "url": "https://api.intra.42.fr/v2/users/esong"
          }
        ],
        "corrector": {
          "id": 174096,
          "login": "junmin",
          "url": "https://api.intra.42.fr/v2/users/junmin"
        },
        "truant": {},
        "filled_at": "2024-03-12T02:27:23.540Z",
        "questions_with_answers": []
      }
    ],
    "teams_uploads": [
      {
        "id": 2124850,
        "final_mark": 67,
        "comment": "initial_errors:  | test_ft_isalpha: OK | test_ft_isdigit: OK | test_ft_isalnum: OK | test_ft_isascii: OK | test_ft_isprint: OK | test_ft_strlen: OK | test_ft_memset: OK | test_ft_bzero: OK | test_ft_memcpy: OK | test_ft_memmove: OK | test_ft_strlcpy: OK | test_ft_strlcat: OK | test_ft_toupper: OK | test_ft_tolower: OK | test_ft_strchr: OK | test_ft_strrchr: OK | test_ft_strncmp: OK | test_ft_memchr: OK | test_ft_memcmp: OK | test_ft_strnstr: OK | test_ft_atoi: OK | test_ft_calloc: OK | test_ft_strdup: OK | test_ft_substr: Error encountered while testing | test_ft_strjoin: OK | test_ft_strtrim: OK | test_ft_split: OK | test_ft_itoa: OK | test_ft_strmapi: OK | test_ft_striteri: OK | test_ft_putchar_fd: OK | test_ft_putstr_fd: OK | test_ft_putendl_fd: OK | test_ft_putnbr_fd: OK | bonus: 9/9 functions correct",
        "created_at": "2024-03-12T08:28:28.971Z",
        "upload_id": 1558
      }
    ]
  },
  {
    "id": 5533766,
    "name": "sooslee's group",
    "url": "https://api.intra.42.fr/v2/teams/5533766",
    "final_mark": 105,
    "project_id": 1314,
    "created_at": "2024-02-26T01:05:38.625Z",
    "updated_at": "2024-03-14T09:35:39.585Z",
    "status": "finished",
    "terminating_at": null,
    "users": [
      {
        "id": 172351,
        "login": "sooslee",
        "url": "https://api.intra.42.fr/v2/users/sooslee",
        "leader": true,
        "occurrence": 0,
        "validated": true,
        "projects_user_id": 3565884
      }
    ],
    "locked?": true,
    "validated?": true,
    "closed?": true,
    "repo_url": "git@vogsphere.42gyeongsan.kr:vogsphere/intra-uuid-3bf59d65-1d41-41d7-b04c-a30559266d13-5533766-sooslee",
    "repo_uuid": "intra-uuid-3bf59d65-1d41-41d7-b04c-a30559266d13-5533766-sooslee",
    "locked_at": "2024-02-26T01:05:38.663Z",
    "closed_at": "2024-03-14T06:32:54.618Z",
    "project_session_id": 11805,
    "project_gitlab_path": "pedago_world/42-cursus/inner-circle/libft",
    "scale_teams": [
      {
        "id": 6513656,
        "scale_id": 32445,
        "comment": "메모리 누수와 관련된 공부를 조금 더 해보시면 괜찮은 코드가 만들어질 것 같습니다. 그리고 split에서 반환값에 대한 why를 탐구하실 필요도 좀 있을 것으로 보입니다. 문제 푸시느라 수고하셨습니다. 화이팅!!",
        "created_at": "2024-03-14T08:12:42.365Z",
        "updated_at": "2024-03-14T09:35:24.573Z",
        "feedback": "친절한 평가 감사합니다. 저 한테 부족한점이 있었는데 친절한 지적으로 제게 모르는점을 깨닫게 해주셨습니다. 감사합니다.",
        "final_mark": 125,
        "flag": {
          "id": 9,
          "name": "Outstanding project",
          "positive": true,
          "icon": "star-1",
          "created_at": "2017-05-18T14:07:37.380Z",
          "updated_at": "2017-05-18T14:12:07.415Z"
        },
        "begin_at": "2024-03-14T09:00:00.000Z",
        "correcteds": [
          {
            "id": 172351,
            "login": "sooslee",
            "url": "https://api.intra.42.fr/v2/users/sooslee"
          }
        ],
        "corrector": {
          "id": 174086,
          "login": "siyukim",
          "url": "https://api.intra.42.fr/v2/users/siyukim"
        },
        "truant": {},
        "filled_at": "2024-03-14T09:34:24.370Z",
        "questions_with_answers": []
      },
      {
        "id": 6513560,
        "scale_id": 32445,
        "comment": "보너스 문제는 풀지 않은 상태로 sooslee님을 평가하러 왔습니다. 보너스 문제에 필요한 개념인 노드, 연결리스트부터 각 보너스 문제 함수를 어떻게 접근하셨는지 꼼꼼하게 설명해주셔서 구현이 잘 된 것을 확인할 수 있었습니다. 그 외의 다른 함수들도 깔끔하고 효율적으로 구현하신 것 같습니다. 배울 점이 많았습니다. 수고하셨습니다. 화이팅 ~ ",
        "created_at": "2024-03-14T07:43:42.002Z",
        "updated_at": "2024-03-14T08:59:05.336Z",
        "feedback": "저도 잘 모르는게 많아 같이 공부하는 느낌으로 설명을 드렸습니다. 저도 하면서 어떻게 설명을 더 할지 고민해조는 시간이 었고 좋은 질문으로 저도 많은 도움이 됐습니다.",
        "final_mark": 125,
        "flag": {
          "id": 9,
          "name": "Outstanding project",
          "positive": true,
          "icon": "star-1",
          "created_at": "2017-05-18T14:07:37.380Z",
          "updated_at": "2017-05-18T14:12:07.415Z"
        },
        "begin_at": "2024-03-14T08:15:00.000Z",
        "correcteds": [
          {
            "id": 172351,
            "login": "sooslee",
            "url": "https://api.intra.42.fr/v2/users/sooslee"
          }
        ],
        "corrector": {
          "id": 174132,
          "login": "yerpark",
          "url": "https://api.intra.42.fr/v2/users/yerpark"
        },
        "truant": {},
        "filled_at": "2024-03-14T08:58:12.075Z",
        "questions_with_answers": []
      },
      {
        "id": 6513409,
        "scale_id": 32445,
        "comment": "평가 중  문제가 생길 만한 부분은 하나도 없었습니다! 굉장히 다른 방식의 신박한 코드들을 작성하셔서 저 역시도 배워 갈 부분이 많은 평가였던 것 같습니다. 특히, lstadd back 부분에서 이중 포인터 리스트에 실제 노드를 접합시키는 과정에서, 저는 next에 추가한 후, 실제 이중 포인터에도 node를 추가했었는데, 그게 없어도 되겠다는 생각이 들었습니다. :D",
        "created_at": "2024-03-14T06:39:28.708Z",
        "updated_at": "2024-03-14T07:44:07.443Z",
        "feedback": "친절하고 자상한 평가 감사합니다. 덕분에 많은걸 배웠습니다.",
        "final_mark": 125,
        "flag": {
          "id": 9,
          "name": "Outstanding project",
          "positive": true,
          "icon": "star-1",
          "created_at": "2017-05-18T14:07:37.380Z",
          "updated_at": "2017-05-18T14:12:07.415Z"
        },
        "begin_at": "2024-03-14T07:15:00.000Z",
        "correcteds": [
          {
            "id": 172351,
            "login": "sooslee",
            "url": "https://api.intra.42.fr/v2/users/sooslee"
          }
        ],
        "corrector": {
          "id": 172329,
          "login": "inam",
          "url": "https://api.intra.42.fr/v2/users/inam"
        },
        "truant": {},
        "filled_at": "2024-03-14T07:41:47.580Z",
        "questions_with_answers": []
      }
    ],
    "teams_uploads": [
      {
        "id": 2127917,
        "final_mark": 85,
        "comment": "initial_errors:  | test_ft_isalpha: OK | test_ft_isdigit: OK | test_ft_isalnum: OK | test_ft_isascii: OK | test_ft_isprint: OK | test_ft_strlen: OK | test_ft_memset: OK | test_ft_bzero: OK | test_ft_memcpy: OK | test_ft_memmove: OK | test_ft_strlcpy: OK | test_ft_strlcat: OK | test_ft_toupper: OK | test_ft_tolower: OK | test_ft_strchr: OK | test_ft_strrchr: OK | test_ft_strncmp: OK | test_ft_memchr: OK | test_ft_memcmp: OK | test_ft_strnstr: OK | test_ft_atoi: OK | test_ft_calloc: OK | test_ft_strdup: OK | test_ft_substr: OK | test_ft_strjoin: OK | test_ft_strtrim: OK | test_ft_split: OK | test_ft_itoa: OK | test_ft_strmapi: OK | test_ft_striteri: KO (Does not compile) | test_ft_putchar_fd: OK | test_ft_putstr_fd: OK | test_ft_putendl_fd: OK | test_ft_putnbr_fd: OK | bonus: 9/9 functions correct",
        "created_at": "2024-03-14T09:35:39.583Z",
        "upload_id": 1558
      }
    ]
  },
  {
    "id": 5533767,
    "name": "sejjeong's group",
    "url": "https://api.intra.42.fr/v2/teams/5533767",
    "final_mark": 41,
    "project_id": 1314,
    "created_at": "2024-02-26T01:06:25.257Z",
    "updated_at": "2024-03-01T06:29:24.457Z",
    "status": "finished",
    "terminating_at": null,
    "users": [
      {
        "id": 174181,
        "login": "sejjeong",
        "url": "https://api.intra.42.fr/v2/users/sejjeong",
        "leader": true,
        "occurrence": 0,
        "validated": true,
        "projects_user_id": 3565885
      }
    ],
    "locked?": true,
    "validated?": false,
    "closed?": true,
    "repo_url": "git@vogsphere.42gyeongsan.kr:vogsphere/intra-uuid-9aad4f4d-597b-4815-aac2-a71cb3b3137d-5533767-sejjeong",
    "repo_uuid": "intra-uuid-9aad4f4d-597b-4815-aac2-a71cb3b3137d-5533767-sejjeong",
    "locked_at": "2024-02-26T01:06:25.285Z",
    "closed_at": "2024-03-01T03:41:15.875Z",
    "project_session_id": 11805,
    "project_gitlab_path": "pedago_world/42-cursus/inner-circle/libft",
    "scale_teams": [
      {
        "id": 6465538,
        "scale_id": 32445,
        "comment": "안녕하세요. 평가를 맡게된 younhbae 입니다. libft 만 네번째 평가를 맡고 있네요. split에서 free를 어떻게 사용해야 할지 감이 잡히지 않았는데, 그부분에 대해서 좋은 의견을 들을 수 있었습니다. 그리고 free이후에 습관적으로 NULL을 해주시는 것이 코딩 습관이 베어있는것 같아서 저도 배워야 겠다는 생각이 들었습니다. libft 는 문제 없이 통과하실 것 같습니다. 앞으로도 잘 부탁드립니다. 감사합니다.",
        "created_at": "2024-03-01T03:45:29.814Z",
        "updated_at": "2024-03-01T06:28:48.770Z",
        "feedback": "여러 테스트 케이스와 어려운 거 중점으로 깊게 테스트 하면서 문제점이 있나 없나 확인할 수 있었습니다. 꼭 통과하도록 하겠습니다. 감사합니다.",
        "final_mark": 125,
        "flag": {
          "id": 9,
          "name": "Outstanding project",
          "positive": true,
          "icon": "star-1",
          "created_at": "2017-05-18T14:07:37.380Z",
          "updated_at": "2017-05-18T14:12:07.415Z"
        },
        "begin_at": "2024-03-01T06:00:00.000Z",
        "correcteds": [
          {
            "id": 174181,
            "login": "sejjeong",
            "url": "https://api.intra.42.fr/v2/users/sejjeong"
          }
        ],
        "corrector": {
          "id": 172366,
          "login": "younhbae",
          "url": "https://api.intra.42.fr/v2/users/younhbae"
        },
        "truant": {},
        "filled_at": "2024-03-01T06:25:17.158Z",
        "questions_with_answers": []
      },
      {
        "id": 6465536,
        "scale_id": 32445,
        "comment": "두번째 평가를 하고 있습니다. 확실히 로우 레벨을 공부하셔서 접근방식이 새로웠고 좀 더 동기부여가 되었습니다.  \r\n파트1의 기초 함수들을  적극활용하여서 뒷부분의 복잡한 함수들을 깔끔하게 풀으셨습니다.\r\n또한 제가 과제를 진행하면서 궁금했던 strlcpy, strlcat 함수들의 반환값에 대한 질문들과 각각의 함수들을 만들면서 느낀점들에 대해서 의견을 들으며 많은 공부가 되었습니다. Makefile이 각각 mandatory 파트와 bonus파트가 분리되어 오브젝트파일과 라이브러리 파일을 만들습니다. norm 검사 또한 잘 통과하였습니다.\r\n고생하셨습니다. ",
        "created_at": "2024-03-01T03:44:32.683Z",
        "updated_at": "2024-03-01T05:59:09.654Z",
        "feedback": "doji님이랑 재밌게 대화하면서 평가 받을 수 있어서 좋았습니다 ㅎㅎ xor 연산의 빠름에 대한 지식과 42과제의 의도 등을 생각해 볼 수 있어서 매우 유익한 시간이었습니다. 감사합니다~",
        "final_mark": 125,
        "flag": {
          "id": 1,
          "name": "Ok",
          "positive": true,
          "icon": "check-4",
          "created_at": "2015-09-14T23:06:52.000Z",
          "updated_at": "2015-09-14T23:06:52.000Z"
        },
        "begin_at": "2024-03-01T05:00:00.000Z",
        "correcteds": [
          {
            "id": 174181,
            "login": "sejjeong",
            "url": "https://api.intra.42.fr/v2/users/sejjeong"
          }
        ],
        "corrector": {
          "id": 174127,
          "login": "doji",
          "url": "https://api.intra.42.fr/v2/users/doji"
        },
        "truant": {},
        "filled_at": "2024-03-01T05:49:03.274Z",
        "questions_with_answers": []
      },
      {
        "id": 6465532,
        "scale_id": 32445,
        "comment": "링크드 리스트에 대한 개념이 아직 부족해서 그거에 대해 많이 물어보려 했었고, 나머지 코드들도 잘 동작하는 것 같았습니다. 감사합니다!",
        "created_at": "2024-03-01T03:41:32.885Z",
        "updated_at": "2024-03-01T05:56:13.582Z",
        "feedback": "코드에 대한 설명을 하면서 다시 복습하게 되어 좋았습니다. 전체적으로 테스트도 해주셔서 좋았습니다. 감사합니다.",
        "final_mark": 125,
        "flag": {
          "id": 1,
          "name": "Ok",
          "positive": true,
          "icon": "check-4",
          "created_at": "2015-09-14T23:06:52.000Z",
          "updated_at": "2015-09-14T23:06:52.000Z"
        },
        "begin_at": "2024-03-01T04:15:00.000Z",
        "correcteds": [
          {
            "id": 174181,
            "login": "sejjeong",
            "url": "https://api.intra.42.fr/v2/users/sejjeong"
          }
        ],
        "corrector": {
          "id": 172368,
          "login": "yuhjeon",
          "url": "https://api.intra.42.fr/v2/users/yuhjeon"
        },
        "truant": {},
        "filled_at": "2024-03-01T04:57:55.907Z",
        "questions_with_answers": []
      }
    ],
    "teams_uploads": [
      {
        "id": 2115247,
        "final_mark": -42,
        "comment": "initial_errors: Forbidden function call detected, Cheating | test_ft_isalpha: KO (Does not compile), KO (Does not compile) | test_ft_isdigit: KO (Does not compile), KO (Does not compile) | test_ft_isalnum: KO (Does not compile), KO (Does not compile) | test_ft_isascii: KO (Does not compile), KO (Does not compile) | test_ft_isprint: KO (Does not compile), KO (Does not compile) | test_ft_strlen: KO (Does not compile), KO (Does not compile) | test_ft_memset: KO (Does not compile), KO (Does not compile) | test_ft_bzero: KO (Does not compile), KO (Does not compile) | test_ft_memcpy: KO (Does not compile), KO (Does not compile) | test_ft_memmove: KO (Does not compile), KO (Does not compile) | test_ft_strlcpy: KO (Does not compile), KO (Does not compile) | test_ft_strlcat: KO (Does not compile), KO (Does not compile) | test_ft_toupper: KO (Does not compile), KO (Does not compile) | test_ft_tolower: KO (Does not compile), KO (Does not compile) | test_ft_strchr: KO (Does not compile), KO (Does not compile) | test_ft_strrchr: KO (Does not compile), KO (Does not compile) | test_ft_strncmp: KO (Does not compile), KO (Does not compile) | test_ft_memchr: KO (Does not compile), KO (Does not compile) | test_ft_memcmp: KO (Does not compile), KO (Does not compile) | test_ft_strnstr: KO (Does not compile), KO (Does not compile) | test_ft_atoi: KO (Does not compile), KO (Does not compile) | test_ft_calloc: KO (Does not compile), KO (Does not compile) | test_ft_strdup: KO (Does not compile), KO (Does not compile) | test_ft_substr: KO (Does not compile), KO (Does not compile) | test_ft_strjoin: KO (Does not compile), KO (Does not compile) | test_ft_strtrim: KO (Does not compile), KO (Does not compile) | test_ft_split: KO (Does not compile), KO (Does not compile) | test_ft_itoa: KO (Does not compile), KO (Does not compile) | test_ft_strmapi: KO (Does not compile), KO (Does not compile) | test_ft_striteri: KO (Does not compile), KO (Does not compile) | test_ft_putchar_fd: KO (Does not compile), KO (Does not compile) | test_ft_putstr_fd: KO (Does not compile), KO (Does not compile) | test_ft_putendl_fd: KO (Does not compile), KO (Does not compile) | test_ft_putnbr_fd: KO (Does not compile), KO (Does not compile) | bonus: 9/9 functions correct",
        "created_at": "2024-03-01T06:29:24.456Z",
        "upload_id": 1558
      }
    ]
  },
  {
    "id": 5533768,
    "name": "seku's group",
    "url": "https://api.intra.42.fr/v2/teams/5533768",
    "final_mark": 102,
    "project_id": 1314,
    "created_at": "2024-02-26T01:07:29.037Z",
    "updated_at": "2024-03-22T15:20:13.192Z",
    "status": "finished",
    "terminating_at": null,
    "users": [
      {
        "id": 174091,
        "login": "seku",
        "url": "https://api.intra.42.fr/v2/users/seku",
        "leader": true,
        "occurrence": 0,
        "validated": true,
        "projects_user_id": 3565886
      }
    ],
    "locked?": true,
    "validated?": true,
    "closed?": true,
    "repo_url": "git@vogsphere.42gyeongsan.kr:vogsphere/intra-uuid-8a260f12-51d7-406d-a0f7-bae894be8c1c-5533768-seku",
    "repo_uuid": "intra-uuid-8a260f12-51d7-406d-a0f7-bae894be8c1c-5533768-seku",
    "locked_at": "2024-02-26T01:07:29.094Z",
    "closed_at": "2024-03-20T11:15:26.422Z",
    "project_session_id": 11805,
    "project_gitlab_path": "pedago_world/42-cursus/inner-circle/libft",
    "scale_teams": [
      {
        "id": 6552322,
        "scale_id": 32445,
        "comment": "늦은 밤 밤샘으로인해 매우 피곤하실텐데도 동료평가에 친절히 응해주셔서 감사했습니다. 이번 평가는 첫번째 트라이라고 하셔서 모든 코드를 매우 세세하게 분석하고, 설명 및 피드백하는 시간을 가졌습니다. 거의 대부분의 코드들이 효율적으로 잘 구성되었고, 동작하였으나 아쉽게도  ft_strchr()에서 '\\0'을 검색하지 못하는 이슈가 있어서 이것만 고친다면 무난하게 125점으로 통과하실 것 같습니다. 화이팅입니다!",
        "created_at": "2024-03-22T13:38:24.632Z",
        "updated_at": "2024-03-22T15:19:53.074Z",
        "feedback": "평가를 각 코드마다 자세히 분석하고 설명해주시는점이 정말 좋았습니다. 특히 제가 생각하지 못했던 lcat에서 strlen의 부분이나, strlcat에서 Return Value에 대해서 자세히 설명해주셔서 좋았습니다. 또한 제가 생각하지 못한 방법으로 접근해주셔서 문제를 해결 할 수 있는 아이디어를 제공해주셨습니다. 평가하는동안 좋은시간이였습니다.",
        "final_mark": 112,
        "flag": {
          "id": 9,
          "name": "Outstanding project",
          "positive": true,
          "icon": "star-1",
          "created_at": "2017-05-18T14:07:37.380Z",
          "updated_at": "2017-05-18T14:12:07.415Z"
        },
        "begin_at": "2024-03-22T14:15:00.000Z",
        "correcteds": [
          {
            "id": 174091,
            "login": "seku",
            "url": "https://api.intra.42.fr/v2/users/seku"
          }
        ],
        "corrector": {
          "id": 174157,
          "login": "seonhwan",
          "url": "https://api.intra.42.fr/v2/users/seonhwan"
        },
        "truant": {},
        "filled_at": "2024-03-22T15:16:21.208Z",
        "questions_with_answers": []
      },
      {
        "id": 6552033,
        "scale_id": 32445,
        "comment": "이번에 libft 처음 평가라고 하셔서 최대한 제가 아는 선에서 열심히 봐드릴려고 했던 거 같습니다. strncmp에서 기계평가에서 리턴값에 차이가 있으면 어떻게 평가를 할지 저도 애매하지만 맞으실 거 같습니다. 그리고 strtrim은 제가 봤던 코드 중 가장 짧고 간단한 코드여서 참신했습니다. 또 putnbr에서 int 최대값 언더플로우 방지를 char로 받아서 방지하는 거를 알게되어 배워갑니다ㅎ 그리고 Makefile에서 보너스 파트 리링크도 잘 방지해주셔서 다른 이상은 없으신 거 같습니다. 이번에 한번에 붙으시길 바랄게요. 고생하셨습니다!!!",
        "created_at": "2024-03-22T12:11:06.685Z",
        "updated_at": "2024-03-22T14:07:02.938Z",
        "feedback": "안녕하세요 taehykwo님 . 코드에 대해서 이야기를 하면서 즐거운 분위기에서 평가를 해주셔서 감사했습니다. memcmp와 strcmp에 대해서 이야기를 나누면서 어떻게 될지, 이야기들을 나누었고, calloc에 대해서 nmemb과 size에 0일때 retrun Value에 대해서 이야기도 나누어서 좋았습니다. 전체적으로 코드를 보시고 궁금한 점들을 답변드릴수 있어서 좋았습니다. ",
        "final_mark": 125,
        "flag": {
          "id": 1,
          "name": "Ok",
          "positive": true,
          "icon": "check-4",
          "created_at": "2015-09-14T23:06:52.000Z",
          "updated_at": "2015-09-14T23:06:52.000Z"
        },
        "begin_at": "2024-03-22T12:45:00.000Z",
        "correcteds": [
          {
            "id": 174091,
            "login": "seku",
            "url": "https://api.intra.42.fr/v2/users/seku"
          }
        ],
        "corrector": {
          "id": 174119,
          "login": "taehykwo",
          "url": "https://api.intra.42.fr/v2/users/taehykwo"
        },
        "truant": {},
        "filled_at": "2024-03-22T13:18:20.533Z",
        "questions_with_answers": []
      },
      {
        "id": 6551336,
        "scale_id": 32445,
        "comment": "it was really fun evaluating seku on his libft project. His code was really clean so it was really interesting to read. He also lacked explanation in some parts but he was fine in most parts so i gave him an ok. He also made a really clean code on strjoin which was really satisfying to read . Overall i think seku has quite a high understanding in this project. I hope he passes this evaluation!",
        "created_at": "2024-03-22T09:38:38.992Z",
        "updated_at": "2024-03-22T11:06:04.433Z",
        "feedback": "안녕하세요 chan-ypa님 제 libft 첫평가를 맡아주셔서 감사합니다. libft가 많은 파일이 있기도 하고 설명을 모호하고 잘 전달드리지 못한부분도 있었는데, 잘 이해해주시고 보충설명을 해주셔서 좋았습니다. strlcat같은 부분이나, memmove에서 underflow , overflow가 나타나는지 아닌지를 말씀해주셔서  좋은 평가가 되었던 것 같습니다. 고생하셨습니다",
        "final_mark": 125,
        "flag": {
          "id": 1,
          "name": "Ok",
          "positive": true,
          "icon": "check-4",
          "created_at": "2015-09-14T23:06:52.000Z",
          "updated_at": "2015-09-14T23:06:52.000Z"
        },
        "begin_at": "2024-03-22T10:15:00.000Z",
        "correcteds": [
          {
            "id": 174091,
            "login": "seku",
            "url": "https://api.intra.42.fr/v2/users/seku"
          }
        ],
        "corrector": {
          "id": 172394,
          "login": "chan-ypa",
          "url": "https://api.intra.42.fr/v2/users/chan-ypa"
        },
        "truant": {},
        "filled_at": "2024-03-22T10:59:55.657Z",
        "questions_with_answers": []
      }
    ],
    "teams_uploads": [
      {
        "id": 2139019,
        "final_mark": 85,
        "comment": "initial_errors:  | test_ft_isalpha: OK | test_ft_isdigit: OK | test_ft_isalnum: OK | test_ft_isascii: OK | test_ft_isprint: OK | test_ft_strlen: OK | test_ft_memset: OK | test_ft_bzero: OK | test_ft_memcpy: OK | test_ft_memmove: OK | test_ft_strlcpy: OK | test_ft_strlcat: OK | test_ft_toupper: OK | test_ft_tolower: OK | test_ft_strchr: OK | test_ft_strrchr: OK | test_ft_strncmp: OK | test_ft_memchr: OK | test_ft_memcmp: OK | test_ft_strnstr: OK | test_ft_atoi: OK | test_ft_calloc: OK | test_ft_strdup: OK | test_ft_substr: OK | test_ft_strjoin: OK | test_ft_strtrim: OK | test_ft_split: OK | test_ft_itoa: OK | test_ft_strmapi: OK | test_ft_striteri: KO (Does not compile) | test_ft_putchar_fd: OK | test_ft_putstr_fd: OK | test_ft_putendl_fd: OK | test_ft_putnbr_fd: OK | bonus: 9/9 functions correct",
        "created_at": "2024-03-22T15:20:13.191Z",
        "upload_id": 1558
      }
    ]
  },
  {
    "id": 5533769,
    "name": "myeochoi's group",
    "url": "https://api.intra.42.fr/v2/teams/5533769",
    "final_mark": 125,
    "project_id": 1314,
    "created_at": "2024-02-26T01:07:35.111Z",
    "updated_at": "2024-03-02T06:42:22.365Z",
    "status": "finished",
    "terminating_at": null,
    "users": [
      {
        "id": 172342,
        "login": "myeochoi",
        "url": "https://api.intra.42.fr/v2/users/myeochoi",
        "leader": true,
        "occurrence": 0,
        "validated": true,
        "projects_user_id": 3565887
      }
    ],
    "locked?": true,
    "validated?": true,
    "closed?": true,
    "repo_url": "git@vogsphere.42gyeongsan.kr:vogsphere/intra-uuid-d5af57cf-56d5-416d-9d08-122ce53f56b1-5533769-myeochoi",
    "repo_uuid": "intra-uuid-d5af57cf-56d5-416d-9d08-122ce53f56b1-5533769-myeochoi",
    "locked_at": "2024-02-26T01:07:35.143Z",
    "closed_at": "2024-03-02T01:48:31.681Z",
    "project_session_id": 11805,
    "project_gitlab_path": "pedago_world/42-cursus/inner-circle/libft",
    "scale_teams": [
      {
        "id": 6468298,
        "scale_id": 32445,
        "comment": "평가를 맡게된 younhbae 입니다. 48개에 달하는 함수를 하나하나씩 상세하게 설명해주셔서, 함수를 리마인드하게 되어 좋았습니다. 나중에 libft가 끝나고서도 strnstr은 다시 작성하는게 좋을 것 같습니다. lst에 대한 개념에 대해서도 상세히 알게되어서 좋았습니다. 1시간 30분동안 설명해주시느라 고생하셨습니다. ",
        "created_at": "2024-03-02T02:51:28.564Z",
        "updated_at": "2024-03-02T06:37:17.990Z",
        "feedback": "안녕하세요. younhbae님에게 libft 평가를 받았습니다. 평가 시간 동안 많은 호기심으로 같이 여러 상황들을 탐구할 수 있었던 유익한 시간이었습니다. 또한 int의 오버플로우의 중요성을 다시 알려주셔서 감사합니다. 고생하셨습니다.",
        "final_mark": 125,
        "flag": {
          "id": 1,
          "name": "Ok",
          "positive": true,
          "icon": "check-4",
          "created_at": "2015-09-14T23:06:52.000Z",
          "updated_at": "2015-09-14T23:06:52.000Z"
        },
        "begin_at": "2024-03-02T05:15:00.000Z",
        "correcteds": [
          {
            "id": 172342,
            "login": "myeochoi",
            "url": "https://api.intra.42.fr/v2/users/myeochoi"
          }
        ],
        "corrector": {
          "id": 172366,
          "login": "younhbae",
          "url": "https://api.intra.42.fr/v2/users/younhbae"
        },
        "truant": {},
        "filled_at": "2024-03-02T05:41:31.062Z",
        "questions_with_answers": []
      },
      {
        "id": 6468295,
        "scale_id": 32445,
        "comment": "is... 함수들을 작성할때 가독성을 위해서 되도록 'c'로 직접 입력해 비교하는걸 권장드립니다. 확장 아스키는 고려하지 않는게 맞다고 생각합니다. 전체적으로 변수명을 줄여서 사용하시는 편인데 반환하는 값이나 반복문의 인자로 사용되는 변수들은 각 기능에 맞게 좀 더 설명이 쉬운 변수명을 사용하는걸 권장드립니다. 말씀드렸다시피 같은 기능을 하는 코드들은 기존에 만든 함수를 호출해 대체할 수 있으니 줄 수를 줄이는데 도움이 될 겁니다. 함수 중 int를 char로 캐스팅하는 경우가 있었습니다. 이 경우 size를 줄여버려서 의도하지 않은 값을 입력할 가능성이 있긴 합니다만 문제에서 요구하는 기능을 고려해 봤을 때 문제가 없어 보여 ok를 드렸습니다. 긴 시간 평가 보시느라 수고 많으셨습니다.",
        "created_at": "2024-03-02T02:46:03.610Z",
        "updated_at": "2024-03-02T06:35:54.472Z",
        "feedback": "안녕하세요. jaegbae님에게 libft 평가를 받았습니다. 과제를 진행하면서 미처 생각하지 못했던 캐스팅의 중요성과, 여러 인자들의 오버 플로우의 가능성 등 많은 것을 배울 수 있는 시간이었습니다. 실제로 비트들이 연산되는 과정에 대해서도 예시와 함께 설명해주셔서 감사했습니다. 고생하셨습니다.",
        "final_mark": 125,
        "flag": {
          "id": 1,
          "name": "Ok",
          "positive": true,
          "icon": "check-4",
          "created_at": "2015-09-14T23:06:52.000Z",
          "updated_at": "2015-09-14T23:06:52.000Z"
        },
        "begin_at": "2024-03-02T05:00:00.000Z",
        "correcteds": [
          {
            "id": 172342,
            "login": "myeochoi",
            "url": "https://api.intra.42.fr/v2/users/myeochoi"
          }
        ],
        "corrector": {
          "id": 174111,
          "login": "jaegbae",
          "url": "https://api.intra.42.fr/v2/users/jaegbae"
        },
        "truant": {},
        "filled_at": "2024-03-02T06:31:47.201Z",
        "questions_with_answers": []
      },
      {
        "id": 6468292,
        "scale_id": 32445,
        "comment": "반갑습니다. 이번에 평가자로 오게 되었습니다. 앞 부분(mandatory part)에서 함수들에 대해 주석으로 테스트케이스를 잘 만들어 놓으셔서 인상깊게 보았습니다. strnstr함수 같은 경우 구조체를 사용하여 문제를 푼 방식도 새로웠네요. 보너스 문제에서도 재귀함수에 대한 높은 이해도로 문제의 요구사항을 잘 해결하신 것 같습니다. 좋은 성적 기대합니다.",
        "created_at": "2024-03-02T02:41:20.517Z",
        "updated_at": "2024-03-02T06:33:46.102Z",
        "feedback": "안녕하세요. ksuh님에게 libft 평가를 받았습니다. 평가를 진행하면서 널가드의 중요성과 함수의 정확한 개념에 대해서 더욱 배울 수 있는 좋은 기회였습니다. 고생많으셨습니다.",
        "final_mark": 125,
        "flag": {
          "id": 1,
          "name": "Ok",
          "positive": true,
          "icon": "check-4",
          "created_at": "2015-09-14T23:06:52.000Z",
          "updated_at": "2015-09-14T23:06:52.000Z"
        },
        "begin_at": "2024-03-02T04:15:00.000Z",
        "correcteds": [
          {
            "id": 172342,
            "login": "myeochoi",
            "url": "https://api.intra.42.fr/v2/users/myeochoi"
          }
        ],
        "corrector": {
          "id": 172367,
          "login": "ksuh",
          "url": "https://api.intra.42.fr/v2/users/ksuh"
        },
        "truant": {},
        "filled_at": "2024-03-02T05:37:48.195Z",
        "questions_with_answers": []
      }
    ],
    "teams_uploads": [
      {
        "id": 2115805,
        "final_mark": 125,
        "comment": "initial_errors:  | test_ft_isalpha: OK | test_ft_isdigit: OK | test_ft_isalnum: OK | test_ft_isascii: OK | test_ft_isprint: OK | test_ft_strlen: OK | test_ft_memset: OK | test_ft_bzero: OK | test_ft_memcpy: OK | test_ft_memmove: OK | test_ft_strlcpy: OK | test_ft_strlcat: OK | test_ft_toupper: OK | test_ft_tolower: OK | test_ft_strchr: OK | test_ft_strrchr: OK | test_ft_strncmp: OK | test_ft_memchr: OK | test_ft_memcmp: OK | test_ft_strnstr: OK | test_ft_atoi: OK | test_ft_calloc: OK | test_ft_strdup: OK | test_ft_substr: OK | test_ft_strjoin: OK | test_ft_strtrim: OK | test_ft_split: OK | test_ft_itoa: OK | test_ft_strmapi: OK | test_ft_striteri: OK | test_ft_putchar_fd: OK | test_ft_putstr_fd: OK | test_ft_putendl_fd: OK | test_ft_putnbr_fd: OK | bonus: 9/9 functions correct",
        "created_at": "2024-03-02T06:42:22.363Z",
        "upload_id": 1558
      }
    ]
  },
  {
    "id": 5533770,
    "name": "inam's group",
    "url": "https://api.intra.42.fr/v2/teams/5533770",
    "final_mark": 0,
    "project_id": 1314,
    "created_at": "2024-02-26T01:08:24.466Z",
    "updated_at": "2024-03-02T08:56:13.756Z",
    "status": "finished",
    "terminating_at": null,
    "users": [
      {
        "id": 172329,
        "login": "inam",
        "url": "https://api.intra.42.fr/v2/users/inam",
        "leader": true,
        "occurrence": 0,
        "validated": true,
        "projects_user_id": 3565888
      }
    ],
    "locked?": true,
    "validated?": false,
    "closed?": true,
    "repo_url": "git@vogsphere.42gyeongsan.kr:vogsphere/intra-uuid-e49bc6b3-4e11-474e-b00c-9ca3bd6acf1e-5533770-inam",
    "repo_uuid": "intra-uuid-e49bc6b3-4e11-474e-b00c-9ca3bd6acf1e-5533770-inam",
    "locked_at": "2024-02-26T01:08:24.493Z",
    "closed_at": "2024-03-02T07:29:02.904Z",
    "project_session_id": 11805,
    "project_gitlab_path": "pedago_world/42-cursus/inner-circle/libft",
    "scale_teams": [
      {
        "id": 6468523,
        "scale_id": 32445,
        "comment": "코드를 전반적으로 간결하게 작성하시려고 노력하신 부분이 멋지다고 생각합니다! 특히 strlcat함수나 strdup함수를 가져와서 split함수 등을 구현하는 데 사용하셨는데, 다른 static함수를 만들어서 사용하는 것 보다도 훨씬 간결하게 함수를 구축하신 부분이 인상적이었습니다. 덕분에 저도 아이디어를 얻어갈 수 있었습니다! strmapi함수와 striteri함수의 함수포인터와 관련한 부분에 있어서는 피평가자분과 같이 고민을 해봤으나 일단은 제시된 프로토타입과 다른 부분이 있어서 no를 드리게 되었습니다. 돌아가서 저도 조금 더 고민해보고 알게되는 부분 있으면 dm드릴게요..! 아 그리고 makefile 작성법에 대해서 많은 고민을 하신 부분에서 학습에 대한 열의를 느낄 수 있었습니다. 긴 시간 고생많으셨습니다!!",
        "created_at": "2024-03-02T07:29:18.065Z",
        "updated_at": "2024-03-02T08:56:06.760Z",
        "feedback": "평가하는 내내 말 하나하나에 신중을 가하시고 배려하시는 게 눈에 보였습니다. 일단, 제가 Makefile에 대한 지식이 많이 부족해서 그냥  나무위키나 공식 문서에 있는 내용을 가져와서 작동하는 방식 혹은 파일만 변경해서 사용을 했었는데, 평가 과정에서 디테일한 부분, 그리고 PRONY 등등 오해하고 있는 부분에 대해서 상세히 알려주셔서 도움이 많이 되었습니다. 또한, 코드 중간중간 여러 디펜스적 질문을 던지셨는데, 제가 대답을 명쾌하게 하지 못할 경우 그거로 끝나는 게 아니라 피드백을 주셔서 관련 지식 역시 많이 얻어갈 수 있었습니다. 그리고 평가 도중 놓치기 쉬운 부분인 포인터 미표기를 잘 캐치하셔서 그 부분을 빠르게 인지하고 픽스할 수 있게 되었습니다. 전체적으로 제가 얻어가는 부분이 아주 많은 평가여서 좋았습니다. 주말에도 나와서 열심이신데, 컨디션 조절 잘 하셔서 쭉 가셨으면 좋겠습니다 :D",
        "final_mark": 0,
        "flag": {
          "id": 1,
          "name": "Ok",
          "positive": true,
          "icon": "check-4",
          "created_at": "2015-09-14T23:06:52.000Z",
          "updated_at": "2015-09-14T23:06:52.000Z"
        },
        "begin_at": "2024-03-02T08:00:00.000Z",
        "correcteds": [
          {
            "id": 172329,
            "login": "inam",
            "url": "https://api.intra.42.fr/v2/users/inam"
          }
        ],
        "corrector": {
          "id": 172396,
          "login": "yeoju",
          "url": "https://api.intra.42.fr/v2/users/yeoju"
        },
        "truant": {},
        "filled_at": "2024-03-02T08:51:27.811Z",
        "questions_with_answers": []
      }
    ],
    "teams_uploads": []
  },
  {
    "id": 5533771,
    "name": "jaegbae's group",
    "url": "https://api.intra.42.fr/v2/teams/5533771",
    "final_mark": 125,
    "project_id": 1314,
    "created_at": "2024-02-26T01:09:02.784Z",
    "updated_at": "2024-03-04T12:12:24.777Z",
    "status": "finished",
    "terminating_at": null,
    "users": [
      {
        "id": 174111,
        "login": "jaegbae",
        "url": "https://api.intra.42.fr/v2/users/jaegbae",
        "leader": true,
        "occurrence": 0,
        "validated": true,
        "projects_user_id": 3565889
      }
    ],
    "locked?": true,
    "validated?": true,
    "closed?": true,
    "repo_url": "git@vogsphere.42gyeongsan.kr:vogsphere/intra-uuid-d3822d82-5011-4652-9050-628580d7ab00-5533771-jaegbae",
    "repo_uuid": "intra-uuid-d3822d82-5011-4652-9050-628580d7ab00-5533771-jaegbae",
    "locked_at": "2024-02-26T01:09:02.813Z",
    "closed_at": "2024-03-04T09:49:46.257Z",
    "project_session_id": 11805,
    "project_gitlab_path": "pedago_world/42-cursus/inner-circle/libft",
    "scale_teams": [
      {
        "id": 6475200,
        "scale_id": 32445,
        "comment": "아무 문제없이 잘하신거 같습니다 split이 기가 막힌 코드였습니다",
        "created_at": "2024-03-04T10:00:09.919Z",
        "updated_at": "2024-03-04T12:12:03.727Z",
        "feedback": "평가 즐거웠습니다. 수고 많으셨습니다 ㅎㅎ",
        "final_mark": 125,
        "flag": {
          "id": 9,
          "name": "Outstanding project",
          "positive": true,
          "icon": "star-1",
          "created_at": "2017-05-18T14:07:37.380Z",
          "updated_at": "2017-05-18T14:12:07.415Z"
        },
        "begin_at": "2024-03-04T12:00:00.000Z",
        "correcteds": [
          {
            "id": 174111,
            "login": "jaegbae",
            "url": "https://api.intra.42.fr/v2/users/jaegbae"
          }
        ],
        "corrector": {
          "id": 174105,
          "login": "seoyokim",
          "url": "https://api.intra.42.fr/v2/users/seoyokim"
        },
        "truant": {},
        "filled_at": "2024-03-04T12:11:02.088Z",
        "questions_with_answers": []
      },
      {
        "id": 6475173,
        "scale_id": 32445,
        "comment": "모든 문제 정확히 풀이하신 듯 하고, 최대한 여러 개념을 사용하고 깔끔하게 작성하려 하신 모습이 보입니다. 몇몇 함수들에서 다른 함수를 호출해 사용하고, list 문제들의 경우 재귀를 활용해 가독성이 뛰어났습니다. 다양한 풀이를 볼 수 있어 좋았습니다!",
        "created_at": "2024-03-04T09:53:21.250Z",
        "updated_at": "2024-03-04T11:45:29.527Z",
        "feedback": "코드에 대한 이해가 뛰어나셔서 다른 코드 스타일임에도 설명에 큰 무리가 없었습니다. 감사합니다. 평가 수고 많으셨습니다 ㅎㅎㅎ",
        "final_mark": 125,
        "flag": {
          "id": 9,
          "name": "Outstanding project",
          "positive": true,
          "icon": "star-1",
          "created_at": "2017-05-18T14:07:37.380Z",
          "updated_at": "2017-05-18T14:12:07.415Z"
        },
        "begin_at": "2024-03-04T11:15:00.000Z",
        "correcteds": [
          {
            "id": 174111,
            "login": "jaegbae",
            "url": "https://api.intra.42.fr/v2/users/jaegbae"
          }
        ],
        "corrector": {
          "id": 174185,
          "login": "geonyole",
          "url": "https://api.intra.42.fr/v2/users/geonyole"
        },
        "truant": {},
        "filled_at": "2024-03-04T11:36:45.693Z",
        "questions_with_answers": []
      },
      {
        "id": 6475169,
        "scale_id": 32445,
        "comment": "이 평가를 언어로 표현할 수 있을까요..? 정말 제가 지금까지 만나본 분들 중 최고입니다.\r\n단순히 함수를 구현하는것에 의의를 두는 것이 아니라 함수가 가진 각 의미와 메뉴얼 페이지를 분석하고, 구현한 함수와 원본함수를 비교하며 처리속도가 다른 이유를 알아내는 열정...!!!!!!!!!!!!!\r\n너무 놀랍고 감동스러운 시간이었습니다.\r\n이 과제가 통과하지 못한다면 그건 정말 서운하네요......... 제가 틀린 이유는 납득하지만 이분의 과제가 통과하지 못한다면 뮤리넷.. 실망할겁니다....\r\n만약 이 과제가 fail이라면 뮬리넷은 이 분께 설명을 듣길 바랍니다. 적어도 뮬리넷보다 논리적임 ㅇㅇ\r\n아무튼 정말 정말 c언어를 개발하신 캠 톤슨과 케이스 리치가 이 설명을 들었다면 눈물을 흘리며 대학원으로 납치하셨을 겁니다.\r\n정말정말 수고하셨습니다.!!!",
        "created_at": "2024-03-04T09:51:52.135Z",
        "updated_at": "2024-03-04T11:41:30.425Z",
        "feedback": "평가를 하는 동안 제 코드에 많은 흥미를 느끼고 설명에 관심을 가져주셔서 계속 떠들었는데도 지치지 않고 잘 들어주셨군요 ㅎㅎ libft과제 자체가 함수가 상당히 많고 man 자체에 놓칠만한 부분이 많아 묻기도 전에 먼저 떠들었는데 즐거우셨다니 다행입니다. 평가 수고 많으셨습니다!!",
        "final_mark": 125,
        "flag": {
          "id": 9,
          "name": "Outstanding project",
          "positive": true,
          "icon": "star-1",
          "created_at": "2017-05-18T14:07:37.380Z",
          "updated_at": "2017-05-18T14:12:07.415Z"
        },
        "begin_at": "2024-03-04T10:30:00.000Z",
        "correcteds": [
          {
            "id": 174111,
            "login": "jaegbae",
            "url": "https://api.intra.42.fr/v2/users/jaegbae"
          }
        ],
        "corrector": {
          "id": 174081,
          "login": "hyebinle",
          "url": "https://api.intra.42.fr/v2/users/hyebinle"
        },
        "truant": {},
        "filled_at": "2024-03-04T11:20:32.861Z",
        "questions_with_answers": []
      }
    ],
    "teams_uploads": [
      {
        "id": 2117450,
        "final_mark": 125,
        "comment": "initial_errors:  | test_ft_isalpha: OK | test_ft_isdigit: OK | test_ft_isalnum: OK | test_ft_isascii: OK | test_ft_isprint: OK | test_ft_strlen: OK | test_ft_memset: OK | test_ft_bzero: OK | test_ft_memcpy: OK | test_ft_memmove: OK | test_ft_strlcpy: OK | test_ft_strlcat: OK | test_ft_toupper: OK | test_ft_tolower: OK | test_ft_strchr: OK | test_ft_strrchr: OK | test_ft_strncmp: OK | test_ft_memchr: OK | test_ft_memcmp: OK | test_ft_strnstr: OK | test_ft_atoi: OK | test_ft_calloc: OK | test_ft_strdup: OK | test_ft_substr: OK | test_ft_strjoin: OK | test_ft_strtrim: OK | test_ft_split: OK | test_ft_itoa: OK | test_ft_strmapi: OK | test_ft_striteri: OK | test_ft_putchar_fd: OK | test_ft_putstr_fd: OK | test_ft_putendl_fd: OK | test_ft_putnbr_fd: OK | bonus: 9/9 functions correct",
        "created_at": "2024-03-04T12:12:24.775Z",
        "upload_id": 1558
      }
    ]
  },
  {
    "id": 5533772,
    "name": "doyoukim's group",
    "url": "https://api.intra.42.fr/v2/teams/5533772",
    "final_mark": 125,
    "project_id": 1314,
    "created_at": "2024-02-26T01:10:07.938Z",
    "updated_at": "2024-03-19T14:43:55.621Z",
    "status": "finished",
    "terminating_at": null,
    "users": [
      {
        "id": 172307,
        "login": "doyoukim",
        "url": "https://api.intra.42.fr/v2/users/doyoukim",
        "leader": true,
        "occurrence": 0,
        "validated": true,
        "projects_user_id": 3565890
      }
    ],
    "locked?": true,
    "validated?": true,
    "closed?": true,
    "repo_url": "git@vogsphere.42gyeongsan.kr:vogsphere/intra-uuid-0e897040-54b2-46ae-b6dd-feecf724259c-5533772-doyoukim",
    "repo_uuid": "intra-uuid-0e897040-54b2-46ae-b6dd-feecf724259c-5533772-doyoukim",
    "locked_at": "2024-02-26T01:10:07.989Z",
    "closed_at": "2024-03-19T08:06:22.048Z",
    "project_session_id": 11805,
    "project_gitlab_path": "pedago_world/42-cursus/inner-circle/libft",
    "scale_teams": [
      {
        "id": 6533698,
        "scale_id": 32445,
        "comment": "안녕하세요 seongkim입니다. doyoukim님과는 평소에 친분이 있어 친하게 지내는 분인데 이렇게 평가에서 만나게 될 줄 은 몰랐습니다. 그래서 약간 즐거운 마음으로 평가를 하러 왔고 지금 피신이 끝나고 본과정이 시작한 이후에 평가는 libft 통과를 위해서 3번 받았지만 제가 평가를 보러간것은 처음입니다. 그래서 이번 평가는 본과정에서 처음 보는 다른 사람의 코드인데 크게 감명을 받은 코드였습니다. \r\n\r\nft_toupper와 ft_tolower는 굉장히 간단한 함수여서 저는 간단하게 생각하고 넘어갔는데 doyoukim은 비트연산자를 사용해서 문제를 푸는 것에 대해서 놀랍다고 생각했습니다.  ft_split에서는 보통 다른 사람들간의 알고리즘의 차이라면 단어의 개수를 세는 방법이 단어 시작을 기준으로 셀것인가 끝을 기준으로 둘것이냐의 차이가 다였던것 같은데 doyoukim님은 strtok를 활용한 알고리즘을 적용하셔서 다른 분들에게는 볼 수 없었던 코드를 보여주셔서 즐거웠습니다. 또 보너스에 리스트에 관한 함수 구현 문제에서 구조체와 구조체 포인터에 대한 이해를 위해서 포인터 맴버 연산자와 맴버 연산자를 모두 사용하시는 것을 보고 놀랍다는 생각이 들었습니다. 또한 파일 디스크립터에 대한 문제에서 또한 test용 main함수를 구현하셨는데 open함수에 O_RDWR | O_CREAT, 0644 등 다양한 매개인자를 주고 있어 이것에 대해서 질문을 하였을때도 정확하고 끊김없이 답변을 하셨습니다. 이것을 통해서 파일 디스크립터에 대한 공부를 완벽하게 하셨구나! 라는 생각이 들었습니다. 또 제가 놓쳤던 ft_calloc의 인자의 곱이 오버플로우가 발생했을때에 대한 처리도 포함되어 코드를 저 처럼 덤벙덤벙거리지 않고 정상을 다해서 짜셨다는 것이 느껴지는 결과물이였습니다. \r\n\r\n고생하셨고 앞으로 남은 기간 맴버가 되는 그날까지 파이팅입니다!",
        "created_at": "2024-03-19T10:54:04.501Z",
        "updated_at": "2024-03-19T14:43:36.494Z",
        "feedback": "안녕하세요. seongkim님! 굉장히 코딩에 능숙하신 분이라 평가 오게 되셔서 많은 점을 배울 수 있을 것 같아 좋았습니다! 우선 seongkim님이 visual code를 활용해서 리뷰를 해주신 점이 굉장히 편해서 제가 평가갈때도 잘 활용할 것 같습니다. 변수에 대입할 때, 자료형이 다른 부분은 항상 캐스팅을 주의해서 넣어줘야 하는 점을 memcpy에서 발견하셔서 알려주셨습니다. dest와 src를 그대로 n_dest와 n_src로 넣었는데 이 부분이 문제가 발생할 수 있는 가능성이 있으므로 앞으로는 주의해서 꼭 캐스팅하라고 해주셔서 앞으론 문제없이 작성할 수 있을 것 같습니다.  따로 file descriptor에 대해서 질문을 해주셔서 제가 기존에 많이 생각했던 부분이라, 0, 1, 2의 표준 입출력 및 오류 부분과, 3부터 새로 open한 파일들이 들어간다는 것을 알려드렸습니다. 특히 open에 실패할 경우 -1이 fd로 들어간다는 점을 알려드렸습니다. 기존에 잘 알고 계셔서 서로 같은 내용인지 잘 확인되었습니다. split 부분에서는 기존에 seongkim 님도 NULL 문자를 단어 끝마다 넣어주는 동작의 함수를 짜고 계셨다고 들었는데, 그렇게 하시다가 다른 방향으로 바꾸셨다 했습니다. 제가 선택한 방식인 strtok 방식에 대해서 이미 한번 알고 계셨기에 잘 이해해주시고 이 방식에 대해서 더 나은 방향성을 알려주셨습니다. 특히 free를 main에서 하게 된다면, 구분자가 없는 문자열의 가장 첫 부분을 free하게 되면 똑같이 str을 free한 것처럼 할 수 있어 이전 수정 버젼 (strlcpy 를 사용하지 않고, 주소 그대로 넣어주는 방식)을 사용할 수 있다는 가능성을 열어주셨습니다. 이런 방식으로도 다시 한번 해보도록 하겠습니다. 좋은 조언 감사드립니다!!",
        "final_mark": 125,
        "flag": {
          "id": 9,
          "name": "Outstanding project",
          "positive": true,
          "icon": "star-1",
          "created_at": "2017-05-18T14:07:37.380Z",
          "updated_at": "2017-05-18T14:12:07.415Z"
        },
        "begin_at": "2024-03-19T12:30:00.000Z",
        "correcteds": [
          {
            "id": 172307,
            "login": "doyoukim",
            "url": "https://api.intra.42.fr/v2/users/doyoukim"
          }
        ],
        "corrector": {
          "id": 172309,
          "login": "seongkim",
          "url": "https://api.intra.42.fr/v2/users/seongkim"
        },
        "truant": {},
        "filled_at": "2024-03-19T14:32:02.174Z",
        "questions_with_answers": []
      },
      {
        "id": 6532721,
        "scale_id": 32445,
        "comment": "안녕하세요, doyoukim님.이런 우연이 있나요! 본과정을 시작한 지 벌써 거의 3주가 지났지만 평가가 처음인 저의 첫 평가 대상이 피신의 마지막 평가자라니. 반갑습니다. 우선, norm 체크 했고, calloc, strdup, split 등의 메모리 leak을 확인하였습니다. tolower/toupper 에서 비트연산자를 사용해서 표현한 부분과 strnstr 에서 strncmp를 사용한 것이 좋았습니다. 모든 함수에 테스트코드가 주석으로 적혀있더라고요. 꼼꼼하게 확인하신 부분이 눈에 띄었습니다. substr의 start가 첫번째 매개변수보다 큰 경우 len 값에 0을 할당하는 부분 제일 마음에 들었습니다. 효율적인 코드라고 생각했어요. fd 를 다룬 코드에서는 fd가 음수인 경우 에러 처리 따로 해주시지 않았는데, 저는 개인적으로 처리해주는 것을 선호하는 편입니다. itoa에서 변수명을 minus로 작성한 부분은 약간 아쉬웠습니다. 변수명이 동작하는 방식과 맞지 않다는 생각이 들었습니다. 주소값을 넘겨서 포인터를 사용하는 것에 강하신 것 같아요. 함수 나누기의 장인! 보너스 문제 설명 감사드립니다. 제가 이해한 부분을 그려서 재설명 해드리고, 확인 받으면서 제대로 알 수 있었습니다. 1시간 30분에 달하는 평가 고생 많으셨고, 앞으로의 본과정도 잘 헤쳐나가시길 바랍니다. 파이팅 두유킴! :)",
        "created_at": "2024-03-19T08:07:39.842Z",
        "updated_at": "2024-03-19T14:32:07.228Z",
        "feedback": "안녕하세요 ahrelee님! 또 만나뵙게되어 영광입니다. tolower 부분과 toupper 부분을 비트 연산자로 한 부분을 설명해드렸는데, 이 연산 과정을 신기하게 생각하셔서 굉장히 뿌듯했습니다. 그리고 함수들의 예외처리 과정을 하나하나 다 확인해주셨는데, 굳이 안넣어도 되는 예외들도 있던 것 같아 많이 도움되었습니다. 그리고 substr 같은 경우 len을 0으로 하여 while문이 돌지않고 자연스럽게 함수가 진행되도록 한 부분을 인상깊게 봐주셔서 이러한 참신한 방법들을 더 연구해볼 생각입니다. 변수명 구분이 조금 안되는 부분(자릿수와 할당 해야하는 부분의 길이를 모두 minus라는 변수명으로 한 점)을 지적해주셨는데, 이는 제 생각에도 다른 사람이 보기에 이해가 낮아질 수 밖에 없는 부분인 것 같아 앞으로 변수명 선택에 주의해야할 것 같다고 생각했습니다. 감사합니다. 그리고 주소값 활용하는 부분이 돋보인다는 점을 언급해주셔서 함수를 더 추가하는 일이 있으면 이런 부분을 더 잘 활용해볼 예정입니다. ahrelee님이 linked list 부분을 구현중에 있으셔서 그림으로 그려서 조금더 설명드렸는데, 포인터의 포인터 개념을 잘 숙지하시고 바로 코드로 작성도 하셨습니다. 충분한 이해에 도움이 된 것 같아 만족스럽습니다! 고생하셨습니다.",
        "final_mark": 125,
        "flag": {
          "id": 1,
          "name": "Ok",
          "positive": true,
          "icon": "check-4",
          "created_at": "2015-09-14T23:06:52.000Z",
          "updated_at": "2015-09-14T23:06:52.000Z"
        },
        "begin_at": "2024-03-19T11:00:00.000Z",
        "correcteds": [
          {
            "id": 172307,
            "login": "doyoukim",
            "url": "https://api.intra.42.fr/v2/users/doyoukim"
          }
        ],
        "corrector": {
          "id": 172410,
          "login": "ahrelee",
          "url": "https://api.intra.42.fr/v2/users/ahrelee"
        },
        "truant": {},
        "filled_at": "2024-03-19T12:41:01.246Z",
        "questions_with_answers": []
      },
      {
        "id": 6532731,
        "scale_id": 32445,
        "comment": "doyoukim님 제가 모르는 연결리스트부분 상세하게 알려주려고 노력해주셨습니다. 제가 공부한 것과 조금 다른 표현이 있어 제가 따로 정리하겠습니다. 비트연산자 활용도 흥미롭게 봤습니다. 저도 따로 공부해보겠습니다. 평가시간이 조금 밀리고 오래 걸렸지만 유익한 시간이었습니다. 다만 스플릿함수는 가독성이 떨어져서 아쉬웠습니다. 수고 많으셨습니다.",
        "created_at": "2024-03-19T08:11:11.826Z",
        "updated_at": "2024-03-19T14:25:47.205Z",
        "feedback": "안녕하세요. bohchoi님. linked list 관련해서 많은 궁금증을 가지고 계셔서, 처음부터 그림을 그려서 설명을 드렸는데, head를 가리키는 lst 포인터 (더블 포인터로 가리킨 매개변수)를 정확히 이해하셨습니다. 저도 이해가 잘 안되는 부분이었는데 그림을 그리다보니 다시 상기되어 쉽게 설명드릴 수 있었던 것 같습니다. bohchoi님이 다음에 linked list 부분을 구현하실 때 꼭 그림을 그려서 잘 참고하셨으면 좋겠습니다. str함수들 관련해서는 기존에 정리하셨던 return value들과 비교하여 정확히 일치하는지 잘 확인해주셨습니다. memory 관련 함수들에서는 unsigned char로 하는 것에 대해 의문을 가지신 부분을 확장 아스키코드로 생각해서 설명드렸습니다. 그리고 함수 자료형을 아무것도 안하면 자동으로 int가 되는 부분을 알려주셨는데 정말 신기했습니다. 이런 내용까지 알고 계셔서 많은 의문점을 해결했습니다! 감사합니다.",
        "final_mark": 125,
        "flag": {
          "id": 1,
          "name": "Ok",
          "positive": true,
          "icon": "check-4",
          "created_at": "2015-09-14T23:06:52.000Z",
          "updated_at": "2015-09-14T23:06:52.000Z"
        },
        "begin_at": "2024-03-19T12:00:00.000Z",
        "correcteds": [
          {
            "id": 172307,
            "login": "doyoukim",
            "url": "https://api.intra.42.fr/v2/users/doyoukim"
          }
        ],
        "corrector": {
          "id": 172305,
          "login": "bohchoi",
          "url": "https://api.intra.42.fr/v2/users/bohchoi"
        },
        "truant": {},
        "filled_at": "2024-03-19T13:43:56.922Z",
        "questions_with_answers": []
      }
    ],
    "teams_uploads": [
      {
        "id": 2134071,
        "final_mark": 125,
        "comment": "initial_errors:  | test_ft_isalpha: OK | test_ft_isdigit: OK | test_ft_isalnum: OK | test_ft_isascii: OK | test_ft_isprint: OK | test_ft_strlen: OK | test_ft_memset: OK | test_ft_bzero: OK | test_ft_memcpy: OK | test_ft_memmove: OK | test_ft_strlcpy: OK | test_ft_strlcat: OK | test_ft_toupper: OK | test_ft_tolower: OK | test_ft_strchr: OK | test_ft_strrchr: OK | test_ft_strncmp: OK | test_ft_memchr: OK | test_ft_memcmp: OK | test_ft_strnstr: OK | test_ft_atoi: OK | test_ft_calloc: OK | test_ft_strdup: OK | test_ft_substr: OK | test_ft_strjoin: OK | test_ft_strtrim: OK | test_ft_split: OK | test_ft_itoa: OK | test_ft_strmapi: OK | test_ft_striteri: OK | test_ft_putchar_fd: OK | test_ft_putstr_fd: OK | test_ft_putendl_fd: OK | test_ft_putnbr_fd: OK | bonus: 9/9 functions correct",
        "created_at": "2024-03-19T14:43:55.620Z",
        "upload_id": 1558
      }
    ]
  },
  {
    "id": 5533773,
    "name": "yeonlee's group",
    "url": "https://api.intra.42.fr/v2/teams/5533773",
    "final_mark": 0,
    "project_id": 1314,
    "created_at": "2024-02-26T01:11:52.157Z",
    "updated_at": "2024-03-13T18:47:53.727Z",
    "status": "finished",
    "terminating_at": null,
    "users": [
      {
        "id": 174092,
        "login": "yeonlee",
        "url": "https://api.intra.42.fr/v2/users/yeonlee",
        "leader": true,
        "occurrence": 0,
        "validated": true,
        "projects_user_id": 3565891
      }
    ],
    "locked?": true,
    "validated?": false,
    "closed?": true,
    "repo_url": "git@vogsphere.42gyeongsan.kr:vogsphere/intra-uuid-e43fe3a8-e5e6-41f0-9edb-d7767096e8ef-5533773-yeonlee",
    "repo_uuid": "intra-uuid-e43fe3a8-e5e6-41f0-9edb-d7767096e8ef-5533773-yeonlee",
    "locked_at": "2024-02-26T01:11:52.183Z",
    "closed_at": "2024-03-13T16:49:14.923Z",
    "project_session_id": 11805,
    "project_gitlab_path": "pedago_world/42-cursus/inner-circle/libft",
    "scale_teams": [
      {
        "id": 6511749,
        "scale_id": 32445,
        "comment": "작성하신 코드들을 가지고 컴파일을 진행했을 때 원하는 대로 안되거나 컴파일이 되지않는 경우가 조금 있었습니다. 몇가지 질문을 했을 때 설명을 잘 해주셨습니다. 다음에는 몇 가지 오류들을 고쳐서 다음 과제로 넘어갈 수 있었으면 좋겠습니다.",
        "created_at": "2024-03-13T16:49:52.275Z",
        "updated_at": "2024-03-13T18:47:22.533Z",
        "feedback": "꼼꼼히 틀린 부분에 대해서 알려주셔서 다음 트라이때는 수정해서 도전할 수 있을 것 같습니다.",
        "final_mark": 0,
        "flag": {
          "id": 1,
          "name": "Ok",
          "positive": true,
          "icon": "check-4",
          "created_at": "2015-09-14T23:06:52.000Z",
          "updated_at": "2015-09-14T23:06:52.000Z"
        },
        "begin_at": "2024-03-13T18:00:00.000Z",
        "correcteds": [
          {
            "id": 174092,
            "login": "yeonlee",
            "url": "https://api.intra.42.fr/v2/users/yeonlee"
          }
        ],
        "corrector": {
          "id": 174160,
          "login": "yeojukim",
          "url": "https://api.intra.42.fr/v2/users/yeojukim"
        },
        "truant": {},
        "filled_at": "2024-03-13T18:44:44.700Z",
        "questions_with_answers": []
      },
      {
        "id": 6511747,
        "scale_id": 32445,
        "comment": null,
        "created_at": "2024-03-13T16:49:27.137Z",
        "updated_at": "2024-03-13T18:17:39.135Z",
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
        "begin_at": "2024-03-13T17:45:00.000Z",
        "correcteds": [
          {
            "id": 174092,
            "login": "yeonlee",
            "url": "https://api.intra.42.fr/v2/users/yeonlee"
          }
        ],
        "corrector": {
          "id": 174095,
          "login": "jisopark",
          "url": "https://api.intra.42.fr/v2/users/jisopark"
        },
        "truant": {
          "id": 174092,
          "login": "yeonlee",
          "url": "https://api.intra.42.fr/v2/users/yeonlee"
        },
        "filled_at": null,
        "questions_with_answers": []
      }
    ],
    "teams_uploads": []
  }
]

"#;

    // Deserialize JSON data to FtTeam struct
    serde_json::from_str::<Vec<FtTeam>>(data).unwrap();
}
