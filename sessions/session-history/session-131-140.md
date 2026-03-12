# セッション履歴: Session 131〜140

## Session 131 (2026-03-12)

**概要**: 屋外検査E2E確認 + 残タスク整理 + ユーザーガイド作成

**実施内容**:
1. **E2E確認**
   - 保存ボタン押下 → DB保存成功を確認
   - SQLiteで1件のレコード確認
2. **ユーザーガイド作成**
   - 処理フローのドキュメントがなかったため新規作成
   - 31-outdoor-inspection-user-guide.md
3. **残タスク整理**
   - NTRIPクライアント未実装を発見
   - device_id紐付け未実装を発見
   - 手動保存→自動保存の変更が必要

**作成ファイル**:
| ファイル | 内容 |
|----------|------|
| [31-outdoor-inspection-user-guide.md](../../docs/missions/m1-sensor-evaluation/gnss/31-outdoor-inspection-user-guide.md) | 屋外検査ユーザーガイド |
| [session131/session-summary.md](../session131/session-summary.md) | セッションサマリー |
| [session132/session-plan.md](../session132/session-plan.md) | 次セッション計画 |

**残タスク（優先順）**:
1. NTRIP認証設定画面
2. NTRIPクライアント実装
3. device_id紐付け実装
4. 自動保存に変更
5. u-center照合

**次セッション（Session 132）でやること**:
- NTRIP認証設定画面の実装

---

## Session 132 (2026-03-12)

**概要**: NTRIP認証設定画面の実装

**実施内容**:
1. **NTRIP仕様確認**
   - 20-ntrip-rtk-implementation.md, 21-ntrip-protocol-spec.md, 22-rtk-configuration.md を確認
   - 設定項目を洗い出し
2. **設定画面実装**
   - ナビゲーションに「設定」タブ追加
   - `/settings` ページ作成
   - ローカルストレージへの保存/読み込み

**作成ファイル**:
| ファイル | 内容 |
|----------|------|
| [Navigation.tsx](../../prototype/m1-gnss/frontend/src/components/Navigation.tsx) | 「設定」タブ追加 |
| [ntrip-settings.ts](../../prototype/m1-gnss/frontend/src/lib/ntrip-settings.ts) | 型定義・ローカルストレージ操作 |
| [settings/page.tsx](../../prototype/m1-gnss/frontend/src/app/settings/page.tsx) | NTRIP設定画面 |
| [session132/session-summary.md](../session132/session-summary.md) | セッションサマリー |
| [session133/session-plan.md](../session133/session-plan.md) | 次セッション計画 |

**次セッション（Session 133）でやること**:
- NTRIPクライアント実装（バックエンド側）

---

## Session 133 (2026-03-12)

**概要**: NTRIPクライアント実装（独自実装を採用）

**実施内容**:
1. **ntrip-clientクレート調査・不採用**
   - RTCMパース済みデータを返すため、生バイト転送に不向き
   - ADR-011を作成して判断を記録
2. **NTRIP API独自実装**
   - TCP + HTTP/1.0ベースのNTRIP Rev1プロトコルを直接実装
   - POST /api/ntrip/connect, disconnect, GET /status
3. **DeviceManagerにwrite_data追加**
   - RTCM転送用メソッド
4. **TDDスキルでレビュー**
   - テーブルテスト形式未使用の指摘
   - APIハンドラーテスト欠落の指摘

**作成ファイル**:
| ファイル | 内容 |
|----------|------|
| [ntrip_api.rs](../../prototype/m1-gnss/backend/src/web/ntrip_api.rs) | NTRIP API実装 |
| [ADR-011-ntrip-implementation.md](../../docs/adr/m1/ADR-011-ntrip-implementation.md) | ntrip-client不採用のADR |
| [session133/session-summary.md](../session133/session-summary.md) | セッションサマリー |
| [session134/session-plan.md](../session134/session-plan.md) | 次セッション計画 |

**次セッション（Session 134）でやること**:
- テストをテーブルテスト形式に書き直す
- フロントエンドに接続/切断ボタン追加

---

## Session 134 (2026-03-12)

**概要**: NTRIP APIテスト改善 + フロントエンド接続ボタン

**実施内容**:
1. **テーブルテスト形式への書き直し**
   - rstest追加、should_successパラメータ必須化
   - Base64境界値テスト追加
2. **APIハンドラー統合テスト追加**
   - GET /status、POST /disconnect のテスト
   - 28テスト全てパス
3. **フロントエンド接続ボタン**
   - 設定画面にNTRIP接続セクション追加
   - 接続/切断ボタン、状態表示、統計情報

**作成ファイル**:
| ファイル | 内容 |
|----------|------|
| [Cargo.toml](../../prototype/m1-gnss/backend/Cargo.toml) | rstest追加 |
| [ntrip_api.rs](../../prototype/m1-gnss/backend/src/web/ntrip_api.rs) | テーブルテスト形式 + API統合テスト |
| [settings/page.tsx](../../prototype/m1-gnss/frontend/src/app/settings/page.tsx) | NTRIP接続/切断ボタン追加 |

**追加対応**:
- NTRIPデバッグログ強化（タイムアウト、DNS解決ログ追加）
- 接続テスト実施 → DNS解決成功、TCP接続タイムアウト
- 原因: 会社ネットワークがポート2101をブロックの可能性

**次セッション（Session 135）でやること**:
- 別ネットワーク（モバイル回線等）でNTRIP接続テスト
- または残タスクの優先度確認

---

## Session 135 (2026-03-12)

**概要**: NTRIP接続テスト（別ネットワーク）+ GGA送信機能追加

**実施内容**:
1. **NTRIP自動切断の原因調査**
   - 原因: VRS型NTRIPサービスではGGAセンテンス送信が必須
   - 仕様書確認: 21-ntrip-protocol-spec.md 5.4節
2. **固定位置GGA送信機能を追加**
   - `generate_gga_sentence()` 関数追加
   - 接続直後に初回GGA送信 + 10秒間隔で定期送信
   - TcpStreamを読み書き分割（tokio::io::split）
3. **動作確認**
   - モバイル回線でNTRIP接続成功
   - RTCMデータ受信継続を確認

**作成ファイル**:
| ファイル | 内容 |
|----------|------|
| [ntrip_api.rs](../../prototype/m1-gnss/backend/src/web/ntrip_api.rs) | GGA送信機能追加 |
| [session135/session-summary.md](../session135/session-summary.md) | セッションサマリー |
| [session136/session-plan.md](../session136/session-plan.md) | 次セッション計画 |

**次セッション（Session 136）でやること**:
- device_id紐付け実装
- または屋外でのRTK動作確認

---

## Session 136 (2026-03-12)

**概要**: device_id紐付け実装の調査・設計

**実施内容**:
1. **残タスク現在地確認**
   - Session 131で整理した残タスクの進捗を確認
   - 残り4タスク: device_id紐付け、自動保存、u-center照合、GGA正式実装
2. **ドメインモデル確認**
   - 26-outdoor-inspection-domain-model.md: `device_id` = 「装置画面で登録済みの場合」
   - 19-gnss-unified-domain-model.md: シリアル番号で紐づけ
3. **実装調査**
   - `InspectionService::run_and_save` が屋内検査時にdeviceを登録（get_or_create）
   - `serial_number` がUNIQUE制約でキー
4. **DB現状確認**
   - devices: 4件、UNIQUE制約あり
   - indoor_inspections: 504件（テストデータ大量）
   - outdoor_inspection_results: 1件（device_id=NULL）
   - 問題: device_id=1のserial_numberが空文字

**決定事項**:
- device_id紐付けはバックエンドでserial_number→device_id解決
- 次セッションでDBクリーンアップ + 実装

**作成ファイル**:
| ファイル | 内容 |
|----------|------|
| [session136/session-summary.md](../session136/session-summary.md) | セッションサマリー |
| [session137/session-plan.md](../session137/session-plan.md) | 次セッション計画 |

**次セッション（Session 137）でやること**:
- DBクリーンアップ（異常データ削除）
- device_id紐付け実装

---

## Session 137 (2026-03-12)

**概要**: device_id紐付け実装 + シリアル番号混同バグ修正

**実施内容**:
1. **DBクリーンアップ**
   - devices id=1（空serial_number）を削除
   - indoor_inspections device_id=1 の100件を削除
   - outdoor_inspection_results テストデータ1件を削除
2. **BE: 屋外検査保存APIに serial_number 追加**
   - `SaveOutdoorResultRequest` に `serial_number` 追加
   - `get_device_by_serial` で device_id 解決
3. **FE: 保存時に serial_number を送信**
   - `saveResult` 引数を serialNumber に変更
4. **バグ発見: シリアル番号の混同**
   - USBシリアル vs F9Pチップシリアルを混同していた
   - DBにはF9Pチップシリアルが保存されていたが、接続時はUSBシリアルを取得していた
5. **バグ修正: F9Pシリアル取得実装**
   - `build_ubx_poll()` でUBX Pollコマンド生成
   - `query_f9p_serial()` でSEC-UNIQIDをPoll → F9Pシリアル取得
   - DeviceResponseに `f9p_serial` フィールド追加
   - 保存時に `f9p_serial` を使用

**変更ファイル**:
| ファイル | 内容 |
|----------|------|
| [common.rs](../../prototype/m1-gnss/backend/src/ubx/common.rs) | build_ubx_poll追加 |
| [manager.rs](../../prototype/m1-gnss/backend/src/device/manager.rs) | f9p_serial + query_f9p_serial |
| [device_api.rs](../../prototype/m1-gnss/backend/src/web/device_api.rs) | f9p_serial in response |
| [outdoor_inspection_api.rs](../../prototype/m1-gnss/backend/src/web/outdoor_inspection_api.rs) | serial_number追加 |
| [api.ts](../../prototype/m1-gnss/frontend/src/lib/api.ts) | f9p_serial追加 |
| [outdoor/page.tsx](../../prototype/m1-gnss/frontend/src/app/inspections/outdoor/page.tsx) | f9p_serial送信 |

**学び**:
- USBシリアル（FTDIチップ）とF9Pチップシリアル（SEC-UNIQID）は別物
- DB紐付けにはF9Pチップシリアルを使用する必要がある

**次セッション（Session 138）でやること**:
- 残タスク消化（自動保存 or u-center照合）
- シリアル番号定義のドキュメント整備

---

## Session 138 (2026-03-12)

**概要**: 自動保存実装 + GGA正式実装（F9P位置を使用）

**実施内容**:
1. **自動保存の実装**
   - 検査完了時に`useEffect`で自動保存
   - 手動保存ボタン削除
2. **GGA正式実装**
   - AppStateに`current_position`フィールド追加
   - NAV-STATUS APIでNAV-PVTも取得し位置更新
   - NTRIP APIがcurrent_positionを参照してGGA生成
3. **IOエラー調査（方針決定）**
   - NTRIP仕様書確認（21-ntrip-protocol-spec.md）
   - 固定位置問題 → 今回修正済み
   - シリアルポート競合 → 次回実機確認で検証

**変更ファイル**:
| ファイル | 内容 |
|----------|------|
| outdoor/page.tsx | 自動保存useEffect追加、手動ボタン削除 |
| device_api.rs | CurrentPosition構造体、AppStateにcurrent_position追加 |
| nav_status_api.rs | NAV-PVT取得・位置更新処理追加 |
| ntrip_api.rs | generate_gga_sentence(lat, lon)に変更 |
| manager.rs | write_dataにエラーログ追加 |

**残タスク**:
1. IOエラー調査（次回実機確認）
2. NTRIPライフサイクル整理
3. u-center照合（最後）

**次セッション（Session 139）でやること**:
- 実機でGGA正式実装の動作確認
- IOエラーがまだ発生するか確認

---

## Session 139 (2026-03-12)

**概要**: API並行リクエスト問題の調査 → Periodic Output採用を決定

**実施内容**:
1. **API並行リクエスト問題の調査**
   - 藤田さんがログを貼り付けて問題を指摘
   - 複数APIが同時にポーリング → パースエラー多発 → 500エラー
2. **原因特定**
   - 各FEパネルが独立したsetIntervalでポーリング（1秒〜5秒ごと）
   - シリアルポートは1チャネル → 複数ポーリングで応答が混在
3. **解決策の検討**
   - FE側でシーケンシャル化、BE側で統合API、Periodic Outputの3案
4. **結論: Periodic Output（定期出力）を採用**
   - u-blox仕様書から「Periodic/polled」タイプを確認
   - CFG-MSGOUT設定で定期出力に設定可能
   - 次セッションでエビデンスを詳細確認

**決定事項**:
| 項目 | 決定 |
|------|------|
| API並行問題の解決策 | Periodic Output（定期出力）を採用 |

**作成ファイル**:
| ファイル | 内容 |
|----------|------|
| [session139/session-summary.md](../session139/session-summary.md) | セッションサマリー |
| [session140/session-plan.md](../session140/session-plan.md) | 次セッション計画 |

**次セッション（Session 140）でやること**:
- u-blox仕様書でPeriodic Outputのエビデンス詳細確認
- CFG-MSGOUT設定の仕様確認
- Periodic Output実装の設計

---
