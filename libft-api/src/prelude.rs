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

pub use crate::api::prelude::*;
pub use crate::auth::*;
pub use crate::common::*;
pub use crate::connector::FtClientReqwestConnector;
pub use crate::info::*;
pub use crate::models::prelude::*;
