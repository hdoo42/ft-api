use rsb_derive::Builder;
use serde::{Deserialize, Serialize};

use crate::prelude::*;
use crate::to_param;
use libft_api_derive::HasVector;

#[derive(Debug, Serialize, Deserialize, Builder)]
pub struct FtApiGroupsRequest {
    pub user_id: Option<FtUserId>,
    pub page: Option<u16>,
    pub per_page: Option<u8>,
}

#[derive(Debug, Serialize, Deserialize, Builder)]
pub struct FtApiGroupsUsersPostRequest {
    pub groups_user: FtApiGroupsUsersPostBody,
}

#[derive(Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct FtApiGroupsUsersPostBody {
    pub group_id: FtGroupId,
    pub user_id: FtUserId,
}

#[derive(Debug, Serialize, Deserialize, Builder)]
pub struct FtApiGroupsUsersPostResponse {
    pub id: i32,
    pub user_id: FtUserId,
    pub group: FtGroup,
}

#[derive(Debug, Serialize, Deserialize, Builder, HasVector)]
#[serde(transparent)]
pub struct FtApiGroupsResponse {
    pub groups: Vec<FtGroup>,
}

impl<FCHC> FtClientSession<'_, FCHC>
where
    FCHC: FtClientHttpConnector + Send + Sync,
{
    pub async fn groups(&self, req: FtApiGroupsRequest) -> ClientResult<FtApiGroupsResponse> {
        let url = "groups";

        let params = vec![to_param!(req, page), to_param!(req, per_page)];

        self.http_session_api.http_get(url, &params).await
    }

    pub async fn groups_users_post(
        &self,
        req: FtApiGroupsUsersPostRequest,
    ) -> ClientResult<FtApiGroupsUsersPostResponse> {
        let url = "groups_users";

        self.http_session_api.http_post(url, &req).await
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    use super::*;

    // #[tokio::test]
    // async fn post_groups() {
    //     let token = FtApiToken::try_get(AuthInfo::build_from_env().unwrap())
    //         .await
    //         .unwrap();
    //
    //     let client = FtClient::new(FtClientReqwestConnector::with_connector(
    //         reqwest::Client::new(),
    //     ));
    //
    //     let session = client.open_session(token);
    //
    //     let res = session
    //         .groups_users_post(FtApiGroupsUsersPostRequest::new(FtApiGroupsUsersPostBody {
    //             group_id: FtGroupId::new(FT_GROUP_ID_TEST_ACCOUNT),
    //             user_id: FtUserId::new(212_750),
    //         }))
    //         .await
    //         .unwrap();
    //
    //     assert_eq!(res.group.id, FtGroupId::new(FT_GROUP_ID_TEST_ACCOUNT));
    // }

    #[tokio::test]
    async fn get_groups() {
        let token = FtApiToken::try_get(AuthInfo::build_from_env().unwrap())
            .await
            .unwrap();

        let client = FtClient::new(FtClientReqwestConnector::with_connector(
            reqwest::Client::new(),
        ));

        let session = client.open_session(token);

        session.groups(FtApiGroupsRequest::new()).await.unwrap();
    }
}
