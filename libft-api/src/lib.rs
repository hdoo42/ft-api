// #![warn(clippy::pedantic)]
//! # libft-api
//!
//! `libft-api` provides typed, asynchronous access to the [42 Intra API](https://api.intra.42.fr/).
//! It wraps common endpoints with strongly typed requests, automatic rate limiting, and reusable
//! session management.
//!
//! ## Quick start
//! ```rust,no_run
//! use libft_api::{prelude::*};
//!
//! # async fn run() -> libft_api::ClientResult<()> {
//! let token = FtApiToken::try_get(AuthInfo::build_from_env()?).await?;
//! let client = FtClient::new(FtClientReqwestConnector::new());
//! let session = client.open_session(token);
//! let response = session
//!     .campus_id_locations(
//!         FtApiCampusIdLocationsRequest::new(FtCampusId::new(GYEONGSAN)).with_per_page(5),
//!     )
//!     .await?;
//! for location in response.location {
//!     println!("{} @ {}", location.user.login, location.host);
//! }
//! # Ok(())
//! # }
//! # tokio::runtime::Runtime::new().unwrap().block_on(run()).unwrap();
//! ```
//!
//! Set the `FT_API_CLIENT_UID` and `FT_API_CLIENT_SECRET` environment variables before building a
//! token. The default `FtClientReqwestConnector` reuses a shared `reqwest` client and applies the
//! crate's rate limiter, so you stay within the platform quotas.
//!
//! ## Modules
//! * `api` — high-level endpoint clients grouped by 42 domain (campus, user, projects, exams).
//! * `models` — serde-powered representations of request and response payloads.
//! * `auth` — helpers for building OAuth tokens and refreshing sessions.
//! * `axum_support` — Axum extractors and middleware for wiring the client into services.
//!
//! Explore the `bin/` directory for runnable examples of each workflow, and enable tracing with
//! `RUST_LOG=info` to inspect HTTP activity during development.
#![feature(macro_metavar_expr_concat)]

pub mod api;
pub mod models;

mod auth;
mod common;
pub mod info;
pub mod prelude;

mod connector;
