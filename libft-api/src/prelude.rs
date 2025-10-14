//! The prelude module for the `libft-api` crate.
//!
//! This module provides convenient glob imports for the most commonly used items in the `libft-api` crate.
//! By importing everything in this module, users can access all the essential functionality without
//! needing to import individual modules.
//!
//! The prelude includes:
//! * API endpoint clients and requests from the `api` module
//! * Authentication types and functions from the `auth` module
//! * Common types like error types, client, parameters, rate limiter, and paginator from the `common` module
//! * The HTTP connector implementation from the `connector` module
//! * Constants and information about 42 campuses and cursus from the `info` module
//! * All model types from the `models` module
//!
//! # Example
//!
//! ```rust
//! use libft_api::prelude::*;
//!
//! async fn example() -> ClientResult<()> {
//!     // All necessary types are available through the prelude
//!     let auth_info = AuthInfo::build_from_env()?;
//!     let token = FtApiToken::try_get(auth_info).await?;
//!     let client = FtClient::new(FtClientReqwestConnector::new());
//!     let session = client.open_session(token);
//!
//!     // Now you can make API calls using the session
//!     let user = session.users_id(FtUsersIdRequest::new(12345)).await?;
//!     println!("User login: {}", user.login.unwrap_or_default());
//!
//!     Ok(())
//! }
//! ```

pub use crate::api::prelude::*;
pub use crate::auth::*;
pub use crate::common::*;
pub use crate::connector::FtClientReqwestConnector;
pub use crate::info::*;
pub use crate::models::prelude::*;
