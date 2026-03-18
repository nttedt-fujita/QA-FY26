---
marp: true
theme: default
paginate: true
header: ''
footer: '品質G FY26 | 員数確認AI化 検討資料'
style: |
  section {
    font-family: 'Noto Sans JP', 'Helvetica Neue', Arial, sans-serif;
  }
  h1 {
    color: #1a5276;
  }
  h2 {
    color: #2874a6;
  }
  table {
    font-size: 0.85em;
  }
  img {
    display: block;
    margin: 0 auto;
  }
---

<!-- _class: lead -->
# 員数確認AI化 検討資料

**品質グループ FY2026**

2026年3月18日

---

# 背景と方向転換

## 外観検査AI化の検討結果

外観検査（プロポ、ペラ）は**投資回収困難**と判断

→ 詳細は別資料参照: [ai-inspection-report.md](../session252/ai-inspection-report.md)

**結論**: より効果が見込める「員数確認」のAI化を検討

---

# 員数確認のAI化

## 外観検査よりも効果が見込める理由

1. **数量が多い**: 500〜2,500個/回の部品が存在
2. **時間がかかる**: 最大400分（6.7時間）の検査工数
3. **シンプルな判定**: 「個数が合っているか」は明確

---

# 現状の員数確認（工数上位6部品）

| 品名 | 入荷数量 | 検査工数 |
|------|----------|----------|
| Slide Mount | 500個 | 400分 |
| スライドポスト | 1,600個 | 240分 |
| Arm fixed holder | 591個 | 240分 |
| Slido post | 600個 | 220分 |
| U-shaped grommet W | 1,000個 | 180分 |
| Ball_valve_mount | 400個 | 160分 |
| **合計** | - | **1,440分（24時間）** |

---

# AI員数確認のフロー

![w:700](images/count-flow.png)

---

# 撮影ブース構成

![w:500](../session252/images/ai-inspection-booth-shoot.png)

---

# アプリ画面イメージ

![w:700](images/count-app.png)

---

# まとめ

## 提案: 員数確認のAI化を検討

| 理由 | 詳細 |
|------|------|
| 外観検査は回収困難 | 500台/年規模では投資回収66年以上 |
| 員数確認は効果大 | 上位6部品で24時間の検査工数 |

## コスト試算（時給3,000円想定）

- 上位6部品: 1,440分 = **72,000円/ロット**の削減見込み

## 次のアクション

**田原さんヒアリング** → 実態把握 → ROI再計算
