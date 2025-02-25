use rvstruct::ValueStruct;
use serde::{Deserialize, Serialize};

use crate::FtDateTimeUtc;

use super::{FtCampusId, FtUserId};

#[derive(Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct FtCampusUser {
    pub id: FtCampusUserId,
    pub user_id: FtUserId,
    pub campus_id: FtCampusId,
    pub is_primary: bool,
    pub created_at: FtDateTimeUtc,
    pub updated_at: FtDateTimeUtc,
}

#[derive(Debug, Eq, Hash, PartialEq, PartialOrd, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtCampusUserId(pub i32);

#[test]
fn deser() {
    let raw_string = r#"
    [{"id":200517,"user_id":207415,"campus_id":68,"is_primary":true,"created_at":"2024-10-31T08:12:23.122Z","updated_at":"2024-10-31T08:12:23.122Z"},{"id":200516,"user_id":207414,"campus_id":68,"is_primary":true,"created_at":"2024-10-31T08:12:23.119Z","updated_at":"2024-10-31T08:12:23.119Z"},{"id":200515,"user_id":207413,"campus_id":68,"is_primary":true,"created_at":"2024-10-31T08:10:37.980Z","updated_at":"2024-10-31T08:10:37.980Z"},{"id":200514,"user_id":207412,"campus_id":68,"is_primary":true,"created_at":"2024-10-31T08:08:47.880Z","updated_at":"2024-10-31T08:08:47.880Z"},{"id":200513,"user_id":207411,"campus_id":68,"is_primary":true,"created_at":"2024-10-31T08:05:17.277Z","updated_at":"2024-10-31T08:05:17.277Z"},{"id":200512,"user_id":207410,"campus_id":68,"is_primary":true,"created_at":"2024-10-31T08:01:06.880Z","updated_at":"2024-10-31T08:01:06.880Z"},{"id":200511,"user_id":207409,"campus_id":68,"is_primary":true,"created_at":"2024-10-31T08:00:38.152Z","updated_at":"2024-10-31T08:00:38.152Z"},{"id":200510,"user_id":207408,"campus_id":68,"is_primary":true,"created_at":"2024-10-31T07:58:11.750Z","updated_at":"2024-10-31T07:58:11.750Z"},{"id":200509,"user_id":207407,"campus_id":68,"is_primary":true,"created_at":"2024-10-31T07:57:40.599Z","updated_at":"2024-10-31T07:57:40.599Z"},{"id":200508,"user_id":207406,"campus_id":68,"is_primary":true,"created_at":"2024-10-31T07:56:16.070Z","updated_at":"2024-10-31T07:56:16.070Z"},{"id":200507,"user_id":207405,"campus_id":68,"is_primary":true,"created_at":"2024-10-31T07:54:12.427Z","updated_at":"2024-10-31T07:54:12.427Z"},{"id":200506,"user_id":207404,"campus_id":68,"is_primary":true,"created_at":"2024-10-31T07:53:29.431Z","updated_at":"2024-10-31T07:53:29.431Z"},{"id":200505,"user_id":207403,"campus_id":68,"is_primary":true,"created_at":"2024-10-31T07:52:26.478Z","updated_at":"2024-10-31T07:52:26.478Z"},{"id":200504,"user_id":207402,"campus_id":68,"is_primary":true,"created_at":"2024-10-31T07:50:21.761Z","updated_at":"2024-10-31T07:50:21.761Z"},{"id":200503,"user_id":207401,"campus_id":65,"is_primary":true,"created_at":"2024-10-31T06:59:24.654Z","updated_at":"2024-10-31T06:59:24.654Z"},{"id":200496,"user_id":38766,"campus_id":75,"is_primary":true,"created_at":"2024-10-30T12:21:58.378Z","updated_at":"2024-10-30T12:22:06.153Z"},{"id":200495,"user_id":207394,"campus_id":65,"is_primary":true,"created_at":"2024-10-30T12:12:48.454Z","updated_at":"2024-10-30T12:12:48.454Z"},{"id":200494,"user_id":207393,"campus_id":65,"is_primary":true,"created_at":"2024-10-30T11:05:02.434Z","updated_at":"2024-10-30T11:05:02.434Z"},{"id":200491,"user_id":207391,"campus_id":65,"is_primary":true,"created_at":"2024-10-30T07:21:11.891Z","updated_at":"2024-10-30T07:21:11.891Z"},{"id":200490,"user_id":207390,"campus_id":65,"is_primary":true,"created_at":"2024-10-30T07:17:34.338Z","updated_at":"2024-10-30T07:17:34.338Z"},{"id":200489,"user_id":207389,"campus_id":65,"is_primary":true,"created_at":"2024-10-30T07:16:34.262Z","updated_at":"2024-10-30T07:16:34.262Z"},{"id":200488,"user_id":207388,"campus_id":65,"is_primary":true,"created_at":"2024-10-30T06:39:47.499Z","updated_at":"2024-10-30T06:39:47.499Z"},{"id":200484,"user_id":207384,"campus_id":65,"is_primary":true,"created_at":"2024-10-29T13:50:55.785Z","updated_at":"2024-10-29T13:50:55.785Z"},{"id":200483,"user_id":207383,"campus_id":65,"is_primary":true,"created_at":"2024-10-29T12:25:34.108Z","updated_at":"2024-10-29T12:25:34.108Z"},{"id":200482,"user_id":207382,"campus_id":65,"is_primary":true,"created_at":"2024-10-29T12:15:32.037Z","updated_at":"2024-10-29T12:15:32.037Z"},{"id":200481,"user_id":207381,"campus_id":65,"is_primary":true,"created_at":"2024-10-29T11:05:55.429Z","updated_at":"2024-10-29T11:05:55.429Z"},{"id":200480,"user_id":207380,"campus_id":65,"is_primary":true,"created_at":"2024-10-29T10:59:35.400Z","updated_at":"2024-10-29T10:59:35.400Z"},{"id":200478,"user_id":207378,"campus_id":65,"is_primary":true,"created_at":"2024-10-29T09:55:41.002Z","updated_at":"2024-10-29T09:55:41.002Z"},{"id":200477,"user_id":207377,"campus_id":65,"is_primary":true,"created_at":"2024-10-29T09:25:42.006Z","updated_at":"2024-10-29T09:25:42.006Z"},{"id":200476,"user_id":207376,"campus_id":65,"is_primary":true,"created_at":"2024-10-29T09:24:51.755Z","updated_at":"2024-10-29T09:24:51.755Z"}]
    "#;

    let _res: Vec<FtCampusUser> = serde_json::from_str(raw_string).unwrap();
}
