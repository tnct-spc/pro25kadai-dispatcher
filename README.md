# pro25kadai-dispatcher

このプロジェクトは、Rust で書かれたプロセスディスパッチャーです。

systemd を使用してこのディスパッチャーを起動します。
ディスパッチャーは複数の別プロセスを起動し、SIGINT シグナルを受け取った場合にはそれらの終了も担当します。

## 機能

- 複数のプロセスを非同期で起動
- SIGINT シグナルを受け取って全てのプロセスを終了
- systemd との統合を想定

## ビルド方法

```bash
cargo build --release
```

非 Linux 環境からのクロスコンパイルにも対応しています。
リンカがないかもしれません（1敗）が、それはどうにかしてください。

```bash
cargo build --target x86_64-unknown-linux-musl --release
```

## 使用方法

1. プロジェクトをクローンまたはダウンロードします。
2. 上記のビルドコマンドを実行します。
3. systemd サービスファイルを作成して登録します（例: `/etc/systemd/system/dispatcher.service`）。

   ```
   [Unit]
   Description=Process Dispatcher
   After=network.target

   [Service]
   ExecStart=/path/to/dispatcher
   KillSignal=SIGINT
   Restart=always
   User=your-user

   [Install]
   WantedBy=multi-user.target
   ```

4. サービスを有効化して起動します。

   ```bash
   sudo systemctl enable dispatcher
   sudo systemctl start dispatcher
   ```

## 設定

`src/main.rs` の `commands` ベクターに、起動したいプロセスをハードコーディングします。
各 `CommandConfig` で名前、ディレクトリ、コマンド、引数を指定します。
