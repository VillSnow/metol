use crate::common::Int;
use serde::{Deserialize, Serialize};

use crate::mi_entities::{note::ReactionAcceptance, Note, Visibility};

/// Request body for `POST /notes/create`.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub visibility: Option<Visibility>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub visible_user_ids: Option<Vec<String>>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cw: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_only: Option<bool>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reaction_acceptance: Option<ReactionAcceptance>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub no_extract_mentions: Option<bool>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub no_extract_hashtags: Option<bool>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub no_extract_emojis: Option<bool>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reply_id: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub renote_id: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_ids: Option<Vec<String>>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub media_ids: Option<Vec<String>>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub poll: Option<Poll>,
}

/// Poll options accepted by `POST /notes/create`.
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

/// Response body for `POST /notes/create`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub created_note: Note,
}
