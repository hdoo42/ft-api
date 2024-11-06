// api/prelude.rs

// Re-export commonly used modules or items from various API submodules
pub use super::campus::{campus_id_locations, campus_id_users};
pub use super::cursus::cursus_id_projects;
pub use super::project::{project_data, projects, projects_id_teams};
pub use super::project_session::{project_sessions_id_scale_teams, project_sessions_id_teams};
pub use super::user::{
    users_id_correction_point_historics, users_id_correction_points_add, users_id_locations_stats,
    users_id_teams,
};

// Optionally, if you have shared types or constants in other files like `common.rs`, re-export those as well.
pub use crate::common::*;
