use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "body")]
#[non_exhaustive]
pub enum SendingWsMsg {
    #[serde(rename = "subNote")]
    SubNote(WsMsgSubNoteBody),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WsMsgSubNoteBody {
    pub id: String,
}

impl SendingWsMsg {
    pub fn sub_note(id: String) -> SendingWsMsg {
        SendingWsMsg::SubNote(WsMsgSubNoteBody { id })
    }
}
