# ローカル Misskey 結合テスト環境

Docker が起動している状態で、リポジトリのルートから結合テストを実行します。

```powershell
cargo test --test misskey_meta -- --ignored --test-threads=1
```

各テストは Rust のテスト支援コードから専用の Compose project を起動します。DB・Redis・ファイル用ボリュームを初期化し、Misskey の DB 初期化、サービスの起動とヘルスチェックを行います。初回管理者の作成も API で自動化し、テスト終了時に環境を破棄します。ブラウザでの設定や手動のトークン発行は不要です。

テストは順番に実行してください。各テストに専用 DB を用意するため、テスト同士のデータや成否に依存しません。Misskey は `misskey/misskey:2026.10.0` を使います。
