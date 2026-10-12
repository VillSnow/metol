use crate::common::Int;
use crate::mi_entities::Note;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub list_id: String,
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_partial: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub include_my_renotes: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub include_renoted_my_notes: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub include_local_renotes: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub with_renotes: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub with_files: Option<bool>,
}

pub type Response = Vec<Note>;
