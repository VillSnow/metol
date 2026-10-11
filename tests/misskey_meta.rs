mod common;

use metol::Client;

#[tokio::test]
#[ignore = "Docker 上の Misskey が必要です。結合テスト用コマンドで実行してください。"]
async fn retrieves_misskey_metadata() {
    let environment = common::MisskeyTestEnvironment::start()
        .await
        .expect("テスト専用 Misskey 環境を起動できること");

    let client = Client::with_endpoints(
        environment.http_url.clone(),
        environment.ws_url.clone(),
        Some(environment.admin_token.clone()),
    );
    let meta = client
        .meta()
        .await
        .expect("Misskey のメタデータを取得できること");

    assert!(
        !meta.lite.version.is_empty(),
        "Misskey のバージョンが返ること"
    );
    assert_eq!(
        meta.lite.uri.trim_end_matches('/'),
        "http://localhost:3000",
        "設定したローカルインスタンスの URI が返ること",
    );
}
