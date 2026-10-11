use crate::{
    common::{ChannelConnection, Int},
    desc_types::MiChannel,
    mi_entities::{self, MetaDetailed, Note},
};

use serde::Serialize;
use std::collections::HashMap;

mod federation;
mod streaming;

type WebSocketStream =
    tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>;

pub struct Client {
    http_url: String,
    ws_url: String,
    token: Option<String>,
    ws: Option<WebSocketStream>,
    max_retry: i32,
    connections: HashMap<ChannelConnection, MiChannel>,
    retry_count: i32,
    latest_reception: HashMap<ChannelConnection, mi_entities::Note>,
    note_pool: Vec<(ChannelConnection, Note)>,
}

impl Client {
    /// HTTPS と WSS を使う通常の Misskey インスタンス向けクライアントを作成します。
    pub fn new(host: String, token: Option<String>) -> Self {
        Self::with_endpoints(format!("https://{host}"), format!("wss://{host}"), token)
    }

    /// HTTP(S) と WS(S) の接続先を明示してクライアントを作成します。
    ///
    /// 接続先にはパスを含めず、サーバーの origin を指定してください。
    /// ローカル開発では `http://127.0.0.1:3000` と `ws://127.0.0.1:3000` のように指定します。
    pub fn with_endpoints(http_url: String, ws_url: String, token: Option<String>) -> Self {
        Self {
            http_url: http_url.trim_end_matches('/').to_owned(),
            ws_url: ws_url.trim_end_matches('/').to_owned(),
            token,
            ws: None,
            max_retry: 10,
            connections: Default::default(),
            retry_count: 0,
            latest_reception: Default::default(),
            note_pool: Default::default(),
        }
    }

    pub async fn meta(&self) -> anyhow::Result<MetaDetailed> {
        #[derive(Serialize)]
        struct ReqBody {
            detail: bool,
        }

        let meta = reqwest::Client::new()
            .post(format!("{}/api/meta", &self.http_url))
            .json(&ReqBody { detail: true })
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        Ok(meta)
    }

    pub fn create_note(&self, text: String) {
        todo!()
    }

    pub async fn create_reaction(&self, node_id: &str, reaction: &str) -> anyhow::Result<()> {
        let res = reqwest::Client::new()
            .post(format!("{}/api/notes/reactions/create", &self.http_url))
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
            .post(format!("{}/api/notes/conversation", &self.http_url))
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
            .post(format!("{}/api/notes/local-timeline", &self.http_url))
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
