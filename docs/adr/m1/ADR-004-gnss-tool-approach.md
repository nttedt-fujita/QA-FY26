# ADR-004: GNSS評価ツールのアプローチ選択

**ステータス**: 承認済み
**日付**: 2026-03-09
**決定者**: 藤田
**影響範囲**: M1-B GNSS設計検証

---

## コンテキスト

GNSS評価（M1-B）において、L1/L2別のC/N0（受信感度）を取得する必要がある。

Session 57でPX4ソースコード調査を実施し、以下が判明した：

- PX4 GPSドライバ（PX4-GPSDrivers `ubx.cpp`）は**UBX-NAV-SIGを処理していない**
- NAV-SATのみ実装されており、1衛星1C/N0値（L1/L2区別なし）
- フライトログ（ULog）からはL1/L2別C/N0を取得できない
- **TTFFも未実装**：NAV-STATUS.ttffは構造体に定義があるが抽出されていない

**調査根拠**:
- [PX4-GPSDrivers ubx.cpp](https://github.com/PX4/PX4-GPSDrivers/blob/main/src/ubx.cpp)
- [PX4-GPSDrivers ubx.h](https://github.com/PX4/PX4-GPSDrivers/blob/main/src/ubx.h)
- [PX4 SatelliteInfo.msg](https://github.com/PX4/PX4-Autopilot/blob/main/msg/SatelliteInfo.msg)

---

## 検討した選択肢

### 選択肢A: PX4ドライバ改造

UBXプロトコル仕様（UBX-22008968 p.152-153）を参照し、PX4-GPSDriversにUBX-NAV-SIG処理を追加する。

**メリット**:
- フライトログにL1/L2別C/N0が統合される
- 既存ツールチェーン（pyulog等）で解析可能
- PX4コミュニティへの貢献になる

**デメリット**:
- PX4ビルド環境の構築が必要
- ファームウェア再書き込みが必要
- PX4アップデート時にマージ作業が発生
- uORBメッセージ定義の拡張も必要
- 実装・検証に時間がかかる

### 選択肢B: 直接UBX通信ツール（採用）

F9Pと直接シリアル通信し、UBX-NAV-SIGを取得する独立ツールを作成する。

**メリット**:
- PX4に依存しない
- 既存ファームウェアをそのまま使える
- 地上での評価に特化した設計が可能
- 実装がシンプルで早く動くものが作れる

**デメリット**:
- 飛行中のリアルタイム監視には使えない
- 別ツールの運用が必要

---

## 決定

**選択肢B（直接UBX通信ツール）を採用する**

### 理由

1. **目的との整合性**: GNSS評価は「設計検証」「受入検査」が目的であり、地上での評価が主。飛行中ログへのL1/L2別C/N0統合は必須ではない。

2. **早期実現**: プロトタイプを早く作り、実際に動かしながら検証項目を確定していく方針（Session 56で決定）に合致。

3. **将来の拡張**: 必要になればPX4改造は後から追加できる。まず動くものを優先。

---

## 影響

- GNSS評価ツールは `tools/gnss-eval/` に配置（M3プロトタイプと分離）
- 技術選定はSession 57で継続
- PX4改造は将来の選択肢として保留

---

## 参照資料

- [11-px4-uorb-mapping.md](../../missions/m1-sensor-evaluation/gnss/11-px4-uorb-mapping.md) — PX4調査結果
- [08-ubx-protocol-index.md](../../missions/m1-sensor-evaluation/gnss/08-ubx-protocol-index.md) — UBXプロトコル仕様索引
- [10-tool-design-notes.md](../../missions/m1-sensor-evaluation/gnss/10-tool-design-notes.md) — ツール設計メモ
