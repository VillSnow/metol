use serde::{Deserialize, Serialize};

use super::{DriveFile, UserLite};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Page {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "user")]
    pub user: UserLite,
    #[serde(rename = "content")]
    pub content: Vec<serde_json::Value>, // FIXME
    #[serde(rename = "variables")]
    pub variables: Vec<serde_json::Value>,
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "summary", deserialize_with = "Option::deserialize")]
    pub summary: Option<String>,
    #[serde(rename = "hideTitleWhenPinned")]
    pub hide_title_when_pinned: bool,
    #[serde(rename = "alignCenter")]
    pub align_center: bool,
    #[serde(rename = "font")]
    pub font: String,
    #[serde(rename = "script")]
    pub script: String,
    #[serde(
        rename = "eyeCatchingImageId",
        deserialize_with = "Option::deserialize"
    )]
    pub eye_catching_image_id: Option<String>,
    #[serde(rename = "eyeCatchingImage")]
    pub eye_catching_image: Box<DriveFile>,
    #[serde(rename = "attachedFiles")]
    pub attached_files: Vec<DriveFile>,
    #[serde(rename = "likedCount")]
    pub liked_count: f64,
    #[serde(rename = "isLiked", skip_serializing_if = "Option::is_none")]
    pub is_liked: Option<bool>,
}
