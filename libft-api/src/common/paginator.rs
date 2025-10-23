use std::{ops::ControlFlow, sync::Arc, time::Duration};

use crate::{api::CollectMode, prelude::*};

use futures::future::BoxFuture;
use tokio::time::sleep;

pub type ReqFn<RS> = for<'a> fn(
    Arc<FtClientSession<'a, FtClientReqwestConnector>>,
    usize,
) -> BoxFuture<'a, ClientResult<RS>>;

pub async fn scroller<'a, T, M, RS, RQ>(
    client: &'a FtClient<FtClientReqwestConnector>,
    thread_num: usize,
    initial_page: usize,
    request_builder: RQ,
) -> Vec<T>
where
    RS: for<'de> serde::de::Deserialize<'de> + HasItems<M, OwnedItem = T>,
    M: CollectMode,
    RQ: Fn(
        Arc<FtClientSession<'a, FtClientReqwestConnector>>,
        usize,
    ) -> BoxFuture<'a, ClientResult<RS>>,
{
    let mut result = Vec::new();
    let token = FtApiToken::try_get(AuthInfo::build_from_env().unwrap())
        .await
        .unwrap();
    let session = Arc::new(client.open_session(token));
    // let total_page = *client.meta.total_page.lock().unwrap();
    let request = Arc::new(request_builder);

    let mut page = initial_page;
    loop {
        let page = &mut page;
        let request = Arc::clone(&request);
        if let ControlFlow::Break(()) = {
            let result = &mut result;
            let session_clone = Arc::clone(&session);
            async move {
                let res = request(session_clone, *page).await;
                match res {
                    Ok(res) => {
                        if *client.meta.total_page.lock().unwrap() as usize <= *page
                            || res.iter_items().next().is_none()
                        {
                            return ControlFlow::Break(());
                        }

                        result.extend(res.into_items());
                        *page += thread_num;
                    }
                    Err(FtClientError::RateLimitError(_)) => {
                        tracing::warn!("rate limit, try again.");
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
            break result;
        }
    }
}
