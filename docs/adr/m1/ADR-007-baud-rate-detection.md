# ADR-007: ボーレート自動検出方式

**ステータス**: 承認済み
**日付**: 2026-03-11
**関連セッション**: Session 90

---

## コンテキスト

ZED-F9PのUART接続において、ボードごとにボーレート設定が異なる可能性がある（デフォルト38400、一部115200等）。検査開始時に正しいボーレートを特定する必要がある。

### 調査結果

- **ZED-F9P側**: 自動ボーレート検出機能なし（Integration Manual確認済み）
- **u-center**: ホスト側で複数ボーレート試行による自動検出を実装

---

## 決定

**ホスト側（DeviceManager）で複数ボーレート試行による自動検出を実装する**

### 実装仕様

1. **候補ボーレート**: `38400` → `115200` → `9600` の順
   - 38400: ZED-F9Pデフォルト
   - 115200: 一般的な高速設定
   - 9600: セーフブートモード
2. **検出方法**: 各ボーレートで `UBX-MON-VER` を送信し、応答を確認
3. **タイムアウト**: 各500ms
4. **最大所要時間**: 1.5秒（3回試行の場合）

### API設計

```rust
impl DeviceManager {
    /// ボーレートを自動検出して接続
    pub async fn connect_auto_detect(&mut self) -> Result<u32, DeviceError>;

    /// 検出されたボーレートを取得
    pub fn detected_baud_rate(&self) -> Option<u32>;
}
```

---

## 理由

- 検査開始時に自動でボーレート検出することで、ボードごとの設定差を吸収
- u-centerと同じアプローチで実績あり
- 最悪1.5秒の遅延は受入検査のワークフローでは許容範囲

---

## 代替案

### A. 手動指定のまま

- 最初の1台で確認したボーレートをロット全体に適用
- シンプルだが、ロット途中で設定が違うボードがあると失敗

### B. 設定ファイルで管理

- ロットIDとボーレートを紐づけて管理
- 事前設定が必要で運用が煩雑

---

## 影響

- `DeviceManager` に `connect_auto_detect()` メソッド追加
- 既存の `with_baud_rate()` は手動指定用として残す

---

## 参照

- [ZED-F9P Integration Manual](../../sessions/session90/ZED-F9P_IntegrationManual_UBX-18010802.pdf) p.13, p.48
- [Session 89 調査計画](../../sessions/session89/baud-rate-investigation-plan.md)
