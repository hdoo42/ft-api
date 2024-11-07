use rsb_derive::Builder;
use serde::{Deserialize, Serialize};

use crate::{
    convert_filter_option_to_tuple, convert_range_option_to_tuple, to_param, ClientResult,
    FtClientHttpConnector, FtClientSession, FtCursusId, FtFilterOption, FtProjectId, FtRangeOption,
    FtSortOption, FtTeam,
};

#[derive(Debug, Serialize, Deserialize, Builder)]
pub struct FtApiProjectsIdTeamsRequest {
    pub project_id: FtProjectId,
    pub cursus_id: Option<FtCursusId>,
    pub sort: Option<Vec<FtSortOption>>,
    pub range: Option<Vec<FtRangeOption>>,
    pub filter: Option<Vec<FtFilterOption>>,
    pub page: Option<u16>,
    pub per_page: Option<u8>,
}

#[derive(Debug, Serialize, Deserialize, Builder)]
#[serde(transparent)]
pub struct FtApiProjectsIdTeamsResponse {
    pub teams: Vec<FtTeam>,
}

impl<'a, FCHC> FtClientSession<'a, FCHC>
where
    FCHC: FtClientHttpConnector + Send + Sync,
{
    pub async fn projects_id_teams(
        &self,
        req: FtApiProjectsIdTeamsRequest,
    ) -> ClientResult<FtApiProjectsIdTeamsResponse> {
        let url = format!("projects/{}/teams", req.project_id);

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
            .http_get(&url, &[filters, range, params].concat())
            .await
    }
}

#[cfg(test)]
mod tests {

    use crate::*;

    use super::*;

    #[tokio::test]
    async fn projects_id_teams_basic_test() {
        let token = FtApiToken::build(AuthInfo::build_from_env().unwrap())
            .await
            .unwrap();

        let client = FtClient::new(FtClientReqwestConnector::with_connector(
            reqwest::Client::new(),
        ));

        let session = client.open_session(&token);
        let res = session
            .projects_id_teams(FtApiProjectsIdTeamsRequest::new(FtProjectId::new(1314)))
            .await;

        assert!(res.is_ok());
    }
}
