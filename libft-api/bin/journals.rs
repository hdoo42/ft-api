use std::io::Write;

use futures::FutureExt;
use libft_api::{campus_id::GYEONGSAN, prelude::*};

#[tokio::main]
async fn main() {
    let client = FtClient::new(FtClientReqwestConnector::new());

    let req: ReqFn<FtApiCampusIdJournalsResponse> = |session, page| {
        async move {
            session
                .campus_id_journals(
                    FtApiCampusIdJournalsRequest::new(
                        FtCampusId(GYEONGSAN),
                        "2025-10-1".to_string(),
                        "2025-10-3".to_string(),
                    )
                    .with_page(page)
                    .with_per_page(100),
                )
                .await
        }
        .boxed()
    };

    let mut result = Vec::new();
    for i in 1..=2 {
        result.push(scroller(&client, 2, i, req).await);
    }

    let mut file = std::fs::File::open("temp.json").unwrap();
    let _ = file.write_all(serde_json::to_string_pretty(&result).unwrap().as_bytes());
}
