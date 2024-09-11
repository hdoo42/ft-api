use rsb_derive::Builder;
use serde::{Deserialize, Serialize};

use crate::{
    convert_filter_option_to_tuple, convert_range_option_to_tuple, ClientResult,
    FtClientHttpConnector, FtClientSession, FtCursusId, FtFilterOption, FtProjectId,
    FtProjectSessionId, FtRangeOption, FtSortOption, FtTeam, FtUserId,
};

#[derive(Debug, Serialize, Deserialize, Builder)]
pub struct FtApiUsersIdTeamsRequest {
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
pub struct FtApiUsersIdTeamsResponse {
    pub teams: Vec<FtTeam>,
}

impl<'a, FCHC> FtClientSession<'a, FCHC>
where
    FCHC: FtClientHttpConnector + Send + Sync,
{
    pub async fn users_id_teams(
        &self,
        req: FtApiUsersIdTeamsRequest,
    ) -> ClientResult<FtApiUsersIdTeamsResponse> {
        let url = &format!("users/{}/teams", req.user_id);

        let filters = convert_filter_option_to_tuple(req.filter.unwrap_or_default());
        let range = convert_range_option_to_tuple(req.range.unwrap_or_default());

        let params = vec![
            ("cursus_id", req.cursus_id.as_ref().map(|v| v.to_string())),
            ("project_id", req.project_id.as_ref().map(|v| v.to_string())),
            (
                "project_session_id",
                req.project_session_id.as_ref().map(|v| v.to_string()),
            ),
            ("page", req.page.as_ref().map(|v| v.to_string())),
            ("per_page", req.per_page.as_ref().map(|v| v.to_string())),
            (
                "sort",
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
    async fn user_id_teams_basic() {
        let token = FtApiToken::build(AuthInfo::build_from_env().unwrap())
            .await
            .unwrap();

        let client = FtClient::new(FtClientReqwestConnector::with_connector(
            reqwest::Client::new(),
        ));

        let session = client.open_session(&token);
        let res = session
            .users_id_teams(FtApiUsersIdTeamsRequest::new(FtUserId::new(
                TEST_USER_YONDOO06_ID,
            )))
            .await;

        assert!(res.is_ok());
    }
}
