# M4: 工程不良データベース化

**担当**: ふじた
**難易度**: 中
**依存関係**: M3と共通設計推奨

---

## 概要

工程不良データをデータベース化し、体系的な不良分析・改善サイクルを確立する。
M3（受入検査DB）とlot_idで紐付け、サプライヤ品質と工程不良の相関分析を可能にする。

---

## 不良コード体系（3階層構造）

> **IPC-9261A** "In-Process DPMO and Estimated Yield for PWAs" の考え方を参考に、
> 不良コードは「不良現象（What）」「不良箇所（Where）」「不良原因（Why）」の3軸で分類

**フォーマット: XX-XX-XX**

| Level 1 (大分類) | Level 2 (中分類) | Level 3 例 |
|-----------------|-----------------|-----------|
| **EL: 電気系** | EL-SO: はんだ付け | EL-SO-01: はんだブリッジ |
| | EL-CM: 部品実装 | EL-CM-01: 部品欠品 |
| | EL-FN: 機能不良 | EL-FN-01: 通信異常 |
| **ME: 機構系** | ME-AS: 組立 | ME-AS-01: ネジ締め不良 |
| | ME-DM: 損傷 | ME-DM-01: キズ |
| **SW: ソフトウェア系** | SW-FW: FW | SW-FW-01: 書込異常 |
| | SW-CL: キャリブレーション | SW-CL-01: 校正値異常 |
| **SE: センサー系** | SE-LI: Lidar | SE-LI-01: 測距精度不良 |
| | SE-GN: GNSS | SE-GN-01: 測位精度不良 |

---

## 原因コード体系（4M1E分類）

> **石川馨（1968）「品質管理入門」に基づく特性要因（4M）分類を拡張**

| コード | 分類 | 原因例 |
|-------|------|--------|
| M1-xx | Man（人） | 手順逸脱、スキル不足、不注意 |
| M2-xx | Machine（設備） | 設備故障、治具摩耗、校正切れ |
| M3-xx | Material（材料） | 部品不良、ロット間ばらつき、保管劣化 |
| M4-xx | Method（方法） | 作業手順不備、設計起因、基準不明確 |
| E1-xx | Environment（環境） | 温度影響、湿度影響、静電気 |

---

## SPC連携

> **ISO 22514-1:2014**: "Process capability is the statistical measure of the inherent process variability for a given characteristic."

```
工程能力指数:
  Cp  = (USL - LSL) / (6σ)           — ばらつきのみ評価
  Cpk = min[(USL - μ)/(3σ), (μ - LSL)/(3σ)]  — 偏りも考慮

判定基準:
  Cpk ≥ 1.67  → 十分（新規工程の目標）
  Cpk ≥ 1.33  → 良好（量産維持の基準）
  1.00 ≤ Cpk < 1.33 → 要改善
  Cpk < 1.00  → 能力不足（工程改善必須）
```

---

## パレート分析用ビュー

```sql
-- 不良コード別パレート（累積%付き）
SELECT
    dc.defect_code,
    dc.defect_name,
    COUNT(*) AS occurrence_count,
    SUM(COUNT(*)) OVER (ORDER BY COUNT(*) DESC)
        * 100.0 / SUM(COUNT(*)) OVER () AS cumulative_pct
FROM defect_records dr
JOIN defect_code_master dc ON dr.defect_code = dc.defect_code
GROUP BY dc.defect_code, dc.defect_name
ORDER BY occurrence_count DESC;
```

---

## 推奨段取り

```
Step 1: 不良コード体系の設計
  ├── 現行の不良記録方法の調査（Excel）
  ├── 上記3階層コード体系 + 4M1E原因コードの策定
  ├── 現場へのヒアリング・コード体系のレビュー
  └── 【成果物】不良コード体系表

Step 2: DB構築・SPC連携
  ├── 不良記録テーブル・SPCテーブルの作成
  ├── 管理図自動生成の仕組み構築
  ├── パレート分析ビューの作成
  └── 【成果物】稼働DB、ダッシュボード

Step 3: 運用・改善サイクル確立
  ├── 現場への展開・教育
  ├── 月次品質レビューへの組み込み
  └── 【成果物】運用マニュアル、月次レポートテンプレート
```

---

## 参考文献

| トピック | ソース |
|---------|--------|
| ISO 22514-1:2014 | 工程能力 |
| IPC-9261A | 工程DPMO |
| SPC | [ITトレンド](https://it-trend.jp/process_management/article/464-0007) |
| FMEA | [skillnote](https://skillnote.jp/knowledge/fmea/) |

---

## ドキュメント一覧

| ファイル | 内容 |
|----------|------|
| [hearing-items-m4.md](hearing-items-m4.md) | M4固有のヒアリング項目 |

---

## 次のアクション

1. **工程不良Excelの入手**（最優先）
2. Excel分析（M3と同様のAs-Is分析）
3. M3プロトタイプへのM4機能追加設計

---

*元データ: [session1/mission-approach-report.md](../../sessions/session1/mission-approach-report.md)*
*更新日: 2026-03-09 (Session 52)*
