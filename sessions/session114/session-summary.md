# Session 114 サマリー

**日付**: 2026-03-11
**目的**: ネットワークRTK（NTRIP）調査

---

## 実施内容

### 1. NTRIPプロトコル調査（1次情報）

- ESA公式ドキュメント（NtripDocumentation.pdf）を取得・分析
- NTRIP Version 1.0 仕様を抽出
- Rev1 vs Rev2 の違いを整理

### 2. ZED-F9P RTK設定調査

- Integration Manual からRTK configuration (Page 16-24) を抽出
- RTCMデータの送信方法を確認（シリアルに流すだけでOK）
- RTK状態確認方法（NAV-PVT.carrSoln）を確認

### 3. Rustライブラリ調査・身元確認

- **nav-solutions/ntrip-client** を推奨ライブラリとして選定
- nav-solutions組織の信頼性を確認（GNSS専門、27リポジトリ、活発）
- 関連ライブラリ rtcm-rs の品質も確認

---

## 主な発見

1. **NTRIPは超シンプル**: HTTP/1.0ベース、Basic認証、RTCMをシリアルに流すだけ
2. **既存ライブラリあり**: nav-solutions/ntrip-client（tokio非同期、MPL-2.0）
3. **認証情報が必要**: ジェノバ等の有料サービスはユーザーID/パスワード必須

---

## 作成ファイル

| ファイル | 内容 |
|----------|------|
| [ntrip-research-summary.md](ntrip-research-summary.md) | 調査結果まとめ（実装方針含む） |
| [ntrip-protocol-spec.md](ntrip-protocol-spec.md) | NTRIP仕様書からの抽出 |
| [rtk-configuration.md](rtk-configuration.md) | ZED-F9P RTK設定（Page 16-24） |
| [integration-manual-toc.md](integration-manual-toc.md) | Integration Manual 目次 |
| NtripDocumentation.pdf | NTRIP仕様書原本（ユーザーDL） |

---

## 決定事項

- NTRIPクライアント機能をアプリに追加する方針
- nav-solutions/ntrip-client を使用
- NTRIP設定UI（5項目: URL、ポート、マウントポイント、ID、パスワード）が必要

---

## 次セッションでやること

1. ntrip-client クレートを Cargo.toml に追加
2. NTRIP接続設定のAPI設計
3. RTCMストリーム転送の実装
4. フロントエンドにNTRIP設定UI追加

---

*作成: 2026-03-11*
