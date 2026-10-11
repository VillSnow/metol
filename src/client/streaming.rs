use crate::{
    common::ChannelConnection,
    desc_types::MiChannel,
    mi_entities::Note,
    receiving_ws_msg::{ReceivingWsMsg, WsMsgChannelBody},
};
use futures::{SinkExt as _, StreamExt as _};
use serde::Serialize;
use std::{cmp::Reverse, collections::HashMap, fmt::Display};
use tokio_tungstenite::{connect_async, tungstenite::Message};
use uuid::Uuid;

#[derive(Debug)]
pub enum RecvError {
    NotReady,
    Disconnected,
    Misc(anyhow::Error),
}

impl Display for RecvError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RecvError::NotReady => f.write_str("NotReady"),
            RecvError::Disconnected => f.write_str("Disconnected"),
            RecvError::Misc(error) => Display::fmt(error, f),
        }
    }
}

impl std::error::Error for RecvError {}

impl From<anyhow::Error> for RecvError {
    fn from(error: anyhow::Error) -> Self {
        RecvError::Misc(error)
    }
}

#[derive(Debug, Default, Clone)]
pub struct ReadTlParams {
    pub offset: Option<usize>,
    pub limit: Option<usize>,
}

impl super::Client {
    pub async fn read_home(&self, params: ReadTlParams) -> anyhow::Result<Vec<Note>> {
        self.read_tl_core(&MiChannel::Home, params).await
    }

    pub async fn read_local(&self, params: ReadTlParams) -> anyhow::Result<Vec<Note>> {
        self.read_tl_core(&MiChannel::Local, params).await
    }

    pub async fn read_hybrid(&self, params: ReadTlParams) -> anyhow::Result<Vec<Note>> {
        self.read_tl_core(&MiChannel::Hybrid, params).await
    }

    pub async fn read_global(&self, params: ReadTlParams) -> anyhow::Result<Vec<Note>> {
        self.read_tl_core(&MiChannel::Global, params).await
    }

    async fn read_tl_core(
        &self,
        tl: &MiChannel,
        params: ReadTlParams,
    ) -> anyhow::Result<Vec<Note>> {
        #[derive(Debug, Default, Serialize)]
        #[serde(rename_all = "camelCase")]
        struct ReqBody<'a> {
            #[serde(default, skip_serializing_if = "Option::is_none")]
            limit: Option<usize>,

            #[serde(default, skip_serializing_if = "Option::is_none")]
            offset: Option<usize>,

            #[serde(default, skip_serializing_if = "Option::is_none")]
            channel_id: Option<String>,

            #[serde(default, skip_serializing_if = "Option::is_none")]
            antenna_id: Option<String>,

            #[serde(default, skip_serializing_if = "Option::is_none")]
            i: Option<&'a str>,
        }

        let endpoint;
        let body;
        match tl {
            MiChannel::Main => todo!(),
            MiChannel::Home => {
                endpoint = format!("{}/api/notes/timeline", &self.http_url);
                body = ReqBody {
                    limit: params.limit,
                    offset: params.offset,
                    i: self.token.as_deref(),
                    ..Default::default()
                };
            }
            MiChannel::Local => {
                endpoint = format!("{}/api/notes/local-timeline", &self.http_url);
                body = ReqBody {
                    limit: params.limit,
                    offset: params.offset,
                    i: self.token.as_deref(),
                    ..Default::default()
                };
            }
            MiChannel::Hybrid => {
                endpoint = format!("{}/api/notes/hybrid-timeline", &self.http_url);
                body = ReqBody {
                    limit: params.limit,
                    offset: params.offset,
                    i: self.token.as_deref(),
                    ..Default::default()
                };
            }
            MiChannel::Global => {
                endpoint = format!("{}/api/notes/global-timeline", &self.http_url);
                body = ReqBody {
                    limit: params.limit,
                    offset: params.offset,
                    i: self.token.as_deref(),
                    ..Default::default()
                };
            }
            MiChannel::Channel { channel_id } => {
                endpoint = format!("{}/api/channels/timeline", &self.http_url);
                body = ReqBody {
                    channel_id: Some(channel_id.to_owned()),
                    limit: params.limit,
                    offset: params.offset,
                    i: self.token.as_deref(),
                    ..Default::default()
                };
            }
            MiChannel::Antenna { antenna_id } => {
                endpoint = format!("{}/api/antennas/notes", &self.http_url);
                body = ReqBody {
                    antenna_id: Some(antenna_id.to_owned()),
                    limit: params.limit,
                    offset: params.offset,
                    i: self.token.as_deref(),
                    ..Default::default()
                };
            }
        }

        let notes: Vec<Note> = reqwest::Client::new()
            .post(endpoint)
            .json(&body)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        Ok(notes)
    }

    // pub async fn connect_to_main(&mut self) -> anyhow::Result<ChannelConnection> {
    //     self.connect_to_tl(MiChannel::Main).await
    // }

    pub async fn connect_to_home(&mut self) -> anyhow::Result<ChannelConnection> {
        self.connect_to_tl(MiChannel::Home).await
    }

    pub async fn connect_to_local(&mut self) -> anyhow::Result<ChannelConnection> {
        self.connect_to_tl(MiChannel::Local).await
    }

    pub async fn connect_to_hybrid(&mut self) -> anyhow::Result<ChannelConnection> {
        self.connect_to_tl(MiChannel::Hybrid).await
    }

    pub async fn connect_to_global(&mut self) -> anyhow::Result<ChannelConnection> {
        self.connect_to_tl(MiChannel::Global).await
    }

    pub async fn connect_to_channel(
        &mut self,
        channel_id: &str,
    ) -> anyhow::Result<ChannelConnection> {
        self.connect_to_tl(MiChannel::Channel {
            channel_id: channel_id.to_owned(),
        })
        .await
    }

    pub async fn connect_to_antenna(
        &mut self,
        antenna_id: &str,
    ) -> anyhow::Result<ChannelConnection> {
        self.connect_to_tl(MiChannel::Antenna {
            antenna_id: antenna_id.to_owned(),
        })
        .await
    }

    pub async fn connect_to_tl(&mut self, tl: MiChannel) -> anyhow::Result<ChannelConnection> {
        let cxn = ChannelConnection::new(Uuid::new_v4().to_string());
        self.connect_to_tl_core(cxn.clone(), tl).await?;

        Ok(cxn)
    }

    async fn connect_to_tl_core(
        &mut self,
        cxn: ChannelConnection,
        tl: MiChannel,
    ) -> anyhow::Result<()> {
        self.ensure_is_connected_to_ws().await?;
        let ws = self.ws.as_mut().unwrap();

        ws.send(Message::text(tl_desc_to_ws_cxn_msg(&cxn, &tl)))
            .await?;
        self.connections.insert(cxn, tl);
        Ok(())
    }

    fn stream_url(&self) -> String {
        if let Some(token) = self.token.as_ref() {
            format!("{}/stream?i={}", &self.ws_url, &token)
        } else {
            format!("{}/stream", &self.ws_url)
        }
    }

    #[tracing::instrument(skip_all, ret, err)]
    async fn reconnect(&mut self) -> anyhow::Result<()> {
        self.ensure_is_disconnected_from_ws().await;
        self.ensure_is_connected_to_ws().await?;

        for (cxn, tl) in self.connections.clone() {
            tracing::info!(?cxn, ?tl);
            self.connect_to_tl_core(cxn.clone(), tl.clone()).await?;
        }
        Ok(())
    }

    pub async fn recv(&mut self) -> Result<ReceivingWsMsg, RecvError> {
        // 通常はWSから受信したメッセージを順次返却する。
        // WSが切断された場合、以下の動作を行う。
        // 1. WSに再接続し、チャンネルに再接続する。
        // 2. チャンネルから直近のノートを取得する。
        // 3. 直近のノートのうち、これまで返却したことのないノートを順次返却する。
        // 4. すべてのノートを返却し終えたら通常の動作に戻る。

        RecvLogic(self).recv().await
    }

    pub async fn disconnect(&mut self, cxn: ChannelConnection) {
        let ws = self.ws.as_mut().unwrap();
        _ = ws
            .send(Message::text(
                serde_json::json!({
                    "type": "disconnect",
                    "body": { "id": cxn.value() }
                })
                .to_string(),
            ))
            .await
            .inspect_err(|err| {
                tracing::warn!(
                    error=?err,
                    cxn_id=?cxn,
                    "error while disconnecting channel; ignoring",
                )
            });

        self.connections.remove(&cxn);

        if self.connections.is_empty() {
            self.ensure_is_disconnected_from_ws().await;
        }
    }

    pub async fn disconnect_all(&mut self) {
        for cxn in self.connections.keys().cloned().collect::<Vec<_>>() {
            self.disconnect(cxn).await;
        }
    }

    pub async fn ensure_is_connected_to_ws(&mut self) -> anyhow::Result<()> {
        if self.ws.is_none() {
            let (ws, _res) = connect_async(self.stream_url()).await?;
            self.ws = Some(ws);
        }

        Ok(())
    }

    pub async fn ensure_is_disconnected_from_ws(&mut self) {
        if let Some(mut ws) = self.ws.take() {
            _ = ws.close(None).await.inspect_err(|err| {
                tracing::warn!(
                    error=?err,
                    "error while disconnecting WebSocket; ignoring",
                )
            });
        }
    }
}

enum RecvLogicMode {
    Normal,
    Exhausting,
}

struct RecvLogic<'a>(&'a mut super::Client);

impl<'a> RecvLogic<'a> {
    pub async fn recv(&mut self) -> Result<ReceivingWsMsg, RecvError> {
        // 通常はWSから受信したメッセージを順次返却する。
        // WSが切断された場合、以下の動作を行う。
        // 1. WSに再接続し、チャンネルに再接続する。
        // 2. チャンネルから直近のノートを取得する。
        // 3. 直近のノートのうち、これまで返却したことのないノートを順次返却する。
        // 4. すべてのノートを返却し終えたら通常の動作に戻る。

        loop {
            let result = self.recv_without_guard().await;
            match &result {
                Ok(ReceivingWsMsg::Channel(WsMsgChannelBody::Note { id, body })) => {
                    let cxn = ChannelConnection::new(id.to_owned());
                    if let Some(latest) = self.0.latest_reception.get(&cxn) {
                        // misskey のノートIDは辞書ソート可能。
                        if body.id <= latest.id {
                            // スキップ
                            continue;
                        }
                    }
                    self.0.latest_reception.insert(cxn, body.clone());
                }
                _ => {}
            }
            return result;
        }
    }

    async fn recv_without_guard(&mut self) -> Result<ReceivingWsMsg, RecvError> {
        // この関数の戻り値は返却済みのノートが含まれる可能性がある。ゆえに without_guard。

        let mut latest_loop_err = None::<RecvError>;
        loop {
            match latest_loop_err.take() {
                None => {
                    self.0.retry_count = 0;

                    if let Some(result) = self.recv_from_pool() {
                        return Ok(result);
                    }

                    let msg = self.recv_from_ws().await;

                    match self.recv_from_ws().await {
                        result @ Ok(_) => return result,
                        Err(err) => {
                            tracing::warn!(
                                error=?err,
                                "Error while receiving from WebSocket; it may be ignored and retried:",
                            );
                            latest_loop_err = Some(err)
                        }
                    }
                }
                Some(err) => {
                    self.0.ensure_is_disconnected_from_ws().await;

                    if self.0.retry_count >= self.0.max_retry {
                        return Err(err);
                    }
                    drop(err);

                    tracing::info!(retry_count = self.0.retry_count);

                    if self.0.retry_count > 0 {
                        tokio::time::sleep(tokio::time::Duration::from_secs_f64(
                            2.0f64.powi(self.0.retry_count - 1),
                        ))
                        .await;
                    }

                    self.0.retry_count += 1;

                    if let Err(err) = self.0.reconnect().await {
                        tracing::warn!(
                            error=?err,
                            "error while reconnecting; it may be ignored and retried",
                        );
                        latest_loop_err = Some(err.into());
                        continue;
                    }

                    if self.0.ws.is_none() {
                        return Err(RecvError::NotReady);
                    };

                    if let Err(err) = self.acquire_pool().await {
                        tracing::warn!(
                            error=?err,
                            "error while acquiring pool; it may be ignored and retried",
                        );
                        latest_loop_err = Some(err.into());
                        continue;
                    }
                }
            }
        }
    }

    #[tracing::instrument(skip_all, ret, err)]
    async fn recv_from_ws(&mut self) -> Result<ReceivingWsMsg, RecvError> {
        self.0.ensure_is_connected_to_ws().await?;
        let ws = self.0.ws.as_mut().unwrap();

        loop {
            match ws.next().await {
                Some(Ok(msg)) => match msg {
                    Message::Text(text) => match serde_json::from_slice(text.as_bytes()) {
                        Ok(msg) => return Ok(msg),
                        Err(err) => {
                            tracing::warn!(
                                ?text,
                                error = ?err,
                                "Unknown text message:"
                            );
                        }
                    },
                    Message::Ping(_) => {
                        _ = ws
                            .send(Message::Pong(Default::default()))
                            .await
                            .inspect_err(|err| tracing::warn!(error=?err, "Error while disconnecting channel; ignoring:"));
                    }
                    Message::Pong(_) => {}
                    Message::Close(_) => {
                        tracing::info!("closing");
                        _ = ws.close(None).await;
                        return Err(RecvError::Disconnected);
                    }
                    other => tracing::warn!("unknown message: {other}"),
                },
                Some(Err(err)) => match err {
                    tokio_tungstenite::tungstenite::Error::ConnectionClosed => {
                        return Err(RecvError::Disconnected);
                    }
                    err => return Err(RecvError::Misc(err.into())),
                },
                None => return Err(RecvError::Disconnected),
            }
        }
    }

    fn recv_from_pool(&mut self) -> Option<ReceivingWsMsg> {
        if let Some((cxn, note)) = self.0.note_pool.pop() {
            tracing::info!("recv_from_pool exhausting");

            Some(ReceivingWsMsg::Channel(WsMsgChannelBody::Note {
                id: cxn.value().to_owned(),
                body: note,
            }))
        } else {
            tracing::info!("recv_from_pool exhausted");

            None
        }
    }

    async fn acquire_pool(&mut self) -> Result<(), anyhow::Error> {
        let mut cxn_groups = HashMap::<_, Vec<_>>::new();
        for (cxn, tl) in &self.0.connections {
            cxn_groups.entry(tl).or_default().push(cxn.clone());
        }

        for (ch, cxn_list) in cxn_groups {
            self.0.note_pool.extend(
                self.0
                    .read_tl_core(
                        ch,
                        ReadTlParams {
                            offset: None,
                            limit: Some(100),
                            ..Default::default()
                        },
                    )
                    .await?
                    .into_iter()
                    .flat_map(|note| cxn_list.iter().map(move |cxn| (cxn.clone(), note.clone()))),
            );
        }

        self.0
            .note_pool
            .sort_unstable_by_key(|(_, note)| Reverse(note.created_at.clone()));

        Ok(())
    }
}

fn tl_desc_to_ws_cxn_msg(cxn_id: &ChannelConnection, tl: &MiChannel) -> String {
    let cxn_id = cxn_id.value();
    match &tl {
        MiChannel::Main => serde_json::json!({
            "type": "connect",
            "body": {
                "id": cxn_id,
                "channel": "main",
                "params": {}
            }
        }),
        MiChannel::Home => serde_json::json!({
            "type": "connect",
            "body": {
                "id": cxn_id,
                "channel": "homeTimeline",
                "params": {}
            }
        }),
        MiChannel::Local => serde_json::json!({
            "type": "connect",
            "body": {
                "id": cxn_id,
                "channel": "localTimeline",
                "params": {}
            }
        }),
        MiChannel::Hybrid => serde_json::json!({
            "type": "connect",
            "body": {
                "id": cxn_id,
                "channel": "hybridTimeline",
                "params": {}
            }
        }),
        MiChannel::Global => serde_json::json!({
            "type": "connect",
            "body": {
                "id": cxn_id,
                "channel": "globalTimeline",
                "params": {}
            }
        }),
        MiChannel::Channel { channel_id } => serde_json::json!({
            "type": "connect",
            "body": {
                "id":  cxn_id,
                "channel": "channel",
                "params": {
                    "channelId": channel_id
                }
            }
        }),
        MiChannel::Antenna { antenna_id } => serde_json::json!({
            "type": "connect",
            "body": {
                "id": cxn_id,
                "channel": "antenna",
                "params": {
                    "antennaId": antenna_id
                }
            }
        }),
    }
    .to_string()
}
