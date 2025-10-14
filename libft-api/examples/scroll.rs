use std::{io::Write, sync::Arc};

use futures::FutureExt;
use libft_api::{info::campus_id::SEOUL, prelude::*};
use tokio::task::JoinSet;
use tracing::info_span;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    info_span!("main");
    let client = Arc::new(FtClient::with_ratelimits(
        FtClientReqwestConnector::new(),
        8,
        14000,
    ));
    let thread_num = 8;
    let mut handles = JoinSet::new();

    let request_builder: ReqFn<FtApiUsersResponse> = |session, page| {
        async move {
            session
                .users(
                    FtApiUsersRequest::new()
                        .with_page(page)
                        .with_per_page(100)
                        .with_filter(vec![
                            FtFilterOption::new(
                                FtFilterField::PrimaryCampusId,
                                vec![SEOUL.to_string()],
                            ),
                            FtFilterOption::new(FtFilterField::Kind, vec!["student".to_string()]),
                        ]),
                )
                .await
        }
        .boxed()
    };

    for i in 1..=thread_num {
        let client = Arc::clone(&client);
        handles.spawn(async move { scroller(&client, thread_num, i, request_builder).await });
    }

    let mut all = Vec::<FtUser>::new();
    while let Some(res) = handles.join_next().await {
        match res {
            Ok(v) => all.extend(v),
            Err(e) => tracing::error!("task failed: {e}"),
        }
    }

    let mut file = std::fs::File::create("campus_users.json").unwrap();
    file.write_all(serde_json::to_string_pretty(&all).unwrap().as_bytes())
        .unwrap();
}
