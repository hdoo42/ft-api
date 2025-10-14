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
    /// Retrieves a list of groups from the 42 Intra API.
    ///
    /// # Parameters
    /// - `req`: A `FtApiGroupsRequest` struct containing the query parameters.
    ///
    /// # Query Parameters
    /// - `user_id`: Optional user ID to filter groups associated with a specific user
    /// - `page`: Optional page number for pagination
    /// - `per_page`: Optional number of items per page for pagination
    ///
    /// # Returns
    /// - `ClientResult<FtApiGroupsResponse>`: Contains a vector of `FtGroup` objects
    ///
    /// # Example
    /// ```rust
    /// use libft_api::prelude::*;
    ///
    /// async fn example() -> ClientResult<()> {
    ///     let token = FtApiToken::try_get(AuthInfo::build_from_env()?).await?;
    ///     let client = FtClient::new(FtClientReqwestConnector::new());
    ///     let session = client.open_session(token);
    ///
    ///     // Get all groups with pagination
    ///     let groups = session
    ///         .groups(
    ///             FtApiGroupsRequest::new()
    ///                 .with_per_page(50)
    ///         )
    ///         .await?;
    ///     println!("Found {} groups", groups.groups.len());
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn groups(&self, req: FtApiGroupsRequest) -> ClientResult<FtApiGroupsResponse> {
        let url = "groups";

        let params = vec![to_param!(req, page), to_param!(req, per_page)];

        self.http_session_api.http_get(url, &params).await
    }

    /// Creates a group-user association in the 42 Intra API.
    ///
    /// # Parameters
    /// - `req`: A `FtApiGroupsUsersPostRequest` struct containing the group-user association data.
    ///
    /// # Returns
    /// - `ClientResult<FtApiGroupsUsersPostResponse>`: Contains the created association details
    ///
    /// # Example
    /// ```rust
    /// use libft_api::prelude::*;
    ///
    /// async fn example() -> ClientResult<()> {
    ///     let token = FtApiToken::try_get(AuthInfo::build_from_env()?).await?;
    ///     let client = FtClient::new(FtClientReqwestConnector::new());
    ///     let session = client.open_session(token);
    ///
    ///     // Create a group-user association (requires appropriate permissions)
    ///     // let association_request = FtApiGroupsUsersPostRequest::new(
    ///     //     FtApiGroupsUsersPostBody {
    ///     //         group_id: FtGroupId::new(123),
    ///     //         user_id: FtUserId::new(456),
    ///     //     }
    ///     // );
    ///     // let result = session.groups_users_post(association_request).await?;
    ///
    ///     Ok(())
    /// }
    /// ```
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
