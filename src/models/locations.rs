use rvstruct::ValueStruct;
use serde::{Deserialize, Serialize};

use crate::models::{DateTimeSerde, FtCampusId, FtUser};

#[derive(Debug, Serialize, Deserialize)]
pub struct FtLocation {
    id: FtLocationId,
    begin_at: DateTimeSerde,
    end_at: Option<DateTimeSerde>,
    primary: bool,
    host: FtHost,
    campus_id: FtCampusId,
    user: FtUser,
}

#[derive(Debug, Eq, Hash, PartialEq, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtLocationId(i64);

#[derive(Debug, Eq, Hash, PartialEq, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtHost(pub String);
