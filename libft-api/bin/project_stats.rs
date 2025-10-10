use std::time::Duration;

use libft_api::{campus_id::GYEONGSAN, prelude::*, EXAM_RANK_03, MINISHELL, PHILOSOPHERS};
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    let token = FtApiToken::try_get(AuthInfo::build_from_env().unwrap())
        .await
        .unwrap();

    let client = FtClient::new(FtClientReqwestConnector::new());
    let session = client.open_session(token);

    let mut page = 1;
    println!("login|project|final_mark|retriable_at|status|team_mate");
    loop {
        let res = session
            .projects_uesrs(
                FtApiProjectsUsersRequest::new()
                    .with_page(page)
                    .with_per_page(100)
                    .with_filter(vec![
                        FtFilterOption::new(
                            FtFilterField::ProjectId,
                            vec![
                                EXAM_RANK_03.to_string(),
                                PHILOSOPHERS.to_string(),
                                MINISHELL.to_string(),
                            ],
                        ),
                        FtFilterOption::new(FtFilterField::Campus, vec![GYEONGSAN.to_string()]),
                    ]),
            )
            .await;

        match res {
            Ok(res) => {
                if res.projects_users.is_empty() {
                    break;
                }
                for ele in res.projects_users {
                    let team_mate = match ele.teams {
                        Some(mut teams) => match teams.pop() {
                            Some(team) => match team.users {
                                Some(users) => users
                                    .into_iter()
                                    .map(|user| user.login)
                                    .collect::<Vec<Option<FtLoginId>>>(),
                                None => vec![None],
                            },
                            None => vec![None],
                        },
                        None => vec![None],
                    };

                    println!(
                        "{:?}|{}|{:?}|{:?}|{}|{:?}",
                        ele.user.expect("projects_users always have FtUser").login,
                        ele.project.name,
                        ele.final_mark,
                        ele.retriable_at,
                        ele.status,
                        team_mate
                    );
                }
                page += 1;
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
}
