use serde::{Deserialize, Serialize};
use std::option::Option;

use crate::api::prelude::*;

use super::{FtDateTimeUtc, FtScaleTeamId};

#[derive(Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct FtCorrectionPointHistory {
    pub id: FtCorrectionPointHistoryId,
    pub created_at: FtDateTimeUtc,
    pub reason: FtCorrectionPointsReason,
    pub scale_team_id: Option<FtScaleTeamId>,
    pub sum: FtCorrectionPointsAmount,
    pub total: FtCorrectionpointsTotal,
    pub updated_at: FtDateTimeUtc,
}

#[derive(Debug, Eq, Hash, PartialEq, PartialOrd, Clone, Serialize, Deserialize)]
pub struct FtCorrectionPointHistoryId(u64);

#[derive(Debug, Eq, Hash, PartialEq, PartialOrd, Clone, Serialize, Deserialize)]
pub struct FtCorrectionpointsTotal(i64);

impl std::fmt::Display for FtCorrectionPointHistoryId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::fmt::Display for FtCorrectionpointsTotal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
