use rvstruct::ValueStruct;
use serde::{Deserialize, Serialize};
use std::option::Option;

use crate::{
    users_id_correction_points_add::{FtCorrectionPointsAmount, FtCorrectionPointsReason},
    FtDateTimeUtc, FtScaleTeamId,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct FtCorrectionPointHistory {
    pub id: CorrectionPointHistoryId,
    pub created_at: FtDateTimeUtc,
    pub reason: FtCorrectionPointsReason,
    pub scale_team_id: Option<FtScaleTeamId>,
    pub sum: FtCorrectionPointsAmount,
    pub total: FtCorrectionpointsTotal,
    pub updated_at: FtDateTimeUtc,
}

#[derive(Debug, Eq, Hash, PartialEq, Clone, Serialize, Deserialize, ValueStruct)]
pub struct CorrectionPointHistoryId(u64);

#[derive(Debug, Eq, Hash, PartialEq, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtCorrectionpointsTotal(i64);
