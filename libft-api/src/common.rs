//! Common functionality used across the 42 Intra API client.
//!
//! This module provides shared utilities that are used throughout the libft-api crate:
//! * **Client**: Core HTTP client and session management functionality
//! * **Error**: Comprehensive error types for various failure scenarios
//! * **Parameter**: Types and utilities for building API query parameters
//! * **Rate Limiter**: Automatic rate limiting to stay within API quotas
//! * **Paginator**: Utilities for handling paginated API responses
//!
//! # Example
//!
//! ```rust
//! use libft_api::prelude::*;
//!
//! async fn example() -> ClientResult<()> {
//!     // Create a client with custom rate limits
//!     let client = FtClient::with_ratelimits(FtClientReqwestConnector::new(), 5, 1000);
//!     let token = FtApiToken::try_get(AuthInfo::build_from_env()?).await?;
//!     let session = client.open_session(token);
//!     
//!     Ok(())
//! }
//! ```

pub use client::*;
mod client;

pub use error::*;
mod error;

pub use param::*;
mod param;

pub use ratelimiter::*;
mod ratelimiter;

pub use paginator::*;
mod paginator;
