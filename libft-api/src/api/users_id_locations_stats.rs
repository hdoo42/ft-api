use std::collections::HashMap;

use chrono::Days;
use chrono::NaiveDate;
use chrono::NaiveTime;
use rsb_derive::Builder;
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::FtSortOption;
use crate::{ClientResult, FtClientHttpConnector, FtClientSession, FtUserId};

#[derive(Debug, Serialize, Deserialize, Builder)]
pub struct FtApiUsersIdLocationsStatsRequest {
    pub user_id: FtUserId,
    pub begin_at: Option<NaiveDate>,
    pub end_at: Option<NaiveDate>,
    pub sort: Option<Vec<FtSortOption>>,
    pub page: Option<u16>,
    pub per_page: Option<u8>,
}

#[derive(Debug, Serialize, Deserialize, Builder)]
#[serde(transparent)]
pub struct FtApiUsersIdLocationsStatsResponse {
    pub stats: HashMap<NaiveDate, NaiveTime>,
}

impl<'a, FCHC> FtClientSession<'a, FCHC>
where
    FCHC: FtClientHttpConnector + Send + Sync,
{
    pub async fn users_id_locations_stats(
        &self,
        req: FtApiUsersIdLocationsStatsRequest,
    ) -> ClientResult<FtApiUsersIdLocationsStatsResponse> {
        let url = &format!("users/{}/locations_stats", req.user_id);

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
                "begin_at".to_string(),
                req.begin_at.map(|date| {
                    date.checked_add_days(Days::new(1))
                        .map(|date| date.to_string())
                        .expect("NaiveDate after checked add failed")
                }),
            ),
            (
                "end_at".to_string(),
                req.end_at.map(|date| {
                    date.checked_add_days(Days::new(1))
                        .map(|date| date.to_string())
                        .expect("NaiveDate after checked add failed")
                }),
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
            .http_get(url, &[params].concat())
            .await
    }
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;
    use users_id_locations_stats::FtApiUsersIdLocationsStatsRequest;

    use crate::*;

    #[tokio::test]
    async fn users_id_locations_stats_basic() {
        let token = FtApiToken::build(AuthInfo::build_from_env().unwrap())
            .await
            .unwrap();

        let client = FtClient::new(FtClientReqwestConnector::with_connector(
            reqwest::Client::new(),
        ));

        let session = client.open_session(&token);
        let res = session
            .users_id_locations_stats(
                FtApiUsersIdLocationsStatsRequest::new(
                    FtUserId::new(TEST_USER_YONDOO06_ID),
                    NaiveDate::from_ymd(2024, 11, 1),
                    NaiveDate::from_ymd(2024, 11, 6),
                )
                .with_page(1)
                .with_per_page(100),
            )
            .await;

        assert!(res.is_ok(), "{:?}", res);
    }
}
