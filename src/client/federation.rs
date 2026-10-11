use crate::mi_entities;
use serde::Serialize;

#[derive(Debug, Default, Clone)]
pub struct FederationInstancesParams {
    pub offset: Option<usize>,
    pub limit: Option<usize>,
}

impl super::Client {
    pub async fn federation_instances(
        &self,
        params: FederationInstancesParams,
    ) -> anyhow::Result<Vec<mi_entities::FederationInstance>> {
        let n = 30;
        let mut offset = params.offset.unwrap_or(0);
        let mut limit = params.limit;

        let mut result = Vec::new();
        loop {
            let buf = self
                .federation_instances_raw(FederationInstancesParams {
                    offset: Some(offset),
                    limit: limit.map(|x| x.min(n)),
                    ..params
                })
                .await?;

            let m = buf.len();
            result.extend(buf.into_iter());

            if m < n {
                break;
            }

            offset += m;
            limit = limit.map(|x| x - m);

            if limit == Some(0) {
                break;
            }
        }

        Ok(result)
    }

    pub async fn federation_instances_raw(
        &self,
        params: FederationInstancesParams,
    ) -> anyhow::Result<Vec<mi_entities::FederationInstance>> {
        #[derive(Debug, Serialize)]
        #[serde(rename_all = "camelCase")]
        struct ReqBody<'a> {
            #[serde(default, skip_serializing_if = "Option::is_none")]
            offset: Option<usize>,

            #[serde(default, skip_serializing_if = "Option::is_none")]
            limit: Option<usize>,

            #[serde(default, skip_serializing_if = "Option::is_none")]
            i: Option<&'a str>,
        }

        let instances = reqwest::Client::new()
            .post(format!("{}/api/federation/instances", &self.http_url))
            .json(&ReqBody {
                offset: params.offset,
                limit: params.limit,
                i: self.token.as_deref(),
            })
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        Ok(instances)
    }
}
