# Session 311-320

## Session 311 (2026-03-25)

**概要**: スライドv7作成（小笠原さんアドバイス対応）

**重要な成果**:
- 宇枝さんスライドから部門目的・担当範囲を特定
- v7スライド作成（導入スライド追加）
- Session 310の整備（session-summary.md作成）
- drawioのフォントサイズ調整（理想像）

**部門目的**: 年間700台の量産に耐えうる高い品質と効率化

**藤田さん担当範囲**:
- 受入検査・工程不良データベース化による品質向上・業務DX
- Airgrow開発機能の品質評価（Lidar・センサー評価）

**作成ファイル**:

| ファイル | 内容 |
|----------|------|
| session311/ceo-presentation-v7.md | スライドv7（導入スライド追加） |
| session311/images/ | 画像5枚（session309からコピー） |

**詳細**: [session311/session-summary.md](../session311/session-summary.md)

---

## Session 312 (2026-03-25)

**概要**: 社長説明後の対応（Linear整備、フィードバック整理）

**重要な成果**:
- Linearプロジェクト3つ作成（M1-A: LiDAR評価、M1-B: GNSS評価、M2: 点群データ検証）
- QA-14作成（M1 GNSS評価ツール — 実機テスト・残タスク完了）
- 社長フィードバック整理（6月末NG、投資対効果を考える）

**社長フィードバック要点**:
- 6月末はありえない（12月から宇枝さんに指示済み）
- 仕損2-3%にシステム入れる意味あるか
- 虫食い状態でなんとかできるのでは

**今後の方針**:
- ヒアリングで全体像把握 → 影響度大の部分を特定 → 深掘り調査
- 投資対効果を意識

**詳細**: [session312/session-summary.md](../session312/session-summary.md)

---

## Session 313 (2026-03-25)

**概要**: 業務フロー全体像把握、ERP調査、PSIプロトタイプ構想整理

**重要な成果**:
- 関係者リスト作成（docs/stakeholders.md）
- ERP全体像・統合vs疎結合の判断基準フレームワーク整理
- データフロー図（As-Is）作成
- PSIプロトタイプ構想: kintone CSV → 小さなDB → 自動集計（小笠原さん・掛川さん合意済み）

**詳細**: [session313/session-summary.md](../session313/session-summary.md)

---

## Session 314 (2026-03-26)

**概要**: PSIデータ分析（1次〜3次）、確度整理、Linear全ミッション計画策定

**重要な成果**:
- PSI Excel（64シート）・群馬通商Excel（26シート）の構造分析
- 突合分析: Agriシート集計とPSI受注行の数値一致確認（粒剤12ヶ月完全一致）
- 確度整理（A/B/C/D分類）で推測と事実を分離
- 作業フロー仮説図（drawio 3ページ）・質問リスト統合
- Linear整備: 6プロジェクト体制、全プロジェクトにマイルストーン設定
- linear-managementスキル拡張（マイルストーン運用、命名規則、進捗監視）

**Linear体制**:
- [M3] 受入検査DB（〜2027/3/31）
- [M3/M4-PSI] PSI自動化（4/1〜6/30）
- [M1-B] GNSS評価（4/1〜5/22、ツール完成4/30）
- [M4] 工程不良DB（7/1〜2027/3/31）
- [M1-A] LiDAR評価（6/1〜12/31、スコープ確定6/30）
- [M2] 点群データ検証（6/1〜12/31、スコープ確定6/30）

**新情報**: GENにkintone CSVをインポートしている[A]、5月に実験機体飛行予定

**詳細**: [session314/session-summary.md](../session314/session-summary.md)

---

## Session 315 (2026-03-26)

**概要**: kintone現状把握の共有・次セッション方針決定

**重要な成果**:
- kintoneにログイン・閲覧が可能になった
- 思ったより複雑なシステムであることが判明
- CSVの出力方法を把握
- アクセス制約: Windows PC（社給PC）からのみアクセスすべき
- 次セッションはWindows PCでkintoneスクショ共有・CSV収集方針決定

**詳細**: [session315/session-summary.md](../session315/session-summary.md)

---

## Session 316 (2026-03-26)

**概要**: kintoneマニュアル分析（7冊）・CSV収集（7アプリ）・ER図/リレーション図作成

**重要な成果**:
- PPTXからテキスト抽出し、kintone全体構造を把握（部署別管理範囲、業務フロー、システム間連携）
- CSV 7アプリ取得（農業問合せ、発送、発送製品、機体マスタ、オプションマスタ、普及支援/受注管理、請求明細）
- 確度更新: kintone用途(C→A)、GEN役割(C→A)、CSV出力方法(D→A)
- ER図・リレーション図作成（全7テーブル + GEN）
- 農業問合せの原因分類は「市場不具合」でありM4「工程不良」ではないことが判明

**残課題**: R1〜R5（ER図/リレーション図の矢印追加、参照キー表更新等）

**作成ファイル**:

| ファイル | 内容 |
|----------|------|
| [session316/kintone-system-analysis.md](../session316/kintone-system-analysis.md) | マニュアル7冊統合分析 |
| [session316/csv-analysis-summary.md](../session316/csv-analysis-summary.md) | CSV 1次分析結果 |
| [session316/confidence-update.md](../session316/confidence-update.md) | 確度更新 |
| [session316/diagrams/kintone-app-relations.drawio](../session316/diagrams/kintone-app-relations.drawio) | リレーション図 |
| [session316/diagrams/kintone-er-diagram.drawio](../session316/diagrams/kintone-er-diagram.drawio) | ER図 |

**詳細**: [session316/session-summary.md](../session316/session-summary.md)

---

## Session 317 (2026-03-26)

**概要**: 要求立ち返り整理・ドメインモデリング（確度付き）・フレームワーク精査・ヒアリング準備・kintone CSV突合（途中）

**重要な成果**:
- As-Isドメインモデル作成（qa-design-reviewスキル準拠、確度A/A(M)/B/C/D付き）
- 確度A(M)を新設: マニュアル記載は実態と異なる可能性あり
- MECE+ロジックツリー+EARS+マトリクスで課題を体系的に分析
- ヒアリング叩き台（3周構造: 現状確認→要求引き出し→突き合わせ深掘り）
- 技術的制約の整理（5カテゴリ）
- kintone「普及支援/受注管理」は**2026年2月以降のレコードのみ**（運用開始が最近）
- GEN↔kintone連携の方向修正（kintone→GENが正）

**方針決定**:
- 課題抽出は網羅的に / 対処は最小工数で / 将来のシステム移管も見据える
- 分析結果は実務者の回答の裏取りに使う（押し付けない）
- ギリギリ求められるハードルを超えるレベルの業務量想定

**作成ファイル**:

| ファイル | 内容 |
|----------|------|
| [session317/domain-model-as-is.md](../session317/domain-model-as-is.md) | As-Isドメインモデル（確度付き） |
| [session317/domain-model-as-is.drawio](../session317/domain-model-as-is.drawio) | ドメインモデル図 |
| [session317/hearing-preparation.md](../session317/hearing-preparation.md) | ヒアリング準備 |
| [session317/requirements-analysis.md](../session317/requirements-analysis.md) | 要求分析（MECE+EARS+マトリクス） |
| [session317/technical-constraints.md](../session317/technical-constraints.md) | 技術的制約の整理 |
| [session317/diagrams/01-mece-process-map.drawio](../session317/diagrams/01-mece-process-map.drawio) | MECE課題マップ |
| [session317/diagrams/02-logic-tree.drawio](../session317/diagrams/02-logic-tree.drawio) | ロジックツリー |
| [session317/diagrams/03-priority-matrix.drawio](../session317/diagrams/03-priority-matrix.drawio) | 優先順位マトリクス |
| [session317/diagrams/04-pyramid-structure.drawio](../session317/diagrams/04-pyramid-structure.drawio) | ピラミッド構造 |

**詳細**: [session317/session-summary.md](../session317/session-summary.md)

---
