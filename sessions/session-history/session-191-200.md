# セッション履歴: Session 191〜200

## Session 191 (2026-03-16)

**概要**: MON-SPAN横軸（周波数オフセット）表示の実装

**実施内容**:
1. SpectrumChartPropsにcenterHz/spanHzを追加
2. 横軸ラベルを周波数オフセット（MHz）で表示
3. u-center風の10MHz刻み目盛りを追加
   - メイン目盛り: 10MHz刻み（ラベル付き）
   - サブ目盛り: 5MHz刻み（グリッド線のみ）
   - 細分目盛り: 2.5MHz刻み（拡大時のみ）

**参考**: Session 187のQiita画像（u-center MON-SPAN画面）

**変更ファイル**:
- `components/MonSpanPanel.tsx` - 周波数オフセット表示、10MHz刻み目盛り
- `components/MonSpanComparePanel.tsx` - 比較画面にもcenterHz/spanHzを渡す

**次セッション**: [session192/session-plan.md](../session192/session-plan.md)

---

## Session 192 (2026-03-16)

**概要**: MON-SPAN比較画面のL2比較色を調整（錯視対策）

**実施内容**:
1. L2比較の色を赤(`#ef4444`)→ピンク/マゼンタ(`#ec4899`)に変更
2. 色知覚の検討（Von Bezold同化効果、同時対比効果）
   - 紫を試す → 緑との同時対比で青っぽく見える
   - ピンク/マゼンタを採用 → オレンジとの混同を解消

**変更ファイル**:
- `components/MonSpanComparePanel.tsx` - L2比較色を赤→ピンクに変更

**次セッション**: [session193/session-plan.md](../session193/session-plan.md)

---

## Session 193 (2026-03-16)

**概要**: 現在地確認 + ドキュメント整備

**実施内容**:
1. session-managementスキルでの文脈確認
   - Session 186-187で計画されたPhase 3（複数台同時対応）が未着手だった
2. ドキュメント整備（Session 186-192の成果物）
   - 削除: session187/ucenter-toc.md, mon-span-spec.md（ubx-mon-messages.mdと重複）
   - 削除: session190/integration-manual-toc.md, integration-manual-spectrum-section.md
   - 残存: session190/integration-manual-spectrum-analyzer.md（37-mon-span-display-spec.mdから参照）
   - 移動: session187/u-center_Userguide_UBX-13005250.pdf → docs/.../gnss/sources/

**残っている作業**:
- M1 Phase 3: 複数台同時対応（未着手）
- M1: 実機での動作確認
- M2/M3: 放置中

**次セッション**: [session194/session-plan.md](../session194/session-plan.md)

---

## Session 194 (2026-03-16)

**概要**: Phase 3（複数台同時対応）の実装方針決定

**実施内容**:
1. DeviceManager 現状分析
   - Phase 1設計で1台のみ接続可能
   - 2台目接続は `MaxDevicesReached` エラー
2. 実装方針の比較検討
   - 案A: DeviceManager拡張 → 並行処理でロック競合懸念
   - **案B: MultiDeviceManager新設** → 採用
   - 案C: Deviceに接続保持 → ライフタイム管理複雑
3. 案Bの実現可能性検証
   - `RealSerialPortProvider` は状態を持たないので複製可能 ✅
   - 各デバイスを `Arc<Mutex<DeviceManager>>` で独立管理 → 並行可能 ✅

**作成ファイル**:
| ファイル | 内容 |
|----------|------|
| [session194/multi-device-manager-design.md](../session194/multi-device-manager-design.md) | MultiDeviceManager 設計書 |
| [ADR-014](../../docs/adr/m1/ADR-014-multi-device-manager.md) | 複数装置同時対応のADR |

**次セッション**: [session195/session-plan.md](../session195/session-plan.md)

---

## Session 195 (2026-03-16)

**概要**: MultiDeviceManager 実装 + serialport API対応（技術的負債解消）

**実施内容**:
1. MultiDeviceManager の実装（TDD）
   - `list_devices()`, `connect()`, `disconnect()`, `get_manager()`, `connected_paths()`
   - 12テスト追加（M1-1〜M4-1）
2. serialport API対応（Session 163で放置されていた技術的負債）
   - `serialport::SerialPort` は `io::Read + io::Write + 固有メソッド` を要求
   - 7ファイルのモック実装を修正

**変更ファイル**:
| ファイル | 変更内容 |
|----------|----------|
| device/mod.rs | multi_manager モジュール追加 |
| device/multi_manager.rs | 新規作成（12テスト含む） |
| device/manager.rs | モック実装修正 |
| inspection/engine.rs | モック実装修正 |
| service/inspection_service.rs | モック実装修正 |
| web/mon_span_api.rs | モック実装修正 |
| web/nav_sat_api.rs | モック実装修正 |
| web/nav_sig_api.rs | モック実装修正 |
| web/nav_status_api.rs | モック実装修正 |

**テスト結果**: 272テストすべてパス

**次セッション**: [session196/session-plan.md](../session196/session-plan.md)

---

## Session 196 (2026-03-16)

**概要**: MultiDeviceManager を API に統合

**実施内容**:
1. RealSerialPortProvider に Clone 派生追加
2. AppState を MultiDeviceManager に変更
3. 後方互換ヘルパー `get_first_device_manager()` を追加
4. 全 API ハンドラー（8ファイル）を移行

**変更ファイル**:
| ファイル | 変更内容 |
|----------|----------|
| device_api.rs | AppState 変更、Clone 追加、ヘルパー追加 |
| gnss_state_api.rs | MultiDeviceManager 経由に変更 |
| nav_sat_api.rs | 同上 |
| nav_sig_api.rs | 同上 |
| nav_status_api.rs | 同上 |
| mon_span_api.rs | 同上 |
| inspection_api.rs | 同上 |
| ntrip_api.rs | RTCM 転送を変更 |

**テスト結果**: 272テストすべてパス

**次セッション**: [session197/session-plan.md](../session197/session-plan.md)

---

## Session 197 (2026-03-16)

**概要**: Phase 3 実機テスト（2台のF9P同時接続）

**実施内容**:
1. 2台のF9P（/dev/ttyUSB0, /dev/ttyUSB1）を接続
2. MultiDeviceManager経由での同時接続テスト
3. 全テスト項目成功（認識、個別接続、同時接続、データ取得）
4. 課題発見: 全APIが `get_first_device_manager()` を使用しており、2台目を指定してデータ取得するAPIがない

**テスト結果**:
| 項目 | 結果 |
|------|------|
| 2台認識 | ✅ |
| 同時接続 | ✅ |
| 1台目データ取得 | ✅ |
| 2台目データ取得 | ⚠️ API未対応 |

**次セッション**: [session198/session-plan.md](../session198/session-plan.md)

---

## Session 198 (2026-03-16)

**概要**: Phase 3 パス指定API追加 + FEデバイス選択UI実装

**実施内容**:
1. パス指定API追加（BE）
   - `/api/devices/{path}/gnss-state` 形式
   - `get_device_manager_by_path()` メソッド追加
2. FEデバイス選択UI実装
   - `getGnssState()` にdevicePathパラメータ追加
   - `useGnssState` にdevicePathオプション追加
   - 屋外検査画面にデバイス選択ドロップダウン追加
3. HashMap順序バグ修正
   - `connected_paths()` がソートせずに返していた問題を修正

**変更ファイル**:
| ファイル | 変更内容 |
|----------|----------|
| device_api.rs | `get_device_manager_by_path()` 追加、ルート追加 |
| gnss_state_api.rs | `get_gnss_state_by_path()` ハンドラ追加 |
| multi_manager.rs | `connected_paths()` にソート追加 |
| api.ts | `getGnssState()` にdevicePathパラメータ追加 |
| useGnssState.ts | devicePathオプション追加 |
| outdoor/page.tsx | デバイス選択状態とUI追加 |

**検証結果**:
| 項目 | 結果 |
|------|------|
| 後方互換API | ✅ ソート順の最初のデバイスを返す |
| パス指定API（USB0） | ✅ エラーなし |
| パス指定API（USB1） | ⚠️ パースエラー（デバイス設定問題） |

**テスト結果**: 272テストすべてパス

**次セッション**: [session199/session-plan.md](../session199/session-plan.md)

---
