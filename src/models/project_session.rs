use rvstruct::ValueStruct;
use serde::{Deserialize, Serialize};

use crate::{FtDateTimeUtc, FtProjectId, FtScale};

#[derive(Debug, Serialize, Deserialize)]
pub struct FtProjectSession {
    pub id: FtProjectSessionId,
    pub objectives: Option<Vec<String>>,
    pub description: Option<String>,
    pub project_id: Option<FtProjectId>,
    pub created_at: Option<FtDateTimeUtc>,
    pub updated_at: Option<FtDateTimeUtc>,
    pub is_subscriptable: Option<bool>,
    pub scales: Option<Vec<FtScale>>,
    pub uploads: Option<Vec<FtUpload>>,
    pub team_behaviour: Option<String>,
    pub solo: Option<bool>,
    pub begin_at: Option<FtDateTimeUtc>,
    pub end_at: Option<FtDateTimeUtc>,
    pub estimate_time: Option<String>,
    pub difficulty: Option<i32>,
    pub duration_days: Option<i32>,
    pub terminating_after: Option<i32>,
    pub campus_id: Option<i32>,
    pub cursus_id: Option<i32>,
    pub max_people: Option<i32>,
    pub commit: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FtUpload {
    pub id: Option<FtUploadId>,
    pub filename: Option<FtFilename>,
    pub url: Option<FtUrl>,
    pub created_at: Option<FtCreatedAt>,
    pub updated_at: Option<FtUpdatedAt>,
    pub file_size: Option<FtFileSize>,
    pub mime_type: Option<FtMimeType>,
}

#[derive(Debug, Eq, Hash, PartialEq, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtUploadId(u32);

#[derive(Debug, Eq, Hash, PartialEq, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtFilename(String);

#[derive(Debug, Eq, Hash, PartialEq, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtUrl(String);

#[derive(Debug, Eq, Hash, PartialEq, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtCreatedAt(String);

#[derive(Debug, Eq, Hash, PartialEq, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtUpdatedAt(String);

#[derive(Debug, Eq, Hash, PartialEq, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtFileSize(u64);

#[derive(Debug, Eq, Hash, PartialEq, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtMimeType(String);

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtProjectSessionId(pub i16);

pub mod ft_project_session_ids {
    pub mod ft_cursus {
        pub mod inner {
            pub const FT_TRANSCENDENCE: i16 = 11835;
            pub const WEBSERV: i16 = 11837;
            pub const INCEPTION: i16 = 11848;
            pub const CPP_MODULE_05: i16 = 11843;
            pub const CPP_MODULE_06: i16 = 11844;
            pub const CPP_MODULE_07: i16 = 11845;
            pub const CPP_MODULE_08: i16 = 11846;
            pub const CPP_MODULE_09: i16 = 11847;
            pub const NETPRACTICE: i16 = 11851;
            pub const CUB3D: i16 = 11850;
            pub const MINIRT: i16 = 11849;
            pub const CPP_MODULE_00: i16 = 11838;
            pub const CPP_MODULE_01: i16 = 11839;
            pub const CPP_MODULE_02: i16 = 11840;
            pub const CPP_MODULE_03: i16 = 11841;
            pub const CPP_MODULE_04: i16 = 11842;
            pub const MINISHELL: i16 = 11852;
            pub const PHILOSOPHER: i16 = 11853;
            pub const PUSH_SWAP: i16 = 11854;
            pub const PIPEX: i16 = 11833;
            pub const MINITALK: i16 = 11834;
            pub const FDF: i16 = 11856;
            pub const FRACT_OL: i16 = 11855;
            pub const SO_LONG: i16 = 11857;
            pub const BORN2BEROOT: i16 = 11831;
            pub const FT_PRINTF: i16 = 11832;
            pub const GET_NEXT_LINE: i16 = 11830;
            pub const LIBFT: i16 = 11805;
        }
    }
    pub mod c_piscine {
        pub const C_PISCINE_C_13: i16 = 11290;
        pub const C_PISCINE_C_12: i16 = 11289;
        pub const C_PISCINE_C_11: i16 = 11288;
        pub const C_PISCINE_C_10: i16 = 11287;
        pub const C_PISCINE_C_09: i16 = 11286;
        pub const C_PISCINE_C_08: i16 = 11285;
        pub const C_PISCINE_C_07: i16 = 11284;
        pub const C_PISCINE_C_06: i16 = 11283;
        pub const C_PISCINE_C_05: i16 = 11282;
        pub const C_PISCINE_C_04: i16 = 11281;
        pub const C_PISCINE_C_03: i16 = 11280;
        pub const C_PISCINE_C_02: i16 = 11279;
        pub const C_PISCINE_C_01: i16 = 11278;
        pub const C_PISCINE_C_00: i16 = 11277;
        pub const C_PISCINE_SHELL_01: i16 = 11291;
        pub const C_PISCINE_SHELL_00: i16 = 11193;
        pub const C_PISCINE_RUSH_02: i16 = 11306;
        pub const C_PISCINE_RUSH_01: i16 = 11305;
        pub const C_PISCINE_RUSH_00: i16 = 11304;
        pub const C_PISCINE_BSQ: i16 = 11353;
    }
}
