# GNSS評価ツール API仕様書

**最終更新**: 2026-03-11 Session 95
**マスターデータ**: `prototype/m1-gnss/backend/src/web/`

---

## 概要

本ドキュメントはトークン節約用の簡易仕様書。
詳細は各APIモジュールのソースコードを参照。

---

## ベースURL

```
http://localhost:8080
```

---

## 装置管理API

**ソース**: [device_api.rs](../../../../prototype/m1-gnss/backend/src/web/device_api.rs)

| メソッド | パス | 説明 |
|---------|------|------|
| GET | `/api/devices` | 装置一覧取得 |
| POST | `/api/devices/{path}/connect` | 接続（ボーレート自動検出） |
| POST | `/api/devices/{path}/disconnect` | 切断 |

### レスポンス例

```json
// GET /api/devices
{
  "devices": [
    {
      "path": "/dev/ttyACM0",
      "vid": "1546",
      "pid": "01A9",
      "serial_number": null,
      "status": "Detected",
      "baud_rate": null
    }
  ]
}
```

---

## ロット管理API

**ソース**: [lot_api.rs](../../../../prototype/m1-gnss/backend/src/web/lot_api.rs)

| メソッド | パス | 説明 |
|---------|------|------|
| GET | `/api/lots` | ロット一覧取得 |
| POST | `/api/lots` | ロット作成 |
| GET | `/api/lots/{id}` | ロット詳細取得 |
| PUT | `/api/lots/{id}` | ロット更新 |

### リクエスト/レスポンス

```json
// POST /api/lots リクエスト
{
  "lot_number": "LOT-2026-001",
  "expected_rate_ms": 100,
  "expected_port_in_proto": "UBX+NMEA",
  "expected_port_out_proto": "UBX+NMEA",
  "memo": "テスト用ロット"
}

// レスポンス
{
  "id": 1,
  "lot_number": "LOT-2026-001",
  "expected_rate_ms": 100,
  "expected_port_in_proto": "UBX+NMEA",
  "expected_port_out_proto": "UBX+NMEA",
  "memo": "テスト用ロット"
}
```

---

## 検査API（未実装）

**予定**: Session 96

| メソッド | パス | 説明 |
|---------|------|------|
| POST | `/api/inspections` | 検査実行 |
| GET | `/api/inspections` | 検査履歴取得 |

---

## エラーレスポンス

```json
{
  "error": "エラーメッセージ",
  "code": "ERROR_CODE"
}
```

| code | 説明 |
|------|------|
| `NOT_FOUND` | リソースが見つからない |
| `DUPLICATE_LOT_NUMBER` | ロット番号重複 |
| `PORT_NOT_FOUND` | ポートが見つからない |
| `TIMEOUT` | タイムアウト |
| `DB_ERROR` | データベースエラー |

---

*作成: 2026-03-11 Session 95*
