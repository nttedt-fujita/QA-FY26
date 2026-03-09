# NAV-PVT パーサー設計判断

**作成日**: 2026-03-09
**Session**: 60

---

## 1. 情報源と信頼度

| 情報 | 出典 | 信頼度 |
|------|------|--------|
| メッセージClass/ID (0x01, 0x07) | 過去調査メモ [08-ubx-protocol-index.md](../../docs/missions/m1-sensor-evaluation/gnss/08-ubx-protocol-index.md) | ○ 複数資料で一致 |
| carrSoln（RTK状態）の値定義 | 同上 | ○ |
| ペイロード構造（バイトオフセット） | **未確認** | △ 仕様書PDF未読 |
| スケールファクター（緯度1e-7等） | **推測** | × 要確認 |
| チェックサム計算方法 | **推測** | × 要確認 |

---

## 2. 振る舞い定義（Phase 1で合意）

### 入力
- UBXフレーム形式のバイト列
- フォーマット: `[0xB5, 0x62, Class, ID, LenL, LenH, Payload..., CK_A, CK_B]`

### 出力（正常系）

| フィールド | 型 | 意味 |
|-----------|-----|------|
| fix_type | u8 | FIXタイプ（0=No fix, 3=3D fix等） |
| carr_soln | u8 | RTK状態（0=なし, 1=Float, 2=Fixed） |
| num_sv | u8 | 使用衛星数 |
| lat | f64 | 緯度（度） |
| lon | f64 | 経度（度） |
| h_acc | f64 | 水平精度（mm） |
| v_acc | f64 | 垂直精度（mm） |

### 異常系

1. ヘッダー不一致 → `ParseError::InvalidHeader`
2. Class/ID不一致 → `ParseError::MessageMismatch`
3. チェックサム不正 → `ParseError::ChecksumError`
4. ペイロード長不足 → `ParseError::InsufficientLength`

---

## 3. 未確認事項（実機テストで検証）

- [ ] ペイロードの各フィールドのバイトオフセット
- [ ] 緯度・経度のスケールファクター（1e-7?）
- [ ] 精度（hAcc, vAcc）の単位（mm? 0.1mm?）
- [ ] チェックサム計算方法（8-bit Fletcher?）

---

## 4. 判断理由

**なぜこの状態で進めるか**:
1. 実機が明日以降でないと確認できない
2. テストコードの構造は振る舞い定義があれば書ける
3. 詳細（バイトオフセット、スケール）は実機テストで調整可能

**リスク**:
- 実機テストでバイト位置がずれている可能性
- その場合はテストデータとパーサーを修正

---

## 5. 参照資料

- [08-ubx-protocol-index.md](../../docs/missions/m1-sensor-evaluation/gnss/08-ubx-protocol-index.md) — UBX仕様書索引
- u-blox F9 HPG 1.32 Interface Description (UBX-22008968) p.145 — NAV-PVT定義（未読）
