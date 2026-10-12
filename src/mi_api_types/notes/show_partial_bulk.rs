use crate::common::Real;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub note_ids: Vec<String>,
}

pub type Response = Vec<PartialNote>;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PartialNote {
    pub id: String,
    pub reactions: std::collections::HashMap<String, Real>,
    pub reaction_emojis: std::collections::HashMap<String, String>,
}
