use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoleLite {
    pub id: String,
    pub name: String,
    pub color: Option<String>,
    pub icon_url: Option<String>,
    pub description: String,
    pub is_moderator: bool,
    pub is_administrator: bool,
    pub display_order: i32,
}
