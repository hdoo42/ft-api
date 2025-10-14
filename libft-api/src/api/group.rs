//! API endpoints related to group information.
//!
//! This module provides access to the 42 Intra API endpoints that deal with group data.
//! It includes functionality for retrieving group information and managing group-user associations.
//!
//! # Endpoints
//!
//! * **groups**: Retrieve a list of groups with optional filtering by user ID and pagination options
//! * **groups_users_post**: Create an association between a user and a group
//!
//! # Example
//!
//! ```rust
//! use libft_api::prelude::*;
//!
//! async fn example() -> ClientResult<()> {
//!     let token = FtApiToken::try_get(AuthInfo::build_from_env()?).await?;
//!     let client = FtClient::new(FtClientReqwestConnector::new());
//!     let session = client.open_session(token);
//!
//!     // Get all groups
//!     let response = session.groups(FtApiGroupsRequest::new()).await?;
//!     println!("Found {} groups", response.groups.len());
//!
//!     // Get groups for a specific user
//!     let user_groups = session
//!         .groups(FtApiGroupsRequest::new().with_user_id(FtUserId::new(12345)))
//!         .await?;
//!
//!     // Create a group-user association (if you have the appropriate permissions)
//!     // let group_user_response = session
//!     //     .groups_users_post(FtApiGroupsUsersPostRequest::new(
//!     //         FtApiGroupsUsersPostBody {
//!     //             group_id: FtGroupId::new(123),
//!     //             user_id: FtUserId::new(12345),
//!     //         },
//!     //     ))
//!     //     .await?;
//!
//!     Ok(())
//! }
//! ```

mod groups;
pub use groups::*;
