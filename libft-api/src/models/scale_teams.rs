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
[
	{
		"id": 31,
		"name": "3b3-1953's group",
		"url": "https://api.intra.42.fr/v2/teams/31",
		"final_mark": 0,
		"project_id": 1,
		"created_at": "2014-11-03T09:25:33.867Z",
		"updated_at": "2019-12-23T14:43:40.579Z",
		"status": "finished",
		"terminating_at": null,
		"users": [
			{
				"id": 12626,
				"login": "3b3-12626",
				"url": "https://api.intra.42.fr/v2/users/3b3-12626",
				"leader": true,
				"occurrence": 0,
				"validated": true,
				"projects_user_id": 48
			}
		],
		"locked?": true,
		"validated?": false,
		"closed?": true,
		"repo_url": "git@vogsphere.42paris.fr:vogsphere/intra-uuid-8197f1df-a53c-4671-8e1c-09a821719f3a-31",
		"repo_uuid": "intra-uuid-8197f1df-a53c-4671-8e1c-09a821719f3a-31",
		"locked_at": "2014-11-03T09:25:33.905Z",
		"closed_at": "2019-02-18T13:37:42.184Z",
		"project_session_id": 276,
		"project_gitlab_path": null,
		"scale_teams": [],
		"teams_uploads": []
	},
	{
		"id": 32,
		"name": "emammadz's group",
		"url": "https://api.intra.42.fr/v2/teams/32",
		"final_mark": 66,
		"project_id": 1,
		"created_at": "2014-11-03T09:25:36.280Z",
		"updated_at": "2019-12-23T14:46:26.174Z",
		"status": "finished",
		"terminating_at": null,
		"users": [
			{
				"id": 12555,
				"login": "emammadz",
				"url": "https://api.intra.42.fr/v2/users/emammadz",
				"leader": true,
				"occurrence": 0,
				"validated": true,
				"projects_user_id": 49
			}
		],
		"locked?": true,
		"validated?": false,
		"closed?": true,
		"repo_url": "git@vogsphere.42paris.fr:vogsphere/intra-uuid-75bdcf17-a482-48de-b8df-a44cf3494e94-32",
		"repo_uuid": "intra-uuid-75bdcf17-a482-48de-b8df-a44cf3494e94-32",
		"locked_at": "2014-11-03T09:25:36.320Z",
		"closed_at": "2014-11-10T13:28:36.897Z",
		"project_session_id": 276,
		"project_gitlab_path": null,
		"scale_teams": [
			{
				"id": 3369,
				"scale_id": 1,
				"comment": "fonction partie obligatoire 1 correct, partie 2 fonction ne gere pas les NULL ",
				"created_at": "2014-12-01T17:57:54.869Z",
				"updated_at": "2018-03-20T16:49:23.525Z",
				"feedback": "tres bonne correction!",
				"final_mark": 50,
				"flag": {
					"id": 1,
					"name": "Ok",
					"positive": true,
					"icon": "check-4",
					"created_at": "2015-09-14T23:06:52.000Z",
					"updated_at": "2015-09-14T23:06:52.000Z"
				},
				"begin_at": "2014-12-02T18:00:00.000Z",
				"correcteds": [
					{
						"id": 12555,
						"login": "emammadz",
						"url": "https://api.intra.42.fr/v2/users/emammadz"
					}
				],
				"corrector": {
					"id": 12275,
					"login": "hdumas",
					"url": "https://api.intra.42.fr/v2/users/hdumas"
				},
				"truant": {},
				"filled_at": null,
				"questions_with_answers": []
			},
			{
				"id": 3368,
				"scale_id": 1,
				"comment": "correction s est deroule avec succes ",
				"created_at": "2014-12-01T17:58:42.516Z",
				"updated_at": "2018-03-20T16:49:23.525Z",
				"feedback": "Tres bonne correction!",
				"final_mark": 75,
				"flag": {
					"id": 1,
					"name": "Ok",
					"positive": true,
					"icon": "check-4",
					"created_at": "2015-09-14T23:06:52.000Z",
					"updated_at": "2015-09-14T23:06:52.000Z"
				},
				"begin_at": "2014-12-02T16:15:00.000Z",
				"correcteds": [
					{
						"id": 12555,
						"login": "emammadz",
						"url": "https://api.intra.42.fr/v2/users/emammadz"
					}
				],
				"corrector": {
					"id": 12421,
					"login": "dmimouni",
					"url": "https://api.intra.42.fr/v2/users/dmimouni"
				},
				"truant": {},
				"filled_at": null,
				"questions_with_answers": []
			},
			{
				"id": 3367,
				"scale_id": 1,
				"comment": "Bon travail dans l'ensemble. Juste une erreur de fd dans ft_putchar_fd.c",
				"created_at": "2014-12-01T17:58:24.868Z",
				"updated_at": "2018-03-20T16:49:23.525Z",
				"feedback": "Tres bon correcteur. Les corrections etaient lentes certes mais precise!",
				"final_mark": 75,
				"flag": {
					"id": 1,
					"name": "Ok",
					"positive": true,
					"icon": "check-4",
					"created_at": "2015-09-14T23:06:52.000Z",
					"updated_at": "2015-09-14T23:06:52.000Z"
				},
				"begin_at": "2014-12-02T13:00:00.000Z",
				"correcteds": [
					{
						"id": 12555,
						"login": "emammadz",
						"url": "https://api.intra.42.fr/v2/users/emammadz"
					}
				],
				"corrector": {
					"id": 10371,
					"login": "vlize",
					"url": "https://api.intra.42.fr/v2/users/vlize"
				},
				"truant": {},
				"filled_at": null,
				"questions_with_answers": []
			},
			{
				"id": 1042,
				"scale_id": 1,
				"comment": "Dommage ! :)",
				"created_at": "2014-11-18T13:31:07.505Z",
				"updated_at": "2018-03-20T16:49:23.525Z",
				"feedback": "bonne correction!",
				"final_mark": 30,
				"flag": {
					"id": 1,
					"name": "Ok",
					"positive": true,
					"icon": "check-4",
					"created_at": "2015-09-14T23:06:52.000Z",
					"updated_at": "2015-09-14T23:06:52.000Z"
				},
				"begin_at": "2014-11-20T15:30:00.000Z",
				"correcteds": [
					{
						"id": 12555,
						"login": "emammadz",
						"url": "https://api.intra.42.fr/v2/users/emammadz"
					}
				],
				"corrector": {
					"id": 11451,
					"login": "3b3-11451",
					"url": "https://api.intra.42.fr/v2/users/3b3-11451"
				},
				"truant": {},
				"filled_at": null,
				"questions_with_answers": []
			},
			{
				"id": 907,
				"scale_id": 1,
				"comment": "La correction c'est tres bien passee.",
				"created_at": "2014-11-17T18:27:49.224Z",
				"updated_at": "2018-03-20T16:49:23.525Z",
				"feedback": "tres bonne correction!",
				"final_mark": 100,
				"flag": {
					"id": 1,
					"name": "Ok",
					"positive": true,
					"icon": "check-4",
					"created_at": "2015-09-14T23:06:52.000Z",
					"updated_at": "2015-09-14T23:06:52.000Z"
				},
				"begin_at": "2014-11-21T15:00:00.000Z",
				"correcteds": [
					{
						"id": 12555,
						"login": "emammadz",
						"url": "https://api.intra.42.fr/v2/users/emammadz"
					}
				],
				"corrector": {
					"id": 11886,
					"login": "alelievr",
					"url": "https://api.intra.42.fr/v2/users/alelievr"
				},
				"truant": {},
				"filled_at": null,
				"questions_with_answers": []
			},
			{
				"id": 1040,
				"scale_id": 1,
				"comment": null,
				"created_at": "2014-11-18T13:30:25.993Z",
				"updated_at": "2018-03-20T16:49:23.525Z",
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
				"begin_at": "2014-11-19T13:00:00.000Z",
				"correcteds": [
					{
						"id": 12555,
						"login": "emammadz",
						"url": "https://api.intra.42.fr/v2/users/emammadz"
					}
				],
				"corrector": {
					"id": 12625,
					"login": "3b3-12625",
					"url": "https://api.intra.42.fr/v2/users/3b3-12625"
				},
				"truant": {
					"id": 12555,
					"login": "emammadz",
					"url": "https://api.intra.42.fr/v2/users/emammadz"
				},
				"filled_at": null,
				"questions_with_answers": []
			}
		],
		"teams_uploads": []
	}
]
         "#;
    let res: Result<Vec<FtTeam>, serde_json::Error> = serde_json::from_str(raw_scaleteam);
    assert!(res.is_ok(), "{:?}", res);
}
