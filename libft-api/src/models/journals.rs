use rvstruct::ValueStruct;
use serde::{Deserialize, Serialize};

use super::{FtCampusId, FtCursusId, FtDateTimeUtc, FtUserId};

#[derive(Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct FtJournal {
    pub id: FtJournalId,
    pub user_id: FtUserId,
    pub item_type: FtJournalItemType,
    pub item_id: FtJournalItemId,
    pub cursus_id: FtCursusId,
    pub campus_id: FtCampusId,
    pub reason: FtJournalReason,
    pub created_at: FtDateTimeUtc,
    pub updated_at: FtDateTimeUtc,
    pub event_at: FtDateTimeUtc,
    pub alumni: bool,
    pub closed: bool,
}

#[derive(Debug, Eq, Hash, PartialEq, PartialOrd, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtJournalReason(pub String);

#[derive(Debug, Eq, Hash, PartialEq, PartialOrd, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtJournalItemId(u32);

#[derive(Debug, Eq, Hash, PartialEq, PartialOrd, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtJournalItemType(pub String);

#[derive(Debug, Eq, Hash, PartialEq, PartialOrd, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtJournalId(u64);
