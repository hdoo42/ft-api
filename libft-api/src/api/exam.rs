//! API endpoints related to exam information.
//!
//! This module provides access to the 42 Intra API endpoints that deal with exam data.
//! It includes functionality for retrieving exam information and managing exam-user associations.
//!
//! # Endpoints
//!
//! * **exams**: Retrieve a list of exams with filtering, pagination, and sorting options
//! * **exams_users_post**: Create an association between a user and an exam
//!
//! # Example
//!
//! ```rust
//! use libft_api::prelude::*;
//!
//! async fn example() -> ClientResult<()> {
//!     let token = FtApiToken::try_get(AuthInfo::build_from_env().unwrap()).await.unwrap();
//!     let client = FtClient::new(FtClientReqwestConnector::new());
//!     let session = client.open_session(token);
//!
//!     // Get all exams
//!     let response = session.exams(FtApiExamsRequest::new()).await?;
//!     println!("Found {} exams", response.exams.len());
//!
//!     // Create an exam-user association (if you have the appropriate permissions)
//!     // let exam_user_response = session
//!     //     .exams_users_post(
//!     //         FtApiExamsUsersPostRequest::new(FtApiExamsUsersPostBody {
//!     //             user_id: FtUserId::new(12345),
//!     //         }),
//!     //         FtExamId::new(22085),
//!     //     )
//!     //     .await?;
//!
//!     Ok(())
//! }
//! ```

mod exams;
pub use exams::*;
