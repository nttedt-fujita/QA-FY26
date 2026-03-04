---
marp: true
theme: default
size: 16:9
paginate: true
header: "ふじた FY2026 ミッション計画"
footer: "品質保証グループ ふじた"
style: |
  section {
    font-family: 'Yu Gothic', 'Hiragino Sans', 'Noto Sans JP', 'Meiryo', sans-serif;
    font-size: 20px;
    line-height: 1.4;
    padding: 40px;
  }
  h1 {
    font-size: 28px;
    color: #1a5276;
    border-bottom: 3px solid #2980b9;
    padding-bottom: 8px;
    text-align: center;
  }
  h2 {
    font-size: 22px;
    color: #2c3e50;
  }
  table {
    font-size: 17px;
  }
  .highlight {
    background: #fef3c7;
    padding: 15px;
    border-radius: 8px;
    border-left: 4px solid #f59e0b;
  }
  .decision-box {
    background: #dbeafe;
    padding: 15px;
    border-radius: 8px;
    border-left: 5px solid #2563eb;
  }
  .two-column {
    display: flex;
    gap: 20px;
  }
  .content-box {
    flex: 1;
    background: #f8fafc;
    padding: 15px;
    border-radius: 8px;
  }
  .tag-qa {
    display: inline-block;
    background: #dbeafe;
    color: #1e40af;
    padding: 2px 8px;
    border-radius: 4px;
    font-size: 14px;
    font-weight: bold;
  }
  .tag-qc {
    display: inline-block;
    background: #fce7f3;
    color: #9d174d;
    padding: 2px 8px;
    border-radius: 4px;
    font-size: 14px;
    font-weight: bold;
  }
---

# ふじた FY2026 ミッション計画

**品質保証グループ ふじた**

4つのミッション — 何を・なぜ・どう進めるか

---

# Need: 自社製品の品質を定量的に保証できる体制を作る

<div class="decision-box">

**私の4ミッションが目指すもの（Need）**:
自社製品の品質を、属人的な判断ではなく **データに基づいて保証できる体制** を作る

</div>

このNeedを実現するために、4つの側面がある:

| 側面 | 内容 | 対応ミッション |
|------|------|--------------|
| **データ駆動の品質判断** | 定量的な基準で品質を評価する | M1, M2 |
| **品質問題の予防・再発防止** | 不良傾向の分析で問題を未然に防ぐ | M4 (+M3) |
| **トレーサビリティの確保** | ロット単位で受入〜工程を追跡できる | M3 (+M4) |
| **サプライヤ品質の可視化** | サイレントチェンジ検出・品質スコア管理 | M3 (+M4) |

<div class="highlight">

**石川さんに確認したいこと**: このNeedの認識は合っていますか？ 他に含めるべき側面はありますか？

</div>

---

# 4つのミッション 全体像

<div class="decision-box">

**Need**: 自社製品の品質を定量的に保証できる体制を作る

</div>

| # | ミッション | 区分 | What（何をする） | Needとの紐づけ |
|---|-----------|------|-----------------|---------------|
| **M1** | センサー評価手法策定 | <span class="tag-qa">QA</span> | Lidar/GNSSの定量評価手法を作る | データ駆動の品質判断 |
| **M2** | 点群データ検証方法策定 | <span class="tag-qa">QA</span> | 点群品質の検証方法を作る | データ駆動の品質判断 |
| **M3** | 受入検査DB化 | <span class="tag-qc">QC基盤</span> | Excel管理 → DB+アプリ化 | トレーサビリティ / サプライヤ管理 |
| **M4** | 工程不良DB化 | <span class="tag-qc">QC基盤</span> | 不良データの体系化・分析基盤 | 予防・再発防止 |

---

# M1: センサー評価手法策定（Lidar / GNSS）

<div class="two-column">
<div class="content-box">

## Lidar評価

**評価指標**: 測距精度・精密度・角度精度・点群密度

**評価方法**:
- 室内: リファレンスターゲット試験（ASTM E2938）
- 屋外: GCPによるフィールド試験（ISO 17123-9準用）
- 環境: 日照・降雨・温度条件試験

</div>
<div class="content-box">

## GNSS評価

**評価指標**: Fix率・TTFF・Fix精度（2DRMS）

**評価方法**:
- 静的精度試験（既知点での観測）
- RTK性能評価（Fix率 > 95%目標）
- 基線長依存性の評価
- 解析ツール: RTKLIB

</div>
</div>

**参照規格**: ASTM E2938-15 / ISO 17123-9:2018 / VDI/VDE 2634

---

# M2: 点群データ検証方法策定

<div class="two-column">
<div class="content-box">

## 品質指標（ASPRS基準）

| 指標 | 内容 | 目標例 |
|------|------|--------|
| 点密度 | pts/m² | ≥ 8 (USGS QL1) |
| 絶対精度 | RMSE_z | < 10cm |
| 相対精度 | ストリップ間 | < 7cm |
| ノイズ率 | 外れ値割合 | < 0.5% |
| 網羅性 | カバー率 | ボイド基準内 |

</div>
<div class="content-box">

## 技術アプローチ

**検証パイプライン構築**（Python + PDAL）

```
取得データ → 密度チェック
           → ノイズ除去
           → GCP照合（RMSE算出）
           → レポート自動生成
```

**検証ツール**:
- CloudCompare（点群比較・解析）
- PDAL（パイプライン処理）
- RTKLIB（GNSS後処理）

</div>
</div>

---

# M3: 受入検査データベース化

<div class="two-column">
<div class="content-box">

## 現状の課題

- PC上のExcelファイルで管理
- 検索はファイルを開いて手作業
- ロット横断の傾向分析が困難
- **サイレントチェンジ**の検出ができていない

## 目標

- タブレットで入力可能なアプリ
- 即座にクエリ・フィルタ
- lot_idでトレーサビリティ確保
- FW/HWバージョンの変更を自動検出

</div>
<div class="content-box">

## DB構成イメージ

```
受入検査DB
├── マスタデータ
│   ├── 部品マスタ
│   ├── サプライヤマスタ
│   └── 検査項目・基準マスタ
├── トランザクション
│   ├── 入荷ロット情報
│   ├── 検査結果
│   └── 不適合処理
└── 分析ビュー
    ├── サプライヤ品質スコアカード
    ├── ロット合格率推移
    └── サイレントチェンジ検出
```

</div>
</div>

---

# M4: 工程不良データベース化

<div class="two-column">
<div class="content-box">

## 不良コード体系（3階層）

| 大分類 | 中分類例 | 不良例 |
|--------|---------|--------|
| **EL**: 電気系 | はんだ付け / 部品実装 | ブリッジ、欠品 |
| **ME**: 機構系 | 組立 / 損傷 | ネジ締め不良、キズ |
| **SW**: ソフト系 | FW / キャリブレーション | 書込異常、校正値異常 |
| **SE**: センサー系 | Lidar / GNSS | 測距・測位精度不良 |

**原因分析**: 4M1E（Man/Machine/Material/Method/Environment）

</div>
<div class="content-box">

## 分析機能

**パレート分析**
- 不良コード別の発生件数を自動集計
- 累積%で重点不良を特定
- 月次品質レビューに組み込み

**SPC（統計的工程管理）**
- 工程能力指数 Cpk を算出
- 管理図で傾向を可視化
- Cpk ≥ 1.33 を量産維持基準に

**ダッシュボード**
- 品質G全員がリアルタイムで閲覧
- 自動レポート生成

</div>
</div>

---

# M3/M4の連携: lot_idによるデータ紐付け

<div class="decision-box">

**M3とM4は一体設計** — lot_idで受入検査と工程不良を紐付ける

</div>

```
受入検査DB (M3)                              工程不良DB (M4)
┌──────────────────┐    lot_id    ┌──────────────────┐
│ 入荷ロット情報    │─────────────→│ 不良記録          │
│  ├ supplier_id   │              │  ├ defect_code   │
│  ├ fw_version    │              │  ├ cause_code    │
│  ├ hw_version    │              │  └ severity      │
│  └ lot_judgment  │              │                  │
└──────────────────┘              └──────────────────┘
            ↓                                ↓
    サプライヤ品質分析  ←──── 相関分析 ────→  工程不良傾向分析
```

**これにより可能になること**:
- 受入合格品が工程で不良 → サプライヤにフィードバック（検査基準見直し）
- FWバージョン変更後に不良増加 → サイレントチェンジの証拠
- 製品追加時もテーブル構造は変わらない（製品コードで区別）

---

# 優先順位: M3/M4先行を提案

<div class="two-column">
<div class="content-box">

## M3/M4を先行する4つの理由

**1. ソフトスキルが直接活きる**
DB設計・SQL・データ分析は既存スキルそのもの
→ 技術的な不確実性が低い

**2. 早期に成果が見える**
DBが動けば品質G全員が使える
→ 異動後の早期実績になる

**3. 他メンバーへの貢献**
いしかわさん・こいたばしさんのデータも集約
→ チーム全体のデータ活用を支える

**4. 一体設計の利点**
lot_idで紐付ける設計は同時進行が効率的

</div>
<div class="content-box">

## M1は並行で調査開始

**M1 Step1（調査・学習）は座学中心**
- 規格学習（ASTM/ISO）
- 自社センサーのスペック整理
- 評価項目の候補リスト作成

→ M3/M4のDB設計と並行可能

## M2はM1の後に本格着手

- M1の成果（評価指標）がM2の入力になる
- 2Q以降に指標定義から開始

</div>
</div>

---

# FY2026 スケジュール

```
  1Q (4-6月)       2Q (7-9月)       3Q (10-12月)     4Q (1-3月)
  ├────────────────┼────────────────┼────────────────┼────────────────┤

  M3/M4  ████████████████  ████████████████  ████████████████
         要件定義           DB構築             アプリ化・運用開始
         スキーマ設計       データ移行

  M1     ░░░░░░░░░░░░░░░░  ████████████████  ████████████████
         規格調査           評価環境構築        標準化
         (並行で軽め)        評価実施

  M2                       ░░░░░░░░░░░░░░░░  ████████████████
                           指標定義           パイプライン構築
```

`████` = メイン作業　`░░░░` = サブ・調査フェーズ

<div class="highlight">

**1Qの集中テーマ**: M3/M4の要件定義 + M1の規格調査（並行）

</div>

---

# 次のステップ

## 直近のアクション（1Q前半）

| # | アクション | 期限目安 |
|---|-----------|---------|
| 1 | **この方針について石川さんと認識合わせ** | 今週中 |
| 2 | 現行Excel運用のヒアリング（受入検査・工程不良） | Week 1-2 |
| 3 | kintone vs 自前開発の比較調査 | Week 2-3 |
| 4 | M3/M4の要件定義開始（requirements-definitionで整理） | Week 3-4 |
| 5 | M1: ASTM/ISO規格の学習開始 | Week 1〜（並行） |

<div class="decision-box">

**確認したいこと**:
- M3/M4先行の方針でOKか？
- 現行の検査記録はどのExcel？（実物を見せてほしい）
- kintoneの利用状況（誰が管理者？どの程度使っている？）

</div>
