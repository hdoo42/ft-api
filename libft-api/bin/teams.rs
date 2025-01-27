use std::{
    collections::HashMap, io::Write, ops::ControlFlow, str::FromStr, sync::Arc, time::Duration,
};

use chrono::{DateTime, TimeDelta, TimeZone, Utc};
use ft_project_session_ids::c_piscine::C_PISCINE_RUSH_00;
use libft_api::{campus_id::*, prelude::*, FT_PISCINE_CURSUS_ID};
use rvstruct::ValueStruct;
use tokio::{sync::Semaphore, task::JoinSet, time::sleep};
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let evaluators = [
        190853, 190800, 172327, 174094, 190903, 190791, 172305, 197400, 197412, 197494, 172309,
        197539, 197459, 190817, 197605, 197606,
    ]
    .map(FtUserId::new);

    let project_teams = get_project_teams().await.teams;

    let begin_at = Utc.with_ymd_and_hms(2025, 1, 28, 5, 0, 0).unwrap();
    let mut bodys = Vec::new();
    for (i, project_team) in project_teams.iter().enumerate() {
        let evaluator = evaluators.get(i % evaluators.len()).unwrap().clone();
        let iter = i / evaluators.len();
        let begin_at = begin_at
            .checked_add_signed(TimeDelta::new(iter as i64 * 60 * 60 * 1, 0).unwrap())
            .map(FtDateTimeUtc::new)
            .unwrap();
        bodys.push(FtApiScaleTeamsMultipleCreateBody {
            begin_at,
            user_id: evaluator,
            team_id: project_team.id.clone(),
        });
    }

    println!("{:?}", bodys);

    let res = post_scale_team(bodys).await.unwrap();

    let file_path = format!(
        "/Users/hdoo/works/gsia/libft-api/libft-api/bin/piscine/third_cohort/first_round/rush_teams_{}.csv",
        Utc::now().format("%Y-%m-%d_%H-%M-%S")
    );

    let mut file = std::fs::File::create(&file_path).expect("Failed to create output file");

    file.write_all("project_idㅣscale_team_idㅣcreated_atㅣupdated_atㅣfinal_markㅣbegin_atㅣcorrectorㅣcorrectedsㅣfilled_atㅣtruantㅣteam.userㅣcommentㅣfeedback\n".as_bytes())?;

    for scale_team in res.scale_teams {
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
            Some(date) => date.0.to_utc().to_string(),
            None => "".to_string(),
        };

        let truant = match scale_team.truant {
            Some(user) => user
                .login
                .map(|l| l.0.to_string())
                .unwrap_or("".to_string()),
            None => "".to_string(),
        };
        let (team_uesr, project_id) = match scale_team.team {
            Some(team) => {
                let user = team
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
                    .unwrap_or("".to_string());
                let project_id = team
                    .project_id
                    .map(|project_id| project_id.to_string())
                    .unwrap_or("".to_string());
                (user, project_id)
            }
            None => ("".to_string(), "".to_string()),
        };
        let final_mark = match scale_team.final_mark {
            Some(final_mark) => final_mark.value().to_string(),
            None => "".to_string(),
        };
        writeln!(
            file,
            "{}ㅣ{}ㅣ{}ㅣ{}ㅣ{}ㅣ{}ㅣ{}ㅣ{}ㅣ{}ㅣ{}ㅣ{}ㅣ{:?}ㅣ{:?}",
            project_id,
            scale_team.id,
            scale_team.created_at.0.to_utc(),
            scale_team.updated_at.0.to_utc(),
            final_mark,
            begin_at,
            corrector,
            correcteds,
            filled_at,
            truant,
            team_uesr,
            scale_team.comment,
            scale_team.feedback
        )
        .expect("Failed to write record");
    }

    println!("Output written to: {}", file_path);

    Ok(())
}

async fn post_scale_team(
    bodys: Vec<FtApiScaleTeamsMultipleCreateBody>,
) -> Result<FtApiScaleTeamsMultipleCreateResponse, libft_api::FtClientError> {
    let token = FtApiToken::try_get(AuthInfo::build_from_env().unwrap())
        .await
        .unwrap();
    let client = FtClient::new(FtClientReqwestConnector::new());
    let session = Arc::new(client.open_session(&token));

    session
        .scale_teams_multiple_create_post(FtApiScaleTeamsMultipleCreateRequest::new(bodys))
        .await
}

async fn get_project_teams() -> FtApiProjectSessionsTeamsResponse {
    let token = FtApiToken::try_get(AuthInfo::build_from_env().unwrap())
        .await
        .unwrap();
    let client = FtClient::new(FtClientReqwestConnector::new());
    let session = Arc::new(client.open_session(&token));
    let res = session
        .project_sessions_id_teams(
            FtApiProjectSessionsTeamsRequest::new(FtProjectSessionId::new(C_PISCINE_RUSH_00))
                .with_per_page(100)
                .with_filter(vec![
                    FtFilterOption::new(FtFilterField::Campus, vec![GYEONGSAN.to_string()]),
                    FtFilterOption::new(
                        FtFilterField::Cursus,
                        vec![FT_PISCINE_CURSUS_ID.to_string()],
                    ),
                ])
                .with_range(vec![FtRangeOption::new(
                    FtRangeField::CreatedAt,
                    vec!["2025-1-20".to_owned(), "2025-1-29".to_owned()],
                )]),
        )
        .await;

    res.unwrap()
}
