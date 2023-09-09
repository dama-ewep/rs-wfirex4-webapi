# rs-wfirex4 API

rs-wfirex4 API は、Rust で書かれたスマートリモコンの操作用のWebAPIです。このAPIを使用することで、TCP経由で赤外線データを送受信することができます。

## 機能

- 赤外線データの送信と受信
- 赤外線データのデバイスやボタンごとのマッピング
- 設定ファイルを使用した柔軟な設定

## 使い方

1. 設定ファイル (`config.toml`) を編集して、必要な設定を行います。
2. APIを起動します。
3. 必要に応じて、赤外線データを送信します。

## 設定ファイル

起動時に、設定ファイルのパスを引数として指定することができます。指定がない場合は、デフォルトの`config.toml`を使用します。
`config.toml` ファイルを使用して、以下の設定を行うことができます：
- サーバのホストとポート
- 接続先のrs-wfirex4のホスト

## APIエンドポイント

以下は、このAPIで利用可能なエンドポイントの一覧です。

### ハートビート

- **URI:** `/heartbeat`
- **Method:** `GET`
- **Description:** 単純な応答を返す死活監視用のエンドポイント。

### デバイスとボタンの一覧取得

- **URI:** `/rs-wfirex4/v1/devices`
- **Method:** `GET`
- **Description:** JSON形式でデバイスとそれに紐づくボタンの一覧を取得できます。

### 赤外線データの送信

- **URI:** `/rs-wfirex4/v1/devices/{deviceName}/buttons/{buttonName}`
- **Method:** `GET`
- **Description:** `{deviceName}` の `{buttonName}` に相当する赤外線を送信します。

### 送信データの確認

- **URI:** `/rs-wfirex4/v1/packet/{deviceName}/buttons/{buttonName}`
- **Method:** `GET`
- **Description:** 送信するデータの確認用のエンドポイント。

## 開発

このプロジェクトは、Rust で開発されています。コントリビューションやフィードバックは歓迎されています。

### 自動生成されるコード(リモコンデータ)

`rs_wfirex4_api/devices.rs` は、`xml_preload` フォルダ内の `Makefile` と python スクリプトを使用して、pre-build プロセスとして`resource/xml`フォルダ以下のxmlを読み込み自動生成されます。
xmlファイルは家電リモコンアプリから以下の手順でエクスポートしたファイルを配置してください。
1. 対象のリモコンを選択
2. 右上の「︙」ボタンから「リモコンデータ受け渡し」を選択
3. 「エクスポート」> 「メールで送信」の順で選択
4. エクスポート対象の家電を選ぶ
5. xmlファイルができるのでGoogleドライブなどに配置して取得してください
