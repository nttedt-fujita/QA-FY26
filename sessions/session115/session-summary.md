# Session 115 サマリー

**日時**: 2026-03-11
**目的**: RTK関連ドキュメントの正式配置

---

## 実施内容

### 1. 全体進捗おさらい

Session 111で整理したM1-GNSS全体タスクを確認:
- NAV-SIG API/FE連携 → ✅ Session 112-113で完了
- RTK FIX率測定 → 未着手（NTRIP必要）
- MON-SPANパーサー → 未着手（単体で可能）

### 2. RTK関連ドキュメント正式配置

Session 114で作成したドラフトを `docs/missions/m1-sensor-evaluation/gnss/` に正式配置:

| 正式配置先 | 内容 |
|-----------|------|
| 20-ntrip-rtk-implementation.md | NTRIP/RTK実装方針・ライブラリ選定 |
| 21-ntrip-protocol-spec.md | NTRIPプロトコル仕様抽出 |
| 22-rtk-configuration.md | ZED-F9P RTK設定 |
| sources/NtripDocumentation.pdf | NTRIP仕様書原本 |

### 3. クリーンアップ

- session114からドラフトファイルを削除
- README.mdを更新（14-22のドキュメント一覧追加）

---

## 作成・更新ファイル

| ファイル | 操作 |
|----------|------|
| docs/.../gnss/20-ntrip-rtk-implementation.md | 新規作成 |
| docs/.../gnss/21-ntrip-protocol-spec.md | 新規作成 |
| docs/.../gnss/22-rtk-configuration.md | 新規作成 |
| docs/.../gnss/sources/NtripDocumentation.pdf | 新規作成 |
| docs/.../gnss/README.md | 更新 |
| sessions/session114/*.md (ドラフト) | 削除 |

---

## 決定事項

- RTK実装は優先度が高くない
- 次の優先タスク: MON-SPANパーサー or 屋内/屋外検査ページ分離

---

## 次セッション（Session 116）でやること

以下のいずれかを優先:
1. MON-SPANパーサー実装（単体で可能）
2. 屋内/屋外検査ページ分離（Session 113の要望）
3. NAV-SIGを検査ロジックに組み込む

---

*作成: 2026-03-11*
