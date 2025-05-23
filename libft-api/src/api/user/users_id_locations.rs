use std::collections::HashMap;

use chrono::Days;
use chrono::NaiveDate;
use rsb_derive::Builder;
use serde::{Deserialize, Serialize};

use crate::{prelude::*, to_param};

#[derive(Debug, Serialize, Deserialize, Builder)]
pub struct FtApiUsersIdLocationsRequest {
    pub user_id: FtUserId,
    pub sort: Option<Vec<FtSortOption>>,
    pub range: Option<Vec<FtRangeOption>>,
    pub filter: Option<Vec<FtFilterOption>>,
    pub page: Option<u16>,
    pub per_page: Option<u8>,
}

#[derive(Debug, Serialize, Deserialize, Builder)]
#[serde(transparent)]
pub struct FtApiUsersIdLocationsResponse {
    pub locations: Vec<FtLocation>,
}

impl<FCHC> FtClientSession<'_, FCHC>
where
    FCHC: FtClientHttpConnector + Send + Sync,
{
    pub async fn users_id_locations(
        &self,
        req: FtApiUsersIdLocationsRequest,
    ) -> ClientResult<FtApiUsersIdLocationsResponse> {
        let url = &format!("users/{}/locations", req.user_id);

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
            .http_get(url, &[filters, range, params].concat())
            .await
    }
}

#[cfg(test)]
mod tests {
    use crate::{prelude::*, TEST_USER_YONDOO_ID};

    /// Checks the filter[active] is working properly.
    #[tokio::test]
    async fn is_active() {
        let token = FtApiToken::build(AuthInfo::build_from_env().unwrap())
            .await
            .unwrap();

        let client = FtClient::new(FtClientReqwestConnector::with_connector(
            reqwest::Client::new(),
        ));

        let session = client.open_session(&token);
        let res = session
            .users_id_locations(
                FtApiUsersIdLocationsRequest::new(FtUserId::new(TEST_USER_YONDOO_ID)).with_filter(
                    vec![FtFilterOption::new(
                        FtFilterField::Active,
                        vec![false.to_string()],
                    )],
                ),
            )
            .await;

        assert!(res.is_ok());
        assert_eq!(res.unwrap().locations.len(), 2);
    }
}
