# NTRIP調査結果サマリー

Session 114で調査したネットワークRTK（NTRIP）の実装に必要な情報。

---

## 1. NTRIPプロトコル概要

**NTRIP** = Networked Transport of RTCM via Internet Protocol

### システム構成

```
[GNSS基地局] → [NTRIPサーバー] → [NTRIPキャスター] → [NTRIPクライアント] → [ローバー]
                                   （HTTPサーバー）        （このアプリ）        （ZED-F9P）
```

### バージョン比較

| 項目 | Rev1 | Rev2 |
|------|------|------|
| HTTP | 1.0 | 1.1 |
| レスポンス | `ICY 200 OK` | `HTTP/1.1 200 OK` |
| Hostヘッダー | 不要 | 必須 |
| 互換性 | 広い | 高機能デバイスのみ |

**推奨**: Rev1で実装（互換性重視）

---

## 2. クライアント実装仕様

### 2.1 接続リクエスト（Rev1）

```http
GET /mountpoint HTTP/1.0
User-Agent: NTRIP MyApp/1.0
Authorization: Basic base64(username:password)

```

**注意**:
- User-Agentは `NTRIP` で始める必要あり
- 最後に空行（CRLF）が必要

### 2.2 認証方式

| 方式 | 説明 | 実装難易度 |
|------|------|-----------|
| **Basic** | Base64(username:password) | 簡単 |
| Digest | MD5チェックサム | 複雑 |

**推奨**: Basic認証で十分（ほとんどのサービスがBasic対応）

### 2.3 レスポンス

**成功時（Rev1）**:
```
ICY 200 OK
```
→ 以降、RTCMバイナリデータがストリーミングで送られてくる

**失敗時**:
```
HTTP/1.0 401 Unauthorized
```

### 2.4 VRS（仮想基地局）対応

VRS対応サービス（ジェノバ等）の場合:
1. 接続後にNMEA GGAメッセージを送信
2. キャスターが受信位置に基づいたVRSデータを生成
3. RTCMストリームを受信

```
$GPGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,47.0,M,,*47
```

---

## 3. Source-Table（ストリーム一覧）

mountpointを空で接続すると、利用可能なストリーム一覧を取得可能。

### レコードタイプ

| タイプ | 説明 |
|--------|------|
| STR | データストリーム |
| CAS | キャスター情報 |
| NET | ネットワーク情報 |

### STRレコードの主要フィールド

| # | フィールド | 例 |
|---|-----------|-----|
| 2 | mountpoint | `TOKYO0` |
| 4 | format | `RTCM 3` |
| 6 | carrier | `2` (L1&L2) |
| 7 | nav-system | `GPS+GLONASS` |
| 12 | nmea | `1` (VRS必須) |
| 16 | authentication | `B` (Basic) |

---

## 4. ZED-F9PへのRTCM転送

### 方式（超シンプル）

受信したRTCMバイナリをそのままシリアルポートに書き込むだけ。

```rust
// NTRIPから受信したRTCMデータ
let rtcm_data: Vec<u8> = ntrip_stream.read().await?;

// ZED-F9Pに送信
serial_port.write(&rtcm_data)?;
```

### ZED-F9Pの動作

1. RTCMデータを受信すると自動的にRTK floatモードに移行
2. ambiguity解決後、RTK fixedモードに移行
3. `NAV-PVT.carrSoln` で状態確認（1=float, 2=fixed）

---

## 5. Rustライブラリ評価

### 推奨: nav-solutions/ntrip-client

| 項目 | 詳細 |
|------|------|
| バージョン | 0.1.1 (2025-10-05) |
| ライセンス | MPL-2.0 |
| GitHub | https://github.com/nav-solutions/ntrip-client |
| 依存 | tokio, reqwest, rtcm-rs |
| ダウンロード | 月75程度 |

**身元確認**:
- **nav-solutions**: GNSS・精密測位に特化した組織
- 27リポジトリ保有（RINEX: 119 stars, GNSS-RTK: 70 stars）
- 活発なDiscordコミュニティあり
- **信頼できる**

### 関連ライブラリ: rtcm-rs

| 項目 | 詳細 |
|------|------|
| バージョン | 0.11.0 (2024-04-28) |
| ライセンス | MIT/Apache-2.0 |
| ダウンロード | 月1,252 |
| 品質 | unsafe 0%、no_std対応、RTCM v3.4完全対応 |

---

## 6. 実装方針

### アーキテクチャ

```
[フロントエンド]
     ↓ API
[バックエンド（Rust）]
     ├── NTRIPクライアント（ntrip-client使用）
     │     ↓ RTCMデータ
     └── シリアル通信（既存）
           ↓ RTCM転送
       [ZED-F9P]
```

### 実装ステップ

1. **ntrip-clientクレート追加**
2. **NTRIP接続設定API追加**
   - キャスターURL、ポート
   - マウントポイント
   - 認証情報
3. **RTCMストリーム受信・転送ループ**
4. **RTK状態表示（既存NAV-PVTを活用）**

---

## 7. 参照資料

### 1次情報

| 資料 | 場所 |
|------|------|
| NTRIP仕様書 (Version 1.0) | [session114/NtripDocumentation.pdf](NtripDocumentation.pdf) |
| NTRIP仕様抽出 | [session114/ntrip-protocol-spec.md](ntrip-protocol-spec.md) |
| ZED-F9P RTK設定 | [session114/rtk-configuration.md](rtk-configuration.md) |

### Web情報

| 情報 | URL |
|------|-----|
| NTRIP Rev1 vs Rev2 | https://www.use-snip.com/kb/knowledge-base/ntrip-rev1-versus-rev2-formats/ |
| ntrip-client crate | https://lib.rs/crates/ntrip-client |
| nav-solutions GitHub | https://github.com/nav-solutions |

---

## 8. NTRIP設定項目（UI必須）

アプリに必要な設定入力項目:

| 項目 | 必須 | 例 |
|------|------|-----|
| キャスターURL | ✅ | `ntrip.jenoba.jp` |
| ポート | ✅ | `2101` |
| マウントポイント | ✅ | `TOKYO_RTCM3` |
| ユーザーID | ✅ | `user123` |
| パスワード | ✅ | `********` |

**注意**: ジェノバ等の有料サービスは契約時にこれらの情報が提供される。

### 認証フロー

```
1. ユーザーが設定画面で上記情報を入力
2. アプリがBase64(ユーザーID:パスワード)を生成
3. Authorization: Basic ヘッダーに設定
4. キャスターに接続
```

---

## 9. 次のアクション

1. [ ] ntrip-client クレートを Cargo.toml に追加
2. [ ] NTRIP接続設定のAPI設計（上記5項目）
3. [ ] RTCMストリーム転送の実装
4. [ ] フロントエンドにNTRIP設定UI追加
5. [ ] 認証情報の保存方法検討（セキュリティ考慮）

---

*作成: Session 114 (2026-03-11)*
