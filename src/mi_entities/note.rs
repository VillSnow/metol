use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::common::Int;

use super::{DriveFile, UserLite, Visibility};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Note {
    pub id: String,

    pub created_at: DateTime<Utc>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<DateTime<Utc>>,

    pub text: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cw: Option<String>,

    pub user_id: String,

    pub user: UserLite,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reply_id: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub renote_id: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reply: Option<Box<Note>>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub renote: Option<Box<Note>>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_hidden: Option<bool>,

    pub visibility: Visibility,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mentions: Option<Vec<String>>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub visible_user_ids: Option<Vec<String>>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_ids: Option<Vec<String>>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<DriveFile>>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub poll: Option<NotePoll>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub emojis: Option<std::collections::HashMap<String, serde_json::Value>>, // FIXME

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub channel: Option<Channel>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_only: Option<bool>,

    pub reaction_acceptance: Option<ReactionAcceptance>,
    pub reaction_emojis: std::collections::HashMap<String, serde_json::Value>, // FIXME
    pub reactions: std::collections::HashMap<String, serde_json::Value>,       // FIXME
    pub reaction_count: Int,
    pub renote_count: Int,
    pub replies_count: Int,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reaction_and_user_pair_cache: Option<Vec<String>>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub clipped_count: Option<Int>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub my_reaction: Option<String>,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotePoll {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<DateTime<Utc>>,

    pub multiple: bool,

    pub choices: Vec<PollChoice>,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PollChoice {
    pub is_voted: bool,
    pub text: String,
    pub votes: Int,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Channel {
    pub id: String,
    pub name: String,
    pub color: String,
    pub is_sensitive: bool,
    pub allow_renote_to_external: bool,
    pub user_id: Option<String>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ReactionAcceptance {
    LikeOnly,
    LikeOnlyForRemote,
    NonSensitiveOnly,
    NonSensitiveOnlyForLocalLikeOnlyForRemote,
}
