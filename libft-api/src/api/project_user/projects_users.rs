use rsb_derive::Builder;
use serde::{Deserialize, Serialize};

use crate::{prelude::*, to_param};

#[derive(Debug, Serialize, Deserialize, Builder)]
pub struct FtApiProjectsUsersPostRequest {
    pub projects_user: FtApiProjectsUsersPostBody,
}

#[derive(Debug, Serialize, Deserialize, Builder)]
pub struct FtApiProjectsUsersPostBody {
    pub project_id: FtProjectId,
    pub user_id: FtUserId,
}

#[derive(Debug, Serialize, Deserialize, Builder)]
#[serde(transparent)]
pub struct FtApiProjectsUsersPostResponse {
    pub projects_user: FtProjectsUser,
}
#[derive(Debug, Serialize, Deserialize, Builder)]
pub struct FtApiProjectsUsersRequest {
    pub user_id: Option<Vec<FtUserId>>,
    pub project_id: Option<Vec<FtProjectId>>,
    pub sort: Option<Vec<FtSortOption>>,
    pub range: Option<Vec<FtRangeOption>>,
    pub filter: Option<Vec<FtFilterOption>>,
    pub page: Option<u16>,
    pub per_page: Option<u8>,
}

#[derive(Debug, Serialize, Deserialize, Builder)]
#[serde(transparent)]
pub struct FtApiProjectsUsersResponse {
    pub projects_users: Vec<FtProjectsUser>,
}

impl<'a, FCHC> FtClientSession<'a, FCHC>
where
    FCHC: FtClientHttpConnector + Send + Sync,
{
    pub async fn projects_uesrs_post(
        &self,
        req: FtApiProjectsUsersPostRequest,
    ) -> ClientResult<FtApiProjectsUsersPostResponse> {
        let url = "projects_users";

        self.http_session_api.http_post(url, &req).await
    }

    pub async fn projects_uesrs(
        &self,
        req: FtApiProjectsUsersRequest,
    ) -> ClientResult<FtApiProjectsUsersResponse> {
        let url = "projects_users";

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

    use crate::*;

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
        let project_ids = ALL_INNER_SUBJECTS_ID
            .into_iter()
            .map(|id| id.to_string())
            .collect();
        let res = session
            .projects_uesrs(FtApiProjectsUsersRequest::new().with_filter(vec![
                FtFilterOption::new(FtFilterField::UserId, vec!["174083".to_owned()]),
                FtFilterOption::new(FtFilterField::ProjectId, project_ids),
            ]))
            .await;

        assert!(res.is_ok());
    }
}
