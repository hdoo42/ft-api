use std::{sync::Arc, time::Duration};

use libft_api::{campus_id::GYEONGSAN, prelude::*, FT_PISCINE_CURSUS_ID};
use tokio::{sync::Semaphore, task, time::sleep};

#[tokio::main]
async fn main() {
    println!("id|user_id|item_type|item_id|reason|created_at|updated_at|event_at");
    let thread_num = 8;
    let permit = Arc::new(Semaphore::new(thread_num));

    for mut page in 1..=thread_num {
        let permit = Arc::clone(&permit);
        task::spawn(async move {
            let _permit = permit.acquire().await.unwrap();
            loop {
                let token = FtApiToken::try_get(AuthInfo::build_from_env().unwrap())
                    .await
                    .unwrap();

                let client = FtClient::new(FtClientReqwestConnector::new());
                let session = Arc::new(client.open_session(&token));
                let res = session
                    .campus_id_journals(
                        FtApiCampusIdJournalsRequest::new(
                            FtCampusId::new(GYEONGSAN),
                            "2025-1-20".to_string(),
                            "2025-1-23".to_string(),
                        )
                        .with_page(page as u16)
                        .with_filter(vec![FtFilterOption::new(
                            FtFilterField::CursusId,
                            vec![FT_PISCINE_CURSUS_ID.to_string()],
                        )]),
                    )
                    .await;

                match res {
                    Ok(res) => {
                        if res.journals.is_empty() {
                            break;
                        }
                        for ele in res.journals {
                            println!(
                                "{},{},{},{},{},{},{},{},{},{}",
                                ele.id,
                                ele.user_id,
                                ele.item_type,
                                ele.item_id,
                                ele.cursus_id,
                                ele.campus_id,
                                ele.reason,
                                ele.created_at.0,
                                ele.updated_at.0,
                                ele.event_at.0,
                            );
                        }
                        page += thread_num;
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
        })
        .await
        .unwrap();
    }
}
