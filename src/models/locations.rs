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

#[cfg(test)]
mod tests {
    use crate::{config_env_var, AuthInfo, FtApiToken, FtClient, FtClientReqwestConnector};

    use super::*;
    #[tokio::test]
    async fn location_deserialize() {
        let info = AuthInfo::from_env(
            config_env_var("FT_API_CLIENT_UID").unwrap(),
            config_env_var("FT_API_CLIENT_SECRET").unwrap(),
        );
        let token = FtApiToken::build(info).await.unwrap();

        let client = FtClient::new(FtClientReqwestConnector::with_connector(
            reqwest::Client::new(),
        ));

        let session = client.open_session(&token);
        let res = session.campus_gs_locations().await;
        assert!(res.is_ok(), "{:?}", res);
    }
}
