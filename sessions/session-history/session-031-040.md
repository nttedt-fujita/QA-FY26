# セッション履歴: Session 31〜40

## Session 31 (2026-03-06)

**概要**: M3ドキュメント整理（サブディレクトリ分類 + READMEインデックス化）+ ドキュメント配置ルール策定。

**背景**: Session 30で発見した「17ファイル乱立」「README陳腐化」問題を解決。整理中に旧READMEがSession 5の議論（kintone推奨）と乖離していることが判明し削除。また二重管理防止のルールをCLAUDE.mdに追記。

**作成ファイル**:
| ファイル | 内容 |
|----------|------|
| [docs/missions/m3-incoming-inspection-db/README.md](../../docs/missions/m3-incoming-inspection-db/README.md) | **新README**（インデックス + 概要） |
| [session31/session-summary.md](../session31/session-summary.md) | セッションサマリー |

**移動したファイル**:
| 移動元 | 移動先 |
|--------|--------|
| sessions/session5/platform-comparison.md | docs/missions/m3-incoming-inspection-db/to-be/ |
| sessions/session25/prototype-approach.md | docs/missions/m3-incoming-inspection-db/to-be/ |
| 17ファイル | as-is/, to-be/, hearing/ に分類 |

**削除ファイル**:
| ファイル | 理由 |
|----------|------|
| to-be/database-design.md（旧README） | Session 5の議論と乖離、古い内容 |
| hearing/hearing-items-integrated.md | hearing-items.mdと二重管理 |

**更新ファイル**:
| ファイル | 内容 |
|----------|------|
| [CLAUDE.md](../../CLAUDE.md) | ドキュメント配置ルール追記 |

**重要な決定**:
- **ドキュメント配置ルール**: sessions/はドラフト、docs/が正式。区切りがついたらdocs/に移動し、以降はそこを更新
- **二重管理防止**: 同じ情報を複数の場所に書かない

**次セッション（Session 32）でやること**:
- to-be/の整合性確認（重複・古い内容がないか）
- quality-framework-research.md を docs/qa-knowledge/ に移動すべきか検討
