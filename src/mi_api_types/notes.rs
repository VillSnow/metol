use crate::common::{Int, Real};
pub mod children;
pub mod clips;
pub mod conversation;
pub mod create;
pub mod delete;
pub mod drafts;
pub mod show;
pub mod favorites {
    pub mod create;
    pub mod delete;
}
pub mod featured;
pub mod global_timeline;
pub mod hybrid_timeline;
pub mod local_timeline;
pub mod mentions;
pub mod polls {
    pub mod recommendation;
    pub mod vote;
}
pub mod reactions;
pub mod renotes;
pub mod replies;
pub mod search;
pub mod search_by_tag;
pub mod show_partial_bulk;
pub mod state;
pub mod thread_muting {
    pub mod create;
    pub mod delete;
}
pub mod timeline;
pub mod translate;
pub mod unrenote;
pub mod user_list_timeline;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::mi_entities::{note::ReactionAcceptance, DriveFile, Note, UserLite, Visibility};

/// Note draft returned by the notes draft endpoints.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NoteDraft {
    pub id: String,
    pub created_at: DateTime<Utc>,
    pub text: Option<String>,
    pub cw: Option<String>,
    pub user_id: String,
    pub user: UserLite,
    pub reply_id: Option<String>,
    pub renote_id: Option<String>,
    pub reply: Option<Box<Note>>,
    pub renote: Option<Box<Note>>,
    pub visibility: Visibility,
    pub visible_user_ids: Vec<String>,
    pub file_ids: Vec<String>,
    pub files: Option<Vec<DriveFile>>,
    pub hashtag: Option<String>,
    pub poll: Option<NoteDraftPoll>,
    pub channel_id: Option<String>,
    pub channel: Option<NoteDraftChannel>,
    pub local_only: bool,
    pub reaction_acceptance: Option<ReactionAcceptance>,
    pub scheduled_at: Option<Real>,
    pub is_actually_scheduled: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NoteDraftPoll {
    pub expires_at: Option<DateTime<Utc>>,
    pub expired_after: Option<Real>,
    pub multiple: bool,
    pub choices: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NoteDraftChannel {
    pub id: String,
    pub name: String,
    pub color: String,
    pub is_sensitive: bool,
    pub allow_renote_to_external: bool,
    pub user_id: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Clip {
    pub id: String,
    pub created_at: DateTime<Utc>,
    pub last_clipped_at: Option<DateTime<Utc>>,
    pub user_id: String,
    pub user: UserLite,
    pub name: String,
    pub description: Option<String>,
    pub is_public: bool,
    pub favorited_count: Real,
    pub is_favorited: Option<bool>,
    pub notes_count: Option<Int>,
}
