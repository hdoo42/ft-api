//! API endpoints related to project session information.
//!
//! This module provides access to the 42 Intra API endpoints that deal with project session data.
//! It includes functionality for retrieving teams and scale teams associated with specific project sessions.
//!
//! # Endpoints
//!
//! * **project_sessions_id_teams**: Retrieve teams associated with a specific project session
//! * **project_sessions_id_scale_teams**: Retrieve scale teams (evaluation teams) associated with a specific project session
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
//!     // Get teams for a specific project session
//!     let project_session_id = FtProjectSessionId::new(12345); // Replace with actual project session ID
//!     let response = session
//!         .project_sessions_id_teams(
//!             FtApiProjectSessionsTeamsRequest::new(project_session_id)
//!         )
//!         .await?;
//!     println!("Found {} teams for the project session", response.teams.len());
//!
//!     Ok(())
//! }
//! ```

mod project_sessions_id_scale_teams;
pub use project_sessions_id_scale_teams::*;
mod project_sessions_id_teams;
pub use project_sessions_id_teams::*;
