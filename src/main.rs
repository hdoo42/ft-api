use std::path::Path;

use ft_api::{
    project_sessions_id_scale_teams::FtApiProjectSessionsScaleTeamsResponse, AuthInfo, FtApiToken,
    FtClient, FtClientReqwestConnector,
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
    let path = Path::new("/Users/hdoo/Downloads/gnl_json.json");
    let b2bfile = std::fs::read_to_string(path);
    let b2b: Vec<FtApiProjectSessionsScaleTeamsResponse> =
        serde_json::from_str(http_body_str.as_str())
            .map_err(|err| map_serde_error(err, Some(http_body_str.as_str())))?;
}
