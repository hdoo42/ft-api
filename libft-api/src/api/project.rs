//! API endpoints related to project information.
//!
//! This module provides access to the 42 Intra API endpoints that deal with project data.
//! It includes functionality for retrieving project information, project data, and project-team associations.
//!
//! # Endpoints
//!
//! * **projects**: Retrieve a list of projects with filtering, pagination, and sorting options
//! * **projects_id_teams**: Get teams associated with a specific project
//! * **project_data**: Additional project-related data access
//!
//! # Example
//!
//! ```rust
//! use libft_api::prelude::*;
//!
//! async fn example() -> ClientResult<()> {
//!     let token = FtApiToken::try_get(AuthInfo::build_from_env().unwrap()).await.unwrap();
//!     let client = FtClient::new(FtClientReqwestConnector::new());
//!     let session = client.open_session(token);
//!
//!     // Get all projects
//!     let response = session.projects(FtApiProjectRequest::new()).await?;
//!     println!("Found {} projects", response.projects.len());
//!
//!     // Get projects for a specific cursus
//!     let cursus_projects = session
//!         .projects(FtApiProjectRequest::new().with_cursus_id(FtCursusId::new(21)))
//!         .await?;
//!
//!     Ok(())
//! }
//! ```

pub use project_data::*;
mod project_data;
pub use projects::*;
mod projects;
pub use projects_id_teams::*;
mod projects_id_teams;
