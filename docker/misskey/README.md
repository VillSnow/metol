# ローカル Misskey 結合テスト用バックエンド

リポジトリのルートで、次のコマンドを実行して起動します。

```sh
docker compose up -d
```

初回起動時は、Misskey のデータベースを初期化します。

```sh
docker compose run --rm misskey pnpm run init
```

次に <http://localhost:3000> を開いて、インスタンスの初期設定を完了してください。
API の接続先は `http://localhost:3000/api` です。

停止するには `docker compose down` を実行します。データベースとアップロードした
ファイルは名前付きボリュームに保持されます。

このテスト用インスタンスと保存データをすべて削除するには、
`docker compose down --volumes` を実行します。

既定のイメージは `misskey/misskey:2026.10.0` です。実行環境ごとに異なるタグを
使う場合は、リポジトリ直下の `.env` に `MISSKEY_IMAGE` を設定してください。
