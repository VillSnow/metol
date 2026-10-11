mod common;

use metol::{
    mi_api_types::notes::{create, show},
    Client,
};

#[tokio::test]
#[ignore = "Docker 上の Misskey が必要です。結合テスト用コマンドで実行してください。"]
async fn creates_and_shows_note() {
    let environment = common::MisskeyTestEnvironment::start()
        .await
        .expect("テスト専用 Misskey 環境を起動できること");

    let client = Client::with_endpoints(
        environment.http_url.clone(),
        environment.ws_url.clone(),
        Some(environment.admin_token.clone()),
    );
    let text = format!("metol integration test {}", std::process::id());

    let created = client
        .create_note(create::Request {
            text: Some(text.clone()),
            ..Default::default()
        })
        .await
        .expect("ノートを作成できること");

    assert_eq!(created.created_note.text.as_deref(), Some(text.as_str()));

    let shown = client
        .show_note(show::Request {
            note_id: created.created_note.id.clone(),
        })
        .await
        .expect("作成したノートを取得できること");

    assert_eq!(shown.id, created.created_note.id);
    assert_eq!(shown.text.as_deref(), Some(text.as_str()));
}
