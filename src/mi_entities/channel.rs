use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::common::Int;

use super::Note;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Channel {
    pub id: String,
    pub created_at: String,
    pub last_noted_at: Option<DateTime<Utc>>,
    pub name: String,
    pub description: Option<String>,
    pub user_id: Option<String>,
    pub banner_url: Option<String>,
    pub pinned_note_ids: Vec<String>,
    pub color: String,
    pub is_archived: bool,
    pub users_count: Int,
    pub notes_count: Int,
    pub is_sensitive: bool,
    pub allow_renote_to_external: bool,
    pub is_following: Option<bool>,
    pub is_favorited: Option<bool>,
    pub pinned_notes: Option<Vec<Box<Note>>>,
}
