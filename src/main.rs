use std::path::Path;

use ft_api::{
    map_serde_error, project_sessions_id_scale_teams::FtApiProjectSessionsScaleTeamsResponse,
    AuthInfo, FtApiToken, FtClient, FtClientReqwestConnector, FtLoginId, FtUser,
};

#[tokio::main]
async fn main() {
    println!("42 Api for Rust");

    console_subscriber::init();
    let info = AuthInfo::build_from_env().unwrap();
    let res = FtApiToken::build(info).await;
    println!("token: {:?}", res);

    if let Ok(token) = res {
        println!("token ok");
        let client = FtClient::new(FtClientReqwestConnector::with_connector(
            reqwest::Client::new(),
        ));

        let session = client.open_session(&token);
        let res = session.campus_gs_locations().await;
        // println!("{}", res.unwrap().location.len());
    }
}
