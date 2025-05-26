use serde::{Deserialize, Serialize};

use crate::{common::Int, Real};

use super::{Achievements, Announcement, Note, OnlineStatus, Page, RoleLite, RolePolicies};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserLite {
    pub id: String,
    pub name: Option<String>,
    pub username: String,
    /// The local host is represented with `null`.
    pub host: Option<String>,
    pub avatar_url: Option<String>,
    pub avatar_blurhash: Option<String>,
    pub avatar_decorations: Vec<AvatarDecoration>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_bot: Option<bool>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_cat: Option<bool>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub require_signin_to_view_contents: Option<bool>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub make_notes_followers_only_before: Option<f64>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub make_notes_hidden_before: Option<f64>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instance: Option<Box<Instance>>,

    pub emojis: std::collections::HashMap<String, String>,

    pub online_status: OnlineStatus,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub badge_roles: Option<Vec<BadgeRole>>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DetailedOnly {
    pub url: Option<String>,
    pub uri: Option<String>,
    pub moved_to: Option<String>,
    pub also_known_as: Option<Vec<String>>,
    pub created_at: String,
    pub updated_at: Option<String>,
    pub last_fetched_at: Option<String>,
    pub banner_url: Option<String>,
    pub banner_blurhash: Option<String>,
    pub is_locked: bool,
    pub is_silenced: bool,
    pub is_suspended: bool,
    pub description: Option<String>,
    pub location: Option<String>,
    pub birthday: Option<String>,
    pub lang: Option<String>,
    pub fields: Vec<Field>,
    pub verified_links: Vec<String>,
    pub followers_count: Int,
    pub following_count: Int,
    pub notes_count: Int,
    pub pinned_note_ids: Vec<String>,
    pub pinned_notes: Vec<Note>,
    pub pinned_page_id: Option<String>,
    pub pinned_page: Box<Page>,
    pub public_reactions: bool,
    pub following_visibility: FollowVisibility,
    pub followers_visibility: FollowVisibility,
    pub chat_scope: ChatScope,
    pub can_chat: bool,
    pub roles: Vec<RoleLite>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub followed_message: Option<String>,

    pub memo: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub moderation_note: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub two_factor_enabled: Option<bool>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub use_password_less_login: Option<bool>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub security_keys: Option<bool>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_following: Option<bool>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_followed: Option<bool>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_pending_follow_request_from_you: Option<bool>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_pending_follow_request_to_you: Option<bool>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_blocking: Option<bool>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_blocked: Option<bool>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_muted: Option<bool>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_renote_muted: Option<bool>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notify: Option<Notify>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub with_replies: Option<bool>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserDetailedNotMe {
    #[serde(flatten)]
    pub lite: UserLite,

    #[serde(flatten)]
    pub detailed_only: DetailedOnly,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MeOnly {
    pub avatar_id: Option<String>,
    pub banner_id: Option<String>,
    pub followed_message: Option<String>,
    pub is_moderator: Option<bool>,
    pub is_admin: Option<bool>,
    pub inject_featured_note: bool,
    pub receive_announcement_email: bool,
    pub always_mark_nsfw: bool,
    pub auto_sensitive: bool,
    pub careful_bot: bool,
    pub auto_accept_followed: bool,
    pub no_crawle: bool,
    pub prevent_ai_learning: bool,
    pub is_explorable: bool,
    pub is_deleted: bool,
    pub two_factor_backup_codes_stock: TwoFactorBackupCodesStock,
    pub hide_online_status: bool,
    pub has_unread_specified_notes: bool,
    pub has_unread_mentions: bool,
    pub has_unread_announcement: bool,
    pub unread_announcements: Vec<Announcement>,
    pub has_unread_antenna: bool,
    pub has_unread_channel: bool,
    pub has_unread_chat_messages: bool,
    pub has_unread_notification: bool,
    pub has_pending_received_follow_request: bool,
    pub unread_notifications_count: Int,
    pub muted_words: Vec<Vec<String>>,
    pub hard_muted_words: Vec<Vec<String>>,
    pub muted_instances: Option<Vec<String>>,
    pub notification_recieve_config: serde_json::Value, // FIXME
    pub email_notification_types: Vec<String>,
    pub achievements: Vec<Achievements>,
    pub logged_in_days: Int,
    pub policies: RolePolicies,
    pub two_factor_enabled: bool,
    pub use_password_less_login: bool,
    pub security_keys: bool,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email_verified: Option<bool>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub security_keys_list: Option<Vec<SecurityKey>>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MeDetailed {
    #[serde(flatten)]
    pub lite: UserLite,

    #[serde(flatten)]
    pub detailed_only: DetailedOnly,

    #[serde(flatten)]
    pub me_only: MeOnly,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AvatarDecoration {
    pub id: String,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub angle: Option<Real>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flip_h: Option<bool>,

    pub url: String,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offset_x: Option<Real>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offset_y: Option<Real>,
}

#[derive(Clone, Default, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Instance {
    pub name: Option<String>,
    pub software_name: Option<String>,
    pub software_version: Option<String>,
    pub icon_url: Option<String>,
    pub favicon_url: Option<String>,
    pub theme_color: Option<String>,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BadgeRole {
    pub name: String,
    pub icon_url: Option<String>,
    pub display_order: Int, // FIXME: 確認
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Field {
    pub name: String,
    pub value: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum FollowVisibility {
    Public,
    Followers,
    Private,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ChatScope {
    Everyone,
    Following,
    Followers,
    Mutual,
    None,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Notify {
    Normal,
    None,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TwoFactorBackupCodesStock {
    Full,
    Partial,
    None,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SecurityKey {
    pub id: String,
    pub name: String,
    pub last_used: String,
}
