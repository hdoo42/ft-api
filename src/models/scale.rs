use rvstruct::ValueStruct;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct FtScale {
    pub id: FtScaleId,
    pub evaluation_id: FtEvaluationId,
    pub name: FtScaleName,
    pub is_primary: bool,
    pub comment: FtComment,
    pub introduction_md: FtIntroductionMd,
    pub disclaimer_md: FtDisclaimerMd,
    pub guidelines_md: FtGuidelinesMd,
    pub created_at: DateTimeSerde,
    pub correction_number: FtCorrectionNumber,
    pub duration: FtDuration,
    pub manual_subscription: FtManualSubscription,
    pub free: bool,
}

#[derive(Debug, Eq, Hash, PartialEq, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtScaleId(i32);

#[derive(Debug, Eq, Hash, PartialEq, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtEvaluationId(i32);

#[derive(Debug, Eq, Hash, PartialEq, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtScaleName(pub String);

#[derive(Debug, Eq, Hash, PartialEq, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtComment(pub String);

#[derive(Debug, Eq, Hash, PartialEq, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtIntroductionMd(pub String);

#[derive(Debug, Eq, Hash, PartialEq, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtDisclaimerMd(pub String);

#[derive(Debug, Eq, Hash, PartialEq, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtGuidelinesMd(pub String);

#[derive(Debug, Eq, Hash, PartialEq, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtCorrectionNumber(i32);

#[derive(Debug, Eq, Hash, PartialEq, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtDuration(i32);

#[derive(Debug, Eq, Hash, PartialEq, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtManualSubscription(bool);

#[derive(Debug, Eq, Hash, PartialEq, Clone, Serialize, Deserialize)]
pub struct FtScaleFlag {
    pub name: String,
    pub positive: bool,
}
