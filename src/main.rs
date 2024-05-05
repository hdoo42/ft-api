use std::path::Path;

use ft_api::{
    map_serde_error, project_sessions_id_scale_teams::FtApiProjectSessionsScaleTeamsResponse,
    AuthInfo, FtApiToken, FtClient, FtClientReqwestConnector, FtUser,
};

#[tokio::main]
async fn main() {
    // println!("42 Api for Rust");
    //
    // console_subscriber::init();
    // let info = AuthInfo::build_from_env().unwrap();
    // let res = FtApiToken::try_get(info).await;
    // println!("token: {:?}", res);
    //
    // if let Ok(token) = res {
    //     println!("token ok");
    //     let client = FtClient::new(FtClientReqwestConnector::with_connector(
    //         reqwest::Client::new(),
    //     ));
    //
    //     let session = client.open_session(&token);
    //     let res = session.campus_gs_locations().await;
    //     // println!("{}", res.unwrap().location.len());
    //     println!("{:?}", res.unwrap().location);
    // }
    let raw_partial_user = r#"
      {
        "id": 183812,
        "login": "nkanaan",
        "url": "https://api.intra.42.fr/v2/users/nkanaan"
      }
        "#;

    let res: Result<FtUser, serde_json::Error> = serde_json::from_str(raw_partial_user);
    println!("{:?}", res);
}
