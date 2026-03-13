# Session 159 サマリー

**日付**: 2026-03-13

---

## 実施内容

1. **NTRIP + UBXポーリング テスト実施**
   - Session 158で準備したMakefileターゲットを使用
   - `make connect` → `make ntrip-connect` → `make rtk-poll`

2. **テスト結果**
   - 5回のポーリングで成功3回、失敗2回（交互パターン）
   - 失敗時: 全6メッセージでIOエラー（Operation timed out）

3. **問題の原因特定**
   - RTCM転送（199ms）とgnss-stateポーリングのタイミング競合
   - RTCM転送直後にgnss-stateが書き込むとタイムアウト発生

---

## 作成ファイル

| ファイル | 内容 |
|----------|------|
| [ntrip-polling-test-report.md](ntrip-polling-test-report.md) | テストレポート（客観的事実のみ） |

---

## 決定事項

| 項目 | 決定 |
|------|------|
| 問題原因 | RTCM転送とgnss-stateのタイミング競合 |

---

## 残課題

- 競合の詳細メカニズム解析
- 対策の検討・実装

---

## 次セッション（Session 160）でやること

1. テストレポートを元に競合メカニズムを詳細解析
2. 対策案の検討
   - NTRIP一時停止方式
   - リトライ方式
   - 排他制御の見直し
3. 対策の実装
