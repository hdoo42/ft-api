//! API endpoints related to campus information.
//!
//! This module provides access to the 42 Intra API endpoints that deal with campus data.
//! It includes functionality for retrieving information about specific campuses, campus locations,
//! users associated with campuses, and campus journals.
//!
//! # Endpoints
//!
//! * **campus_id**: Retrieve information about a specific campus by its ID
//! * **campus_id_locations**: Get location information for a specific campus
//! * **campus_id_users**: Get users associated with a specific campus
//! * **campus_id_journals**: Retrieve journal information for a specific campus
//! * **campus_users**: Get campus user associations
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
//!     // Get all campuses
//!     let response = session.campus_id(FtApiCampusIdRequest::new()).await?;
//!     println!("Retrieved {} campuses", response.campus.len());
//!
//!     // Get specific campus (e.g., Paris campus)
//!     let paris_response = session
//!         .campus_id(FtApiCampusIdRequest::new().with_campus_id(FtCampusId::new(1)))
//!         .await?;
//!     println!("Paris campus: {:?}", paris_response.campus.first());
//!
//!     Ok(())
//! }
//! ```

pub mod campus_id_journals;
pub use campus_id_journals::*;
pub mod campus_id_locations;
pub use campus_id_locations::*;
pub mod campus_id_users;
pub use campus_id_users::*;
pub mod campus_id;
pub use campus_id::*;
pub mod campus_users;
pub use campus_users::*;
