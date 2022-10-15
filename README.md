# react-laravel-app

## 環境構築（所要時間：15 分）

### STEP1：コンテナ起動（5 分）

```
docker compose up -d --build
```

### STEP2：環境変数定義（5 分）

src/.env を docker-compose の DB サービスで定義している環境変数と対応するように書き換え

### STEP3：DB マイグレーション（5 分）

app コンテナに侵入し以下コマンドを実行

```
php artisan migrate
```
