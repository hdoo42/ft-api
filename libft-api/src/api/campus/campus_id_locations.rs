use rsb_derive::Builder;
use serde::{Deserialize, Serialize};
use tracing::debug;

use crate::{
    convert_filter_option_to_tuple, convert_range_option_to_tuple, to_param, ClientResult,
    FtCampusId, FtClientHttpConnector, FtClientSession, FtFilterOption, FtLocation, FtRangeOption,
    FtSortOption, FtUserId, FT_HTTP_PAGE_SIZE_100,
};

#[derive(Debug, Serialize, Deserialize, Builder)]
pub struct FtApiCampusLocationsRequest {
    pub user_id: Option<FtUserId>,
    pub campus_id: FtCampusId,
    pub sort: Option<Vec<FtSortOption>>,
    pub range: Option<Vec<FtRangeOption>>,
    pub filter: Option<Vec<FtFilterOption>>,
    pub page: Option<u16>,
    pub per_page: Option<u8>,
}

#[derive(Debug, Serialize, Deserialize, Builder)]
#[serde(transparent)]
pub struct FtApiCampusLocationsResponse {
    pub location: Vec<FtLocation>,
}

impl<'a, FCHC> FtClientSession<'a, FCHC>
where
    FCHC: FtClientHttpConnector + Send + Sync,
{
    pub async fn campus_id_locations(
        &self,
        req: FtApiCampusLocationsRequest,
    ) -> ClientResult<FtApiCampusLocationsResponse> {
        let url = &format!("campus/{}/locations", req.campus_id);

        let filters = convert_filter_option_to_tuple(req.filter.unwrap_or_default()).unwrap();
        let range = convert_range_option_to_tuple(req.range.unwrap_or_default()).unwrap();

        let params = vec![
            to_param!(req, page),
            to_param!(req, per_page),
            to_param!(req, user_id),
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
        debug!("{:#?}", params);

        self.http_session_api
            .http_get(url, &[filters, range, params].concat())
            .await
    }
}

#[cfg(test)]
mod tests {
    use crate::{prelude::*, GS_CAMPUS_ID};

    #[tokio::test]
    async fn location_with_params() {
        tracing_subscriber::fmt::init();
        let token = FtApiToken::build(AuthInfo::build_from_env().unwrap())
            .await
            .unwrap();

        let client = FtClient::new(FtClientReqwestConnector::with_connector(
            reqwest::Client::new(),
        ));

        let session = client.open_session(&token);
        let res = session
            .campus_id_locations(
                FtApiCampusLocationsRequest::new(FtCampusId::new(GS_CAMPUS_ID)).with_per_page(100),
            )
            .await;

        assert!(res.is_ok());
        match res {
            Ok(res) => {
                assert_eq!(100, res.location.len())
            }
            Err(_) => {}
        }
    }
}
