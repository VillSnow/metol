use anyhow::{bail, ensure, Context, Result};
use serde::Deserialize;
use std::process::{Command, Output};

const SETUP_PASSWORD: &str = "metol-local-setup-password";

#[derive(Deserialize)]
struct BootstrapResponse {
    token: String,
}

pub struct MisskeyTestEnvironment {
    project_name: String,
    pub http_url: String,
    pub ws_url: String,
    pub admin_token: String,
}

impl MisskeyTestEnvironment {
    pub async fn start() -> Result<Self> {
        let project_name = format!("metol-it-{}-{}", std::process::id(), unique_id());
        let mut environment = Self {
            project_name,
            http_url: String::new(),
            ws_url: String::new(),
            admin_token: String::new(),
        };

        environment.compose_status(&["run", "--rm", "misskey", "pnpm", "run", "init"])?;
        environment.compose_status(&["up", "-d", "--wait"])?;

        let output = environment.compose_output(&["port", "misskey", "3000"])?;
        let mapping = String::from_utf8(output.stdout)
            .context("Misskey の公開ポート番号を読み取れません")?
            .trim()
            .to_owned();
        if mapping.is_empty() {
            bail!("Misskey の公開ポート番号を取得できませんでした");
        }

        environment.http_url = format!("http://{mapping}");
        environment.ws_url = format!("ws://{mapping}");
        environment.admin_token = environment.create_initial_admin().await?;

        Ok(environment)
    }

    async fn create_initial_admin(&self) -> Result<String> {
        let response = reqwest::Client::new()
            .post(format!("{}/api/admin/accounts/create", self.http_url))
            .json(&serde_json::json!({
                "username": "metol_it_admin",
                "password": "metol-local-test-password",
                "setupPassword": SETUP_PASSWORD,
            }))
            .send()
            .await
            .context("初期管理者の作成 API に接続できません")?;

        let status = response.status();
        let body = response
            .text()
            .await
            .context("初期管理者の作成 API の応答を読み取れません")?;
        ensure!(
            status.is_success(),
            "初期管理者を作成できません: HTTP {status}: {body}"
        );

        let bootstrap: BootstrapResponse =
            serde_json::from_str(&body).context("初期管理者の作成 API の応答を解析できません")?;
        Ok(bootstrap.token)
    }

    fn compose_status(&self, args: &[&str]) -> Result<()> {
        let status = self.compose_command(args).status()?;
        if !status.success() {
            let logs = self
                .compose_command(&["logs", "--no-color"])
                .output()
                .map(|output| {
                    format!(
                        "\nCompose ログ:\n{}{}",
                        String::from_utf8_lossy(&output.stdout),
                        String::from_utf8_lossy(&output.stderr)
                    )
                })
                .unwrap_or_else(|error| format!("\nCompose ログを取得できません: {error}"));
            bail!("docker compose {:?} が失敗しました: {status}{logs}", args);
        }
        Ok(())
    }

    fn compose_output(&self, args: &[&str]) -> Result<Output> {
        let output = self.compose_command(args).output()?;
        if !output.status.success() {
            bail!(
                "docker compose {:?} が失敗しました: {}",
                args,
                String::from_utf8_lossy(&output.stderr)
            );
        }
        Ok(output)
    }

    fn compose_command(&self, args: &[&str]) -> Command {
        let mut command = Command::new("docker");
        command
            .arg("compose")
            .arg("--project-name")
            .arg(&self.project_name)
            .args(args)
            .current_dir(env!("CARGO_MANIFEST_DIR"));
        command
    }
}

impl Drop for MisskeyTestEnvironment {
    fn drop(&mut self) {
        let status = self
            .compose_command(&["down", "--volumes", "--remove-orphans"])
            .status();
        if !matches!(status, Ok(status) if status.success()) {
            eprintln!(
                "Misskey テスト環境 {} の後片付けに失敗しました",
                self.project_name
            );
        }
    }
}

fn unique_id() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};

    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("システム時刻を取得できること")
        .as_nanos()
        .to_string()
}
