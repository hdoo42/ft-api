mod campus;
mod cursus;
mod exam;
mod group;
mod project;
mod project_session;
mod project_user;
mod scale_team;
mod user;

pub mod prelude;

pub trait HasVec<T> {
    fn get_vec(&self) -> &Vec<T>;
    fn take_vec(self) -> Vec<T>;
}
