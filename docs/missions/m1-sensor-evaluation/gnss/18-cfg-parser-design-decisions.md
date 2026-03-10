# CFG-RATE / CFG-PRT パーサー設計判断

**作成日**: 2026-03-10 Session 79
**対象**: CFG-RATE (0x06 0x08), CFG-PRT (0x06 0x00)

---

## 設計判断

### 1. timeRef範囲外の扱い（CFG-RATE）

| 項目 | 決定 |
|------|------|
| 対象 | timeRef フィールド（0〜5が仕様範囲） |
| 決定 | 6以上の値は `Unknown` として扱う（エラーにしない） |
| 理由 | 将来の仕様拡張で新しい時刻基準が追加される可能性がある。パース自体は成功させ、呼び出し側で判断できるようにする |

### 2. 複数プロトコル同時（CFG-PRT）

| 項目 | 決定 |
|------|------|
| 対象 | inProtoMask / outProtoMask |
| 決定 | 複数ビットが同時に立つケースをサポートする |
| 理由 | ビットマスクなので複数プロトコル同時有効は仕様上当然。実際の受信機設定でも一般的 |

### 3. portID≠3 の扱い（CFG-PRT）

| 項目 | 決定 |
|------|------|
| 対象 | portID フィールド（0=I2C, 1=UART1, 2=UART2, 3=USB, 4=SPI） |
| 決定 | portID≠3 の場合はエラーを返す |
| 理由 | GNSS評価ツールではUSB接続のみを想定。他ポートの詳細なmode解析は不要（Session 78で決定済み） |

---

## 参照

- [cfg-rate-prt-behavior.md](../../../../sessions/session78/cfg-rate-prt-behavior.md) — 振る舞い記述
- [cfg-rate-prt-spec.md](../../../../sessions/session78/cfg-rate-prt-spec.md) — 仕様書

---

*作成: 2026-03-10 Session 79*
*正式配置: 2026-03-10 Session 79*
