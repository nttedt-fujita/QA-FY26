# Session 298 サマリー

**日付**: 2026-03-24
**概要**: kintone契約方式の整理 / 4フェーズ構成への拡張

---

## やったこと

### 1. kintone契約方式の整理

小笠原さんヒアリング結果を反映:

- **現状**: NTT東日本基盤上でkintone利用 → API連携に許可が必要
- **課題**: 統合在庫管理システムにはAPI連携が必須
- **対応方針**: 許可取得より独自契約のほうが早い
- **進捗**: 和田さん・宮崎さんが対応中

→ [kintone-contract-summary.md](kintone-contract-summary.md) に整理

### 2. 4フェーズ構成への拡張

元の3フェーズに「入出庫の自動記録」を追加:

| Phase | 名称 | 内容 |
|-------|------|------|
| 1 | 集計業務の自動化 | PSI集計の手作業削減 |
| 2 | 入出庫の自動記録 | 受入検査・棚のQRコード化 |
| 3 | 委託先在庫の見える化 | 製造委託先の実在庫可視化 |
| 4 | SCM全体の一元管理 | 統合ダッシュボード + AI |

→ [phase-rename-proposal.md](phase-rename-proposal.md) に決定事項

### 3. fy26-roadmap.mdの更新

4フェーズ構成に合わせてロードマップを更新:
- Phase 2「入出庫の自動記録」を追加
- kintone契約方式セクションを追加
- マイルストーン・効果まとめを更新

→ [session295/fy26-roadmap.md](../session295/fy26-roadmap.md)

---

## 作成ファイル

| ファイル | 内容 |
|----------|------|
| [kintone-contract-summary.md](kintone-contract-summary.md) | kintone契約方式の整理 |
| [phase-rename-proposal.md](phase-rename-proposal.md) | 4フェーズ構成の決定事項 |

---

## 更新ファイル

| ファイル | 内容 |
|----------|------|
| [session295/fy26-roadmap.md](../session295/fy26-roadmap.md) | 4フェーズ構成に拡張 |

---

## 重要な判断

| 判断 | 内容 |
|------|------|
| kintone独自契約 | 理想像にはAPI連携が必須 → 独自契約推奨 |
| 4フェーズ構成 | Phase 2「入出庫の自動記録」を追加 |
| フェーズ名称 | 課題ベース（誰の何を解決するか）で命名 |

---

## 次やること（Session 299）

- 作成した図のスライドへの組み込み
- スライド全体の整合性確認（4フェーズ構成に合わせて）
- 必要ならPhase 2用のdrawio作成

→ [session299/session-plan.md](../session299/session-plan.md)

---

*作成: Session 298 (2026-03-24)*
