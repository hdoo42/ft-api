use rsb_derive::Builder;
use rvstruct::ValueStruct;
use serde::{Deserialize, Serialize};

use crate::{ClientResult, FtClientHttpConnector, FtClientSession, FtUser, FtUserId};

#[derive(Debug, Serialize, Deserialize, Builder)]
#[serde(transparent)]
pub struct FtApiUsersIdCorrectionPointsAddResponse {
    pub res: FtUser,
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Serialize, Deserialize, Builder)]
pub struct FtApiUsersIdCorrectionPointsAddRequest {
    pub id: FtUserId,
    pub reason: FtCorrectionPointsReason,
    pub amount: FtCorrectionPointsAmount,
}

#[derive(Debug, Eq, Hash, PartialEq, PartialOrd, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtCorrectionPointsReason(String);

#[derive(Debug, Eq, Hash, PartialEq, PartialOrd, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtCorrectionPointsAmount(i32);
impl<FCHC> FtClientSession<'_, FCHC>
where
    FCHC: FtClientHttpConnector + Send + Sync,
{
    ///
    ///
    /// # Errors
    ///
    /// This function will return an error if
    pub async fn users_id_correction_points_add(
        &self,
        request: FtApiUsersIdCorrectionPointsAddRequest,
    ) -> ClientResult<FtApiUsersIdCorrectionPointsAddResponse> {
        let url = &format!("users/{}/correction_points/add", request.id);

        self.http_session_api.http_post(url, &request).await
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn correction_points_add_request_serde() {
        let req = FtApiUsersIdCorrectionPointsAddRequest {
            id: FtUserId::new(crate::info::TEST_USER_YONDOO_ID),
            reason: FtCorrectionPointsReason::new("test".to_owned()),
            amount: FtCorrectionPointsAmount::new(42),
        };

        let raw = r#"{"id":180844,"reason":"test","amount":42}"#;

        assert_eq!(raw, serde_json::to_string(&req).unwrap());
    }

    #[tokio::test]
    async fn correction_points_add_test() {
        let token = FtApiToken::build(AuthInfo::build_from_env().unwrap())
            .await
            .unwrap();

        let client = FtClient::new(FtClientReqwestConnector::with_connector(
            reqwest::Client::new(),
        ));

        let session = client.open_session(token);
        let res = session
            .users_id_correction_points_add(FtApiUsersIdCorrectionPointsAddRequest {
                id: FtUserId::new(crate::info::TEST_USER_YONDOO_ID),
                reason: FtCorrectionPointsReason::new("test".to_owned()),
                amount: FtCorrectionPointsAmount::new(42),
            })
            .await;

        assert!(res.is_ok());
    }
}
