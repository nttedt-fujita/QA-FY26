# Session 204 計画

**目的**: LED点滅テストバイナリ実装 + 実機検証

**前提**: Session 203で設計完了、識別方式としてLED点滅を採用

---

## やること

| # | 作業 | 読むべきファイル |
|---|------|-----------------|
| 1 | blink_testバイナリ実装 | `session203/multi-device-inspection-design.md` |
| 2 | 実機でLED点滅検証 | - |
| 3 | 結果に応じて次の方針決定 | - |

---

## 詳細

### 1. blink_testバイナリ実装

Session 203の設計に基づいて実装：

- `Cargo.toml` に `[[bin]]` 追加
- `src/bin/blink_test.rs` 作成
- `build_ubx_poll(0x0A, 0x04)` で MON-VER ポーリング送信

### 2. 実機でLED点滅検証

```bash
cargo run --bin blink_test -- /dev/ttyACM0 3
```

確認ポイント：
- 基板のLEDが点滅するか
- どのLED（TX/RX/電源）が反応するか

### 3. 結果に応じて次の方針決定

**LED点滅が成功した場合**:
- API設計（`POST /api/devices/{path}/blink`）
- FE画面に「点滅」ボタン追加

**LED点滅が失敗した場合**:
- 抜き差し検知方式にフォールバック
- デバイス切断イベントの検知実装

---

## 参照

- [Session 203 summary](../session203/session-summary.md)
- [複数台検査フロー設計](../session203/multi-device-inspection-design.md)
