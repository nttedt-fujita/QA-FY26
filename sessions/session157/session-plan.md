# Session 157 計画

**目的**: NTRIP/RTK並行動作の調査完了 + 設計判断

**前提**: Session 156で仕様書調査を実施、間接的証拠は得られたが直接的確認が残っている

---

## やること

| # | 作業 | 読むべきファイル |
|---|------|-----------------|
| 1 | 全二重通信の追加調査 | u-blox PDF p.270-274（CFG-UART/USB設定） |
| 2 | DeviceManagerのコード確認 | prototype/m1-gnss/backend/src/device/manager.rs |
| 3 | NTRIP APIのコード確認 | prototype/m1-gnss/backend/src/web/ntrip_api.rs |
| 4 | 設計判断 | - |

---

## 詳細

### 1. 全二重通信の追加調査

Session 156で間接的証拠（tx/rxバッファ独立）は得られたが、「full duplex」という直接的記述は未発見。

**確認すべきこと**:
- Integration Manualの通信セクション（p.270-274: CFG-UART1, CFG-UART2, CFG-USB）
- UARTの全二重/半二重に関する記述

**抽出コマンド**:
```bash
python tools/pdf_page_extractor.py sessions/session155/u-blox-F9-HPG-1.32_InterfaceDescription_UBX-22008968.pdf 270-274 sessions/session157/ublox-uart-config.md
```

### 2. DeviceManagerのコード確認

**確認すべきこと**:
- read/writeのロック実装（Mutex粒度）
- 同時read/writeが可能か、相互ブロックするか

### 3. NTRIP APIのコード確認

**確認すべきこと**:
- RTCM転送ループの実装
- どのようにDeviceManagerにアクセスしているか
- ロック保持時間

### 4. 設計判断

調査結果をもとに:
- 並行動作可能 → 現在のアーキテクチャで進める
- シーケンシャル必須 → フロー見直し、ADR-013作成

---

## 参照

- [Session 156 summary](../session156/session-summary.md)
- [ntrip-rtk-spec-findings.md](../session156/ntrip-rtk-spec-findings.md)
- [ntrip-rtk-investigation-plan.md](../session155/ntrip-rtk-investigation-plan.md)
