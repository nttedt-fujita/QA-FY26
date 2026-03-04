# セッション履歴: Session 1〜10

## Session 1 (2026-03-04)

**概要**: 初回セッション。プロジェクト基盤構築とQA基礎知識・ミッション別アプローチの調査レポートを作成。

**背景**: 藤田さんがソフト開発チームからソフトQAとして品質Gに異動。品質保証の知識がないため、基礎知識の習得とミッション遂行のアプローチ調査が必要。

**作成ファイル**:
| ファイル | 内容 |
|----------|------|
| [session1/session-plan.md](../session1/session-plan.md) | Session 1計画 |
| [session1/qa-fundamentals-report.md](../session1/qa-fundamentals-report.md) | QA基礎知識調査レポート（QA/QC、ISO 9001、AQL、FMEA等） |
| [session1/mission-approach-report.md](../session1/mission-approach-report.md) | ミッション別アプローチ調査レポート（Lidar/GNSS/点群/受入検査DB/工程不良DB） |
| [session1/files-reviewed.md](../session1/files-reviewed.md) | 確認した情報源一覧 |
| [session1/session-summary.md](../session1/session-summary.md) | セッションサマリー |

**重要な発見**:
- ソフト開発スキルはQA業務に直接応用可能（DB設計、データ分析、テスト設計）
- M3(受入検査DB)とM4(工程不良DB)はlot_idで紐付けて一体設計すべき
- 国際規格（ASTM E2938, ISO 17123-9, ASPRS）が各ミッションの参照基準として使える
