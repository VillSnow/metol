use crate::{
    common::ChannelConnection,
    desc_types::MiChannel,
    mi_entities::{self, MetaDetailed, Note},
};

use serde::{de::DeserializeOwned, Serialize};
use std::collections::HashMap;

mod federation;
#[rustfmt::skip]
mod notes;
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

    async fn http_api<B: Serialize, R: DeserializeOwned>(
        &self,
        path: &str,
        body: &B,
    ) -> anyhow::Result<R> {
        let mut request = reqwest::Client::new()
            .post(format!("{}/api{}", self.http_url, path))
            .json(body);
        if let Some(token) = self.token.as_ref() {
            request = request.query(&[("i", token)]);
        }
        Ok(request.send().await?.error_for_status()?.json().await?)
    }

    async fn http_api_empty<B: Serialize>(&self, path: &str, body: &B) -> anyhow::Result<()> {
        let mut request = reqwest::Client::new()
            .post(format!("{}/api{}", self.http_url, path))
            .json(body);
        if let Some(token) = self.token.as_ref() {
            request = request.query(&[("i", token)]);
        }
        request.send().await?.error_for_status()?;
        Ok(())
    }
}
