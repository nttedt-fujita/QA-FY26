# Session 21 計画

**目的**: 名寄せ機能の実装、月別分析結果のレビュー

---

## 背景（Session 20の結論）

- 月別分析スクリプト完成（39テスト全パス）
- 名寄せ方針をドキュメント化済み: [../session20/naming-normalization-policy.md](../session20/naming-normalization-policy.md)
- データ異常あり（2026-11/12、入荷日不明35件）

---

## やること

### 1. 名寄せ機能の実装

- 検査内容の表記揺れ一覧を抽出
- `mapping/検査内容_名寄せルール.csv` を作成
- 分析スクリプトに名寄せ適用機能を追加

### 2. データ異常の確認

- 2026-11/12 の日付が入ったレコードを特定
- 入荷日不明35件の内訳を確認

### 3. （余裕があれば）M1-B GNSS関連

- 合格基準のエビデンス収集（Web調査）
- 末永さんヒアリング準備

---

## 参考資料

- [Session 20 分析結果](../session20/csv-output/)
- [Session 20 名寄せ方針](../session20/naming-normalization-policy.md)
- [Session 18 GNSSドキュメント](../../docs/missions/m1-sensor-evaluation/gnss/)
