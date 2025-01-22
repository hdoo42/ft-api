use std::{ops::ControlFlow, sync::Arc, time::Duration};

use chrono::Utc;
use libft_api::{campus_id::*, prelude::*};
use tokio::{sync::Semaphore, task::JoinSet, time::sleep};
use tracing::{info, info_span};

#[tokio::main]
async fn main() {
    let thread_num = 4;
    let mut handles = JoinSet::new();

    let permit = Arc::new(Semaphore::new(thread_num));
    for mut page in 1..=thread_num {
        let permit = Arc::clone(&permit);
        handles.spawn(async move {
            let _permit = permit.acquire().await.unwrap();
            let mut result = Vec::new();
            loop {
                if let ControlFlow::Break(()) = get_users(&mut result, thread_num, &mut page).await
                {
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
                if let ControlFlow::Break(()) =
                    get_projects_users(&mut result, &id, &mut page).await
                {
                    break result;
                }
            }
        });
    }

    while let Some(Ok(res)) = handles.join_next().await {
        result.extend(res);
        info!("{}", result.len());
    }

    println!("user_id,login,project_name,marked_at,final_mark,updated_at");
    result.into_iter().for_each(|projects_user| {
        println!(
            "{:?},{:?},{:?},{:?},{:?},{}",
            projects_user.user.id,
            projects_user.user.login,
            projects_user.project.name,
            projects_user.marked_at,
            projects_user.final_mark,
            Utc::now()
        )
    });
}

async fn get_projects_users(
    result: &mut Vec<FtProjectsUser>,
    id: &FtUserId,
    page: &mut i32,
) -> ControlFlow<()> {
    let token = FtApiToken::try_get(AuthInfo::build_from_env().unwrap())
        .await
        .unwrap();
    let client = FtClient::new(FtClientReqwestConnector::new());
    let session = Arc::new(client.open_session(&token));
    let res = session
        .users_id_projects_users(
            FtApiUsersIdProjectsUsersRequest::new(id.clone())
                .with_per_page(100)
                .with_page(*page as u16),
        )
        .await;
    match res {
        Ok(res) => {
            if res.projects_users.is_empty() {
                return ControlFlow::Break(());
            }
            result.extend(res.projects_users);
            *page += 1;
        }
        Err(FtClientError::RateLimitError(_)) => {
            eprintln!("rate limit, try again: {}", id);
            sleep(Duration::new(1, 42)).await
        }
        Err(e) => {
            eprintln!("other error: {e}");
            return ControlFlow::Break(());
        }
    }
    ControlFlow::Continue(())
}

async fn get_users(
    result: &mut Vec<FtUser>,
    thread_num: usize,
    page: &mut usize,
) -> ControlFlow<()> {
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
                .with_range(vec![FtRangeOption::new(
                    FtRangeField::CreatedAt,
                    vec!["2025-1-1".to_string(), "2025-2-1".to_string()],
                )])
                .with_filter(vec![
                    FtFilterOption::new(
                        FtFilterField::PrimaryCampusId,
                        vec![GYEONGSAN.to_string()],
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
            res.users
                .iter()
                .for_each(|user| println!("{:?}, {:?}", user.id, user.login));
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
