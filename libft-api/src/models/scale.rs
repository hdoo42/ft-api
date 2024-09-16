use rvstruct::ValueStruct;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct FtScale {
    pub id: FtScaleId,
    pub correction_number: Option<FtCorrectionNumber>,
    pub is_primary: Option<bool>,
    pub evaluation_id: Option<FtEvaluationId>,
    pub name: Option<FtScaleName>,
    pub comment: Option<FtComment>,
    pub introduction_md: Option<FtIntroductionMd>,
    pub disclaimer_md: Option<FtDisclaimerMd>,
    pub guidelines_md: Option<FtGuidelinesMd>,
    pub created_at: Option<FtDateTimeUtc>,
    pub duration: Option<FtDuration>,
    pub manual_subscription: Option<FtManualSubscription>,
    pub free: Option<bool>,
    pub flags: Option<Vec<FtFlag>>,
    pub languages: Option<Vec<FtLanguage>>,
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
