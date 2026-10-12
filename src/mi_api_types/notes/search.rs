use crate::common::Int;
use crate::mi_entities::Note;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub query: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub range_start_at: Option<Int>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub range_end_at: Option<Int>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub since_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub until_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub since_date: Option<Int>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub until_date: Option<Int>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<Int>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offset: Option<Int>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<String>,
}

pub type Response = Vec<Note>;
