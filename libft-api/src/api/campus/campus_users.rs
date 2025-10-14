use rsb_derive::Builder;
use serde::{Deserialize, Serialize};

use crate::prelude::*;
use crate::to_param;
use libft_api_derive::HasVector;

#[derive(Debug, Serialize, Deserialize, Builder)]
pub struct FtApiCampusUsersRequest {
    pub user_id: Option<FtUserId>,
    pub sort: Option<Vec<FtSortOption>>,
    pub range: Option<Vec<FtRangeOption>>,
    pub filter: Option<Vec<FtFilterOption>>,
    pub page: Option<u16>,
    pub per_page: Option<u8>,
}

#[derive(Debug, Serialize, Deserialize, Builder, HasVector)]
#[serde(transparent)]
pub struct FtApiCampusUsersResponse {
    pub campus_users: Vec<FtCampusUser>,
}

impl<FCHC> FtClientSession<'_, FCHC>
where
    FCHC: FtClientHttpConnector + Send + Sync,
{
    /// Retrieves campus user associations from the 42 Intra API.
    ///
    /// This method fetches information about campus user associations, which link users to campuses.
    /// If a user_id is provided, it retrieves campus associations for that specific user.
    /// If no user_id is provided, it retrieves all campus user associations.
    ///
    /// # Parameters
    /// - `req`: A `FtApiCampusUsersRequest` struct containing the query parameters.
    ///
    /// # Query Parameters
    /// - `user_id`: Optional user ID to retrieve campus associations for a specific user
    /// - `sort`: Optional vector of sort options to order the results
    /// - `range`: Optional vector of range options to filter results by date ranges
    /// - `filter`: Optional vector of filter options to filter the results
    /// - `page`: Optional page number for pagination
    /// - `per_page`: Optional number of items per page for pagination
    ///
    /// # Returns
    /// - `ClientResult<FtApiCampusUsersResponse>`: Contains a vector of `FtCampusUser` objects
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
    ///     // Get all campus-user associations
    ///     let campus_users_response = session
    ///         .campus_users(FtApiCampusUsersRequest::new())
    ///         .await?;
    ///     println!("Found {} campus-user associations", campus_users_response.campus_users.len());
    ///
    ///     // Get campus associations for a specific user
    ///     let user_campus_assoc = session
    ///         .campus_users(
    ///             FtApiCampusUsersRequest::new()
    ///                 .with_user_id(FtUserId::new(12345))
    ///         )
    ///         .await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn campus_users(
        &self,
        req: FtApiCampusUsersRequest,
    ) -> ClientResult<FtApiCampusUsersResponse> {
        let url = match req.user_id {
            Some(user_id) => &format!("users/{user_id}/campus_users"),
            None => "campus_users",
        };

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
        let res = session.campus_users(FtApiCampusUsersRequest::new()).await;

        assert!(res.is_ok());
    }
}
