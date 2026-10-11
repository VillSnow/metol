use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::common::Int;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SuspensionState {
    None,
    ManuallySuspended,
    GoneSuspended,
    AutoSuspendedForNotResponding,
    SoftwareSuspende,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FederationInstance {
    pub id: String,
    pub first_retrieved_at: DateTime<Utc>,
    pub host: String,
    pub users_count: Int,
    pub notes_count: Int,
    pub following_count: Int,
    pub followers_count: Int,
    pub is_not_responding: bool,
    pub is_suspended: bool,
    pub suspension_state: SuspensionState,
    pub is_blocked: bool,
    pub software_name: Option<String>,
    pub software_version: Option<String>,
    pub open_registrations: Option<bool>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub maintainer_name: Option<String>,
    pub maintainer_email: Option<String>,
    pub is_silenced: bool,
    pub is_media_silenced: Option<bool>,
    pub icon_url: Option<String>,
    pub favicon_url: Option<String>,
    pub theme_color: Option<String>,
    pub info_updated_at: Option<DateTime<Utc>>,
    pub latest_request_received_at: Option<DateTime<Utc>>,
    pub moderation_note: Option<String>,
}
