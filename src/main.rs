use ft_api::{config_env_var, AuthInfo, FtApiToken, FtClient, FtClientReqwestConnector};

#[tokio::main]
async fn main() {
    println!("42 Api for Rust");

    // console_subscriber::init();
    //     let info = AuthInfo::from_env(
    //         config_env_var("FT_API_CLIENT_UID").unwrap(),
    //         config_env_var("FT_API_CLIENT_SECRET").unwrap(),
    //     );
    //     let res = FtApiToken::build(info).await;
    //
    //     if let Ok(token) = res {
    //         println!("token ok");
    //         let client = FtClient::new(FtClientReqwestConnector::with_connector(
    //             reqwest::Client::new(),
    //         ));
    //
    //         let session = client.open_session(&token);
    //         let res = session.campus_gs_locations().await;
    //         println!("{:?}", res);
    //     }
}
