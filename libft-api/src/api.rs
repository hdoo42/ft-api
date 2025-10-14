//! Endpoint-specific clients for the 42 Intra API.
//!
//! Each submodule mirrors an API domain (campus, user, project, exam, and so on) and exposes
//! request/response types plus the associated `FtClientSession` helpers for issuing calls.

pub mod campus;
pub mod cursus;
pub mod exam;
pub mod group;
pub mod project;
pub mod project_session;
pub mod project_user;
pub mod scale_team;
pub mod user;

pub mod prelude;

/// Convenience abstraction for wrapper types that contain a `Vec<T>` under a single field.
pub trait HasVec<T> {
    fn get_vec(&self) -> &Vec<T>;
    fn take_vec(self) -> Vec<T>;
}
