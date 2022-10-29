# react-laravel-app

## hot reload for cargo

```
cargo watch -x run
```

## 環境構築 backend of laravel（所要時間：15 分）

### STEP1：コンテナ起動（5 分）

```
docker compose up -d --build
```

### STEP2：環境変数定義（5 分）

src 配下に.env ファイルを作成

```
cd src
cp .env.example .env
```

src/.env を docker-compose の DB サービスで定義している環境変数と対応するように書き換え

### STEP3：DB マイグレーション（5 分）

dbのコンテナの名前を調べる。
以下コマンドを実行し、NAME列 appのコンテナ名を覚えておく。
```
docker compose ps
```

app コンテナに侵入しマイグレーションを実行

APPコンテナ侵入コマンド
```
docker exec -it (さっき覚えた名前)  bash
```

マイグレーションコマンド
```
php artisan migrate
```

新しいテーブル作成用のマイグレーションファイル作成コマンド
```
php artisan make:migration create_tests_table --create=tasks
```

migrationファイルにカラムを記入。
再度マイグレーションコマンドを実行。


## 環境構築　 backend of axum

```
cargo build

cargo run

```

上記で localhost:3333 に疎通

## 環境構築 frontend(所要時間 15 分)

### Node インストール

### Yarn インストール

### frontend のルートディレクトリで npm package をインストール

### 環境変数を定義

frontend 配下に.env ファイルを作成する

以下環境変数を定義

```
VITE_ENDPONT=http://localhost:{PORT_NUMBER}
```

## システム構成

### backend : Laravel(Php)

### Frontend : Vite for React(Typescript)

### RestAPI

## API 仕様書の書き方

ルートディレクトリで DockerComposeUp し、コンテナを生成  
http://localhost:8001/  
上記にアクセス
File タブの ImportFile を選択し、root/api/openapi.yaml を選択し Open  
編集を行い File タブの SaveAsYAML でファイルを保存
保存したファイルを api/配下に保存し、PR 作成
