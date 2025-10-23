//! Endpoint-specific clients for the 42 Intra API.
//!
//! Each submodule mirrors an API domain (campus, user, project, exam, and so on) and exposes
//! request/response types plus the associated `FtClientSession` helpers for issuing calls.
//!
//! This module provides structured access to various 42 Intra API endpoints organized by domain:
//! * **Campus**: Information about 42 campuses and their locations
//! * **Cursus**: Curriculum-related information and user cursus associations
//! * **User**: User profiles and related data
//! * **Project**: Project information and user project associations
//! * **Exam**: Exam session information
//! * **Group**: Group-related functionality
//! * **Scale Team**: Evaluation team functionality
//! * **Project Session**: Project session data
//!
//! # Example
//!
//! # Example                                                                                
//! ```rust                                                                                  
//! use libft_api::{prelude::*, info::ft_campus_id::GYEONGSAN};                              
//!                                                                                          
//! # async fn run() -> ClientResult<()> {                                                   
//! let token = FtApiToken::try_get(AuthInfo::build_from_env()?).await?;                     
//! let client = FtClient::new(FtClientReqwestConnector::new());                             
//! let session = client.open_session(token);                                                
//! let response = session                                                                   
//!     .campus_id_locations(                                                                
//!         FtApiCampusIdLocationsRequest::new(FtCampusId::new(GYEONGSAN)).with_per_page(1),
//!     )                                                                                    
//!     .await?;                                                                             
//! for location in response.location {                                                      
//!     println!("{:?} @ {:?}", location.user.login, location.host);                         
//! }                                                                                        
//! # Ok(())                                                                                 
//! # }                                                                                      
//! # tokio::runtime::Runtime::new().unwrap().block_on(run()).unwrap();                      
//! ```                                                                                      

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

// 모드 마커
pub trait CollectMode {}
pub enum Values {}
impl CollectMode for Values {}
pub enum Entries {}
impl CollectMode for Entries {}

pub trait HasItems<M: CollectMode> {
    type OwnedItem;
    type BorrowedItem<'a>
    where
        Self: 'a;

    type IntoItems: IntoIterator<Item = Self::OwnedItem>;
    type IterItems<'a>: Iterator<Item = Self::BorrowedItem<'a>>
    where
        Self: 'a;

    fn into_items(self) -> Self::IntoItems;
    fn iter_items(&self) -> Self::IterItems<'_>;
}
