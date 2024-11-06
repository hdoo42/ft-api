use rsb_derive::Builder;
use serde::{Deserialize, Serialize};

use crate::{
    convert_filter_option_to_tuple, convert_range_option_to_tuple, ClientResult,
    FtClientHttpConnector, FtClientSession, FtCursusId, FtFilterOption, FtLoginId, FtProjectId,
    FtProjectSessionId, FtRangeOption, FtSortOption, FtTeam, FtUser, FtUserId,
};

#[derive(Debug, Serialize, Deserialize, Builder)]
pub struct FtApiUsersIdRequest {
    pub login: FtLoginId,
    pub sort: Option<Vec<FtSortOption>>,
    pub range: Option<Vec<FtRangeOption>>,
    pub filter: Option<Vec<FtFilterOption>>,
    pub page: Option<u16>,
    pub per_page: Option<u8>,
}

#[derive(Debug, Serialize, Deserialize, Builder)]
#[serde(transparent)]
pub struct FtApiUsersIdResponse {
    pub user: FtUser,
}

impl<'a, FCHC> FtClientSession<'a, FCHC>
where
    FCHC: FtClientHttpConnector + Send + Sync,
{
    pub async fn users_id(&self, req: FtApiUsersIdRequest) -> ClientResult<FtApiUsersIdResponse> {
        let url = &format!("users/{}", req.login);

        let filters = convert_filter_option_to_tuple(req.filter.unwrap_or_default()).unwrap();
        let range = convert_range_option_to_tuple(req.range.unwrap_or_default()).unwrap();

        let params = vec![
            (
                "page".to_string(),
                req.page.as_ref().map(std::string::ToString::to_string),
            ),
            (
                "per_page".to_string(),
                req.per_page.as_ref().map(std::string::ToString::to_string),
            ),
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
    async fn user_id_teams_basic() {
        let token = FtApiToken::build(AuthInfo::build_from_env().unwrap())
            .await
            .unwrap();

        let client = FtClient::new(FtClientReqwestConnector::with_connector(
            reqwest::Client::new(),
        ));

        let session = client.open_session(&token);
        let res = session
            .users_id(FtApiUsersIdRequest::new(FtLoginId::new(
                "taejikim".to_owned(),
            )))
            .await;

        assert!(res.is_ok());
    }
}
