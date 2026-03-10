# UBXパーサー テストシナリオ（Session 72）

**作成日**: 2026-03-10
**参照仕様書**: [session64/ubx-messages-spec.md](../session64/ubx-messages-spec.md)

---

## 1. NAV-STATUS (0x01 0x03)

### 用途
- TTFF取得
- RTK状態判定
- FIX有効性判定

### 構造体フィールド

| フィールド | 型 | 仕様書オフセット | 説明 |
|-----------|-----|-----------------|------|
| itow | u32 | 0-3 | GPS Time of Week (ms) |
| gps_fix | u8 | 4 | FIXタイプ（0=NoFix, 2=2D, 3=3D, etc.） |
| gps_fix_ok | bool | 5 (bit 0) | 位置・速度が有効か |
| diff_soln | bool | 5 (bit 1) | 差分補正適用済みか |
| carr_soln | u8 | 7 (bit 6-7) | RTK状態（0=なし, 1=Float, 2=Fixed） |
| spoof_det_state | u8 | 7 (bit 3-4) | スプーフィング検出状態 |
| psm_state | u8 | 7 (bit 0-1) | パワーセーブモード状態 |
| ttff | u32 | 8-11 | Time to First Fix (ms) |
| msss | u32 | 12-15 | 起動からの経過時間 (ms) |

### 振る舞い

| # | 振る舞い | 種別 |
|---|---------|------|
| 1 | 有効なNAV-STATUSフレームから、全フィールドを抽出できる | 正常系 |
| 2 | gpsFix値（0=NoFix, 2=2D, 3=3D, etc.）を取得できる | 正常系 |
| 3 | flags.gpsFixOkで「位置・速度が有効か」を判定できる | 正常系 |
| 4 | flags2.carrSolnでRTK状態（0=なし, 1=Float, 2=Fixed）を取得できる | 正常系 |
| 5 | flags2.spoofDetStateでスプーフィング検出状態を取得できる | 正常系 |
| 6 | ttffでTime to First Fixを取得できる | 正常系 |
| 7 | 不正なヘッダーでエラーを返す | 異常系 |
| 8 | Class/ID不一致でエラーを返す | 異常系 |
| 9 | チェックサム不正でエラーを返す | 異常系 |
| 10 | ペイロード長不足でエラーを返す | 異常系 |

### テストシナリオ

| # | シナリオ | 入力 | 期待結果 | should_succeed |
|---|---------|------|----------|----------------|
| 1 | 3D Fix + RTK Fixed | gpsFix=3, carrSoln=2, gpsFixOk=1 | 各値が正しく抽出される | true |
| 2 | 3D Fix + RTK Float | gpsFix=3, carrSoln=1, gpsFixOk=1 | carrSoln=1 | true |
| 3 | No Fix | gpsFix=0, carrSoln=0, gpsFixOk=0 | gpsFix=0, gpsFixOk=false | true |
| 4 | 2D Fix | gpsFix=2, gpsFixOk=1 | gpsFix=2 | true |
| 5 | TTFF値の抽出 | ttff=5000ms | ttff=5000 | true |
| 6 | スプーフィング検出 | spoofDetState=2 | spoofDetState=2 | true |
| 7 | ヘッダー不正 | sync1=0x00 | InvalidHeader | false |
| 8 | Class/ID不一致 | class=0x05 | MessageMismatch | false |
| 9 | チェックサム不正 | CK_B壊す | ChecksumError | false |
| 10 | ペイロード長不足 | 6バイトのみ | InsufficientLength | false |

---

## 2. NAV-DOP (0x01 0x04)

### 用途
- 精度劣化係数（DOP）取得

### 構造体フィールド

| フィールド | 型 | 仕様書オフセット | 説明 |
|-----------|-----|-----------------|------|
| itow | u32 | 0-3 | GPS Time of Week (ms) |
| g_dop | f64 | 4-5 | Geometric DOP (スケール0.01) |
| p_dop | f64 | 6-7 | Position DOP |
| t_dop | f64 | 8-9 | Time DOP |
| v_dop | f64 | 10-11 | Vertical DOP |
| h_dop | f64 | 12-13 | Horizontal DOP |
| n_dop | f64 | 14-15 | Northing DOP |
| e_dop | f64 | 16-17 | Easting DOP |

### 振る舞い

| # | 振る舞い | 種別 |
|---|---------|------|
| 1 | 有効なNAV-DOPフレームから、全DOP値を抽出できる | 正常系 |
| 2 | DOP値はスケール0.01で変換される（156 → 1.56） | 正常系 |
| 3 | 最小値（1）→ 0.01、最大値（65535）→ 655.35 | 境界値 |
| 4 | 不正なヘッダー/Class/ID/チェックサム/長さでエラーを返す | 異常系 |

### テストシナリオ

| # | シナリオ | 入力 | 期待結果 | should_succeed |
|---|---------|------|----------|----------------|
| 1 | 良好なDOP値 | pDOP=156, hDOP=120, vDOP=100 | pDOP=1.56, hDOP=1.20, vDOP=1.00 | true |
| 2 | 高いDOP値（精度悪い） | pDOP=2500 | pDOP=25.0 | true |
| 3 | 最小値（境界） | 全DOP=1 | 全て0.01 | true |
| 4 | 最大値（境界） | 全DOP=65535 | 全て655.35 | true |
| 5 | ヘッダー不正 | sync1=0x00 | InvalidHeader | false |
| 6 | Class/ID不一致 | id=0x07(PVT) | MessageMismatch | false |
| 7 | チェックサム不正 | CK_A壊す | ChecksumError | false |
| 8 | ペイロード長不足 | 10バイト | InsufficientLength | false |

---

## 3. MON-RF (0x0A 0x38)

### 用途
- ジャミング状態監視

### 構造体フィールド

**ヘッダー部**:
| フィールド | 型 | 仕様書オフセット | 説明 |
|-----------|-----|-----------------|------|
| version | u8 | 0 | メッセージバージョン |
| n_blocks | u8 | 1 | RFブロック数 |

**RfBlock（繰り返し）**:
| フィールド | 型 | オフセット | 説明 |
|-----------|-----|-----------|------|
| block_id | u8 | 0 | RFブロックID (0=L1, 1=L2/L5) |
| jamming_state | u8 | 1 (bit 0-1) | ジャミング状態 |
| ant_status | u8 | 2 | アンテナ状態 |
| ant_power | u8 | 3 | アンテナ電源状態 |
| noise_per_ms | u16 | 12-13 | ノイズレベル |
| agc_cnt | u16 | 14-15 | AGCモニタ |
| cw_suppression | u8 | 16 | CW干渉抑制レベル |

### 振る舞い

| # | 振る舞い | 種別 |
|---|---------|------|
| 1 | 有効なMON-RFフレームから、nBlocks個のRFブロックを抽出できる | 正常系 |
| 2 | 各ブロックのjammingState（0=不明, 1=OK, 2=警告, 3=危険）を取得できる | 正常系 |
| 3 | 各ブロックのantStatus（アンテナ状態）を取得できる | 正常系 |
| 4 | 各ブロックのagcCnt, cwSuppressionを取得できる | 正常系 |
| 5 | nBlocks=0の場合、空のブロックリストを返す | 境界値 |
| 6 | nBlocks=2（L1+L2）の場合、2ブロック分を正しくパースする | 正常系 |
| 7 | 不正なヘッダー/Class/ID/チェックサム/長さでエラーを返す | 異常系 |

### テストシナリオ

| # | シナリオ | 入力 | 期待結果 | should_succeed |
|---|---------|------|----------|----------------|
| 1 | L1のみ、ジャミングなし | nBlocks=1, blockId=0, jammingState=1 | blocks.len()=1, jammingState=1(OK) | true |
| 2 | L1+L2、ジャミング警告 | nBlocks=2, jammingState=[1,2] | blocks[1].jammingState=2(Warning) | true |
| 3 | ジャミング危険 | jammingState=3 | jammingState=3(Critical) | true |
| 4 | アンテナ異常（SHORT） | antStatus=3 | antStatus=3 | true |
| 5 | アンテナ異常（OPEN） | antStatus=4 | antStatus=4 | true |
| 6 | nBlocks=0（境界） | nBlocks=0 | blocks.len()=0 | true |
| 7 | AGC値の抽出 | agcCnt=4000 | agcCnt=4000 | true |
| 8 | CW干渉抑制 | cwSuppression=128 | cwSuppression=128 | true |
| 9 | ヘッダー不正 | sync1=0x00 | InvalidHeader | false |
| 10 | Class/ID不一致 | class=0x01 | MessageMismatch | false |
| 11 | チェックサム不正 | CK_B壊す | ChecksumError | false |
| 12 | ペイロード長不足 | nBlocks=2だが1ブロック分しかない | InsufficientLength | false |

---

## 実装状態

| メッセージ | テスト数 | 実装 | 状態 |
|-----------|---------|------|------|
| NAV-PVT | 3 | ✅ | 既存 |
| NAV-STATUS | 4 | ✅ | 完了 |
| NAV-DOP | 3 | ✅ | 完了 |
| MON-RF | 5 | ✅ | 完了 |

**合計**: 15テスト全パス
