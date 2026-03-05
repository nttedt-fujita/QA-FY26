# Session 10 サマリー

**日付**: 2026-03-05
**前セッション**: Session 9（技術調査・ドキュメント化）

---

## 実施内容

### 1. モジュラーモノリスの調査・ドキュメント化

- モジュラーモノリスの定義を明確化
- マイクロサービスとの違いを整理
- **M3/M4への適用評価**: モジュラーモノリスが適切、マイクロサービスは過剰

### 2. 石川さん向けスライド（2種類）

| # | ファイル | 用途 |
|---|----------|------|
| 1 | ishikawa-slide-draft.md | オリジナルPPTX更新の素材（技術方針、16ページ） |
| 2 | hearing-agenda-slide.md | ヒアリング・相談用アジェンダ（7ページ） |

- **ワークフロー**: Ubuntu PCでmd作成 → git push → Windows PCでpull → パワポ内Claudeに渡す

### 3. ヒアリング準備

- P0（大枠の認識合わせ）の質問を優先した印刷用シートを作成
- **課題認識のすり合わせ**（P0-0）を追加：「何が困っているか」を最初に自由回答で聞く

### 4. ワークフロー整理

- `docs/slide-workflow.md` にUbuntu→Windows→PPTXの手順を追記

---

## 作成ファイル

| ファイル | 内容 |
|----------|------|
| [modular-monolith-report.md](modular-monolith-report.md) | モジュラーモノリス調査レポート |
| [ishikawa-slide-draft.md](ishikawa-slide-draft.md) | 技術方針ドラフト（Marp形式、16ページ） |
| [hearing-agenda-slide.md](hearing-agenda-slide.md) | ヒアリング・相談アジェンダ（7ページ） |
| [hearing-sheet-p0.md](hearing-sheet-p0.md) | ヒアリング用シート（印刷用、課題認識の確認を追加） |

---

## 主な発見・結論

### モジュラーモノリス

| 項目 | 結論 |
|------|------|
| **定義** | 単一デプロイ + 内部モジュール分離のハイブリッドアーキテクチャ |
| **マイクロサービスとの違い** | デプロイ単位、インフラ複雑性、コスト |
| **M3/M4への適用** | ✅ 適切（規模が小さい、一人運用、コスト重視） |
| **マイクロサービス** | ❌ 過剰（Netflix規模ではない） |

### 石川さんへの説明ポイント

1. **3つの選択肢**を提示（kintone単体 / kintone + 外部分析 / AWS自前開発）
2. **分析が肝なら案C（AWS自前開発）**が仮の方針
3. **最終判断はヒアリング後**（「案Cありき」ではない）
4. 懸念点（一人運用、開発期間、異動リスク）と対策を明示

---

## 次セッション（Session 11）でやること

| # | タスク | 備考 |
|---|--------|------|
| 1 | 石川さんとのヒアリング実施 | P0の質問を中心に |
| 2 | ヒアリング結果の整理 | 要求仮説の検証・更新 |
| 3 | 技術選定の判断（ヒアリング結果を踏まえて） | kintone or AWS |

---

## 参照資料

### 今回作成した資料

- [modular-monolith-report.md](modular-monolith-report.md)
- [ishikawa-slide-draft.md](ishikawa-slide-draft.md)
- [hearing-sheet-p0.md](hearing-sheet-p0.md)

### 過去セッションの資料

- [session9/kintone-vs-aws-report.md](../session9/kintone-vs-aws-report.md)
- [session8/hearing-items-integrated.md](../session8/hearing-items-integrated.md)
- [docs/fujita-mission-slide.md](../../docs/fujita-mission-slide.md)
