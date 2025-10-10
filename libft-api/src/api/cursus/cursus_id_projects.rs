use rsb_derive::Builder;
use serde::{Deserialize, Serialize};

use crate::{
    convert_filter_option_to_tuple, convert_range_option_to_tuple, to_param, ClientResult,
    FtClientHttpConnector, FtClientSession, FtCursusId, FtFilterOption, FtProject, FtProjectId,
    FtRangeOption, FtSortOption,
};

#[derive(Debug, Serialize, Deserialize, Builder)]
pub struct FtApiCursusIdProjectsRequest {
    pub cursus_id: FtCursusId,
    pub project_id: Option<FtProjectId>,
    pub sort: Option<Vec<FtSortOption>>,
    pub range: Option<Vec<FtRangeOption>>,
    pub filter: Option<Vec<FtFilterOption>>,
    pub page: Option<u16>,
    pub per_page: Option<u8>,
}

#[derive(Debug, Serialize, Deserialize, Builder)]
#[serde(transparent)]
pub struct FtApiCursusIdProjectsResponse {
    pub projects: Vec<FtProject>,
}

impl<FCHC> FtClientSession<'_, FCHC>
where
    FCHC: FtClientHttpConnector + Send + Sync,
{
    pub async fn cursus_id_projects(
        &self,
        req: FtApiCursusIdProjectsRequest,
    ) -> ClientResult<FtApiCursusIdProjectsResponse> {
        let url = &format!("cursus/{}/projects", req.cursus_id);

        let range = convert_range_option_to_tuple(req.range.unwrap_or_default()).unwrap();
        let filters = convert_filter_option_to_tuple(req.filter.unwrap_or_default()).unwrap();

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
    use crate::{
        AuthInfo, FtApiToken, FtClient, FtClientReqwestConnector, FtCursusId, FT_CURSUS_ID,
    };

    #[tokio::test]
    async fn basic() {
        let token = FtApiToken::build(AuthInfo::build_from_env().unwrap())
            .await
            .unwrap();

        let client = FtClient::new(FtClientReqwestConnector::with_connector(
            reqwest::Client::new(),
        ));

        let session = client.open_session(token);
        let res = session
            .cursus_id_projects(FtApiCursusIdProjectsRequest::new(FtCursusId::new(
                FT_CURSUS_ID,
            )))
            .await;

        assert!(res.is_ok());
    }
}
