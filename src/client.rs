use serde::Serialize;

use crate::{common::Int, mi_entities::Note};

pub struct Client {
    host: String,
    token: Option<String>,
}

impl Client {
    pub fn new(host: impl Into<String>, token: Option<impl Into<String>>) -> Self {
        Self {
            host: host.into(),
            token: token.map(|x| x.into()),
        }
    }

    pub fn create_note(&self, text: String) {
        todo!()
    }

    pub async fn create_reaction(&self, node_id: &str, reaction: &str) -> anyhow::Result<()> {
        let res = reqwest::Client::new()
            .post(format!("https://{}/api/notes/reactions/create", &self.host))
            .json(&serde_json::json!({"noteId": node_id,"reaction": reaction,"i": self.token.as_ref().unwrap()}))
            .send()
            .await?;
        dbg!("{res:#?}");
        res.error_for_status()?;
        Ok(())
    }

    pub async fn conversation(&self, note_id: &str) -> anyhow::Result<Vec<Note>> {
        #[derive(Debug, Serialize)]
        #[serde(rename_all = "camelCase")]
        struct ReqBody<'a> {
            note_id: &'a str,

            #[serde(default, skip_serializing_if = "Option::is_none")]
            limit: Option<Int>,

            #[serde(default, skip_serializing_if = "Option::is_none")]
            offset: Option<Int>,

            #[serde(default, skip_serializing_if = "Option::is_none")]
            i: Option<&'a str>,
        }
        let c = reqwest::Client::new();
        let notes: Vec<Note> = c
            .post(format!("https://{}/api/notes/conversation", &self.host))
            .json(&ReqBody {
                note_id,
                limit: None,
                offset: None,
                i: self.token.as_deref(),
            })
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        Ok(notes)
    }

    pub async fn read_ltl(&self) -> anyhow::Result<Vec<Note>> {
        #[derive(Debug, Serialize)]
        #[serde(rename_all = "camelCase")]
        struct ReqBody<'a> {
            #[serde(default, skip_serializing_if = "Option::is_none")]
            limit: Option<Int>,

            #[serde(default, skip_serializing_if = "Option::is_none")]
            offset: Option<Int>,

            #[serde(default, skip_serializing_if = "Option::is_none")]
            i: Option<&'a str>,
        }
        let c = reqwest::Client::new();
        let notes: Vec<Note> = c
            .post(format!("https://{}/api/notes/local-timeline", &self.host))
            .json(&ReqBody {
                limit: None,
                offset: Some(100),
                i: self.token.as_deref(),
            })
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        Ok(notes)
    }
}
