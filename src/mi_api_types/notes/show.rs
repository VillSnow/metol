use serde::{Deserialize, Serialize};

use crate::mi_entities::Note;

/// Request body for `POST /notes/show`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub note_id: String,
}

/// Response body for `POST /notes/show`.
pub type Response = Note;
