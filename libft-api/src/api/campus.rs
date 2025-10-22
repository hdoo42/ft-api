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
