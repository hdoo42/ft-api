//! API endpoints related to project-user associations.
//!
//! This module provides access to the 42 Intra API endpoints that deal with project-user relationships.
//! It includes functionality for retrieving and creating associations between users and projects.
//!
//! # Endpoints
//!
//! * **projects_users**: Retrieve project-user associations with filtering, pagination, and sorting options
//! * **projects_users_post**: Create a new association between a user and a project
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
//!     // Get project-user associations for a specific user
//!     let response = session
//!         .projects_uesrs(
//!             FtApiProjectsUsersRequest::new()
//!                 .with_filter(vec![
//!                     FtFilterOption::new(FtFilterField::UserId, vec!["12345".to_owned()])
//!                 ])
//!         )
//!         .await?;
//!     println!("Found {} project-user associations", response.projects_users.len());
//!
//!     // Create a new project-user association (if you have the appropriate permissions)
//!     // let new_assoc = session
//!     //     .projects_uesrs_post(FtApiProjectsUsersPostRequest::new(
//!     //         FtApiProjectsUsersPostBody {
//!     //             project_id: FtProjectId::new(123),
//!     //             user_id: FtUserId::new(12345),
//!     //         },
//!     //     ))
//!     //     .await?;
//!
//!     Ok(())
//! }
//! ```

mod projects_users;
pub use projects_users::*;
