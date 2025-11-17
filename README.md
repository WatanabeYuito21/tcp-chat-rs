# TCP Chat

TCPベースのテキスト送受信プログラム

## Build

```bash
cargo build --release
```

## How to use

### サーバーの起動

Terminal1で以下実行

```bash
cargo run --bin server
```

サーバーは`127.0.0.1:8080`で起動します。

### クライアントの起動

Terminal2で以下実行

```bash
cargo run --bin client
```

クライアントが起動したら、メッセージを入力してEnterキーを押すとサーバーに送信される。

サーバーからのエコーバックが表示される。

終了するには`quit`を入力
