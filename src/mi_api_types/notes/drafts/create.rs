use crate::common::Int;
use crate::mi_entities::{note::ReactionAcceptance, Visibility};
use serde::{Deserialize, Serialize};

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
    pub hashtag: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_only: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reaction_acceptance: Option<ReactionAcceptance>,
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
    pub poll: Option<super::Poll>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scheduled_at: Option<Int>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_actually_scheduled: Option<bool>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub created_draft: super::super::NoteDraft,
}
