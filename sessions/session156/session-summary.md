# Session 156 サマリー

**日付**: 2026-03-13
**体調**: 低調・寝不足・疲労気味
**調査状態**: 作業中（Session 157で継続）

---

## 実施内容

1. **PDFルールの追加**
   - `~/.claude/rules/15-pdf-handling.md` を作成
   - PDFを直接読まず、Pythonスクリプトで抽出してから読むルールを強制

2. **NTRIP/RTK仕様調査**
   - プロジェクト内仕様書4つを読了（20, 21, 22, 32番）
   - u-blox Interface DescriptionからMON-COMMSセクションを抽出

3. **確認項目に対する回答**
   | 確認項目 | 結果 |
   |----------|------|
   | 1. ZED-F9P全二重通信 | 間接的証拠あり（tx/rxバッファ独立）、直接的記述は未発見 |
   | 2. RTCMデータ流量 | 50-500 Bytes/sec（低帯域、問題なし） |
   | 3. DeviceManagerのロック | **要コード確認**（Session 157） |
   | 4. 定期出力設定 | NTRIP接続前後で変更不要 |

---

## 作成ファイル

| ファイル | 内容 |
|----------|------|
| [ntrip-rtk-spec-findings.md](ntrip-rtk-spec-findings.md) | 仕様調査結果 |
| [ublox-toc.md](ublox-toc.md) | u-blox PDF目次（p.1-5） |
| [ublox-toc-2.md](ublox-toc-2.md) | u-blox PDF目次（p.6-14） |
| [ublox-mon-comms.md](ublox-mon-comms.md) | MON-COMMS仕様（p.126-130） |

---

## 残課題

1. **全二重通信の直接的確認**
   - Integration Manualの通信セクション（p.270-274: CFG-UART1, CFG-UART2, CFG-USB）を抽出して確認
   - 「full duplex」という直接的な記述を探す

2. **DeviceManagerのコード確認**
   - `device/manager.rs` のread/write実装
   - `ntrip_api.rs` のRTCM転送ループ
   - ロック競合の可能性を評価

---

## 次セッション（Session 157）でやること

1. 全二重通信の追加調査（Integration Manual p.270-274）
2. DeviceManagerのコード確認
3. 調査結果をもとに設計判断（ADR-013が必要か）
