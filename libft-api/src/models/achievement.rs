use rvstruct::ValueStruct;
use serde::{Deserialize, Serialize};

use super::FtUrl;

//
// FtAchievement and its field structs
//

// Assuming FtAchievementId is not defined elsewhere:
#[derive(Debug, Eq, Hash, PartialEq, PartialOrd, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtAchievementId(pub u64);

#[derive(Debug, Eq, Hash, PartialEq, PartialOrd, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtAchievementName(pub String);

#[derive(Debug, Eq, Hash, PartialEq, PartialOrd, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtAchievementDescription(pub String);

#[derive(Debug, Eq, Hash, PartialEq, PartialOrd, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtAchievementTier(pub String);

#[derive(Debug, Eq, Hash, PartialEq, PartialOrd, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtAchievementKind(pub String);

#[derive(Debug, Eq, Hash, PartialEq, PartialOrd, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtAchievementImage(pub String);

#[derive(Debug, Eq, Hash, PartialEq, PartialOrd, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtAchievementNbrOfSuccess(pub u64);

#[derive(Debug, Eq, Hash, PartialEq, PartialOrd, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtAchievementUsersUrl(pub FtUrl);

#[derive(Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct FtAchievement {
    pub id: FtAchievementId,
    pub name: FtAchievementName,
    pub description: FtAchievementDescription,
    pub tier: FtAchievementTier,
    pub kind: FtAchievementKind,
    pub visible: bool,
    pub image: FtAchievementImage,
    pub nbr_of_success: Option<FtAchievementNbrOfSuccess>,
    pub users_url: FtAchievementUsersUrl,
}
