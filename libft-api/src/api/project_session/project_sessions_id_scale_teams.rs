use rsb_derive::Builder;
use serde::{Deserialize, Serialize};

use crate::{
    ClientResult, FtClientHttpConnector, FtClientSession, FtProjectSessionId, FtScaleTeam,
};

#[derive(Debug, Serialize, Deserialize, Builder)]
#[serde(transparent)]
pub struct FtApiProjectSessionsScaleTeamsResponse {
    pub scale_teams: Vec<FtScaleTeam>,
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Serialize, Deserialize, Builder)]
pub struct FtApiProjectSessionsScaleTeamsRequest {
    pub project_session_id: FtProjectSessionId,
}

impl<'a, FCHC> FtClientSession<'a, FCHC>
where
    FCHC: FtClientHttpConnector + Send + Sync,
{
    pub async fn project_sessions_scale_teams(
        &self,
        reqest: FtApiProjectSessionsScaleTeamsRequest,
    ) -> ClientResult<FtApiProjectSessionsScaleTeamsResponse> {
        let url = &format!("project_sessions/{}/scale_teams", reqest.project_session_id);

        self.http_session_api
            .http_get(url, &crate::common::FT_HTTP_EMPTY_GET_PARAMS.clone())
            .await
    }
}

#[cfg(test)]
mod tests {
    use ft_project_session_ids::ft_cursus::inner::LIBFT;

    use crate::prelude::*;

    #[tokio::test]
    async fn location_deserialize() {
        let token = FtApiToken::build(AuthInfo::build_from_env().unwrap())
            .await
            .unwrap();

        let client = FtClient::new(FtClientReqwestConnector::with_connector(
            reqwest::Client::new(),
        ));

        let req = FtApiProjectSessionsScaleTeamsRequest::new(FtProjectSessionId::new(LIBFT));

        let session = client.open_session(&token);
        let res = session.project_sessions_scale_teams(req).await;
        assert!(res.is_ok());
    }
}
