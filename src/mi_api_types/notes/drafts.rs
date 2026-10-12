use crate::common::Int;
use serde::{Deserialize, Serialize};

pub mod count;
pub mod create;
pub mod delete;
pub mod list;
pub mod update;

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Poll {
    pub choices: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub multiple: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<Int>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expired_after: Option<Int>,
}
