use rsb_derive::Builder;
use serde::{Deserialize, Serialize};

use crate::{prelude::*, to_param};

#[derive(Debug, Serialize, Deserialize, Builder)]
pub struct FtApiCampusUsersRequest {
    pub user_id: Option<FtUserId>,
    pub sort: Option<Vec<FtSortOption>>,
    pub range: Option<Vec<FtRangeOption>>,
    pub filter: Option<Vec<FtFilterOption>>,
    pub page: Option<u16>,
    pub per_page: Option<u8>,
}

#[derive(Debug, Serialize, Deserialize, Builder)]
#[serde(transparent)]
pub struct FtApiCampusUsersResponse {
    pub campus_users: Vec<FtCampusUser>,
}

impl<FCHC> FtClientSession<'_, FCHC>
where
    FCHC: FtClientHttpConnector + Send + Sync,
{
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
        let token = FtApiToken::build(AuthInfo::build_from_env().unwrap())
            .await
            .unwrap();

        let client = FtClient::new(FtClientReqwestConnector::with_connector(
            reqwest::Client::new(),
        ));

        let session = client.open_session(&token);
        let res = session.campus_users(FtApiCampusUsersRequest::new()).await;

        assert!(res.is_ok());
    }
}
