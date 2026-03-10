# Session 70 計画

**目的**: AS-DT1仕様書の追加抽出 + 質問リスト修正

---

## やること

### 1. ユーザーズガイド追加抽出（Pythonスクリプト）

**対象ページ**: 29-32（Session 67で抽出漏れ）

**抽出すべき情報**:
- 放熱板の仕様（p8に記載、図含む）
- カバーガラスのお手入れ方法
- その他の注意事項

**方法**: `sessions/session67/extract_as_dt1_spec.py` を参考に追加抽出スクリプト作成

### 2. 質問リストv5作成

- 「放熱設計アドバイス」の質問を削除
- 仕様サマリーに放熱板仕様を追記
- 抽出した情報で他の質問も見直し

### 3. GNSS評価ツール（時間があれば）

- UBXパーサー実装（NAV-STATUS/NAV-DOP/MON-RF）— TDD
- DevContainer内でのテスト実行確認

---

## 参照

- [session69/session-summary.md](../session69/session-summary.md)
- [session67/extract_as_dt1_spec.py](../session67/extract_as_dt1_spec.py) — 抽出スクリプト参考
- [session69/放熱板image.png](../session69/放熱板image.png) — 抽出対象の確認用

---

*計画作成: 2026-03-10 Session 69終了時*
