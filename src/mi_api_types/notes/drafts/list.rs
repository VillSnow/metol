use crate::common::Int;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Request {
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
    pub scheduled: Option<bool>,
}

pub type Response = Vec<super::super::NoteDraft>;
