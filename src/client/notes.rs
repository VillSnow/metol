use crate::mi_api_types::notes::{
    create::{Request as CreateNoteRequest, Response as CreateNoteResponse},
    show::{Request as ShowNoteRequest, Response as ShowNoteResponse},
};

impl super::Client {
    pub async fn create_note(
        &self,
        request: CreateNoteRequest,
    ) -> anyhow::Result<CreateNoteResponse> {
        let mut builder = reqwest::Client::new()
            .post(format!("{}/api/notes/create", &self.http_url))
            .json(&request);
        if let Some(i) = self.token.as_ref() {
            builder = builder.query(&("i", i));
        }
        let res = builder.send().await?;
        tracing::debug!(?res);

        let res = res.error_for_status()?.json().await?;
        Ok(res)
    }

    pub async fn show_note(&self, request: ShowNoteRequest) -> anyhow::Result<ShowNoteResponse> {
        let mut builder = reqwest::Client::new()
            .post(format!("{}/api/notes/show", &self.http_url))
            .json(&request);
        if let Some(i) = self.token.as_ref() {
            builder = builder.query(&("i", i));
        }
        let res = builder.send().await?;
        tracing::debug!(?res);

        let res = res.error_for_status()?.json().await?;
        Ok(res)
    }
}
