use std::{io::Write, sync::Arc};

use futures::FutureExt;
use libft_api::{info::ft_campus_id::GYEONGSAN, prelude::*};
use tokio::task::JoinSet;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let client = Arc::new(FtClient::new(FtClientReqwestConnector::new()));

    let req: ReqFn<_> = |session, page| {
        async move {
            session
                .users(
                    FtApiUsersRequest::new()
                        .with_page(page)
                        .with_per_page(100)
                        .with_filter(vec![FtFilterOption::new(
                            FtFilterField::PrimaryCampusId,
                            vec![GYEONGSAN.to_string()],
                        )])
                        .with_range(vec![FtRangeOption::new(
                            FtRangeField::CreatedAt,
                            vec!["2025-09-01".to_owned(), "2025-10-20".to_owned()],
                        )]),
                )
                .await
        }
        .boxed()
    };

    let mut handles = JoinSet::new();
    for i in 1..=8 {
        let client = Arc::clone(&client);
        handles.spawn(async move { scroller(&client, 8, i, req).await });
    }

    let mut result = Vec::new();
    while let Some(res) = handles.join_next().await {
        match res {
            Ok(v) => result.extend(v),
            Err(e) => tracing::error!("task failed: {e}"),
        }
    }

    let mut file = std::fs::File::create("pisciner.json").unwrap();
    file.write_all(serde_json::to_string_pretty(&result).unwrap().as_bytes())
        .unwrap();
}
