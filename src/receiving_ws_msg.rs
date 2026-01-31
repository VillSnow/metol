use serde::{Deserialize, Serialize};

use crate::mi_entities::Note;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "body")]
#[non_exhaustive]
pub enum ReceivingWsMsg {
    #[serde(rename = "channel")]
    Channel(WsMsgChannelBody),
    #[serde(rename = "noteUpdated")]
    NoteUpdated(NoteUpdatedBody),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum WsMsgChannelBody {
    #[serde(rename = "note")]
    Note { id: String, body: Note },

    #[serde(rename = "mention")]
    Mention { id: String, body: Note },

    #[serde(rename = "reply")]
    Reply { id: String, body: Note },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum NoteUpdatedBody {
    #[serde(rename = "reacted")]
    NoteUpdatedBodyReacted {
        id: String,
        body: NoteUpdatedBodyReactedBody,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoteUpdatedBodyReactedBody {
    pub emoji: Option<Emoji>,

    pub reaction: String,

    #[serde(rename = "userId")]
    pub user_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Emoji {
    name: String,
    url: String,
}
