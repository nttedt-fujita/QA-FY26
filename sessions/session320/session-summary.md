# Session 320 サマリー

**日時**: 2026-03-27
**目的**: 掛川さんヒアリング結果の整理 + PSI自動化の実現可能性再評価

---

## 実施内容

### 1. 掛川さんヒアリング結果の資料化

xmindメモ（PSI転記作業についてのヒアリング.pdf）を読み取り、[hearing-results-kakegawa.md](hearing-results-kakegawa.md) を作成。

主な確度更新:

| 項目 | 変更 | 理由 |
|------|------|------|
| P-2 出荷実績 | B → **A** | kintone発送依頼通知 + 群馬通商Excelから転記と判明 |
| P-4 生産数 | D → **B** | 「生産関連Excel」の存在判明。GEN経由ではない |
| P-5 在庫（朝霞） | D → **B+** | kintone 72マスタのロケーション列で拠点区別（画面確認） |
| 仕損定義 | D → **A** | 初期不良の再送（EC含む） |
| GEN利用 | C → **A（不使用）** | PSI転記ではGEN不使用 |

### 2. 既存資料の更新

- [psi-dataflow-understanding.drawio](../session318/psi-dataflow-understanding.drawio) — GEN→PSI矢印削除、kintone起点に統一、生産関連Excel追加
- [kakegawa-questions.md](../session318/kakegawa-questions.md) — 解決済み/未解決の整理
- [domain-model-as-is.md](../session317/domain-model-as-is.md) — E11, E18, E19, R5の確度更新

### 3. P-6（発注情報）の矛盾発見

**事実の矛盾**:
- 掛川さん: GEN不使用[A]
- マニュアル: GEN=発注管理[A(M)]（S298）
- PSI実データ: バッテリ発注情報が書かれている[A]
- kintone: 発注機能なし[A]

→ 掛川さんにTeamsで質問済み（回答待ち）

### 4. PSI自動化の実現可能性再評価

[psi-automation-feasibility.md](psi-automation-feasibility.md) を作成。

**結論: 実現可能、着手できる**
- データソースの80%以上が特定済み
- kintone CSV手動DL→小さなDB→自動集計の骨格は有効（S313構想通り）
- kintone APIはNTT東日本基盤の制約あり（S298）、当面CSV手動DL
- 未確定のP-6は後から追加可能

### 5. Linear issue作成（4件）

| Issue | タイトル |
|-------|---------|
| QA-19 | GEN画面調査（スクショベースで現状把握・リレーション整理） |
| QA-20 | 長屋さんヒアリング（予測シート・Agri更新作業の詳細） |
| QA-21 | 宮崎さんに確認（kintone CSV取得期間の指定方法） |
| QA-22 | 宇枝さんに確認（PSI Excel廃止可否・代替システム取込） |

---

## 成果物

| ファイル | 内容 |
|----------|------|
| [hearing-results-kakegawa.md](hearing-results-kakegawa.md) | 掛川さんヒアリング結果（確度更新+P-6矛盾） |
| [psi-automation-feasibility.md](psi-automation-feasibility.md) | PSI自動化の実現可能性再評価 |

---

## 次セッションでやること

| # | やること | 理由 |
|---|---------|------|
| 1 | **GEN画面分析**（Windows PCで実施） | PSI以外のDX化ポイント発見のため。QA-19 |
| 2 | **Phase 1の設計着手** | 実現可能性OKなので設計に進む。kintone CSV取込のDB設計 |
| 3 | **P-6回答の反映** | 掛川さんからの回答が来ていれば反映 |

**GEN分析はWindows PC**で実施予定（スクショベース）。Phase 1設計は通常PCで可能。

---

*作成: Session 320 (2026-03-27)*
