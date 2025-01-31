use std::sync::Arc;

use libft_api::prelude::*;
use tokio::{sync::Semaphore, task::JoinSet};

#[derive(Debug)]
struct ExamSet {
    exam_id: FtExamId,
    project_id: FtProjectId,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let permit = Arc::new(Semaphore::new(1));

    let target_users = [];

    let mut handles = JoinSet::new();

    for id in target_users {
        let permit = Arc::clone(&permit);
        handles.spawn(async move {
            let _permit = permit.acquire().await.unwrap();

            let token = FtApiToken::build(AuthInfo::build_from_env().unwrap())
                .await
                .unwrap();
            let client = FtClient::new(FtClientReqwestConnector::new());

            let session = client.open_session(&token);

            let exam_res = session
                .exams_users_post(FtApiExamsUsersPostRequest::new(FtApiExamsUsersPostBody {
                    user_id: FtUserId::new(id),
                }))
                .await;
        });
    }

    while let Some(Ok(res)) = handles.join_next().await {
        println!("{:?}", res);
    }
}
