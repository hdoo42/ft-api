//! Data structures for 42 API location-related entities.
//!
//! This module contains data structures that represent location information
//! from the 42 Intra API, including user locations and related identifiers.

use crate::models::prelude::*;
use rvstruct::ValueStruct;
use serde::{Deserialize, Serialize};

/// Represents a location record from the 42 Intra API.
///
/// A location represents where a user is currently logged in or was last active.
#[derive(Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct FtLocation {
    pub id: FtLocationId,
    pub begin_at: FtDateTimeUtc,
    pub end_at: Option<FtDateTimeUtc>,
    pub primary: bool,
    pub host: FtHost,
    pub campus_id: FtCampusId,
    pub user: FtUser,
}

/// A unique identifier for a location record.
#[derive(Debug, Eq, Hash, PartialEq, PartialOrd, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtLocationId(i64);

/// Represents a host or computer where a user is located.
///
/// # Example
/// c1r1s1 (cluster 1, row 1, seat 1)
#[derive(Debug, Eq, Hash, PartialEq, PartialOrd, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtHost(pub String);
