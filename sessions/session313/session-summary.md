# Session 313 サマリー

**日時**: 2026-03-25
**目的**: 業務フロー全体像の把握、ERPの理解、PSIプロトタイプ構想の整理

---

## 実施内容

### 1. 関係者リストの作成

プロジェクトに関わる全関係者を整理し、正式ドキュメントとして配置。

- 品質G、受入検査、SCM、業務推進（業推）、営業支援、外部委託先を網羅
- 和田さん・宮崎さんはSCMではなく「業務推進（業推）」に修正
- 長屋さん（営業支援、元SCM）を追加

**成果物**: [docs/stakeholders.md](../../docs/stakeholders.md)

### 2. ERP全体像の調査

製造業における「あるべき姿」を理解するためにERPの主要モジュール・業務フロー・品質管理モジュールを整理。

- 御社の現状（GEN/kintone/Excel）とERPモジュールの対比
- 年400-500台規模ではフルERPのROIが合わない可能性が高い

**成果物**: [docs/technical-research/erp-overview.md](../../docs/technical-research/erp-overview.md)

### 3. システム統合vs疎結合の判断基準フレームワーク

GEN/kintone問題に対する判断基準を整理。

- **Gartner ペースレイヤード戦略**: 変化速度で層別し統合/分離を判断
- **5C評価**: Consistency, Currency, Cost, Competency, Change の5軸
- **EAIパターン**: Point-to-Point / Hub-and-Spoke / API連携 / DWH の4パターン

**成果物**: [docs/technical-research/integration-decision-framework.md](../../docs/technical-research/integration-decision-framework.md)

### 4. データフロー図（As-Is）作成

現在のシステム間データフローを可視化。

- 痛みポイント8つ（手動転記、システム間連携なし、品質管理欠落等）
- 未確認事項8つ（ヒアリングで確認すべき項目）

**成果物**: [sessions/session313/data-flow-as-is.drawio](data-flow-as-is.drawio)

### 5. PSIデータフロー整理とプロトタイプ構想

小笠原さん・掛川さんとの会話から得た情報を整理。

**PSI更新の現状**:
- 親情報はkintone出力CSV
- 群馬通商Excel（在庫管理、消し込みのみ）と受注案件一覧Agriシートを使って掛川さんが手動集計
- 予測シートは長屋さん（営業支援）が更新

**藤田さんのアイデア（小笠原さん・掛川さん合意済み）**:
- 関連情報を集約する小さなDBシステム
- CSVインポートで入力・更新を自動化
- 群馬通商Excelは初回取り込みのみ（消し込みのみで新規追加なし）
- → **実質、kintone CSVが唯一のインプット**

**フレームワーク評価**:
- ペースレイヤード → Systems of Innovation → 独立システムが妥当
- 5C評価 → 全て低〜中 → 疎結合で正解
- EAIパターン → DWH的アプローチに該当

**成果物**: [sessions/session313/psi-dataflow-and-prototype-idea.md](psi-dataflow-and-prototype-idea.md)

---

## 作成ファイル

| ファイル | 内容 | 配置先 |
|----------|------|--------|
| [docs/stakeholders.md](../../docs/stakeholders.md) | 関係者一覧 | docs/（正式） |
| [docs/technical-research/erp-overview.md](../../docs/technical-research/erp-overview.md) | ERP全体像 | docs/（正式） |
| [docs/technical-research/integration-decision-framework.md](../../docs/technical-research/integration-decision-framework.md) | 統合vs疎結合の判断基準 | docs/（正式） |
| [data-flow-as-is.drawio](data-flow-as-is.drawio) | データフロー図（As-Is） | sessions/（ドラフト） |
| [psi-dataflow-and-prototype-idea.md](psi-dataflow-and-prototype-idea.md) | PSIデータフロー・プロトタイプ構想 | sessions/（ドラフト） |

---

## 次セッションへの引き継ぎ

1. **PSIプロトタイプの設計に入る**
   - kintone CSVのカラム構成を確認（掛川さんに聞く）
   - DB構造・技術スタック検討
   - 未確認事項7つの確認（psi-dataflow-and-prototype-idea.md参照）

2. **データフロー図の精緻化**
   - ヒアリング結果を反映して更新

---

*作成: Session 313 (2026-03-25)*
