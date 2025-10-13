use std::{ops::ControlFlow, sync::Arc, time::Duration};

use libft_api::{campus_id::*, prelude::*};
use tokio::{task::JoinSet, time::sleep};
use tracing::{info, info_span};

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

    for i in 0..8 {
        let client = Arc::clone(&client);
        handles.spawn(async move {
            let mut result = Vec::new();
            let token = FtApiToken::try_get(AuthInfo::build_from_env().unwrap())
                .await
                .unwrap();
            let session = Arc::new(client.open_session(token));
            let mut page = i;
            loop {
                let page = &mut page;
                if let ControlFlow::Break(()) = {
                    let result = &mut result;
                    let session_clone = Arc::clone(&session);
                    async move {
                        let res = session_clone
                            .users(
                                FtApiUsersRequest::new()
                                    .with_page(*page)
                                    .with_per_page(100)
                                    .with_filter(vec![
                                        FtFilterOption::new(
                                            FtFilterField::PrimaryCampusId,
                                            vec![SEOUL.to_string()],
                                        ),
                                        FtFilterOption::new(
                                            FtFilterField::Kind,
                                            vec!["student".to_string()],
                                        ),
                                    ]),
                            )
                            .await;
                        match res {
                            Ok(res) => {
                                if res.users.is_empty() {
                                    return ControlFlow::Break(());
                                }
                                result.extend(res.users);
                                info!("{}", result.len());
                                *page += thread_num;
                            }
                            Err(FtClientError::RateLimitError(_)) => {
                                eprintln!("rate limit, try again.");
                                sleep(Duration::new(1, 42)).await
                            }
                            Err(e) => {
                                eprintln!("other error: {e}");
                                return ControlFlow::Break(());
                            }
                        }
                        ControlFlow::Continue(())
                    }
                }
                .await
                {
                    break result
                        .into_iter()
                        .filter_map(|user| user.id)
                        .collect::<Vec<FtUserId>>();
                }
            }
        });
    }

    // reserve size from X-Total
    let mut result = Vec::new();
    while let Some(Ok(data)) = handles.join_next().await {
        result.extend(data);
    }
}
