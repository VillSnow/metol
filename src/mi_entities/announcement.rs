use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Announcement {
    pub id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub text: String,
    pub title: String,
    pub image_url: Option<String>,
    pub icon: Icon,
    pub display: Display,
    pub need_confirmation_to_read: bool,
    pub silence: bool,
    pub for_you: bool,
    pub is_read: Option<bool>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Icon {
    Info,
    Warning,
    Error,
    Success,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Display {
    Dialog,
    Normal,
    Banner,
}
