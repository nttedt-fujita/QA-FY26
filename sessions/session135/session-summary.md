# Session 135 サマリー

**日時**: 2026-03-12
**目的**: NTRIP接続テスト（別ネットワーク）+ GGA送信機能追加

---

## 実施内容

### 1. NTRIP自動切断の原因調査

- Session 134でNTRIP接続後すぐに切断される問題を調査
- ログ確認: `NTRIPサーバーが接続を閉じました`
- 原因: **VRS型NTRIPサービスではGGAセンテンスの送信が必須**
- 仕様書確認: 21-ntrip-protocol-spec.md 5.4節「NMEA REQUEST MESSAGES」

### 2. 固定位置GGA送信機能を追加

- `generate_gga_sentence()` 関数を追加
- 東京近辺の固定座標（テスト用）
- 接続直後に初回GGA送信
- 10秒間隔で定期送信
- TcpStreamを `tokio::io::split()` で読み書き分割

### 3. 動作確認

- モバイル回線でNTRIP接続成功（ICY 200 OK）
- GGA送信後もRTCMデータ受信継続を確認
- デバイス未接続のため転送は失敗（想定通り）

---

## 作成/変更ファイル

| ファイル | 内容 |
|----------|------|
| [ntrip_api.rs](../../prototype/m1-gnss/backend/src/web/ntrip_api.rs) | GGA送信機能追加 |

---

## テスト結果

- **237テスト** 全てパス
- バックエンドビルド成功

---

## 技術メモ

### GGAセンテンス形式

```
$GPGGA,HHMMSS.SS,DDMM.MMMM,N,DDDMM.MMMM,E,Q,NN,H.H,ALT,M,GEOID,M,,*CS
```

- 緯度: DDMM.MMMM形式（35.6762° → 3540.5720）
- 経度: DDDMM.MMMM形式（139.6503° → 13939.0180）
- チェックサム: $と*の間のXOR

### 今後の改善（正式実装時）

- ZED-F9Pから受信したGGAを使用（現在は固定位置）
- 送信間隔の調整（現在10秒固定）

---

## 残タスク（M1-B NTRIP関連）

1. device_id紐付け実装
2. 自動保存に変更（手動保存から）
3. u-center照合
4. GGA送信の正式実装（F9PのGGAを使用）

---

## 次セッションでやること

- device_id紐付け実装
- または屋外でのRTK動作確認

---

*作成: Session 135 終了時*
