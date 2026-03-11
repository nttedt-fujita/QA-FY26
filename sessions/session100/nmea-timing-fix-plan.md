# UBX通信タイミング問題 対策実装計画

## 問題の概要

F9PがデフォルトでNMEAメッセージを定期出力しており、UBX poll送信後にNMEAが先に届いてしまう。

### 症状

- `communication`: Error（NMEAデータを受信、`invalid sync`）
- `serial`: Error（前のレスポンスの途中から受信）
- `fw`, `rate`, `port`: Pass（たまたまタイミングが合った）

### 根本原因

1. **NMEAメッセージの混入**: F9Pが1秒ごとにNMEAを出力
2. **drain_buffer後にNMEAが届く**: 50ms待機中にNMEAが来る
3. **UBXフレーム同期がない**: 先頭が `B5 62` でなくてもそのまま読んでしまう

---

## 対策方針

**案A + B の組み合わせ**で実装する。

| 案 | 内容 | 目的 |
|----|------|------|
| A | receive_ubxで`B5 62`を探す | フォールバック（NMEAが来ても読み飛ばす） |
| B | 検査中にNMEA出力をOFF | 根本的にNMEA混入を防ぐ |

---

## 実装計画

### Step 1: 案A - receive_ubx で `B5 62` 同期

**ファイル**: `backend/src/device/manager.rs`

**変更内容**:
1. `receive_ubx` メソッドを修正
2. 受信データから `B5 62` を探す
3. `B5 62` が見つかるまでスキップ
4. UBXフレームのヘッダー（class, id, length）を読む
5. ペイロード + チェックサムを読む
6. 完全なUBXフレームを返す

**擬似コード**:
```rust
pub fn receive_ubx(&mut self, timeout: Duration) -> Result<Vec<u8>, DeviceManagerError> {
    // 1. B5 62 を探す（タイムアウトまで）
    loop {
        let byte = read_byte()?;
        if byte == 0xB5 {
            let next = read_byte()?;
            if next == 0x62 {
                break; // UBXヘッダー発見
            }
        }
    }

    // 2. class, id, length を読む（4バイト）
    let header = read_bytes(4)?;
    let length = u16::from_le_bytes([header[2], header[3]]);

    // 3. ペイロード + チェックサムを読む
    let payload = read_bytes(length + 2)?;

    // 4. フレーム全体を返す
    Ok([0xB5, 0x62, header, payload].concat())
}
```

**テスト追加**:
- NMEAが先に来てもUBXフレームを正しく読める
- タイムアウト内にB5 62が見つからない場合はエラー

---

### Step 2: 案B - 検査開始/終了時のNMEA ON/OFF

**ファイル**: `backend/src/inspection/engine.rs`

**変更内容**:
1. `run` メソッドの冒頭でNMEAをOFF
2. `run` メソッドの最後でNMEAをON（成功/失敗に関わらず）

**NMEA OFF方法**:
```
UBX-CFG-VALSET (RAM only)
Key: CFG-UART1OUTPROT-NMEA (0x10740002)
Value: 0
```

**NMEA ON方法**:
```
UBX-CFG-VALSET (RAM only)
Key: CFG-UART1OUTPROT-NMEA (0x10740002)
Value: 1
```

**注意**:
- RAM設定のみ変更（電源OFF/ONで元に戻る）
- UART1のみ対象（検査対象ポート）

**新規ファイル**: `backend/src/ubx/cfg_valset.rs`
- CFG-VALSET メッセージ生成関数

---

## 実装順序

1. **Step 1（案A）を実装**
   - manager.rs の receive_ubx を修正
   - テスト追加
   - 実機テストで効果確認

2. **Step 2（案B）を実装**
   - cfg_valset.rs を新規作成
   - engine.rs の run を修正
   - テスト追加
   - 実機テストで効果確認

---

## 期待される結果

- 5項目すべてPass（または期待値不一致のFail）
- `invalid sync` エラーが発生しない
- 検査終了後はNMEA出力が復帰

---

## 参照資料

- [Session 99 サマリー](../session99/session-summary.md)
- [u-blox F9 Interface Description](https://www.u-blox.com/en/docs/UBX-21022436)
  - CFG-VALSET: Section 3.9.2
  - CFG-UART1OUTPROT-NMEA: Configuration keys list

---

*作成: Session 100 (2026-03-11)*
