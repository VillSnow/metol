use anyhow::{bail, Context};
use chrono::{DateTime, Utc};
use serde::Deserialize;

use crate::Client;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MisskeyMergeBaseResult {
    pub merge_base_date: DateTime<Utc>,
    pub ahead_by: i32,
    pub behind_by: i32,
}

pub async fn misskey_merge_base(host: &str) -> anyhow::Result<MisskeyMergeBaseResult> {
    let client = Client::new(host.to_owned(), None);
    let meta = client.meta().await?;
    dbg!(&meta);
    let repo = meta.lite.repository_url.context("empty repository_url")?;

    let repo_init = "https://github.com/";
    if repo.find(repo_init) != Some(0) {
        bail!("repository_url is not github")
    };
    let repo = &repo[repo_init.len()..];
    dbg!(&repo);
    let mut split = repo.split("/").filter(|s| !s.is_empty());
    let user = split.next().context("unexpected url format")?;
    let repo = split.next().context("unexpected url format")?;

    let repo = repo.trim_end_matches("/");

    let url = format!(
        "https://api.github.com/repos/{user}/{repo}/compare/misskey-dev:develop...{user}:{ver}",
        user = user,
        repo = repo,
        ver = meta.lite.version
    );
    dbg!(&url);

    #[derive(Debug, Deserialize)]
    struct CompareResult {
        merge_base_commit: CompareResultMergeBaseCommit,
        ahead_by: i32,
        behind_by: i32,
    }
    #[derive(Debug, Deserialize)]
    struct CompareResultMergeBaseCommit {
        commit: CompareResultMergeBaseCommitCommit,
    }
    #[derive(Debug, Deserialize)]
    struct CompareResultMergeBaseCommitCommit {
        committer: CompareResultMergeBaseCommitCommitCommitter,
    }
    #[derive(Debug, Deserialize)]
    struct CompareResultMergeBaseCommitCommitCommitter {
        date: DateTime<Utc>,
    }

    let res: CompareResult = reqwest::Client::new()
        .get(url)
        .header("user-agent", "reqwest")
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;

    Ok(MisskeyMergeBaseResult {
        merge_base_date: res.merge_base_commit.commit.committer.date,
        ahead_by: res.ahead_by,
        behind_by: res.behind_by,
    })
}
