# NAV-SIGパーサー + signal_stats 振る舞い仕様

**作成日**: 2026-03-11
**更新日**: 2026-03-11
**セッション**: 107, 109
**Phase**: TDD Phase 1（振る舞いの記述）

---

## 1. 概要

NAV-SIG (0x01 0x43) パーサーおよびsignal_stats集計関数の振る舞い仕様。
屋外検査でL1/L2別のC/N0（受信感度）およびL2受信率を取得するために使用。

**参照**:
- [outdoor-inspection-needs.md](../session106/outdoor-inspection-needs.md) — 要求整理
- [ubx-signal-identifiers.md](../../docs/missions/m1-sensor-evaluation/gnss/ubx-signal-identifiers.md) — sigId定義
- [ADR-008](../../docs/adr/m1/ADR-008-outdoor-inspection-criteria.md) — 屋外検査の合格基準

---

## 2. パース機能

### 2.1 正常系

| 入力 | 期待される振る舞い |
|------|-------------------|
| 正しいNAV-SIGメッセージ | NavSig構造体を返す（iTOW, version, signals） |
| numSigs=0（信号なし） | 空のsignalsリストを持つNavSigを返す |
| numSigs=120（最大） | 120個のSignalInfoを持つNavSigを返す |

### 2.2 異常系

| 入力 | 期待される振る舞い |
|------|-------------------|
| ヘッダー不正（0xB5 0x62でない） | InvalidHeaderエラー |
| Class/ID不一致（0x01 0x43でない） | MessageMismatchエラー |
| チェックサム不正 | ChecksumErrorエラー |
| データ長不足（フレーム全体） | InsufficientLengthエラー |
| ペイロード長とnumSigsの不整合 | PayloadLengthMismatchエラー |

---

## 3. SignalInfo構造体

| フィールド | 型 | 説明 | 仕様書オフセット |
|-----------|-----|------|-----------------|
| gnss_id | u8 | GNSS識別子 | 8 + n×16 |
| sv_id | u8 | 衛星番号 | 9 + n×16 |
| sig_id | u8 | 信号識別子（L1/L2区別） | 10 + n×16 |
| freq_id | u8 | GLONASS周波数スロット+7 | 11 + n×16 |
| pr_res | i16 | 疑似距離残差 [0.1m] | 12 + n×16 |
| cno | u8 | C/N0 [dBHz] | 14 + n×16 |
| quality_ind | u8 | 品質指標（0-7） | 15 + n×16 |
| corr_source | u8 | 補正ソース | 16 + n×16 |
| iono_model | u8 | 電離層モデル | 17 + n×16 |
| sig_flags | u16 | フラグ | 18 + n×16 |

**reserved**: offset 20-23 (U1[4]) — パースするが構造体には含めない

---

## 4. L1/L2判定

### 4.1 is_l1()

| gnss_id | GNSS | L1条件（sigId） |
|---------|------|----------------|
| 0 | GPS | 0 |
| 1 | SBAS | 0 |
| 2 | Galileo | 0, 1 |
| 3 | BeiDou | 0, 1, 5 |
| 5 | QZSS | 0, 1 |
| 6 | GLONASS | 0 |
| その他 | — | false |

### 4.2 is_l2()

| gnss_id | GNSS | L2条件（sigId） |
|---------|------|----------------|
| 0 | GPS | 3, 4 |
| 3 | BeiDou | 2, 3 |
| 5 | QZSS | 4, 5 |
| 6 | GLONASS | 2 |
| その他 | — | false |

**注意**: Galileo (gnss_id=2) にはL2帯がない（E5a/E5bはL5相当）。

---

## 5. ユーティリティメソッド

### 5.1 NavSig

| メソッド | 入力 | 出力 | 振る舞い |
|---------|------|------|---------|
| l1_signals() | — | Vec<&SignalInfo> | L1帯の信号リスト |
| l2_signals() | — | Vec<&SignalInfo> | L2帯の信号リスト |
| get_cno_pair(gnss_id, sv_id) | GNSS ID, 衛星番号 | Option<(u8, u8)> | L1/L2両方あれば(l1_cno, l2_cno)、片方欠けたらNone |

### 5.2 SignalInfo

| メソッド | 入力 | 出力 | 振る舞い |
|---------|------|------|---------|
| is_l1() | — | bool | L1帯ならtrue |
| is_l2() | — | bool | L2帯ならtrue |
| is_used() | — | bool | sigFlags bit 3がセットされていればtrue |

---

## 6. 屋外検査要求との対応

| 要求ID | 要求（What） | 対応方法 |
|--------|-------------|---------|
| O1 | L1/L2別の受信感度確認 | get_cno_pair(), l1_signals(), l2_signals() |
| O3 | L1/L2受信状態一覧 | signals リストをL1/L2でフィルタリング |
| O2 | 仰角・方位角一覧 | **NAV-SATが必要**（今回スコープ外） |

---

## 7. signal_stats集計関数（ADR-008より）

### 7.1 qualityIndの定義

| 値 | 意味 | L2受信中？ |
|----|------|-----------|
| 0 | no signal | ❌ |
| 1 | searching signal | ❌ |
| 2 | signal acquired | ❌ |
| 3 | signal detected but unusable | ❌ |
| 4 | code locked and time synchronized | ❌ |
| 5-7 | code and carrier locked | ✅ |

**決定**: qualityInd ≥ 5 を「L2受信中」とする（RTKに必要な搬送波ロック）

### 7.2 gps_visible_count()

| 入力 | 期待される振る舞い |
|------|-------------------|
| GPS L1信号2つ（SV1, SV5） | 2を返す |
| GPS L1信号1つ + L2信号1つ（同一SV1） | 1を返す（ユニーク衛星数） |
| GPS信号なし | 0を返す |
| GLONASS L1信号のみ | 0を返す（GPSのみカウント） |

**定義**: gnssId=0（GPS）かつis_l1()=trueの信号を持つ衛星のユニーク数（svIdでカウント）

### 7.3 gps_l2_reception_count()

| 入力 | 期待される振る舞い |
|------|-------------------|
| GPS L2信号1つ（qualityInd=5） | 1を返す |
| GPS L2信号1つ（qualityInd=4） | 0を返す（閾値未満） |
| GPS L2信号2つ（同一SV、qualityInd=5,6） | 1を返す（ユニーク衛星数） |
| GPS L2信号なし | 0を返す |

**定義**: gnssId=0（GPS）かつis_l2()=trueかつqualityInd≥5の信号を持つ衛星のユニーク数

### 7.4 gps_l2_reception_rate()

| 入力 | 期待される振る舞い |
|------|-------------------|
| 可視2、L2受信1 | 0.5を返す |
| 可視4、L2受信2 | 0.5を返す |
| 可視2、L2受信0 | 0.0を返す |
| 可視0（GPS信号なし） | 0.0を返す（0除算回避） |

**定義**: gps_l2_reception_count / gps_visible_count（可視0の場合は0.0）

---

## 8. テストシナリオリスト（Phase 2用）

### 8.1 パース機能

- [ ] GPS L1信号1つをパースできる
- [ ] GPS L1 + L2（2信号）をパースできる
- [ ] 複数GNSS（GPS + GLONASS）をパースできる
- [ ] numSigs=0をパースできる
- [ ] ヘッダー不正でInvalidHeaderエラー
- [ ] Class不一致でMessageMismatchエラー
- [ ] ID不一致でMessageMismatchエラー
- [ ] チェックサム不正でChecksumErrorエラー
- [ ] データ長不足でInsufficientLengthエラー

### 8.2 L1/L2判定

**GPS (gnssId=0)**:
- [ ] GPS sigId=0 → is_l1()=true, is_l2()=false
- [ ] GPS sigId=3 → is_l1()=false, is_l2()=true
- [ ] GPS sigId=4 → is_l1()=false, is_l2()=true

**SBAS (gnssId=1)**:
- [ ] SBAS sigId=0 → is_l1()=true, is_l2()=false

**Galileo (gnssId=2)**:
- [ ] Galileo sigId=0 → is_l1()=true, is_l2()=false
- [ ] Galileo sigId=1 → is_l1()=true, is_l2()=false

**BeiDou (gnssId=3)**:
- [ ] BeiDou sigId=0 → is_l1()=true, is_l2()=false
- [ ] BeiDou sigId=5 → is_l1()=true, is_l2()=false（B1C）
- [ ] BeiDou sigId=2 → is_l1()=false, is_l2()=true
- [ ] BeiDou sigId=3 → is_l1()=false, is_l2()=true

**QZSS (gnssId=5)**:
- [ ] QZSS sigId=0 → is_l1()=true, is_l2()=false
- [ ] QZSS sigId=1 → is_l1()=true, is_l2()=false（L1S）
- [ ] QZSS sigId=4 → is_l1()=false, is_l2()=true
- [ ] QZSS sigId=5 → is_l1()=false, is_l2()=true

**GLONASS (gnssId=6)**:
- [ ] GLONASS sigId=0 → is_l1()=true, is_l2()=false
- [ ] GLONASS sigId=2 → is_l1()=false, is_l2()=true

**その他**:
- [ ] 未知のgnssId/sigId → is_l1()=false, is_l2()=false

### 8.3 ユーティリティ

- [ ] l1_signals()がL1帯のみ返す
- [ ] l2_signals()がL2帯のみ返す
- [ ] get_cno_pair()がL1/L2両方あるときSomeを返す
- [ ] get_cno_pair()がL1のみのときNoneを返す
- [ ] is_used()がsigFlags bit 3を正しく判定

### 8.4 signal_stats集計関数

**gps_visible_count**:
- [ ] GPS L1信号2つ（別衛星SV1, SV5）→ 2
- [ ] GPS L1+L2（同一衛星SV1）→ 1（ユニーク衛星数）
- [ ] GPS信号なし → 0
- [ ] GLONASS信号のみ → 0（GPSのみカウント）

**gps_l2_reception_count**:
- [ ] GPS L2（qualityInd=5）→ 1（搬送波ロック）
- [ ] GPS L2（qualityInd=7）→ 1（上限値）
- [ ] GPS L2（qualityInd=4）→ 0（閾値未満）
- [ ] GPS L2なし → 0

**gps_l2_reception_rate**:
- [ ] 可視2、L2受信1 → 0.5
- [ ] 可視2、L2受信2 → 1.0（100%）
- [ ] 可視2、L2受信0 → 0.0
- [ ] 可視0（GPS信号なし）→ 0.0（0除算回避）

---

## 9. 次セッションでの実装

Phase 3-5でこの振る舞い仕様に基づいて実装する。

1. 既存の`nav_sig.rs`のバグ修正（sigIdの誤り）
2. テストコード作成
3. 実装・テスト実行

---

*作成: 2026-03-11 Session 107*
