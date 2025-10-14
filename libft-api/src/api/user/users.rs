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
    pub async fn users_post(
        &self,
        req: FtApiUsersPostRequest,
    ) -> ClientResult<FtApiUserPostsResponse> {
        let url = "users";

        self.http_session_api.http_post(url, &req).await
    }

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
        let res = session.users(FtApiUsersRequest::new()).await;

        assert!(res.is_ok());
    }
}
