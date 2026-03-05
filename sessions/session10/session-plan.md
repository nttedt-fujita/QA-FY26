# Session 10 計画

**作成日**: 2026-03-05
**前セッション**: Session 9（技術調査・ドキュメント化）

---

## 今回やること

### 1. モジュラーモノリスの定義を明確化

| 調査内容 |
|---------|
| モジュラーモノリスとは何か |
| マイクロサービスとの違い |
| M3/M4での適用可否 |

### 2. 石川さん向けスライドのドラフト作成

- Marp形式で作成
- 「仮の計画」であること、ヒアリング次第で変わることを前提に
- 懸念点も織り込んで説明
- 完成後、別PCのパワポ内Claudeに渡す予定

**含める内容**:
- 藤田さんの担当ミッション（M1〜M4）
- M3/M4のアプローチ（AWS自前開発 vs kintone）
- 技術選定の判断材料（Session 9の調査結果を要約）
- 次のステップ（ヒアリング → プロトタイプ）

### 3. ヒアリング準備（時間があれば）

- P0の質問を優先して整理
- ヒアリングシートの印刷準備

---

## 参照資料

### Session 9で作成した調査レポート

| ファイル | 内容 |
|----------|------|
| [session9/typescript-vs-go-report.md](../session9/typescript-vs-go-report.md) | TypeScript vs Go 比較 |
| [session9/kintone-vs-aws-report.md](../session9/kintone-vs-aws-report.md) | kintone vs AWS 比較 |
| [session9/spc-control-chart-guide.md](../session9/spc-control-chart-guide.md) | SPC・管理図 解説 |
| [session9/quicksight-report.md](../session9/quicksight-report.md) | QuickSight 調査 |

### その他

| ファイル | 内容 |
|----------|------|
| [session8/hearing-items-integrated.md](../session8/hearing-items-integrated.md) | ヒアリング項目統合版 |
| [docs/fujita-mission-slide.md](../../docs/fujita-mission-slide.md) | 既存のミッションスライド（参考） |

---

## 背景

Session 9で以下を実施済み：
- TypeScript vs Go 比較調査 → 言語選択よりアーキテクチャ選択が先
- kintone vs AWS 比較 → 分析が肝なら自前開発
- SPC・管理図・パレート図の解説
- QuickSight調査 → 管理図も実装可能、有効な選択肢

**方向性**: AWS自前開発がメイン路線、ただし最終判断はプロトタイプ + ヒアリング後
