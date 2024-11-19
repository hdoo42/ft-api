use rsb_derive::Builder;
use serde::{Deserialize, Serialize};

use crate::{
    convert_filter_option_to_tuple, convert_range_option_to_tuple, to_param, ClientResult,
    FtClientHttpConnector, FtClientSession, FtFilterOption, FtProjectSessionId, FtRangeOption,
    FtSortOption, FtTeam,
};

#[derive(Debug, Serialize, Deserialize, Builder)]
#[serde(transparent)]
pub struct FtApiProjectSessionsTeamsResponse {
    pub teams: Vec<FtTeam>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct FtApiProjectSessionsTeamsRequest {
    pub project_session_id: FtProjectSessionId,
    pub sort: Option<Vec<FtSortOption>>,
    pub filter: Option<Vec<FtFilterOption>>,
    pub range: Option<Vec<FtRangeOption>>,
    pub page: Option<u16>,
    pub per_page: Option<u8>,
}

impl<'a, FCHC> FtClientSession<'a, FCHC>
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
    use super::*;
    use crate::{
        ft_project_session_ids::ft_cursus::inner::LIBFT, AuthInfo, FtApiToken, FtClient,
        FtClientReqwestConnector, FtFilterField, FtProjectSessionId,
    };

    #[tokio::test]
    async fn location_deserialize() {
        let token = FtApiToken::build(AuthInfo::build_from_env().unwrap())
            .await
            .unwrap();

        let client = FtClient::new(FtClientReqwestConnector::with_connector(
            reqwest::Client::new(),
        ));

        let reqest = FtApiProjectSessionsTeamsRequest::new(FtProjectSessionId::new(LIBFT));

        let session = client.open_session(&token);
        let result = session.project_sessions_id_teams(reqest).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn location_deserialize_with_filter() {
        let token = FtApiToken::build(AuthInfo::build_from_env().unwrap())
            .await
            .unwrap();

        let client = FtClient::new(FtClientReqwestConnector::with_connector(
            reqwest::Client::new(),
        ));

        let req = FtApiProjectSessionsTeamsRequest::new(FtProjectSessionId::new(LIBFT))
            .with_filter(vec![FtFilterOption::new(
                FtFilterField::Campus,
                vec!["69".to_owned()],
            )]);

        let session = client.open_session(&token);
        let res = session.project_sessions_id_teams(req).await;

        assert!(res.is_ok());
    }
}
