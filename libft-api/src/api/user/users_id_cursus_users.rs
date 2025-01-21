use rsb_derive::Builder;
use serde::{Deserialize, Serialize};

use crate::{prelude::*, to_param};

#[derive(Debug, Serialize, Deserialize, Builder)]
pub struct FtApiUsersIdCursusUsersRequest {
    pub user_id: FtUserId,
    pub sort: Option<Vec<FtSortOption>>,
    pub range: Option<Vec<FtRangeOption>>,
    pub filter: Option<Vec<FtFilterOption>>,
    pub page: Option<u16>,
    pub per_page: Option<u8>,
}

#[derive(Debug, Serialize, Deserialize, Builder)]
pub struct FtApiUsersIdCursusUsersPostRequest {
    pub cursus_users: FtApiCursusUsersBody,
}

#[derive(Debug, Serialize, Deserialize, Builder)]
pub struct FtApiCursusUsersBody {
    pub cursus_id: i8,
    pub user_id: FtUserId,
    pub begin_at: String,
    pub has_coalition: bool,
}

#[derive(Debug, Serialize, Deserialize, Builder)]
#[serde(transparent)]
pub struct FtApiUsersIdCursusUsersResponse {
    pub cursus_user: Vec<FtCursusUser>,
}

#[derive(Debug, Serialize, Deserialize, Builder)]
#[serde(transparent)]
pub struct FtApiUsersIdCursusUsersPostResponse {
    pub cursus_user: Vec<FtCursusUser>,
}

impl<FCHC> FtClientSession<'_, FCHC>
where
    FCHC: FtClientHttpConnector + Send + Sync,
{
    /// Retrieves the cursus users for a specific user.
    ///
    /// # Parameters
    ///
    /// - `req`: An `FtApiUsersIdCursusUsersRequest` object containing the parameters for the API call.
    ///     - `user_id`: `FtUserId`
    ///         The user ID.
    ///     - `sort` (optional): `Vec<FtSortOption>`
    ///         Must be one of: `id`, `cursus_id`, `user_id`, `created_at`, `updated_at`, `end_at`,
    ///         `begin_at`, `has_coalition`, `blackholed_at`, `level`.
    ///         The sort field. Sorted by `created_at` desc, `id` desc by default.
    ///     - `filter` (optional): `Vec<FtFilterOption>`
    ///         Must be one of: `id`, `cursus_id`, `user_id`, `created_at`, `updated_at`, `end_at`,
    ///         `begin_at`, `has_coalition`, `blackholed_at`, `level`, `active`, `campus_id`, `end`,
    ///         `future`, `blackholed`.
    ///         Filtering on one or more fields.
    ///     - `range` (optional): `Vec<FtRangeOption>`
    ///         Must be one of: `id`, `cursus_id`, `user_id`, `created_at`, `updated_at`, `end_at`,
    ///         `begin_at`, `has_coalition`, `blackholed_at`, `level`.
    ///         Select on a particular range.
    ///     - `page` (optional): `u16`
    ///         The current page number.
    ///     - `per_page` (optional): `u8`
    ///         The number of items per page, defaults to 30, maximum 100.
    ///
    /// # Returns
    ///
    /// Returns a `ClientResult<FtApiUsersIdCursusUsersResponse>`.
    ///
    /// # Errors
    ///
    /// This function will return an error if the HTTP request fails or the response cannot be parsed.
    ///
    /// # Panics
    ///
    /// This function will panic if:
    /// - Converting filter or range options to query parameters fails.
    /// - Serialization of sort fields fails.
    /// - The `user_id` cannot be formatted into the URL.
    ///
    /// # Example
    ///
    /// ```rust
    /// use libft_api::{prelude::*, TEST_USER_YONDOO06_ID};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let token = FtApiToken::build(AuthInfo::build_from_env().unwrap())
    ///         .await
    ///         .unwrap();
    ///
    ///     let client = FtClient::new(FtClientReqwestConnector::with_connector(
    ///         reqwest::Client::new(),
    ///     ));
    ///
    ///     let session = client.open_session(&token);
    ///
    ///     let req = FtApiUsersIdCursusUsersRequest::new(FtUserId::new(TEST_USER_YONDOO06_ID))
    ///         .with_page(1)
    ///         .with_per_page(30);
    ///
    ///     let response = session.users_id_cursus_users(req).await.unwrap();
    ///
    /// }
    /// ```
    ///
    /// # Panics
    ///
    /// This function will panic if the conversion of filter or range options fails, or if serialization of sort fields fails.
    pub async fn users_id_cursus_users(
        &self,
        req: FtApiUsersIdCursusUsersRequest,
    ) -> ClientResult<FtApiUsersIdCursusUsersResponse> {
        let url = &format!("users/{}/cursus_users", req.user_id);

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

    pub async fn cursus_users_post(
        &self,
        req: FtApiUsersIdCursusUsersRequest,
    ) -> ClientResult<FtApiUsersIdCursusUsersResponse> {
        let url = "cursus_users";

        self.http_session_api.http_post(url, &req).await
    }

    pub async fn users_id_cursus_users_post(
        &self,
        req: FtApiUsersIdCursusUsersRequest,
    ) -> ClientResult<FtApiUsersIdCursusUsersResponse> {
        let url = &format!("users/{}/cursus_users", req.user_id.clone());

        self.http_session_api.http_post(url, &req).await
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[tokio::test]
    async fn basic() {
        let token = FtApiToken::build(AuthInfo::build_from_env().unwrap())
            .await
            .unwrap();

        let client = FtClient::new(FtClientReqwestConnector::with_connector(
            reqwest::Client::new(),
        ));

        let session = client.open_session(&token);
        let res = session
            .users_id_cursus_users(FtApiUsersIdCursusUsersRequest::new(FtUserId::new(174083)))
            .await;

        assert!(res.is_ok(), "{:?}", res.unwrap());
    }
}
