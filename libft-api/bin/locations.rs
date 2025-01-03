use std::time::Duration;

use libft_api::{prelude::*, GS_CAMPUS_ID};
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    let token = FtApiToken::try_get(AuthInfo::build_from_env().unwrap())
        .await
        .unwrap();

    let client = FtClient::new(FtClientReqwestConnector::new());
    let session = client.open_session(&token);

    let mut page = 1;
    loop {
        let res = session
            .campus_id_locations(
                FtApiCampusLocationsRequest::new(FtCampusId::new(GS_CAMPUS_ID))
                    .with_page(page)
                    .with_per_page(100)
                    .with_range(vec![FtRangeOption::new(
                        FtRangeField::BeginAt,
                        vec!["2024-10-1".to_owned(), "2025-1-1".to_owned()],
                    )]),
            )
            .await;

        println!("host,date");
        match res {
            Ok(res) => {
                if res.location.is_empty() {
                    break;
                }
                for ele in res.location {
                    println!("{},{}", ele.host, ele.begin_at.0);
                }
                page += 1;
            }
            Err(FtClientError::RateLimitError(_)) => {
                eprintln!("rate limit, try again.");
                sleep(Duration::new(1, 42)).await
            }
            Err(e) => {
                eprintln!("other error: {e}");
                break;
            }
        }
    }
}
