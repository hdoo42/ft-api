//! Data structures for 42 Intra API entities.
//!
//! This module contains all the data structures that represent entities from the 42 Intra API.
//! Each submodule corresponds to a specific type of entity, such as users, projects, campuses,
//! cursus, and more. These structures are used for serialization and deserialization of API
//! requests and responses.
//!
//! The models follow a consistent naming convention where each entity has:
//! * A main struct (e.g., `FtUser`, `FtProject`) that represents the entity
//! * Value structs (e.g., `FtUserId`, `FtLoginId`) for strongly-typed identifiers
//! * Enum types (e.g., `FtKind`, `FtPoolMonth`) for categorical values
//!
//! Most models implement serialization traits to support JSON conversion for API interactions.
//! The data structures are designed to closely match the structure of the 42 Intra API responses
//! while providing type safety and ergonomic access to the data.
//!
//! # Example
//!
//! ```rust
//! use libft_api::models::user::{FtUser, FtUserId};
//! use rvstruct::ValueStruct;
//!
//! // Example of how a user model might be used
//! fn process_user(user: FtUser) {
//!     if let Some(user_id) = user.id {
//!         println!("Processing user with ID: {}", user_id.value());
//!     }
//! }
//! ```

pub mod achievement;
pub mod campus;
pub mod campus_user;
pub mod correction_point_history;
pub mod cursus_user;
pub mod datetime;
pub mod exam;
pub mod feedback;
pub mod flag;
pub mod group;
pub mod image;
pub mod journals;
pub mod language;
pub mod locations;
pub mod project;
pub mod project_data;
pub mod project_session;
pub mod projects_users;
pub mod role;
pub mod scale;
pub mod scale_teams;
pub mod team;
pub mod title;
pub mod user;

pub mod prelude;
