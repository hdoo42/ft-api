use rsb_derive::Builder;
use rvstruct::ValueStruct;
use serde::{Deserialize, Serialize};

use crate::{ClientResult, FtClientHttpConnector, FtClientSession, FtUser, FtUserId};

#[derive(Debug, Serialize, Deserialize, Builder)]
#[serde(transparent)]
pub struct FtApiUsersIdCorrectionPointsAddResponse {
    pub res: FtUser,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct FtApiUsersIdCorrectionPointsAddRequest {
    pub id: FtUserId,
    pub reason: FtCorrectionPointsReason,
    pub amount: FtCorrectionPointsAmount,
}

#[derive(Debug, Eq, Hash, PartialEq, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtCorrectionPointsReason(String);

#[derive(Debug, Eq, Hash, PartialEq, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtCorrectionPointsAmount(i32);
impl<'a, FCHC> FtClientSession<'a, FCHC>
where
    FCHC: FtClientHttpConnector + Send + Sync,
{
    pub async fn users_id_correction_points_add(
        &self,
        request: FtApiUsersIdCorrectionPointsAddRequest,
    ) -> ClientResult<FtApiUsersIdCorrectionPointsAddResponse> {
        let url = &format!("users/{}/correction_points/add", request.id);

        self.http_session_api.http_post(url, &request).await
    }
}

#[cfg(test)]
mod users_tests {
    use crate::{
        users::{
            FtApiUsersIdCorrectionPointsAddRequest, FtCorrectionPointsAmount,
            FtCorrectionPointsReason,
        },
        AuthInfo, FtApiToken, FtClient, FtClientReqwestConnector, FtUserId,
    };

    #[test]
    fn users_id_correction_points_add_request() {
        let req = FtApiUsersIdCorrectionPointsAddRequest {
            id: FtUserId::new(crate::info::TEST_USER_YONDOO06_ID),
            reason: FtCorrectionPointsReason::new("test".to_owned()),
            amount: FtCorrectionPointsAmount::new(42),
        };

        let raw = r#"{"id":185472,"reason":"test","amount":42}"#;

        assert_eq!(raw, serde_json::to_string(&req).unwrap())
    }

    #[tokio::test]
    async fn users_id_correction_points_add_test() {
        let token = FtApiToken::build(AuthInfo::build_from_env().unwrap())
            .await
            .unwrap();

        let client = FtClient::new(FtClientReqwestConnector::with_connector(
            reqwest::Client::new(),
        ));

        let session = client.open_session(&token);
        let res = session
            .users_id_correction_points_add(FtApiUsersIdCorrectionPointsAddRequest {
                id: FtUserId::new(185472),
                reason: FtCorrectionPointsReason::new("test".to_owned()),
                amount: FtCorrectionPointsAmount::new(42),
            })
            .await;

        assert!(res.is_ok());
    }
}
