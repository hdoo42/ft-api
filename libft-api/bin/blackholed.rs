use std::{ops::ControlFlow, sync::Arc, time::Duration};

use libft_api::{campus_id::*, prelude::*};
use tokio::{sync::Semaphore, task::JoinSet, time::sleep};
use tracing::{info, info_span};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    info_span!("main");
    let thread_num = 6;
    let mut handles = JoinSet::new();

    let permit = Arc::new(Semaphore::new(thread_num));
    for mut page in 1..=thread_num {
        let permit = Arc::clone(&permit);
        handles.spawn(async move {
            let _permit = permit.acquire().await.unwrap();
            let mut result = Vec::new();
            loop {
                if let ControlFlow::Break(()) = users(&mut result, thread_num, &mut page).await {
                    break result
                        .into_iter()
                        .filter_map(|user| user.id)
                        .collect::<Vec<FtUserId>>();
                }
            }
        });
    }

    let mut ids = Vec::new();
    while let Some(Ok(res)) = handles.join_next().await {
        ids.extend(res);
    }
    info!("{:#?}", ids);

    let mut handles = JoinSet::new();

    let mut result = Vec::new();
    for id in ids {
        let permit = Arc::clone(&permit);
        handles.spawn(async move {
            let _permit = permit.acquire().await.unwrap();
            let mut result = Vec::new();
            let mut page = 1;
            loop {
                if let ControlFlow::Break(()) = cursus_users(&mut result, &id, &mut page).await {
                    break result;
                }
            }
        });
    }

    while let Some(Ok(res)) = handles.join_next().await {
        result.extend(res);
        info!("{}", result.len());
    }

    println!("user_id,login,email,begin_at,end_at,cursus");
    result.into_iter().for_each(|cursus_user| {
        println!(
            "{:?},{:?},{:?},{:?},{:?},{:?}",
            cursus_user.user.id,
            cursus_user.user.login,
            cursus_user.user.email,
            cursus_user.begin_at,
            cursus_user.end_at,
            cursus_user.cursus.name,
        )
    });
}

async fn cursus_users(
    result: &mut Vec<FtCursusUser>,
    id: &FtUserId,
    page: &mut i32,
) -> ControlFlow<()> {
    let token = FtApiToken::try_get(AuthInfo::build_from_env().unwrap())
        .await
        .unwrap();
    let client = FtClient::new(FtClientReqwestConnector::new());
    let session = Arc::new(client.open_session(&token));
    let res = session
        .users_id_cursus_users(
            FtApiUsersIdCursusUsersRequest::new(id.clone())
                .with_per_page(100)
                .with_page(*page as u16),
        )
        .await;
    match res {
        Ok(res) => {
            if res.cursus_user.is_empty() {
                return ControlFlow::Break(());
            }
            result.extend(res.cursus_user);
            *page += 1;
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
async fn campus_users(thread_num: usize, page: &mut usize) -> ControlFlow<Vec<FtCampusUser>> {
    let token = FtApiToken::try_get(AuthInfo::build_from_env().unwrap())
        .await
        .unwrap();
    let client = FtClient::new(FtClientReqwestConnector::new());
    let session = Arc::new(client.open_session(&token));
    let res = session
        .campus_users(
            FtApiCampusUsersRequest::new()
                .with_per_page(100)
                .with_page(*page as u16)
                .with_filter(vec![
                    FtFilterOption::new(FtFilterField::CampusId, vec![SINGAPORE.to_string()]),
                    FtFilterOption::new(FtFilterField::Status, vec!["student".to_string()]),
                ]),
        )
        .await;
    let mut result = Vec::new();

    match res {
        Ok(res) => {
            if res.campus_users.is_empty() {
                return ControlFlow::Break(result);
            }
            result.extend(res.campus_users);
            *page += thread_num;
        }
        Err(FtClientError::RateLimitError(_)) => {
            eprintln!("rate limit, try again.");
            sleep(Duration::new(1, 42)).await
        }
        Err(e) => {
            eprintln!("other error: {e}");
            return ControlFlow::Break(result);
        }
    }
    ControlFlow::Continue(())
}

async fn users(result: &mut Vec<FtUser>, thread_num: usize, page: &mut usize) -> ControlFlow<()> {
    let token = FtApiToken::try_get(AuthInfo::build_from_env().unwrap())
        .await
        .unwrap();
    let client = FtClient::new(FtClientReqwestConnector::new());
    let session = Arc::new(client.open_session(&token));
    let res = session
        .users(
            FtApiUsersRequest::new()
                .with_per_page(100)
                .with_page(*page as u16)
                .with_filter(vec![
                    FtFilterOption::new(
                        FtFilterField::PrimaryCampusId,
                        vec![
                            RABAT.to_string(),
                            ISKANDARPUTERI.to_string(),
                            MILANO.to_string(),
                            NABLUS.to_string(),
                            LUANDA.to_string(),
                            WARSAW.to_string(),
                            ANTANANARIVO.to_string(),
                        ],
                    ),
                    FtFilterOption::new(FtFilterField::Kind, vec!["student".to_string()]),
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
