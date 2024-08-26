use ft_api::{
    ft_project_session_ids::ft_cursus::inner::LIBFT,
    project_sessions_id_teams::FtApiProjectSessionsTeamsRequest, AuthInfo, FtApiToken, FtClient,
    FtClientReqwestConnector, FtFilterField, FtFilterOption, FtProjectSessionId,
};

#[tokio::main]
async fn main() {
    let token = FtApiToken::build(AuthInfo::build_from_env().unwrap())
        .await
        .unwrap();

    let client = FtClient::new(FtClientReqwestConnector::with_connector(
        reqwest::Client::new(),
    ));

    let req =
        FtApiProjectSessionsTeamsRequest::new(FtProjectSessionId::new(LIBFT)).with_filter(vec![
            FtFilterOption::new(FtFilterField::Campus, vec!["69".to_owned()]),
        ]);

    let session = client.open_session(&token);
    let res = session.project_sessions_id_teams(req).await;

    assert!(res.is_ok());
    println!("{:?}", res.unwrap());
}
