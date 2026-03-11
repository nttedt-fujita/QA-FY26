# Session 88 サマリー

**日付**: 2026-03-11
**目的**: 全体進捗おさらい + ドキュメントメンテナンス

---

## 実施内容

### 1. 設計と実装の整合性レビュー

- ドメインモデル（Session 86）とDB設計（schema.sql）の整合性を確認 → **一致**
- Session 85-87で作成された全ドキュメントを確認

### 2. ADR構造変更

| 変更前 | 変更後 |
|--------|--------|
| `docs/adr/ADR-XXX.md`（フラット） | `docs/adr/{m1,m3,common}/ADR-XXX.md`（ミッション別） |

- ADR番号はグローバル連番を維持
- 新規ADR-006（M1/M3統合方針）を`common/`に配置

### 3. ドキュメント正式配置

| ドキュメント | 配置先 |
|-------------|--------|
| `gnss-unified-domain-model.md` | `docs/missions/m1-sensor-evaluation/gnss/19-gnss-unified-domain-model.md` |
| `m1-m3-relationship.md` | `docs/adr/common/ADR-006-m1-m3-integration.md` |

### 4. 不要ドキュメント削除

Session 85-86の途中版5ファイルを削除:
- `incoming-inspection-domain-model.md`（Session 85）
- `db-repository-behavior.md`（Session 85）
- `incoming-inspection-domain-model-v2.md`（Session 86）
- `domain-model-integration.md`（Session 86）
- `unified-db-schema.md`（Session 86）
- `gnss-unified-domain-model.md`（Session 86、正式配置後に削除）
- `m1-m3-relationship.md`（Session 86、ADR化後に削除）

### 5. ドキュメント更新

- **CLAUDE.md**: ドキュメント配置ルール更新、ADR一覧をミッション別に再構成
- **docs/index.md**: ADR一覧を新構造に更新
- **ADRリンク修正**: ADR-003, 004, 005の相対パスを修正

### 6. ルール追加

CLAUDE.mdに「正式配置後はsessions/から削除」を追加（廃止マークではなく削除が適切）

---

## 作成・更新ファイル

| ファイル | 内容 |
|----------|------|
| `docs/adr/common/ADR-006-m1-m3-integration.md` | 新規ADR |
| `docs/missions/m1-sensor-evaluation/gnss/19-gnss-unified-domain-model.md` | 統合ドメインモデル（正式配置） |
| `CLAUDE.md` | ドキュメント配置ルール、ADR一覧更新 |
| `docs/index.md` | ADR一覧更新 |
| `docs/adr/m1/ADR-004-gnss-tool-approach.md` | リンク修正 |
| `docs/adr/m1/ADR-005-gnss-tool-tech-stack.md` | リンク修正 |
| `docs/adr/m3/ADR-003-lot-list-view.md` | リンク修正 |

---

## hooks観察

- **観察1**: Session 85-87で作成したドキュメントが整理されず放置
- **対処**: CLAUDE.mdに「正式配置後は削除」ルールを追加
- **hooks候補**: Stop（セッション終了時）に「不要ドキュメントの削除チェック」

---

## 次セッション（Session 89）でやること

- FTDI対応 + ボーレート設定
- InspectionEngineとRepositoryの統合
- 実機テスト準備

---

*作成: 2026-03-11 Session 88*
