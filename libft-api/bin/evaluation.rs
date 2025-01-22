use std::{io::Write, ops::ControlFlow, sync::Arc, time::Duration};

use chrono::Utc;
use libft_api::{campus_id::*, prelude::*, FT_PISCINE_CURSUS_ID};
use tokio::{sync::Semaphore, task::JoinSet, time::sleep};
use tracing::{info, info_span};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    let thread_num = 8;
    let permit = Arc::new(Semaphore::new(thread_num));

    let mut handles = JoinSet::new();

    let mut result = Vec::new();
    for mut page in 1..=thread_num {
        let permit = Arc::clone(&permit);
        handles.spawn(async move {
            let _permit = permit.acquire().await.unwrap();
            let mut result = Vec::new();
            loop {
                if let ControlFlow::Break(()) =
                    get_scale_teams(&mut result, &mut page, thread_num).await
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

    let file_path = format!(
        "/Users/hdoo/works/gsia/libft-api/libft-api/bin/piscine/third_cohort/first_round/scale_teams_{}.csv",
        Utc::now().format("%Y-%m-%d_%H-%M-%S")
    );

    let mut file = std::fs::File::create(&file_path).expect("Failed to create output file");

    file.write_all("created_atㅣ updated_atㅣ begin_atㅣ correctorㅣ correctedsㅣ filled_atㅣ truantㅣ commentㅣ feedback\n".as_bytes())?;

    for scale_team in result {
        let corrector = match scale_team.corrector {
            FtCorrector::User(ft_user) => {
                ft_user.login.map(|login| login.0).unwrap_or("".to_string())
            }
            FtCorrector::String(s) => s,
        };
        let correcteds = match scale_team.correcteds {
            FtCorrecteds::String(s) => s,
            FtCorrecteds::Vec(vec) => vec
                .into_iter()
                .map(|user| user.login.map(|l| l.0).unwrap_or("".to_string()))
                .collect::<Vec<String>>()
                .join(","),
        };
        let begin_at = match scale_team.begin_at {
            Some(date) => date.0.to_utc().to_string(),
            None => "".to_string(),
        };
        let filled_at = match scale_team.filled_at {
            Some(date) => date.0.to_rfc3339(),
            None => "".to_string(),
        };

        let truant = match scale_team.truant {
            Some(user) => user
                .login
                .map(|l| l.0.to_string())
                .unwrap_or("".to_string()),
            None => "".to_string(),
        };
        let team = match scale_team.team {
            Some(team) => team
                .users
                .map(|users| {
                    users
                        .into_iter()
                        .map(|user| {
                            user.login
                                .map(|l| l.0.to_string())
                                .unwrap_or("".to_string())
                        })
                        .collect::<Vec<String>>()
                        .join(",")
                })
                .unwrap_or("".to_string()),
            None => "".to_string(),
        };
        writeln!(
            file,
            "{}ㅣ{}ㅣ{}ㅣ{}ㅣ{}ㅣ{}ㅣ{}ㅣ{}ㅣ{:?}ㅣ{:?}",
            scale_team.created_at.0.to_rfc3339(),
            scale_team.updated_at.0.to_rfc3339(),
            begin_at,
            corrector,
            correcteds,
            filled_at,
            truant,
            team,
            scale_team.comment,
            scale_team.feedback,
        )
        .expect("Failed to write record");
    }

    println!("Output written to: {}", file_path);
    Ok(())
}

async fn get_scale_teams(
    result: &mut Vec<FtScaleTeam>,
    page: &mut usize,
    thread_num: usize,
) -> ControlFlow<()> {
    let token = FtApiToken::try_get(AuthInfo::build_from_env().unwrap())
        .await
        .unwrap();
    let client = FtClient::new(FtClientReqwestConnector::new());
    let session = Arc::new(client.open_session(&token));
    let res = session
        .scale_teams(
            FtApiScaleTeamsRequest::new()
                .with_range(vec![FtRangeOption::new(
                    FtRangeField::CreatedAt,
                    vec!["2025-1-1".to_string(), "2025-3-1".to_string()],
                )])
                .with_filter(vec![
                    FtFilterOption::new(FtFilterField::CampusId, vec![GYEONGSAN.to_string()]),
                    FtFilterOption::new(
                        FtFilterField::CursusId,
                        vec![FT_PISCINE_CURSUS_ID.to_string()],
                    ),
                ])
                .with_per_page(100)
                .with_page(*page as u16),
        )
        .await;
    match res {
        Ok(res) => {
            if res.scale_teams.is_empty() {
                return ControlFlow::Break(());
            }
            result.extend(res.scale_teams);
            *page += thread_num;
        }
        Err(FtClientError::RateLimitError(_)) => sleep(Duration::new(1, 42)).await,
        Err(e) => {
            eprintln!("other error: {e}");
            return ControlFlow::Break(());
        }
    }
    ControlFlow::Continue(())
}
