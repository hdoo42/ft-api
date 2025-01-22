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
    let permit = Arc::new(Semaphore::new(7));

    let target_users = [
        213148, 213147, 213146, 213145, 213144, 213143, 212752, 212751, 212750, 212749,
    ];
    let exam_sets = Arc::new([
        ExamSet {
            exam_id: FtExamId::new(22086),
            project_id: FtProjectId::new(1302),
        },
        ExamSet {
            exam_id: FtExamId::new(22087),
            project_id: FtProjectId::new(1303),
        },
        ExamSet {
            exam_id: FtExamId::new(22088),
            project_id: FtProjectId::new(1304),
        },
    ]);

    let mut handles = JoinSet::new();

    for id in target_users {
        let permit = Arc::clone(&permit);
        let exam_sets = Arc::clone(&exam_sets);
        handles.spawn(async move {
            let _permit = permit.acquire().await.unwrap();

            let token = FtApiToken::build(AuthInfo::build_from_env().unwrap())
                .await
                .unwrap();
            let client = FtClient::new(FtClientReqwestConnector::new());

            let session = client.open_session(&token);

            for exam_set in exam_sets.iter() {
                let project_res = session
                    .projects_uesrs_post(FtApiProjectsUsersPostRequest::new(
                        FtApiProjectsUsersPostBody::new(
                            exam_set.project_id.clone(),
                            FtUserId::new(id),
                        ),
                    ))
                    .await;

                match project_res {
                    Ok(_) => println!(
                        "Successfully subscribed user {} to project {}",
                        id, exam_set.project_id
                    ),
                    Err(e) => println!(
                        "Failed to subscribe user {} to project {}: {:?}",
                        id, exam_set.project_id, e
                    ),
                }

                let exam_res = session
                    .exams_users_post(
                        FtApiExamsUsersPostRequest::new(FtApiExamsUsersPostBody {
                            user_id: FtUserId::new(id),
                        }),
                        exam_set.exam_id.clone(),
                    )
                    .await;

                match exam_res {
                    Ok(_) => println!(
                        "Successfully subscribed user {} to exam {}",
                        id, exam_set.exam_id
                    ),
                    Err(e) => println!(
                        "Failed to subscribe user {} to exam {}: {:?}",
                        id, exam_set.exam_id, e
                    ),
                }
            }
        });
    }

    while let Some(Ok(res)) = handles.join_next().await {
        println!("{:?}", res);
    }
}
