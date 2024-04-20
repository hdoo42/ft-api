use std::fmt::Display;

pub use client::*;
mod client;

pub use error::*;
mod error;

use self::Inner::*;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GsInfo(Inner);

#[derive(Clone, PartialEq, Eq, Hash)]
enum Inner {
    CampusId = 69,
}

impl GsInfo {
    pub const CAMPUS_ID: GsInfo = GsInfo(CampusId);
}

impl Display for GsInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self::CampusId)
    }
}

impl Display for Inner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self::CampusId)
    }
}
