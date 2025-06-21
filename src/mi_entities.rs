use serde::{Deserialize, Serialize};

pub mod achievement;
pub mod announcement;
pub mod channel;
pub mod drive_file;
pub mod field;
pub mod meta;
pub mod note;
pub mod page;
pub mod role_lite;
pub mod roll_policies;
pub mod user;

pub use achievement::Achievements;
pub use announcement::Announcement;
pub use channel::Channel;
pub use drive_file::DriveFile;
pub use meta::{MetaDetailed, MetaLite};
pub use note::Note;
pub use page::Page;
pub use role_lite::RoleLite;
pub use roll_policies::RolePolicies;
pub use user::{UserDetailedNotMe, UserLite};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct User {
    pub id: String,

    pub username: String,

    pub host: Option<String>,

    pub name: Option<String>,

    #[serde(rename = "onlineStatus")]
    pub online_status: OnlineStatus,

    #[serde(rename = "avatarUrl")]
    pub avatar_url: String,

    #[serde(rename = "avatarBlurhash")]
    pub avatar_blurhash: Option<String>,

    // emojis: Vec<Emoji>,
    pub instance: Option<UserInstance>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum OnlineStatus {
    #[serde(rename = "online")]
    Online,

    #[serde(rename = "active")]
    Active,

    #[serde(rename = "offline")]
    Offline,

    #[serde(rename = "unknown")]
    Unknown,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct UserInstance {
    name: Option<String>,

    #[serde(rename = "softwareName")]
    software_name: Option<String>,

    #[serde(rename = "softwareVersion")]
    software_version: Option<String>,

    #[serde(rename = "iconUrl")]
    icon_url: Option<String>,

    #[serde(rename = "faviconUrl")]
    favicon_url: Option<String>,

    #[serde(rename = "themeColor")]
    theme_color: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Visibility {
    #[serde(rename = "public")]
    Public,

    #[serde(rename = "home")]
    Home,

    #[serde(rename = "followers")]
    Followers,

    #[serde(rename = "specified")]
    Specified,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Poll {
    #[serde(rename = "expiresAt")]
    pub expires_at: Option<String>,

    pub multiple: bool,

    pub choices: Vec<PollChoice>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct PollChoice {
    #[serde(rename = "isVoted")]
    pub is_voted: bool,

    pub text: String,

    pub votes: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct EmojiSimple {
    pub name: String,
    pub category: Option<String>,
    pub url: String,
    pub aliases: Vec<String>,
    pub local_only: Option<bool>,
    pub is_sensitive: Option<bool>,
    pub role_ids_that_can_be_used_this_emoji_as_reaction: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Emoji {
    pub id: String,

    pub aliases: Vec<String>,

    pub name: String,

    pub category: Option<String>,

    pub host: Option<String>,

    pub url: String,

    pub license: Option<String>,

    #[serde(rename = "isSensitive")]
    pub is_sensitive: bool,

    #[serde(rename = "localOnly")]
    pub local_only: bool,

    #[serde(rename = "roleIdsThatCanBeUsedThisEmojiAsReaction")]
    pub role_ids_that_can_be_used_this_emoji_as_reaction: Vec<String>,
}
