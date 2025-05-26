use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Achievements {
    pub name: String,

    /// UNIX時間 (ミリ秒)
    pub unlocked_at: i64,
}
