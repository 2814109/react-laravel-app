# react-laravel-app

## 環境構築

### STEP1：コンテナ起動

```
docker compose up -d --build
```

### STEP2：環境変数定義

src/.env を docker-compose の DB サービスで定義している環境変数と対応するように書き換え

### STEP3：DB マイグレーション

app コンテナに侵入し以下コマンドを実行

```
php artisan migrate
```
