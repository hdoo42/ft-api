use rsb_derive::Builder;
use serde::{Deserialize, Serialize};

use crate::prelude::*;
use crate::to_param;
use libft_api_derive::HasVector;

#[derive(Debug, Serialize, Deserialize, Builder, HasVector)]
#[serde(transparent)]
pub struct FtApiProjectSessionsTeamsResponse {
    pub teams: Vec<FtTeam>,
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Serialize, Deserialize, Builder)]
pub struct FtApiProjectSessionsTeamsRequest {
    pub project_session_id: FtProjectSessionId,
    pub sort: Option<Vec<FtSortOption>>,
    pub filter: Option<Vec<FtFilterOption>>,
    pub range: Option<Vec<FtRangeOption>>,
    pub page: Option<usize>,
    pub per_page: Option<u8>,
}

impl<FCHC> FtClientSession<'_, FCHC>
where
    FCHC: FtClientHttpConnector + Send + Sync,
{
    pub async fn project_sessions_id_teams(
        &self,
        req: FtApiProjectSessionsTeamsRequest,
    ) -> ClientResult<FtApiProjectSessionsTeamsResponse> {
        let url = &format!("project_sessions/{}/teams", req.project_session_id);

        let filters = convert_filter_option_to_tuple(req.filter.unwrap_or_default()).unwrap();
        let ranges = convert_range_option_to_tuple(req.range.unwrap_or_default()).unwrap();

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
            .http_get(url, &[filters, params, ranges].concat())
            .await
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::ft_project_session_ids::ft_cursus::inner::LIBFT;

    use super::*;

    #[tokio::test]
    async fn basic() {
        let token = FtApiToken::try_get(AuthInfo::build_from_env().unwrap())
            .await
            .unwrap();

        let client = FtClient::new(FtClientReqwestConnector::with_connector(
            reqwest::Client::new(),
        ));

        let reqest =
            FtApiProjectSessionsTeamsRequest::new(FtProjectSessionId::new(LIBFT)).with_per_page(1);

        let session = client.open_session(token);
        let result = session.project_sessions_id_teams(reqest).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn with_filter() {
        let token = FtApiToken::try_get(AuthInfo::build_from_env().unwrap())
            .await
            .unwrap();

        let client = FtClient::new(FtClientReqwestConnector::with_connector(
            reqwest::Client::new(),
        ));

        let session = client.open_session(token);
        let res = session
            .project_sessions_id_teams(
                FtApiProjectSessionsTeamsRequest::new(FtProjectSessionId::new(LIBFT))
                    .with_per_page(1)
                    .with_filter(vec![FtFilterOption::new(
                        FtFilterField::Campus,
                        vec!["69".to_owned()],
                    )]),
            )
            .await;

        assert!(res.is_ok());
    }
}
