# NAV-SIGパーサー 振る舞い仕様

**作成日**: 2026-03-11
**セッション**: 107
**Phase**: TDD Phase 1（振る舞いの記述）

---

## 1. 概要

NAV-SIG (0x01 0x43) パーサーの振る舞い仕様。
屋外検査でL1/L2別のC/N0（受信感度）を取得するために使用。

**参照**:
- [outdoor-inspection-needs.md](../session106/outdoor-inspection-needs.md) — 要求整理
- [ubx-signal-identifiers.md](../../docs/missions/m1-sensor-evaluation/gnss/ubx-signal-identifiers.md) — sigId定義

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

## 7. テストシナリオリスト（Phase 2用）

### 7.1 パース機能

- [ ] GPS L1信号1つをパースできる
- [ ] GPS L1 + L2（2信号）をパースできる
- [ ] 複数GNSS（GPS + GLONASS）をパースできる
- [ ] numSigs=0をパースできる
- [ ] ヘッダー不正でInvalidHeaderエラー
- [ ] Class不一致でMessageMismatchエラー
- [ ] ID不一致でMessageMismatchエラー
- [ ] チェックサム不正でChecksumErrorエラー
- [ ] データ長不足でInsufficientLengthエラー

### 7.2 L1/L2判定

- [ ] GPS sigId=0 → is_l1()=true, is_l2()=false
- [ ] GPS sigId=3 → is_l1()=false, is_l2()=true
- [ ] GPS sigId=4 → is_l1()=false, is_l2()=true
- [ ] GLONASS sigId=0 → is_l1()=true
- [ ] GLONASS sigId=2 → is_l2()=true
- [ ] Galileo sigId=0 → is_l1()=true, is_l2()=false
- [ ] 未知のsigId → is_l1()=false, is_l2()=false

### 7.3 ユーティリティ

- [ ] l1_signals()がL1帯のみ返す
- [ ] l2_signals()がL2帯のみ返す
- [ ] get_cno_pair()がL1/L2両方あるときSomeを返す
- [ ] get_cno_pair()がL1のみのときNoneを返す
- [ ] is_used()がsigFlags bit 3を正しく判定

---

## 8. 次セッションでの実装

Phase 3-5でこの振る舞い仕様に基づいて実装する。

1. 既存の`nav_sig.rs`のバグ修正（sigIdの誤り）
2. テストコード作成
3. 実装・テスト実行

---

*作成: 2026-03-11 Session 107*
