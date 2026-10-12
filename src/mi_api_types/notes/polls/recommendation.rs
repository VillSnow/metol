use crate::common::Int;
use crate::mi_entities::Note;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<Int>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offset: Option<Int>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exclude_channels: Option<bool>,
}

pub type Response = Vec<Note>;
