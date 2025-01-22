use chrono::{TimeDelta, Utc};
use libft_api::{campus_id::*, prelude::*, FT_GROUP_ID_TEST_ACCOUNT, FT_PISCINE_CURSUS_ID};
use std::sync::Arc;
use tokio::{sync::Semaphore, task::JoinSet};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let permit = Arc::new(Semaphore::new(7));
    let mut handles = JoinSet::new();
    let test_user_ids = 3..10;

    for id in test_user_ids {
        let permit = Arc::clone(&permit);
        handles.spawn(async move {
            let _permit = permit.acquire().await.unwrap();
            post_users(id).await
        });
    }

    let mut ids = Vec::new();
    while let Some(Ok(res)) = handles.join_next().await {
        ids.extend(res);
    }

    let newly_created_users = ids
        .into_iter()
        .filter_map(|res| res.user.id)
        .collect::<Vec<FtUserId>>();

    let mut handles = JoinSet::new();

    let mut result = Vec::new();
    for id in newly_created_users.clone() {
        let permit = Arc::clone(&permit);
        handles.spawn(async move {
            let _permit = permit.acquire().await.unwrap();
            assign_group(id).await
        });
    }

    while let Some(Ok(res)) = handles.join_next().await {
        result.extend(res);
    }
    println!("success: {}", result.len());

    let mut handles = JoinSet::new();

    let mut result = Vec::new();
    for id in newly_created_users {
        let permit = Arc::clone(&permit);
        handles.spawn(async move {
            let _permit = permit.acquire().await.unwrap();
            add_cursus(id, FtCursusId::new(FT_PISCINE_CURSUS_ID)).await
        });
    }

    while let Some(Ok(res)) = handles.join_next().await {
        result.extend(res);
    }
    println!("success: {}", result.len());
}

async fn assign_group(id: FtUserId) -> Result<FtApiGroupsUsersPostResponse, FtClientError> {
    let token = FtApiToken::try_get(AuthInfo::build_from_env().unwrap())
        .await
        .unwrap();
    let client = FtClient::new(FtClientReqwestConnector::new());
    let session = Arc::new(client.open_session(&token));

    session
        .groups_users_post(FtApiGroupsUsersPostRequest::new(FtApiGroupsUsersPostBody {
            group_id: FtGroupId::new(FT_GROUP_ID_TEST_ACCOUNT),
            user_id: id,
        }))
        .await
}

async fn add_cursus(
    id: FtUserId,
    cursus: FtCursusId,
) -> Result<FtApiUsersIdCursusUsersPostResponse, FtClientError> {
    let token = FtApiToken::try_get(AuthInfo::build_from_env().unwrap())
        .await
        .unwrap();
    let client = FtClient::new(FtClientReqwestConnector::new());
    let session = Arc::new(client.open_session(&token));

    session
        .users_id_cursus_users_post(FtApiUsersIdCursusUsersPostRequest::new(
            FtApiCursusUsersBody {
                cursus_id: cursus,
                user_id: id,
                begin_at: Utc::now()
                    .checked_add_signed(TimeDelta::new(60, 0).unwrap())
                    .unwrap()
                    .to_string(),
                has_coalition: false,
            },
        ))
        .await
}

async fn post_users(id: usize) -> Result<FtApiUserPostsResponse, FtClientError> {
    let token = FtApiToken::build(AuthInfo::build_from_env().unwrap())
        .await
        .unwrap();
    let client = FtClient::new(FtClientReqwestConnector::new());

    let session = client.open_session(&token);

    session
        .users_post(FtApiUsersPostRequest::new(FtApiUserPostBody {
            email: "yondoo@42gyeongsan.kr".to_string(),
            campus_id: FtCampusId::new(GYEONGSAN),
            first_name: "TEST".to_string(),
            last_name: "ACCOUNT".to_string(),
            login: format!("exam-gs{:02}", id),
            password: format!("Exam-gs{:02}@4242", id),
            kind: FtKind::Student,
            pool_month: "january".to_string(),
            pool_year: 2025,
        }))
        .await
}
