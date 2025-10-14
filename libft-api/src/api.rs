//! Endpoint-specific clients for the 42 Intra API.
//!
//! Each submodule mirrors an API domain (campus, user, project, exam, and so on) and exposes
//! request/response types plus the associated `FtClientSession` helpers for issuing calls.
//!
//! This module provides structured access to various 42 Intra API endpoints organized by domain:
//! * **Campus**: Information about 42 campuses and their locations
//! * **Cursus**: Curriculum-related information and user cursus associations
//! * **User**: User profiles and related data
//! * **Project**: Project information and user project associations
//! * **Exam**: Exam session information
//! * **Group**: Group-related functionality
//! * **Scale Team**: Evaluation team functionality
//! * **Project Session**: Project session data
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
//!     // Access user endpoint through the session
//!     let user_response = session.users_id(FtUsersIdRequest::new(12345)).await?;
//!     println!("User login: {}", user_response.login);
//!     
//!     Ok(())
//! }
//! ```

pub mod campus;
pub mod cursus;
pub mod exam;
pub mod group;
pub mod project;
pub mod project_session;
pub mod project_user;
pub mod scale_team;
pub mod user;

pub mod prelude;

/// Convenience abstraction for wrapper types that contain a `Vec<T>` under a single field.
///
/// This trait simplifies access to vector fields in API response types.
///
/// # Example
///
/// ```rust
/// use libft_api::api::HasVec;
///
/// struct FtApiUsersResponse {
///     users: Vec<String>,
/// }
///
/// impl HasVec<String> for FtApiUsersResponse {
///     fn get_vec(&self) -> &Vec<String> {
///         &self.users
///     }
///     
///     fn take_vec(self) -> Vec<String> {
///         self.users
///     }
/// }
/// ```
pub trait HasVec<T> {
    /// Get a reference to the contained vector.
    fn get_vec(&self) -> &Vec<T>;

    /// Take ownership of the contained vector.
    fn take_vec(self) -> Vec<T>;
}
