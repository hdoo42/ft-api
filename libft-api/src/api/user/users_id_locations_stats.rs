use crate::to_param;
            to_param!(req, page),
            to_param!(req, per_page),
#[cfg(test)]
mod tests {
    use chrono::{Days, Local, NaiveDate};
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
        let end_at = Local::now().date_naive();
        let begin_at = end_at
            .checked_sub_days(Days::new(5))
            .expect("This is just 5 days ago");
        let res = session
            .users_id_locations_stats(
                FtApiUsersIdLocationsStatsRequest::new(FtUserId::new(TEST_USER_YONDOO06_ID))
                    .with_begin_at(begin_at)
                    .with_end_at(end_at)
                    .with_page(1)
                    .with_per_page(100),
            )
            .await;

        assert!(res.is_ok(), "{:?}", res);
    }
}
