use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RolePolicies {
    pub gtl_available: bool,
    pub ltl_available: bool,
    pub can_public_note: bool,
    pub mention_limit: i64,
    pub can_invite: bool,
    pub invite_limit: i64,
    pub invite_limit_cycle: i64,
    pub invite_expiration_time: i64,
    pub can_manage_custom_emojis: bool,
    pub can_manage_avatar_decorations: bool,
    pub can_search_notes: bool,
    pub can_use_translator: bool,
    pub can_hide_ads: bool,
    pub drive_capacity_mb: i64,
    pub max_file_size_mb: Option<i64>,
    pub always_mark_nsfw: bool,
    pub can_update_bio_media: Option<bool>,
    pub pin_limit: i64,
    pub antenna_limit: i64,
    pub word_mute_limit: i64,
    pub webhook_limit: i64,
    pub clip_limit: i64,
    pub note_each_clips_limit: i64,
    pub user_list_limit: i64,
    pub user_each_user_lists_limit: i64,
    pub rate_limit_factor: f64,
    pub avatar_decoration_limit: i64,
    pub can_import_antennas: Option<bool>,
    pub can_import_blocking: Option<bool>,
    pub can_import_following: Option<bool>,
    pub can_import_muting: Option<bool>,
    pub can_import_user_lists: Option<bool>,
    pub chat_availability: Option<ChatAvailability>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ChatAvailability {
    Available,
    Readonly,
    Unavailable,
}
