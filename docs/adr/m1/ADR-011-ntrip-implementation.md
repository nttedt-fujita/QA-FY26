# ADR-011: NTRIP実装方式

**作成**: Session 133 (2026-03-12)

## ステータス

承認済み

## コンテキスト

GNSS評価ツールにNTRIPクライアント機能を追加する必要がある。
NTRIPサーバーからRTCMデータを受信し、ZED-F9Pに転送することでRTK測位を実現する。

### 候補

1. **ntrip-clientクレート使用** (nav-solutions/ntrip-client v0.1.1)
2. **独自実装** (TCP + HTTP/1.0ベースのNTRIP Rev1プロトコル)

## 調査結果

### ntrip-clientクレート

- **開発元**: nav-solutions（GNSS専門、信頼できる）
- **依存**: tokio, reqwest, rtcm-rs
- **問題点**: **RTCMパース済みデータ（`rtcm_rs::Message`）を返す**

```rust
// ntrip-clientの戻り値
impl Stream for NtripHandle {
    type Item = Message; // <- パース済みのRTCMメッセージ
}
```

### ZED-F9Pへの転送要件

ZED-F9Pにはシリアルポート経由で**生のRTCMバイナリ**をそのまま書き込む必要がある:

```rust
// 必要な処理
serial_port.write(&rtcm_raw_bytes)?;
```

ntrip-clientは`Message`（パース済み）を返すため、生バイトを取得する手段がない。

## 決定

**独自実装**を採用する。

### 理由

1. ZED-F9Pには生のRTCMバイナリを転送する必要がある
2. ntrip-clientはパース済みデータを返すため、要件に合わない
3. NTRIP Rev1プロトコルはシンプル（HTTP/1.0 + Basic認証）
4. 実装量は少ない（TCP接続 + HTTPリクエスト送信 + バイト転送）

### 実装概要

```rust
// TCP接続
let stream = TcpStream::connect(format!("{}:{}", host, port)).await?;

// NTRIP Rev1リクエスト
let request = format!(
    "GET /{} HTTP/1.0\r\n\
     User-Agent: NTRIP GnssEvalTool/1.0\r\n\
     Authorization: Basic {}\r\n\
     \r\n",
    mountpoint, base64_auth
);
stream.write_all(request.as_bytes()).await?;

// RTCMバイナリをそのままZED-F9Pに転送
loop {
    let n = stream.read(&mut buf).await?;
    device_manager.write_data(&buf[..n])?;
}
```

## 参照

- [20-ntrip-rtk-implementation.md](../../missions/m1-sensor-evaluation/gnss/20-ntrip-rtk-implementation.md) - NTRIP実装方針
- [21-ntrip-protocol-spec.md](../../missions/m1-sensor-evaluation/gnss/21-ntrip-protocol-spec.md) - NTRIP仕様書抽出
- [ntrip-client](https://crates.io/crates/ntrip-client) - 不採用クレート

## 教訓

- **外部クレートは実際のAPIを確認してから採用判断する**
- ドキュメントの「RTCMメッセージを受信」という記述だけでは、生バイトかパース済みかは分からない
- 調査段階でソースコードの`Stream::Item`型を確認すべきだった

---

*変更履歴*:

| 日付 | 変更内容 | 関連セッション |
|------|----------|---------------|
| 2026-03-12 | 初版作成 | Session 133 |
