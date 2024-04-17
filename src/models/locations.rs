use rvstruct::ValueStruct;
use serde::{Deserialize, Serialize};

use crate::models::{DateTimeSerde, FtCampusId, FtUser};

#[derive(Debug, Serialize, Deserialize)]
pub struct FtLocations {
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

#[cfg(test)]
mod tests {
    use util::config_env_var;

    use crate::{AccessToken, AuthInfo};

    use super::*;
    #[tokio::test]
    async fn location_deserialize() {
        let info = AuthInfo::from_env(
            config_env_var("FT_API_CLIENT_UID").unwrap(),
            config_env_var("FT_API_CLIENT_SECRET").unwrap(),
        );
        let token = AccessToken::build(info).await;
    }
}
