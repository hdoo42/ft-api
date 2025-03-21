use rsb_derive::Builder;
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::{
    convert_filter_option_to_tuple, convert_range_option_to_tuple, to_param, ClientResult,
    FtClientHttpConnector, FtClientSession, FtCursusId, FtFilterOption, FtProjectId,
    FtProjectSessionId, FtProjectsUser, FtRangeOption, FtSortOption, FtUserId,
};

#[derive(Debug, Serialize, Deserialize, Builder)]
pub struct FtApiUsersIdProjectsUsersRequest {
    pub cursus_id: Option<FtCursusId>,
    pub user_id: FtUserId,
    pub project_id: Option<FtProjectId>,
    pub project_session_id: Option<FtProjectSessionId>,
    pub sort: Option<Vec<FtSortOption>>,
    pub range: Option<Vec<FtRangeOption>>,
    pub filter: Option<Vec<FtFilterOption>>,
    pub page: Option<u16>,
    pub per_page: Option<u8>,
}

#[derive(Debug, Serialize, Deserialize, Builder)]
#[serde(transparent)]
pub struct FtApiUsersIdProjectsUsersResponse {
    pub projects_users: Vec<FtProjectsUser>,
}

impl<'a, FCHC> FtClientSession<'a, FCHC>
where
    FCHC: FtClientHttpConnector + Send + Sync,
{
    pub async fn users_id_projects_users(
        &self,
        req: FtApiUsersIdProjectsUsersRequest,
    ) -> ClientResult<FtApiUsersIdProjectsUsersResponse> {
        let url = &format!("users/{}/projects_users", req.user_id);
        info!(url = url);

        let filters = convert_filter_option_to_tuple(req.filter.unwrap_or_default()).unwrap();
        let range = convert_range_option_to_tuple(req.range.unwrap_or_default()).unwrap();

        let params = vec![
            to_param!(req, page),
            to_param!(req, per_page),
            to_param!(req, project_session_id),
            to_param!(req, project_id),
            to_param!(req, cursus_id),
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
    use crate::*;

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
            .users_id_projects_users(FtApiUsersIdProjectsUsersRequest::new(FtUserId::new(
                TEST_USER_YONDOO_ID,
            )))
            .await
            .unwrap();

        // assert!(res.is_ok());
    }
}
