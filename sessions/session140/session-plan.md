# Session 140 計画

**目的**: Periodic Output（定期出力）のエビデンス確認と設計

**前提**: Session 139でAPI並行問題の原因を特定、Periodic Outputが解決策と結論

---

## やること

| # | 作業 | 読むべきファイル |
|---|------|-----------------|
| 1 | u-blox仕様書でPeriodic Outputのエビデンス詳細確認 | 08-ubx-protocol-index.md, 24-ubx-spec-extract.md |
| 2 | CFG-MSGOUT設定の仕様確認 | u-blox PDF p.234（CFG-MSGOUT） |
| 3 | UBX-CFG-VALSET（設定コマンド）の仕様確認 | u-blox PDF p.63-97（UBX-CFG） |
| 4 | Periodic Output実装の設計 | - |
| 5 | 実装・動作確認 | - |

---

## 確認すべきエビデンス

### 1. Periodic Outputの仕組み

- CFG-MSGOUT-UBX_NAV_xxx_USB のレート設定
- 「1」に設定すると毎エポック出力
- 「0」に設定するとポーリングのみ

### 2. 設定方法

- UBX-CFG-VALSET メッセージで設定
- または u-center で事前設定

### 3. 受信アーキテクチャ

現状: APIごとにポーリング
新: F9Pからの自動出力を受信 → 内部状態を更新 → APIはその状態を返す

---

## 設計の方向性（仮説）

```
[F9P] ---(定期出力)---> [受信スレッド] ---> [内部状態]
                                               ↑
                                          [API] ← 最新状態を返すだけ
```

- 受信スレッドがF9Pからのメッセージを継続的に受信
- 各メッセージタイプごとに最新状態を保持
- APIは最新状態を返すだけ（ポーリングしない）

---

## 参照

- [Session 139 summary](../session139/session-summary.md)
- [08-ubx-protocol-index.md](../../docs/missions/m1-sensor-evaluation/gnss/08-ubx-protocol-index.md)
- u-blox Interface Description (UBX-22008968) p.234

---

*計画作成: 2026-03-12 Session 139終了時*
