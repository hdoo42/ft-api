use crate::prelude::*;
use crate::to_param;
use libft_api_derive::HasItems;
use rsb_derive::Builder;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Builder, HasItems)]
#[serde(transparent)]
pub struct FtApiProjectSessionsScaleTeamsResponse {
    pub scale_teams: Vec<FtScaleTeam>,
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Serialize, Deserialize, Builder)]
pub struct FtApiProjectSessionsScaleTeamsRequest {
    pub project_session_id: FtProjectSessionId,
    pub sort: Option<Vec<FtSortOption>>,
    pub range: Option<Vec<FtRangeOption>>,
    pub filter: Option<Vec<FtFilterOption>>,
    pub page: Option<u16>,
    pub per_page: Option<u8>,
}

impl<FCHC> FtClientSession<'_, FCHC>
where
    FCHC: FtClientHttpConnector + Send + Sync,
{
    pub async fn project_sessions_scale_teams(
        &self,
        request: FtApiProjectSessionsScaleTeamsRequest,
    ) -> ClientResult<FtApiProjectSessionsScaleTeamsResponse> {
        let url = &format!(
            "project_sessions/{}/scale_teams",
            request.project_session_id
        );

        let filters = convert_filter_option_to_tuple(request.filter.unwrap_or_default()).unwrap();
        let range = convert_range_option_to_tuple(request.range.unwrap_or_default()).unwrap();

        let params = vec![
            to_param!(request, page),
            to_param!(request, per_page),
            (
                "sort".to_string(),
                request.sort.as_ref().map(|v| {
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
    use ft_project_session_ids::ft_cursus::inner::LIBFT;

    use super::*;

    #[tokio::test]
    async fn basic() {
        let token = FtApiToken::try_get(AuthInfo::build_from_env().unwrap())
            .await
            .unwrap();

        let client = FtClient::new(FtClientReqwestConnector::with_connector(
            reqwest::Client::new(),
        ));

        let req = FtApiProjectSessionsScaleTeamsRequest::new(FtProjectSessionId::new(LIBFT))
            .with_per_page(1);

        let session = client.open_session(token);
        let res = session.project_sessions_scale_teams(req).await;
        assert!(res.is_ok());
    }
}
