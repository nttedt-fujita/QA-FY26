# Session 185 計画

**目的**: 衛星信号テーブルのUI改善

**前提**: Session 184でGNSSフィルタ連動機能を実装済み

---

## やること

### 1. 衛星信号テーブルのUI改善

**現状の課題**:
- L1信号とL2信号でバーが多すぎて見づらい
- どの衛星がどのGNSSか一目で分からない

**改善案**:
- 緑のバーの上部分（または背景）にGNSSの色を薄く付けてグルーピング
- 視覚的にGNSS別で信号を識別しやすくする

### 2. L2受信率の挙動確認

**確認事項**:
- L2受信率ゲージ（一番大きいバー）がGNSSフィルタに連動しないのは正常か？
- 現状: GPS固定のL2受信率を表示（ADR-008の合格基準）
- 選択GNSSのL2受信率も表示すべきか検討

---

## 読むべきファイル

| ドキュメント | 内容 |
|-------------|------|
| [ADR-008](../../docs/adr/m1/ADR-008-outdoor-inspection-criteria.md) | 屋外検査の合格基準 |
| [NavSigPanel.tsx](../../prototype/m1-gnss/frontend/src/components/NavSigPanel.tsx) | 現在の実装 |

---

## 参照

- [Session 184 サマリー](../session184/session-summary.md)
