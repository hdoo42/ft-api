use rsb_derive::Builder;
use serde::{Deserialize, Serialize};

use crate::prelude::*;
use crate::to_param;
use libft_api_derive::HasVector;

#[derive(Debug, Serialize, Deserialize, Builder)]
pub struct FtApiUsersPostRequest {
    pub user: FtApiUserPostBody,
}

#[derive(Debug, Serialize, Deserialize, Builder)]
pub struct FtApiUserPostBody {
    pub email: String,
    pub campus_id: FtCampusId,
    pub first_name: String,
    pub last_name: String,
    pub login: String,
    pub password: String,
    pub pool_month: String,
    pub pool_year: i16,
    pub kind: FtKind,
}

#[derive(Debug, Serialize, Deserialize, Builder)]
pub struct FtApiUsersRequest {
    pub sort: Option<Vec<FtSortOption>>,
    pub range: Option<Vec<FtRangeOption>>,
    pub filter: Option<Vec<FtFilterOption>>,
    pub page: Option<usize>,
    pub per_page: Option<u8>,
}

#[derive(Debug, Serialize, Deserialize, Builder)]
#[serde(transparent)]
pub struct FtApiUserPostsResponse {
    pub user: FtUser,
}

#[derive(Debug, Serialize, Deserialize, Builder, HasVector)]
#[serde(transparent)]
pub struct FtApiUsersResponse {
    pub users: Vec<FtUser>,
}

impl<FCHC> FtClientSession<'_, FCHC>
where
    FCHC: FtClientHttpConnector + Send + Sync,
{
    /// Creates a new user in the 42 Intra API.
    ///
    /// This method creates a new user account with the provided details.
    ///
    /// # Parameters
    /// - `req`: A `FtApiUsersPostRequest` struct containing the user creation data.
    ///
    /// # Returns
    /// - `ClientResult<FtApiUserPostsResponse>`: Contains the created `FtUser` object
    ///
    /// # Example
    /// ```rust
    /// use libft_api::prelude::*;
    /// use crate::models::user::FtKind;
    ///
    /// async fn example() -> ClientResult<()> {
    ///     let token = FtApiToken::try_get(AuthInfo::build_from_env()?).await?;
    ///     let client = FtClient::new(FtClientReqwestConnector::new());
    ///     let session = client.open_session(token);
    ///
    ///     // Create a new user (requires appropriate permissions)
    ///     // let new_user_request = FtApiUsersPostRequest::new(
    ///     //     FtApiUserPostBody {
    ///     //         email: "newuser@example.com".to_string(),
    ///     //         campus_id: FtCampusId::new(1),
    ///     //         first_name: "First".to_string(),
    ///     //         last_name: "Last".to_string(),
    ///     //         login: "newuser".to_string(),
    ///     //         password: "securepassword".to_string(),
    ///     //         pool_month: "february".to_string(),
    ///     //         pool_year: 2024,
    ///     //         kind: FtKind::Student,
    ///     //     }
    ///     // );
    ///     // let new_user_response = session.users_post(new_user_request).await?;
    ///     // println!("Created user with ID: {:?}", new_user_response.user.id);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn users_post(
        &self,
        req: FtApiUsersPostRequest,
    ) -> ClientResult<FtApiUserPostsResponse> {
        let url = "users";

        self.http_session_api.http_post(url, &req).await
    }

    /// Retrieves a list of users from the 42 Intra API.
    ///
    /// This method fetches user information with various filtering and pagination options.
    ///
    /// # Parameters
    /// - `req`: A `FtApiUsersRequest` struct containing the query parameters.
    ///
    /// # Query Parameters
    /// - `sort`: Optional vector of sort options to order the results
    /// - `range`: Optional vector of range options to filter results by date ranges
    /// - `filter`: Optional vector of filter options to filter the results
    /// - `page`: Optional page number for pagination
    /// - `per_page`: Optional number of items per page for pagination
    ///
    /// # Returns
    /// - `ClientResult<FtApiUsersResponse>`: Contains a vector of `FtUser` objects
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
    ///     // Get all users with pagination
    ///     let users_response = session
    ///         .users(
    ///             FtApiUsersRequest::new()
    ///                 .with_per_page(50)
    ///         )
    ///         .await?;
    ///     println!("Found {} users", users_response.users.len());
    ///
    ///     // Get users filtered by specific criteria
    ///     let filtered_users = session
    ///         .users(
    ///             FtApiUsersRequest::new()
    ///                 .with_filter(vec![
    ///                     FtFilterOption::new(
    ///                         FtFilterField::CampusId,
    ///                         vec!["1".to_string()] // Paris campus
    ///                     )
    ///                 ])
    ///         )
    ///         .await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn users(&self, req: FtApiUsersRequest) -> ClientResult<FtApiUsersResponse> {
        let url = "users";
        let filters = convert_filter_option_to_tuple(req.filter.unwrap_or_default()).unwrap();
        let range = convert_range_option_to_tuple(req.range.unwrap_or_default()).unwrap();

        let params = vec![
            to_param!(req, page),
            to_param!(req, per_page),
            (
                "sort".to_string(),
                req.sort.as_ref().map(|v| {
                    v.iter()
                        .map(|v| {
                            format!(
                                "{}{}",
                                if v.descending { "-" } else { "" },
                                serde_plain::to_string(&v.field).unwrap()
                            )
                        })
                        .collect::<Vec<_>>()
                        .join(",")
                }),
            ),
        ];

        self.http_session_api
            .http_get(url, &[filters, range, params].concat())
            .await
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[tokio::test]
    async fn basic() {
        let token = FtApiToken::try_get(AuthInfo::build_from_env().unwrap())
            .await
            .unwrap();

        let client = FtClient::new(FtClientReqwestConnector::with_connector(
            reqwest::Client::new(),
        ));

        let session = client.open_session(token);
        let res = session
            .users(FtApiUsersRequest::new().with_per_page(1))
            .await;

        assert!(res.is_ok());
    }
}
