# Session 85 計画

**目的**: DB Repository実装（Phase 1 Step 4）

---

## 背景

Phase 1 の残りステップ:
- Step 4: DB Repository ← **今回**
- Step 5: 基本UI

実機テストはStep 5完了後に統合テストとして実施予定。

---

## やること

### 1. DB Repository設計（TDD Phase 1-2）

**振る舞い記述**:
- 検査結果の保存
- 検査結果の検索（シリアル番号、日付範囲）
- 過去データの取得

**スキーマ設計**:
- `inspection_results` テーブル
- `devices` テーブル（オプション）

### 2. DB Repository実装（TDD Phase 3-5）

**ライブラリ選定**:
- rusqlite（軽量、組み込み用途に適合）

### 3. FTDI対応検討

**Session 83で判明した問題**:
- 実機はFTDI経由UART接続（VID=0x0403, PID=0x6015）
- F9P直接のVID/PID（0x1546:0x01A9）ではない
- ボーレートは38400bps（115200に統一したい）

**対応方針を決定**:
- A案: FTDIのVID/PIDをフィルタに追加
- B案: 手動ポート指定機能を追加
- C案: 両方対応

---

## 完了条件

- [ ] DB Repositoryがビルド可能
- [ ] 検査結果の保存・取得テストがパス
- [ ] FTDI対応方針が決定

---

## Phase 1 実機テストまでのロードマップ

```
Session 85: DB Repository実装
     ↓
Session 86: 基本UI実装（装置一覧）
     ↓
Session 87: 基本UI実装（検査結果表示）
     ↓
Session 88: FTDI対応＋ボーレート設定
     ↓
Session 89: Phase 1 統合テスト（実機）
            - T1-1〜T1-7 実施
```

### Phase 1 実機テスト計画（再掲）

| No | テスト項目 | 確認内容 | Pass基準 |
|----|-----------|----------|----------|
| T1-1 | 装置検出 | USB接続で装置が表示される | 3秒以内に検出 |
| T1-2 | シリアル番号取得 | SEC-UNIQIDが正しく読める | 9バイトのユニークID取得 |
| T1-3 | FWバージョン取得 | MON-VERが正しく読める | FW/HW文字列が取得できる |
| T1-4 | 設定確認 | CFG-RATE/CFG-PRTが読める | 期待値と一致 |
| T1-5 | 検査フロー | 5項目の検査が完了 | 全項目でPass/Fail判定 |
| T1-6 | DB保存 | 検査結果がSQLiteに保存 | SELECTで確認可能 |
| T1-7 | 接続/切断 | 装置を抜き差し | エラーなく再検出 |

---

## 参照資料

- [17-gnss-tool-implementation-plan.md](../../docs/missions/m1-sensor-evaluation/gnss/17-gnss-tool-implementation-plan.md) — 実装計画
- [session84/session-summary.md](../session84/session-summary.md) — 前セッションサマリー
- [session83/session-summary.md](../session83/session-summary.md) — 実機テスト結果

---

*計画作成: 2026-03-11 Session 84終了時*
