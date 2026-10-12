use crate::common::Int;
use serde::{Deserialize, Serialize};

pub mod create;
pub mod delete;

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub note_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<Int>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub since_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub until_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub since_date: Option<Int>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub until_date: Option<Int>,
}

pub type Response = Vec<NoteReaction>;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NoteReaction {
    pub id: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub user: crate::mi_entities::UserLite,
    pub r#type: String,
}
