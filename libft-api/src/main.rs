use libft_api::{prelude::*, FT_CURSUS_ID};

mod test;
enum TestTest {}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let token = FtApiToken::build(AuthInfo::build_from_env().unwrap())
        .await
        .unwrap();

    let client = FtClient::new(FtClientReqwestConnector::with_connector(
        reqwest::Client::new(),
    ));
    let session = client.open_session(&token);

    let mut results = vec![];
    let mut page = 1;
    while let Ok(result) = session
        .cursus_id_projects(
            FtApiCursusIdProjectsRequest::new(FtCursusId::new(FT_CURSUS_ID))
                .with_per_page(100)
                .with_page(page),
        )
        .await
    {
        if result.projects.is_empty() {
            break;
        }
        page += 1;
        results.push(result);
    }

    for res in results {
        res.projects
            .iter()
            .for_each(|project| println!("{},{}", project.id, project.name));
    }
}
