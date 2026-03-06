# Session 31 サマリー

**日付**: 2026-03-06
**目的**: M3ドキュメント整理（サブディレクトリ分類 + READMEインデックス化）

---

## 実施内容

### 1. サブディレクトリ分類

17ファイルを3つのサブディレクトリに整理:

```
m3-incoming-inspection-db/
├── README.md           # インデックス + 概要
├── as-is/              # 現状分析（7ファイル）
├── to-be/              # あるべき姿・設計（7ファイル）
└── hearing/            # ヒアリング関連（4ファイル）
```

### 2. 旧READMEの削除

- 旧README（Session 2作成）はSession 5の議論（kintone推奨）と乖離していた
- Pythonやフルスクラッチ開発の話が書かれていたが、方針はkintone + 外部分析
- **削除**（database-design.mdとして残す必要なし）

### 3. 技術方針ドキュメントの配置

Session 5, 25で作成したドキュメントを正式な場所に配置:

| 移動元 | 移動先 |
|--------|--------|
| sessions/session5/platform-comparison.md | to-be/platform-comparison.md |
| sessions/session25/prototype-approach.md | to-be/prototype-approach.md |

### 4. ドキュメント配置ルールの策定

CLAUDE.mdに追記:

- sessions/で作成したドキュメントは**ドラフト扱い**
- 区切りのいいところで**docs/に配置**、以降はそこを更新
- 二重管理を防ぐため、詳細は1箇所に置き参照リンクで済ませる

---

## 成果物

| ファイル | 内容 |
|----------|------|
| [docs/missions/m3-incoming-inspection-db/README.md](../../docs/missions/m3-incoming-inspection-db/README.md) | **新README**（インデックス + 概要） |
| [docs/missions/m3-incoming-inspection-db/to-be/platform-comparison.md](../../docs/missions/m3-incoming-inspection-db/to-be/platform-comparison.md) | 技術方針（kintone vs 自前開発） |
| [docs/missions/m3-incoming-inspection-db/to-be/prototype-approach.md](../../docs/missions/m3-incoming-inspection-db/to-be/prototype-approach.md) | Phase分け方針 |

**更新ファイル**:
| ファイル | 内容 |
|----------|------|
| [CLAUDE.md](../../CLAUDE.md) | ドキュメント配置ルール追記 |

**削除ファイル**:
| ファイル | 理由 |
|----------|------|
| to-be/database-design.md（旧README） | Session 5の議論と乖離、古い内容 |
| hearing/hearing-items-integrated.md | hearing-items.mdと二重管理、Session 8で止まっていた |

---

## 重要な決定

### ドキュメント配置ルール

1. **sessions/** = ドラフト（セッション中の作業記録）
2. **docs/** = 正式ドキュメント（区切りがついたら移動）
3. 以降の更新は **docs/** 側を行う
4. 二重管理を防ぐ

---

## 次セッションでやること

- to-be/の整合性確認（重複・古い内容がないか）
- quality-framework-research.md（Session 25）を docs/qa-knowledge/ に移動すべきか検討
