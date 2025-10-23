//! The prelude module for API endpoints in the `libft-api` crate.
//!
//! This module provides convenient glob imports for all API endpoint types, requests, and responses
//! from the various API domain modules (campus, cursus, exam, group, project, project_session,
//! project_user, scale_team, and user). By importing everything in this module, users can access
//! all API-related functionality without needing to import individual modules.
//!
//! The prelude includes:
//! * All request and response types for API endpoints
//! * All session methods for making API calls
//! * The `HasVec` trait for working with vector-based responses
//!
//! # Example
//!
//! ```rust
//! use libft_api::prelude::*;
//! use libft_api::api::prelude::*; // API-specific prelude
//!
//! async fn example() -> ClientResult<()> {
//!     let token = FtApiToken::try_get(AuthInfo::build_from_env().unwrap()).await.unwrap();
//!     let client = FtClient::new(FtClientReqwestConnector::new());
//!     let session = client.open_session(token);
//!
//!     // All API functionality is available through the session
//!     let users = session.users(FtApiUsersRequest::new()).await?;
//!     let projects = session.projects(FtApiProjectRequest::new()).await?;
//!
//!     println!("Retrieved {} users and {} projects", users.users.len(), projects.projects.len());
//!
//!     Ok(())
//! }
//! ```

pub use super::campus::*;
pub use super::cursus::*;
pub use super::exam::*;
pub use super::group::*;
pub use super::project::*;
pub use super::project_session::*;
pub use super::project_user::*;
pub use super::scale_team::*;
pub use super::user::*;

pub use super::Entries;
pub use super::HasItems;
pub use super::Values;
pub use libft_api_derive::HasItems;
