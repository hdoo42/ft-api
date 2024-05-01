use rvstruct::ValueStruct;
use serde::{Deserialize, Serialize};

use crate::{scale::FtScale, *};

#[derive(Debug, Serialize, Deserialize)]
pub struct FtScaleTeam {
    pub id: FtScaleTeamId,
    pub scale_id: FtScaleId,
    pub comment: FtScaleTeamComment,
    pub created_at: DateTimeSerde,
    pub updated_at: DateTimeSerde,
    pub feedback: FtScaleTeamFeedback,
    pub flag: FtScaleFlag,
    pub begin_at: DateTimeSerde,
    pub corrector: FtUser,
    pub correcteds: FtUser,
    pub filled_at: DateTimeSerde,
    pub truant: FtUser,
    pub scale: FtScale,
    pub team: FtTeam,
    pub feedbacks: Vec<FtFeedBack>,
}

#[derive(Debug, Eq, Hash, PartialEq, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtScaleTeamId(i32);
#[derive(Debug, Eq, Hash, PartialEq, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtScaleTeamComment(pub String);
#[derive(Debug, Eq, Hash, PartialEq, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtScaleTeamFeedback(pub String);
