//! API endpoints related to cursus information.
//!
//! This module provides access to the 42 Intra API endpoints that deal with curriculum data.
//! It includes functionality for retrieving information about projects associated with specific cursus.
//!
//! # Endpoints
//!
//! * **cursus_id_projects**: Retrieve projects associated with a specific cursus by its ID
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
//!     // Get projects for the common core cursus (ID 21)
//!     let response = session
//!         .cursus_id_projects(FtApiCursusIdProjectsRequest::new(FtCursusId::new(21)))
//!         .await?;
//!     println!("Found {} projects in the cursus", response.projects.len());
//!
//!     Ok(())
//! }
//! ```

mod cursus_id_projects;
pub use cursus_id_projects::*;
