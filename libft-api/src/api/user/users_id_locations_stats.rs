use std::collections::HashMap;

use chrono::Days;
use chrono::NaiveDate;
use rsb_derive::Builder;
use serde::{Deserialize, Serialize};

use crate::to_param;
use crate::{ClientResult, FtClientHttpConnector, FtClientSession, FtUserId};

#[derive(Debug, Serialize, Deserialize, Builder)]
pub struct FtApiUsersIdLocationsStatsRequest {
    pub user_id: FtUserId,
    pub begin_at: Option<NaiveDate>,
    pub end_at: Option<NaiveDate>,
    pub time_zone: Option<String>,
    pub page: Option<u16>,
    pub per_page: Option<u8>,
}

#[derive(Debug, Serialize, Deserialize, Builder)]
#[serde(transparent)]
pub struct FtApiUsersIdLocationsStatsResponse {
    pub stats: HashMap<NaiveDate, String>,
}

impl<FCHC> FtClientSession<'_, FCHC>
where
    FCHC: FtClientHttpConnector + Send + Sync,
{
    pub async fn users_id_locations_stats(
        &self,
        req: FtApiUsersIdLocationsStatsRequest,
    ) -> ClientResult<FtApiUsersIdLocationsStatsResponse> {
        let url = &format!("users/{}/locations_stats", req.user_id);

        let params = vec![
            to_param!(req, page),
            to_param!(req, per_page),
            (
                "begin_at".to_string(),
                req.begin_at.map(|date| date.to_string()),
            ),
            (
                "end_at".to_string(),
                req.end_at
                    .map(|date| {
                        date.checked_add_days(Days::new(1))
                            .expect("NaiveDate after checked add failed")
                    })
                    .map(|date| date.to_string()),
            ),
            ("time_zone".to_string(), req.time_zone),
        ];

        self.http_session_api
            .http_get(url, &[params].concat())
            .await
    }
}

#[cfg(test)]
mod tests {
    use crate::{prelude::*, TEST_USER_YONDOO_ID};
    use chrono::{Days, Local};

    #[tokio::test]
    async fn specific_date_range() {
        let token = FtApiToken::build(AuthInfo::build_from_env().unwrap())
            .await
            .unwrap();

        let client = FtClient::new(FtClientReqwestConnector::with_connector(
            reqwest::Client::new(),
        ));

        let session = client.open_session(token);
        let end_at = Local::now().date_naive();
        let begin_at = end_at
            .checked_sub_days(Days::new(5))
            .expect("This is just 5 days ago");
        let res = session
            .users_id_locations_stats(
                FtApiUsersIdLocationsStatsRequest::new(FtUserId::new(TEST_USER_YONDOO_ID))
                    .with_begin_at(begin_at)
                    .with_end_at(end_at),
            )
            .await;

        assert!(res.is_ok());
    }
}
