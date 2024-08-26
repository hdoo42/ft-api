use rsb_derive::Builder;
use serde::{Deserialize, Serialize};

#[derive(Debug, Eq, Hash, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FtFilterField {
    Active,
    ActiveCursus,
    BeginAt,
    Campus,
    CampusId,
    Closed,
    ClosedAt,
    CreatedAt,
    Cursus,
    Deadline,
    DeadlineAt,
    End,
    EndAt,
    FinalMark,
    Future,
    Host,
    Id,
    Inactive,
    Locked,
    LockedAt,
    Name,
    Primary,
    PrimaryCampus,
    ProjectId,
    ProjectSessionId,
    RepoUrl,
    RepoUuid,
    Status,
    Terminating,
    TerminatingAt,
    UpdatedAt,
    UserId,
    WithMark,
}

#[derive(Debug, Eq, Hash, PartialEq, Clone, Serialize, Deserialize)]
pub struct FtSortOption {
    pub field: FtSortField,
    pub descending: bool,
}

#[derive(Debug, Eq, Hash, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct FtFilterOption {
    pub field: FtFilterField,
    pub value: Vec<String>,
}

#[derive(Debug, Eq, Hash, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FtSortField {
    Id,
    UserId,
    BeginAt,
    EndAt,
    Primary,
    Host,
    CampusId,
}

// Function to convert Vec<FtFilterOption> to Vec<(&str, Option<String>)>
pub fn convert_to_tuples(
    filter_options: Vec<FtFilterOption>,
) -> Vec<(&'static str, Option<String>)> {
    filter_options
        .into_iter()
        .map(|option| {
            let field = match option.field {
                FtFilterField::Active => "filter[active]",
                FtFilterField::ActiveCursus => "filter[active_cursus]",
                FtFilterField::BeginAt => "filter[begin_at]",
                FtFilterField::Campus => "filter[campus]",
                FtFilterField::CampusId => "filter[campus_id]",
                FtFilterField::Closed => "filter[closed]",
                FtFilterField::ClosedAt => "filter[closed_at]",
                FtFilterField::CreatedAt => "filter[created_at]",
                FtFilterField::Cursus => "filter[cursus]",
                FtFilterField::Deadline => "filter[deadline]",
                FtFilterField::DeadlineAt => "filter[deadline_at]",
                FtFilterField::End => "filter[end]",
                FtFilterField::EndAt => "filter[end_at]",
                FtFilterField::FinalMark => "filter[final_mark]",
                FtFilterField::Future => "filter[future]",
                FtFilterField::Host => "filter[host]",
                FtFilterField::Id => "filter[id]",
                FtFilterField::Inactive => "filter[inactive]",
                FtFilterField::Locked => "filter[locked]",
                FtFilterField::LockedAt => "filter[locked_at]",
                FtFilterField::Name => "filter[name]",
                FtFilterField::Primary => "filter[primary]",
                FtFilterField::PrimaryCampus => "filter[primary_campus]",
                FtFilterField::ProjectId => "filter[project_id]",
                FtFilterField::ProjectSessionId => "filter[project_session_id]",
                FtFilterField::RepoUrl => "filter[repo_url]",
                FtFilterField::RepoUuid => "filter[repo_uuid]",
                FtFilterField::Status => "filter[status]",
                FtFilterField::Terminating => "filter[terminating]",
                FtFilterField::TerminatingAt => "filter[terminating_at]",
                FtFilterField::UpdatedAt => "filter[updated_at]",
                FtFilterField::UserId => "filter[user_id]",
                FtFilterField::WithMark => "filter[with_mark]",
            };
            let values = if option.value.is_empty() {
                None
            } else {
                Some(option.value.join(","))
            };
            (field, values)
        })
        .collect()
}
