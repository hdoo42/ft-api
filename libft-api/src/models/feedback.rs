use rvstruct::ValueStruct;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct FtFeedback {
    pub comment: FtComment,
    pub created_at: FtDateTimeUtc,
    pub feedbackable_id: FtFeedbackableId,
    pub feedbackable_type: FtFeedbackableType,
    pub id: FtFeedbackId,
    pub rating: Option<FtRating>,
    pub user: Option<FtUser>,
}

#[derive(Debug, Eq, Hash, PartialEq, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtFeedbackId(i32);

#[derive(Debug, Eq, Hash, PartialEq, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtFeedbackableId(i32);

#[derive(Debug, Eq, Hash, PartialEq, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtFeedbackableType(String);

#[derive(Debug, Eq, Hash, PartialEq, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtRating(i32);

#[test]
fn deser_feedbacks() {
    let raw_feedbacks = r#"
    [
      {
    "id": 6124782,
    "user": {
      "login": "wsunwoo",
      "id": 172378,
      "url": "https://profile.intra.42.fr/users/wsunwoo"
    },
    "feedbackable_type": "ScaleTeam",
    "feedbackable_id": 6121743,
    "comment": "너무 정성드려서 피드백을 해주셔서 정말 감사했어요!!",
    "rating": 5,
    "created_at": "2023-11-27T04:18:08.698Z",
    "feedback_details": [
      { "id": 19139610, "rate": 4, "kind": "nice" },
      { "id": 19139609, "rate": 4, "kind": "rigorous" },
      { "id": 19139608, "rate": 4, "kind": "interested" },
      { "id": 19139607, "rate": 4, "kind": "punctuality" }
    ]
  },
  {
    "id": 6124859,
    "user": {
      "login": "siyekim",
      "id": 172395,
      "url": "https://profile.intra.42.fr/users/siyekim"
    },
    "feedbackable_type": "ScaleTeam",
    "feedbackable_id": 6121857,
    "comment": "he is so kind and his explanation has so many details. i want all my evaluator to be alike him.\r\nhe is 10/10 evaluator. okay thank you. 4$",
    "rating": 5,
    "created_at": "2023-11-27T05:56:00.374Z",
    "feedback_details": [
      { "id": 19139910, "rate": 4, "kind": "nice" },
      { "id": 19139909, "rate": 4, "kind": "rigorous" },
      { "id": 19139908, "rate": 4, "kind": "interested" },
      { "id": 19139907, "rate": 4, "kind": "punctuality" }
    ]
  }
  ]
    "#;

    let res: Seresult<Vec<FtFeedback>> = serde_json::from_str(raw_feedbacks);
    assert!(res.is_ok(), "{:?}", res);
}
