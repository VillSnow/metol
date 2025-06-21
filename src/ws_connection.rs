use anyhow::bail;
use futures::{SinkExt, StreamExt};
use serde_json::json;
use tokio_tungstenite::{connect_async, tungstenite::Message};
use tracing::{info, warn};
use uuid::Uuid;

use crate::{receiving_ws_msg, sending_ws_msg};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MiChannel {
    Global,
    Home,
    Hybrid,
    Local,
    Main,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ChannelConnection {
    pub id: String,
}

pub enum ConnectionError {
    Disconnected,
}

pub struct WsConnection {
    ws: tokio_tungstenite::WebSocketStream<
        tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>,
    >,
}

impl WsConnection {
    pub async fn new(
        host: impl AsRef<str>,
        api_key: Option<impl AsRef<str>>,
    ) -> anyhow::Result<WsConnection> {
        let req;
        if let Some(api_key) = api_key {
            req = format!("wss://{}/stream?i={}", host.as_ref(), api_key.as_ref());
        } else {
            req = format!("wss://{}/stream", host.as_ref());
        }
        let (ws, _res) = connect_async(req).await?;

        Ok(WsConnection { ws })
    }

    #[tracing::instrument(skip(self))]
    pub async fn connect_to_channel(
        &mut self,
        channel: MiChannel,
    ) -> anyhow::Result<ChannelConnection> {
        let id = Uuid::new_v4().to_string();

        let msg = match channel {
            MiChannel::Global => json!({
                "type": "connect",
                "body": {
                    "id": id.clone(),
                    "channel": "globalTimeline",
                    "params": {}
                }
            }),
            MiChannel::Home => json!({
                "type": "connect",
                "body": {
                    "id": id.clone(),
                    "channel": "homeTimeline",
                    "params": {}
                }
            }),
            MiChannel::Hybrid => json!({
                "type": "connect",
                "body": {
                    "id": id.clone(),
                    "channel": "hybridTimeline",
                    "params": {}
                }
            }),
            MiChannel::Local => json!({
                "type": "connect",
                "body": {
                    "id": id.clone(),
                    "channel": "localTimeline",
                    "params": {}
                }
            }),
            MiChannel::Main => json!({
                "type": "connect",
                "body": {
                    "id": id.clone(),
                    "channel": "main",
                    "params": {}
                }
            }),
        };

        self.ws.send(Message::text(msg.to_string())).await?;

        Ok(ChannelConnection { id })
    }

    #[tracing::instrument(skip(self))]
    pub async fn receive(
        &mut self,
    ) -> anyhow::Result<Result<receiving_ws_msg::ReceivingWsMsg, ConnectionError>> {
        loop {
            match self.ws.next().await {
                Some(Ok(msg)) => match msg {
                    Message::Text(text) => match serde_json::from_slice(text.as_bytes()) {
                        Ok(msg) => {
                            return Ok(Ok(msg));
                        }
                        Err(err) => {
                            warn!("unknown text message: {text}, {err:?}");
                        }
                    },
                    Message::Ping(_) => self.send_pong().await?,
                    Message::Pong(_) => {}
                    Message::Close(_) => {
                        info!("closing");
                        _ = self.send_close().await;
                    }
                    other => warn!("unknown message: {other}"),
                },
                Some(Err(err)) => bail!(err),
                None => {
                    return Ok(Err(ConnectionError::Disconnected));
                }
            }
        }
    }

    pub async fn send(
        &mut self,
        msg: sending_ws_msg::SendingWsMsg,
    ) -> anyhow::Result<Result<(), ConnectionError>> {
        let r = self
            .ws
            .send(Message::text(serde_json::to_string(&msg)?))
            .await;

        use tokio_tungstenite::tungstenite::Error;
        match r {
            Ok(()) => Ok(Ok(())),
            Err(err) => match err {
                Error::ConnectionClosed | Error::AlreadyClosed => {
                    Ok(Err(ConnectionError::Disconnected))
                }
                _ => Err(err)?,
            },
        }
    }

    async fn send_pong(&mut self) -> anyhow::Result<()> {
        self.ws.send(Message::Pong(Default::default())).await?;
        Ok(())
    }

    async fn send_close(&mut self) -> anyhow::Result<()> {
        self.ws.send(Message::Close(None)).await?;
        Ok(())
    }
}
