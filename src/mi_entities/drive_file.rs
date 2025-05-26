use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::common::Int;

use super::UserLite;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DriveFile {
    pub id: String,
    pub created_at: DateTime<Utc>,
    pub name: String,
    pub r#type: String,
    pub md5: String,
    pub size: f64,
    pub is_sensitive: bool,
    pub blurhash: Option<String>,
    pub properties: Box<Properties>,
    pub url: String,
    pub thumbnail_url: Option<String>,
    pub comment: Option<String>,
    pub folder_id: Option<String>,
    pub folder: Option<DriveFolder>,
    pub user_id: Option<String>,
    pub user: Option<Box<UserLite>>,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DriveFolder {
    pub id: String,
    pub created_at: DateTime<Utc>,
    pub name: String,

    #[serde(default, deserialize_with = "Option::deserialize")]
    pub parent_id: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub folders_count: Option<Int>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub files_count: Option<Int>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent: Option<Box<DriveFolder>>,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub width: Option<f64>, //FIXME

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub height: Option<f64>, //FIXME

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub orientation: Option<f64>, //FIXME

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avg_color: Option<String>,
}
