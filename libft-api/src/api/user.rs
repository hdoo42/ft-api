//! API endpoints related to user information.
//!
//! This module provides access to the 42 Intra API endpoints that deal with user data.
//! It includes functionality for retrieving user profiles, user locations, projects, cursus information,
//! correction points, and more.
//!
//! # Endpoints
//!
//! * **users**: Retrieve a list of users with filtering, pagination, and sorting options
//! * **users_post**: Create a new user (if you have the appropriate permissions)
//! * **users_id**: Get information about a specific user by their ID or login
//! * **users_id_locations**: Get location information for a specific user
//! * **users_id_locations_stats**: Get location statistics for a specific user
//! * **users_id_teams**: Get teams associated with a specific user
//! * **users_id_cursus_users**: Get cursus information for a specific user
//! * **users_id_projects_users**: Get project associations for a specific user
//! * **users_id_correction_point_historics**: Get correction point history for a specific user
//! * **users_id_correction_points_add**: Add correction points to a specific user
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
//!     // Get all users (with appropriate permissions)
//!     let users_response = session.users(FtApiUsersRequest::new()).await?;
//!     println!("Found {} users", users_response.users.len());
//!
//!     // Get a specific user by ID
//!     let user_response = session.users_id(FtUsersIdRequest::new(FtUserIdentifier::UserId(FtUserId::new(12345)))).await?;
//!     if let Some(login) = &user_response.login {
//!         println!("User login: {}", login.value());
//!     }
//!
//!     // Get a user's location data
//!     let location_response = session.users_id_locations(FtUsersIdLocationsRequest::new(FtUserIdentifier::UserId(FtUserId::new(12345)))).await?;
//!     println!("Found {} location records", location_response.get_vec().len());
//!
//!     Ok(())
//! }
//! ```

mod users;
pub use users::*;
mod users_id;
pub use users_id::*;
mod users_id_correction_point_historics;
pub use users_id_correction_point_historics::*;
mod users_id_correction_points_add;
pub use users_id_correction_points_add::*;
mod users_id_locations;
pub use users_id_locations::*;
mod users_id_locations_stats;
pub use users_id_locations_stats::*;
mod users_id_teams;
pub use users_id_teams::*;
mod users_id_cursus_users;
pub use users_id_cursus_users::*;
mod users_id_projects_users;
pub use users_id_projects_users::*;
