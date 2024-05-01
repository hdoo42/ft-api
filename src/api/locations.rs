use rsb_derive::Builder;
use serde::{Deserialize, Serialize};

use crate::{ClientResult, FtClientHttpConnector, FtClientSession, FtLocation, GsInfo};

#[derive(Debug, Serialize, Deserialize, Builder)]
#[serde(transparent)]
pub struct FtApiCampusLocationsResponse {
    pub location: Vec<FtLocation>,
}

impl<'a, FCHC> FtClientSession<'a, FCHC>
where
    FCHC: FtClientHttpConnector + Send + Sync,
{
    pub async fn campus_gs_locations(&self) -> ClientResult<FtApiCampusLocationsResponse> {
        let url = &format!("campus/{}/locations", GsInfo::CAMPUS_ID);

        self.http_session_api
            .http_get(url, &crate::common::FT_HTTP_EMPTY_GET_PARAMS.clone())
            .await
    }
}

#[cfg(test)]
mod tests {
    use crate::{AuthInfo, FtApiToken, FtClient, FtClientReqwestConnector};

    #[tokio::test]
    async fn location_deserialize() {
        let token = FtApiToken::build(AuthInfo::build_from_env().unwrap())
            .await
            .unwrap();

        let client = FtClient::new(FtClientReqwestConnector::with_connector(
            reqwest::Client::new(),
        ));

        let session = client.open_session(&token);
        let res = session.campus_gs_locations().await;
        assert!(res.is_ok(), "{:?}", res);
    }
}
