use std::fmt::Display;

use self::Inner::*;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GsInfo(Inner);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum Inner {
    CampusId,
}

impl GsInfo {
    pub const CAMPUS_ID: GsInfo = GsInfo(CampusId);
}

impl Display for GsInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Display for Inner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CampusId => write!(f, "69"),
        }
    }
}

pub const TEST_USER_YONDOO06_ID: i32 = 185472;
