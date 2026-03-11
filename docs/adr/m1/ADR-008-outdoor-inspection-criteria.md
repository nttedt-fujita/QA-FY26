# ADR-008: 屋外検査の合格基準

**ステータス**: 承認済み
**作成日**: 2026-03-11
**決定者**: 藤田
**関連セッション**: Session 53, 106, 108

---

## コンテキスト

Phase 1（屋内検査5項目）完了後、屋外検査機能を追加する。
屋外検査の合格基準を決定する必要がある。

### 背景

- 現状は明確な合格基準がない（小板橋さんヒアリングより）
- Session 53で業界標準を調査済み
- Session 108でL2受信率・FIX率について追加調査

---

## 決定

### 合格基準

| 項目 | 基準値 | 根拠 | 備考 |
|------|--------|------|------|
| L1受信感度 | **≥30 dBHz** | 業界標準（トラッキングロック閾値） | Inside GNSS, 学術論文 |
| L2受信率 | **GPS 50%以上** | 独自基準（RTK FIX衛星数確保） | N増し確認で傾向分析予定 |
| RTK FIX時間 | **≤30秒** | 業界標準KPI | 環境依存あり |
| RTK FIX率 | **>95%** | 業界標準KPI | 連続監視で測定 |

### ツール要件

屋外検査ツールで以下の情報を表示・蓄積する：

| 機能 | UBXメッセージ | 用途 |
|------|--------------|------|
| L1/L2別C/N0一覧 | NAV-SIG | 受信感度評価 |
| L2受信率リアルタイム表示 | NAV-SIG集計 | 合格判定 |
| FIX率リアルタイム表示 | NAV-PVT.carrSoln集計 | 合格判定 |
| L2帯スペクトラム | MON-SPAN | 異常パターン検出 |
| 傾向分析用データ蓄積 | 全データ | N増し確認 |

---

## 詳細決定事項

### L2受信中の定義: qualityInd ≥ 5

**決定**: NAV-SIGのqualityInd ≥ 5（搬送波ロック以上）を「L2受信中」とする。

**検討した選択肢**:

| 案 | 条件 | 意味 | 採否 |
|----|------|------|------|
| 案A | qualityInd ≥ 4 | コードロック以上（単独測位可能） | ❌ |
| **案B** | **qualityInd ≥ 5** | **搬送波ロック以上（RTK可能）** | **✅ 採用** |
| 案C | qualityInd ≥ 1 | 信号が見えている（緩い） | ❌ |

**理由**: RTKで使うには搬送波ロックが必要。コードロック（≥4）だけではRTK FIXに貢献できない。

**qualityInd定義（u-blox仕様）**:

| 値 | 意味 |
|----|------|
| 0 | no signal |
| 1 | searching signal |
| 2 | signal acquired |
| 3 | signal detected but unusable |
| 4 | code locked and time synchronized |
| 5-7 | code and carrier locked and time synchronized |

---

### 集計ロジックの設計: フリー関数（案C）

**決定**: 集計ロジック（L2受信率計算等）はフリー関数として実装する。

**検討した選択肢**:

| 案 | 特徴 | 採否 |
|----|------|------|
| A: NavSigメソッド | シンプル、プロトタイプ向け | ❌ |
| B: SignalStats構造体 | 本格設計、責任分離 | ❌ |
| **C: フリー関数** | **シンプル＆拡張性** | **✅ 採用** |
| D: トレイト拡張 | Rustらしさ | ❌ |

**理由**:
- 単一責任の原則: パーサーと集計を明確に分離
- テスト容易性: 関数単位でテスト可能
- 拡張性: 関数追加のみで拡張可能
- ステートレス: 状態を持たないためシンプル

**デメリット（軽微）**:
- IDEで`nav_sig.`と打っても補完候補が出ない（ドキュメントでカバー）
- メソッドチェーン不可（必要性低い）

**将来の移行**: 使いにくければ案D（トレイト拡張）に移行可能。

**実装例**:

```rust
pub mod signal_stats {
    pub fn gps_visible_count(signals: &[SignalInfo]) -> usize { ... }
    pub fn gps_l2_reception_count(signals: &[SignalInfo]) -> usize { ... }
    pub fn gps_l2_reception_rate(signals: &[SignalInfo]) -> f32 { ... }
}

// 使い方
let rate = signal_stats::gps_l2_reception_rate(&nav_sig.signals);
```

---

### 閾値判定の責任の所在: 検査ロジック

**決定**: 閾値判定（≥30 dBHz、≥50%等）は検査ロジック（OutdoorInspector等）に持たせる。

| モジュール | 担当する振る舞い | 閾値を持つ？ |
|-----------|-----------------|------------|
| NavSigParser | バイト列→構造体への変換 | ❌ |
| signal_stats | L2受信率の計算、C/N0取得 | ❌ |
| **OutdoorInspector** | **合否判定** | **✅** |

**理由**:
- 単一責任の原則: パーサーは「パース」だけ
- 閾値の変更容易性: 30dBHzが変わってもパーサーに影響しない
- テスト容易性: パーサーのテストと判定のテストを分離

---

## 根拠

### L1受信感度: ≥30 dBHz

- トラッキングループ（DLL）が30 dBHz以下でロックを失う
- u-blox F9P推奨値: 良好設計で40-50 dBHz
- No.5異常機は13 dBHz（明らかに異常）

**ソース**:
- [Inside GNSS - Measuring GNSS Signal Strength](https://insidegnss.com/measuring-gnss-signal-strength/)
- [PMC - Weak Signal Tracking](https://pmc.ncbi.nlm.nih.gov/articles/PMC5038690/)

### L2受信率: GPS 50%以上

**業界標準は存在しない**。以下の理由で50%を仮設定：

- GPS可視衛星は常時10-15機程度
- 50%でL2受信 = 5-8機でL2利用可能
- RTK FIXに必要な最低5衛星を確保できる
- N増し確認で傾向を見ながら調整予定

**補足情報**:
- GLONASS L2は24衛星中4衛星に欠陥あり（信頼性問題）
- L2は保護されていないスペクトラム、干渉に弱い

**ソース**:
- [SwiftNav - L1/L2 vs L1/L5](https://www.swiftnav.com/resource/blog/l1-l2-vs-l1-l5-evaluating-dual-frequency-gnss-for-high-precision-applications)
- [NGS RTK Guidelines](https://www.ngs.noaa.gov/PUBS_LIB/NGSRealTimeUserGuidelines.v2.1.pdf)

### RTK FIX時間: ≤30秒

- 業界標準KPI: <30秒、FIX率>95%
- 理想環境: 2-3秒
- 良好環境: 1-10秒
- 困難環境: 30秒以上

**ソース**:
- [marXact - RTK FIX Wait Time](https://support.marxact.com/article/108-why-and-how-long-do-i-have-to-wait-for-a-rtk-fix)
- [RTK KPIs That Matter](https://rtkdata.com/blog/kpis-that-matter-for-rtk/)

### RTK FIX率: >95%

- 業界標準KPI
- NAV-PVT.carrSoln=2（Fixed）の割合で測定

**ソース**:
- [RTK KPIs That Matter](https://rtkdata.com/blog/kpis-that-matter-for-rtk/)

---

## RF波形（MON-SPAN）による異常検出

L2受信率だけでなく、RF波形からも異常を検出できる可能性がある。

### No.5異常機の実績

| 観点 | No.5（異常） | 正常機 |
|------|-------------|--------|
| RF2（L2帯）PGA | **38dB** | 54dB |
| 波形形状 | スパイク状 | 台形（理想） |

### 検出可能な異常パターン

- 出力レベル異常（低すぎる）
- 波形形状異常（スパイク、歪み）
- 帯域内ノイズ

→ 定量的な閾値は今後のN増し確認で決定

---

## 未確定事項（ヒアリング必要）

| 項目 | 確認先 | 質問 |
|------|--------|------|
| L1感度30dBHz | 小板橋さん | この基準で合意できるか？ |
| L2受信率50% | 小板橋さん | 妥当な値か？ |
| RTK FIX時間30秒 | 末永さん | 運用上十分か？ |
| スペアナ波形の正常/異常基準 | 小板橋さん | 定量的な閾値は？ |

---

## 代替案

### L2受信率の代替案

1. **案A: GPS 50%以上**（採用）
   - 根拠: RTK FIX衛星数確保
   - メリット: シンプル
   - デメリット: 業界標準なし

2. **案B: 全GNSS 30%以上**
   - 根拠: GPS+GLONASS+Galileo+BeiDou全体で評価
   - メリット: 現実の運用に近い
   - デメリット: 計算が複雑

3. **案C: L2受信衛星数 ≥5**
   - 根拠: RTK FIX最低要件
   - メリット: 直接的
   - デメリット: 可視衛星数で変動

→ 案Aを採用。N増し確認で傾向を見て再検討。

---

## 影響範囲

- 屋外検査ツールの実装
- 合格判定ロジック
- データ蓄積・分析機能

---

## 変更履歴

| 日付 | 変更内容 | 関連セッション |
|------|----------|---------------|
| 2026-03-11 | 初版作成 | Session 108 |
