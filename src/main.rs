use ft_api::{
    locations::FtApiCampusLocationsRequest, AuthInfo, FtApiToken, FtCampusId, FtClient,
    FtClientReqwestConnector, GS_CAMPUS_ID,
};

#[tokio::main]
async fn main() {
    println!("42 Api for Rust");

    console_subscriber::init();
    let info = AuthInfo::build_from_env().unwrap();
    let res = FtApiToken::try_get(info).await;
    println!("token: {:?}", res);

    if let Ok(token) = res {
        println!("token ok");
        let client = FtClient::new(FtClientReqwestConnector::with_connector(
            reqwest::Client::new(),
        ));

        let session = client.open_session(&token);
        let mut res = Vec::new();

        for page in 216..=218 {
            let resp = session
                .campus_id_locations(
                    FtApiCampusLocationsRequest::new(FtCampusId::new(GS_CAMPUS_ID))
                        .with_page(page as u16)
                        .with_per_page(100),
                )
                .await;
            println!("page: {} {:?}", page, resp);
            if resp.is_err() {
                break;
            }
            res.push(resp);
        }
    }
}
