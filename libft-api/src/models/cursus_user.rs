use crate::prelude::*;
use rvstruct::ValueStruct;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct FtCursusUser {
    pub id: FtCursusUserId,
    pub grade: Option<FtGrade>,
    pub begin_at: Option<FtDateTimeUtc>,
    pub blackholed_at: Option<FtDateTimeUtc>,
    pub created_at: FtDateTimeUtc,
    pub cursus: FtCursus,
    pub cursus_id: FtCursusId,
    pub end_at: Option<FtDateTimeUtc>,
    pub has_coalition: bool,
    pub level: FtCursusUserLevel,
    pub skills: Vec<FtSkill>,
    pub updated_at: Option<FtDateTimeUtc>,
    pub user: FtUser,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FtSkill {
    pub id: FtSkillId,
    pub name: FtSkillName,
    pub level: FtSkillLevel,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtCursusUserLevel(pub f64);

#[derive(Debug, Eq, Hash, PartialEq, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtGrade(String);

#[derive(Debug, Eq, Hash, PartialEq, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtCursusUserId(i32);

#[derive(Debug, Eq, Hash, PartialEq, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtSkillId(i32);

#[derive(Debug, Eq, Hash, PartialEq, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtSkillName(String);

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtSkillLevel(f64);

#[test]
fn parse_to_struct() {
    let raw = r#"[
	{
		"grade": null,
		"level": 0.0,
		"skills": [],
		"blackholed_at": null,
		"id": 257957,
		"begin_at": "2024-02-19T01:00:00.000Z",
		"end_at": "2024-02-23T14:42:00.000Z",
		"cursus_id": 66,
		"has_coalition": false,
		"created_at": "2024-02-16T10:08:27.641Z",
		"updated_at": "2024-02-16T10:08:27.641Z",
		"user": {
			"id": 174083,
			"email": "taejikim@student.42gyeongsan.kr",
			"login": "taejikim",
			"first_name": "Taejin",
			"last_name": "Kim",
			"usual_full_name": "Taejin Kim",
			"usual_first_name": null,
			"url": "https://api.intra.42.fr/v2/users/taejikim",
			"phone": "hidden",
			"displayname": "Taejin Kim",
			"kind": "student",
			"image": {
				"link": "https://cdn.intra.42.fr/users/6e1e1701b6021d964fe901c33baf9d3a/taejikim.jpg",
				"versions": {
					"large": "https://cdn.intra.42.fr/users/f8bdcd63a54f2ac23db80bdc1b53d5c2/large_taejikim.jpg",
					"medium": "https://cdn.intra.42.fr/users/f755b7e79f4aa3a40f77a721265bb785/medium_taejikim.jpg",
					"small": "https://cdn.intra.42.fr/users/80bf2f21270e52de0b0a50a531376804/small_taejikim.jpg",
					"micro": "https://cdn.intra.42.fr/users/1201f77cd351ffead2c158fc62fa8b82/micro_taejikim.jpg"
				}
			},
			"staff?": false,
			"correction_point": 24,
			"pool_month": "january",
			"pool_year": "2024",
			"location": null,
			"wallet": 24,
			"anonymize_date": "2027-11-06T00:00:00.000+09:00",
			"data_erasure_date": "2027-11-06T00:00:00.000+09:00",
			"created_at": "2024-01-10T04:04:38.895Z",
			"updated_at": "2024-11-06T12:26:03.805Z",
			"alumnized_at": null,
			"alumni?": false,
			"active?": true
		},
		"cursus": {
			"id": 66,
			"created_at": "2023-03-08T08:57:17.181Z",
			"name": "C-Piscine-Reloaded",
			"slug": "c-piscine-reloaded",
			"kind": "external"
		}
	},
	{
		"grade": "Learner",
		"level": 7.91,
		"skills": [
			{
				"id": 10,
				"name": "Network \u0026 system administration",
				"level": 6.89
			},
			{
				"id": 3,
				"name": "Rigor",
				"level": 6.63
			},
			{
				"id": 17,
				"name": "Object-oriented programming",
				"level": 5.45
			},
			{
				"id": 2,
				"name": "Imperative programming",
				"level": 4.88
			},
			{
				"id": 4,
				"name": "Unix",
				"level": 4.29
			},
			{
				"id": 1,
				"name": "Algorithms \u0026 AI",
				"level": 4.1
			},
			{
				"id": 5,
				"name": "Graphics",
				"level": 3.13
			}
		],
		"blackholed_at": "2025-12-09T00:42:00.000Z",
		"id": 257459,
		"begin_at": "2024-02-26T00:42:00.000Z",
		"end_at": null,
		"cursus_id": 21,
		"has_coalition": true,
		"created_at": "2024-02-13T07:05:25.794Z",
		"updated_at": "2024-02-13T07:05:25.794Z",
		"user": {
			"id": 174083,
			"email": "taejikim@student.42gyeongsan.kr",
			"login": "taejikim",
			"first_name": "Taejin",
			"last_name": "Kim",
			"usual_full_name": "Taejin Kim",
			"usual_first_name": null,
			"url": "https://api.intra.42.fr/v2/users/taejikim",
			"phone": "hidden",
			"displayname": "Taejin Kim",
			"kind": "student",
			"image": {
				"link": "https://cdn.intra.42.fr/users/6e1e1701b6021d964fe901c33baf9d3a/taejikim.jpg",
				"versions": {
					"large": "https://cdn.intra.42.fr/users/f8bdcd63a54f2ac23db80bdc1b53d5c2/large_taejikim.jpg",
					"medium": "https://cdn.intra.42.fr/users/f755b7e79f4aa3a40f77a721265bb785/medium_taejikim.jpg",
					"small": "https://cdn.intra.42.fr/users/80bf2f21270e52de0b0a50a531376804/small_taejikim.jpg",
					"micro": "https://cdn.intra.42.fr/users/1201f77cd351ffead2c158fc62fa8b82/micro_taejikim.jpg"
				}
			},
			"staff?": false,
			"correction_point": 24,
			"pool_month": "january",
			"pool_year": "2024",
			"location": null,
			"wallet": 24,
			"anonymize_date": "2027-11-06T00:00:00.000+09:00",
			"data_erasure_date": "2027-11-06T00:00:00.000+09:00",
			"created_at": "2024-01-10T04:04:38.895Z",
			"updated_at": "2024-11-06T12:26:03.805Z",
			"alumnized_at": null,
			"alumni?": false,
			"active?": true
		},
		"cursus": {
			"id": 21,
			"created_at": "2019-07-29T08:45:17.896Z",
			"name": "42cursus",
			"slug": "42cursus",
			"kind": "main"
		}
	},
	{
		"grade": null,
		"level": 10.54,
		"skills": [
			{
				"id": 4,
				"name": "Unix",
				"level": 12.37
			},
			{
				"id": 1,
				"name": "Algorithms \u0026 AI",
				"level": 8.22
			},
			{
				"id": 3,
				"name": "Rigor",
				"level": 7.23
			},
			{
				"id": 14,
				"name": "Adaptation \u0026 creativity",
				"level": 6.11
			},
			{
				"id": 7,
				"name": "Group \u0026 interpersonal",
				"level": 0.79
			}
		],
		"blackholed_at": null,
		"id": 251025,
		"begin_at": "2024-01-11T00:42:00.000Z",
		"end_at": "2024-02-05T09:00:00.000Z",
		"cursus_id": 9,
		"has_coalition": true,
		"created_at": "2024-01-10T04:04:40.872Z",
		"updated_at": "2024-01-10T04:04:40.872Z",
		"user": {
			"id": 174083,
			"email": "taejikim@student.42gyeongsan.kr",
			"login": "taejikim",
			"first_name": "Taejin",
			"last_name": "Kim",
			"usual_full_name": "Taejin Kim",
			"usual_first_name": null,
			"url": "https://api.intra.42.fr/v2/users/taejikim",
			"phone": "hidden",
			"displayname": "Taejin Kim",
			"kind": "student",
			"image": {
				"link": "https://cdn.intra.42.fr/users/6e1e1701b6021d964fe901c33baf9d3a/taejikim.jpg",
				"versions": {
					"large": "https://cdn.intra.42.fr/users/f8bdcd63a54f2ac23db80bdc1b53d5c2/large_taejikim.jpg",
					"medium": "https://cdn.intra.42.fr/users/f755b7e79f4aa3a40f77a721265bb785/medium_taejikim.jpg",
					"small": "https://cdn.intra.42.fr/users/80bf2f21270e52de0b0a50a531376804/small_taejikim.jpg",
					"micro": "https://cdn.intra.42.fr/users/1201f77cd351ffead2c158fc62fa8b82/micro_taejikim.jpg"
				}
			},
			"staff?": false,
			"correction_point": 24,
			"pool_month": "january",
			"pool_year": "2024",
			"location": null,
			"wallet": 24,
			"anonymize_date": "2027-11-06T00:00:00.000+09:00",
			"data_erasure_date": "2027-11-06T00:00:00.000+09:00",
			"created_at": "2024-01-10T04:04:38.895Z",
			"updated_at": "2024-11-06T12:26:03.805Z",
			"alumnized_at": null,
			"alumni?": false,
			"active?": true
		},
		"cursus": {
			"id": 9,
			"created_at": "2015-11-04T10:58:13.979Z",
			"name": "C Piscine",
			"slug": "c-piscine",
			"kind": "piscine"
		}
	}
]
    "#;

    let result: FtApiUsersIdCursusUsersResponse = serde_json::from_str(raw).unwrap();
}
