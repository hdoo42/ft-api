use rsb_derive::Builder;
use serde::{Deserialize, Serialize};

#[derive(Debug, Eq, Hash, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FtRangeField {
    BeginAt,
    CampusId,
    ClosedAt,
    CreatedAt,
    DeadlineAt,
    EndAt,
    FinalMark,
    Host,
    Id,
    LockedAt,
    Name,
    Primary,
    ProjectId,
    ProjectSessionId,
    Reason,
    RepoUrl,
    RepoUuid,
    ScaleTeamId,
    Status,
    Sum,
    TerminatingAt,
    Total,
    UpdatedAt,
    UserDataId,
    UserId,
}

#[derive(Debug, Eq, Hash, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FtFilterField {
    Active,
    Kind,
    ActiveCursus,
    BeginAt,
    Campus,
    CampusId,
    Closed,
    ClosedAt,
    CreatedAt,
    Cursus,
    CursusId,
    Deadline,
    DeadlineAt,
    End,
    EndAt,
    FinalMark,
    Future,
    Host,
    Id,
    Inactive,
    Locked,
    LockedAt,
    Name,
    Primary,
    PrimaryCampus,
    PrimaryCampusId,
    ProjectId,
    ProjectSessionId,
    Reason,
    RepoUrl,
    RepoUuid,
    Status,
    Terminating,
    TerminatingAt,
    UpdatedAt,
    UserId,
    WithMark,
}

#[derive(Debug, Eq, Hash, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct FtSortOption {
    pub field: FtSortField,
    pub descending: bool,
}

#[derive(Debug, Eq, Hash, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct FtRangeOption {
    pub range: FtRangeField,
    pub value: Vec<String>,
}

#[derive(Debug, Eq, Hash, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct FtFilterOption {
    pub field: FtFilterField,
    pub value: Vec<String>,
}

#[derive(Debug, Eq, Hash, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FtSortField {
    Id,
    UserId,
    BeginAt,
    EndAt,
    Primary,
    Host,
    CampusId,
}
use std::error::Error;

/// Trait to convert a field into a query parameter key.
pub trait ToQueryParam {
    /// Converts the field into a query parameter key.
    ///
    /// # Errors
    ///
    /// Returns an error if serialization fails or the field cannot be converted to a string.
    fn to_query_key(&self) -> Result<String, Box<dyn Error>>;
}

impl ToQueryParam for FtFilterField {
    /// Converts the `FtFilterField` into a query parameter key for filtering.
    ///
    /// # Errors
    ///
    /// Returns an error if serialization fails or if the enum cannot be converted to a string.
    fn to_query_key(&self) -> Result<String, Box<dyn Error>> {
        let binding = serde_json::to_value(self)?;
        let field = binding.as_str().ok_or("Failed to convert enum to string")?;
        Ok(format!("filter[{field}]"))
    }
}

impl ToQueryParam for FtRangeField {
    /// Converts the `FtRangeField` into a query parameter key for range selection.
    ///
    /// # Errors
    ///
    /// Returns an error if serialization fails or if the enum cannot be converted to a string.
    fn to_query_key(&self) -> Result<String, Box<dyn Error>> {
        let binding = serde_json::to_value(self)?;
        let field = binding.as_str().ok_or("Failed to convert enum to string")?;
        Ok(format!("range[{field}]"))
    }
}

/// Converts a list of options into query parameter tuples.
///
/// # Arguments
///
/// * `options` - A vector of tuples containing a field that implements `ToQueryParam` and a vector of values.
///
/// # Type Parameters
///
/// * `T` - A type that implements `ToQueryParam`.
///
/// # Errors
///
/// Returns an error if converting a field to a query key fails.
pub fn convert_options_to_tuple<T: ToQueryParam>(
    options: Vec<(T, Vec<String>)>,
) -> Result<Vec<(String, Option<String>)>, Box<dyn Error>> {
    options
        .into_iter()
        .map(|(field, values)| {
            let key = field.to_query_key()?;
            let value = if values.is_empty() {
                None
            } else {
                Some(values.join(","))
            };
            Ok((key, value))
        })
        .collect()
}

/// Converts a list of filter options into query parameter tuples.
///
/// # Arguments
///
/// * `filter_options` - A vector of `FtFilterOption` structs.
///
/// # Errors
///
/// Returns an error if converting a field to a query key fails.
pub fn convert_filter_option_to_tuple(
    filter_options: Vec<FtFilterOption>,
) -> Result<Vec<(String, Option<String>)>, Box<dyn Error>> {
    let options = filter_options
        .into_iter()
        .map(|option| (option.field, option.value))
        .collect();
    convert_options_to_tuple(options)
}

/// Converts a list of range options into query parameter tuples.
///
/// # Arguments
///
/// * `range_options` - A vector of `FtRangeOption` structs.
///
/// # Errors
///
/// Returns an error if converting a field to a query key fails.
pub fn convert_range_option_to_tuple(
    range_options: Vec<FtRangeOption>,
) -> Result<Vec<(String, Option<String>)>, Box<dyn Error>> {
    let options = range_options
        .into_iter()
        .map(|option| (option.range, option.value))
        .collect();
    convert_options_to_tuple(options)
}

#[macro_export]
macro_rules! to_param {
    ($req:expr, $field:ident) => {
        (
            stringify!($field).to_string(),
            $req.$field.as_ref().map(std::string::ToString::to_string),
        )
    };
}
