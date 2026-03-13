# セッション履歴: Session 151〜160

## Session 151 (2026-03-12)

**概要**: 定期出力無効化問題の原因特定・修正

**実施内容**:
1. **「不明データ」の正体を特定**
   - ログ分析でMON-SPANのスペクトラムデータと判明
   - バイト値0x20〜0xA0（dBHz値のパターン）
2. **根本原因を特定**
   - `Layer::Bbr` のみでは即座に有効にならない
   - BBRは電源再投入時に有効になる設定
3. **修正**
   - `Layer::RamAndBbr` (0x03) を追加
   - RAM + BBR両方に書き込むことで即座に有効化
4. **実機検証**
   - 「不明データあり」が0件に改善
   - ただしタイムアウト・パースエラーは依然発生

**決定事項**:
| 項目 | 決定 |
|------|------|
| 定期出力無効化レイヤー | `Layer::RamAndBbr` |

**変更ファイル**:
| ファイル | 内容 |
|----------|------|
| [cfg_valset.rs](../../prototype/m1-gnss/backend/src/ubx/cfg_valset.rs) | Layer::RamAndBbr追加 |
| [device_api.rs](../../prototype/m1-gnss/backend/src/web/device_api.rs) | RamAndBbrに変更 |
| [ADR-012](../../docs/adr/m1/ADR-012-periodic-output-and-unified-api.md) | 変更履歴追記 |

**残った課題**:
- タイムアウト・パースエラーが依然発生（別の原因）

**次セッション（Session 152）でやること**:
- **単発ポーリングの動作確認**（NAV-PVT, NAV-SATなど個別に）
- 単発で問題なければ統合APIの問題を切り分け
- タイムアウト・パースエラーの原因調査

---

## Session 152 (2026-03-12)

**概要**: 統合APIのタイムアウト問題を解決

**実施内容**:
1. **問題の再現確認**
   - 単発APIは全て成功
   - 統合APIでタイムアウト・パースエラー発生
2. **原因特定**
   - テストスクリプトで応答時間計測
   - NAV-STATUS: 1.5秒、MON-SPAN: 1.0秒
   - **統合APIのタイムアウト500msが短すぎる**
3. **修正**
   - `gnss_state_api.rs` のタイムアウトを500ms → 2000msに延長
4. **検証**
   - 100回連続テスト: 成功100 / 失敗0

**決定事項**:
| 項目 | 決定 |
|------|------|
| 統合APIタイムアウト | 2000ms（2秒） |

**変更ファイル**:
| ファイル | 内容 |
|----------|------|
| [gnss_state_api.rs](../../prototype/m1-gnss/backend/src/web/gnss_state_api.rs) | タイムアウト延長 |
| [tools/test-polling.sh](../../tools/test-polling.sh) | 応答時間計測スクリプト |
| [tools/test-100.sh](../../tools/test-100.sh) | 100回連続テストスクリプト |

**残った課題**:
- FE側の状態表示改善（BE処理中が見えない、リクエスト重複）
- ADR-012の更新

**次セッション（Session 153）でやること**:
- FE側の状態表示改善
  - リクエスト重複防止
  - 「取得中」「終了処理中」表示

---

## Session 153 (2026-03-12)

**概要**: 屋外動作確認 + FWバージョン取得修正

**実施内容**:
1. **屋外テストデータ確認**
   - DBに27件の検査結果が保存されていることを確認
   - **問題発見**: L1 C/N0が全て0dBHz
   - L2受信率は50-56%で取得できている
2. **FWバージョン取得方法の修正**
   - 小板橋さんからの指摘: u-centerのFWVERはMON-VERのextension内
   - `sw_version` → `extensions`の`FWVER=...`を抽出するよう修正

**決定事項**:
| 項目 | 決定 |
|------|------|
| FWバージョン取得元 | MON-VER extension内の`FWVER=`から抽出 |

**変更ファイル**:
| ファイル | 内容 |
|----------|------|
| [mon_ver.rs](../../prototype/m1-gnss/backend/src/ubx/mon_ver.rs) | `fw_version()`, `protocol_version()` メソッド追加 |
| [engine.rs](../../prototype/m1-gnss/backend/src/inspection/engine.rs) | FwVersion解析でFWVERを使うように変更 |

**残った課題**:
- L1 C/N0が0の問題（NAV-SIG集計ロジック要調査）
- FE側の状態表示改善

**次セッション（Session 154）でやること**:
- **FE側の状態表示改善**（リクエスト重複防止、取得中表示）
- L1 C/N0が0になる原因調査

---

## Session 154 (2026-03-12)

**概要**: FE側の状態表示改善

**実施内容**:
1. **FE状態表示改善**
   - InspectionStateを拡張: `idle | starting | running | completing | completed`
   - ボタン連打防止（starting状態で重複開始を防止）
   - 「開始中...」「集計中...」の表示を追加
   - ビルド成功確認済み
2. **L1 C/N0=0問題の調査**
   - コードレビュー完了
   - 原因候補: GPS L1信号が0件の場合にminL1Cno=0
   - 屋内のため再現確認不可

**変更ファイル**:
| ファイル | 内容 |
|----------|------|
| [useOutdoorInspection.ts](../../prototype/m1-gnss/frontend/src/hooks/useOutdoorInspection.ts) | InspectionState拡張 |
| [outdoor/page.tsx](../../prototype/m1-gnss/frontend/src/app/inspections/outdoor/page.tsx) | 状態表示UI更新 |

**残った課題**:
- L1 C/N0=0問題のデバッグログ追加（次回の屋外テスト前に実施）
- u-centerコンフィグファイル解析（オプション）

**次セッション（Session 155）でやること**:
- FEにL1 C/N0デバッグ用のconsole.logを追加
- 屋外テストでGPS L1信号の有無とcno値を確認

---

## Session 155 (2026-03-13)

**概要**: NTRIP/RTK分離テストの方針整理

**実施内容**:
1. **L1 C/N0デバッグログ追加**（実装済み、テスト未実施）
2. **NTRIP/RTK分離の懸念点整理**
   - 現状: NTRIP接続とRTK検査が独立操作
   - 疑問: NTRIP接続中に定期出力読み取りは並行動作できるか？
3. **仕様書リストアップ**
   - 20-ntrip-rtk-implementation.md
   - 21-ntrip-protocol-spec.md
   - 22-rtk-configuration.md
   - 32-cfg-msgout-periodic-output.md
   - 外部: ZED-F9P Integration Manual
4. **確認項目リストアップ**
   - ZED-F9P全二重通信、RTCMデータ流量、DeviceManagerロック等
5. **TDDスキル改善方針**
   - 「抜け漏れありますか？」→「この判断基準に照らして見落としはありますか？」

**作成ファイル**:
| ファイル | 内容 |
|----------|------|
| [ntrip-rtk-investigation-plan.md](../session155/ntrip-rtk-investigation-plan.md) | 調査計画・確認項目・セッション計画 |
| [tdd-workflow-tradeoff-memo_20260313.md](../session155/tdd-workflow-tradeoff-memo_20260313.md) | TDDワークフロー改善メモ（ユーザー作成） |

**配置PDF（Session 156で抽出予定）**:
| ファイル | 内容 |
|----------|------|
| u-blox-F9-HPG-1.32_InterfaceDescription_UBX-22008968.pdf | ZED-F9P Interface Description |
| Xiao_2017_IOP_Conf._Ser.__Mater._Sci._Eng._242_012131.pdf | 学術論文 |

**変更ファイル**:
| ファイル | 内容 |
|----------|------|
| [outdoor/page.tsx](../../prototype/m1-gnss/frontend/src/app/inspections/outdoor/page.tsx) | L1 C/N0デバッグログ追加 |

**残った課題**:
- L1 C/N0=0問題（デバッグログ追加済み、屋外テスト未実施）
- 仕様書の実際の調査（Session 156）
- TDDスキルの実際の更新（Session 156）

**次セッション（Session 156）でやること**:
1. TDDスキル改善（ファイル更新）
2. プロジェクト内仕様書（20, 21, 22, 32番）を読む
3. 確認項目に対する回答をまとめる

---

## Session 156 (2026-03-13)

**概要**: NTRIP/RTK仕様調査 + PDFルール追加

**状態**: 低調・寝不足・疲労気味

**実施内容**:
1. **PDFルール追加**
   - `~/.claude/rules/15-pdf-handling.md` を作成
   - PDFを直接読まず、Pythonスクリプトで抽出してから読むルールを強制
2. **NTRIP/RTK仕様調査**
   - プロジェクト内仕様書4つ読了（20, 21, 22, 32番）
   - u-blox Interface DescriptionからMON-COMMSセクション抽出（p.126-130）
3. **確認項目に対する回答**
   - ZED-F9P全二重通信: 間接的証拠あり（tx/rxバッファ独立）、直接的記述は未発見
   - RTCMデータ流量: 50-500 Bytes/sec（低帯域、問題なし）
   - DeviceManagerのロック: 要コード確認（Session 157）
   - 定期出力設定: NTRIP接続前後で変更不要

**作成ファイル**:
| ファイル | 内容 | 状態 |
|----------|------|------|
| [ntrip-rtk-spec-findings.md](../session156/ntrip-rtk-spec-findings.md) | 仕様調査結果 | 作業中 |
| [ublox-toc.md](../session156/ublox-toc.md) | u-blox PDF目次（p.1-5） | 作業中 |
| [ublox-toc-2.md](../session156/ublox-toc-2.md) | u-blox PDF目次（p.6-14） | 作業中 |
| [ublox-mon-comms.md](../session156/ublox-mon-comms.md) | MON-COMMS仕様（p.126-130） | 作業中 |

**残課題**:
- 全二重通信の直接的確認（Integration Manual p.270-274）
- DeviceManagerのコード確認

**次セッション（Session 157）でやること**:
1. 全二重通信の追加調査（Integration Manual p.270-274）
2. DeviceManagerのコード確認
3. 設計判断（ADR-013が必要か）

---

## Session 157 (2026-03-13)

**概要**: NTRIP/RTK調査完了 + 方針転換（バックエンドAPIテストへ）

**実施内容**:
1. **全二重通信の追加調査**
   - u-blox Interface Description p.270-274を抽出
   - 「full duplex」直接記述なし、ただしCFG-UARTxINPROT/OUTPROTが独立設定 → 全二重前提
2. **DeviceManager/NTRIP APIコード確認**
   - ロックは`AppState`の`Mutex<DeviceManager>`で実現
   - RTCM転送: 数msのロック保持、理論上は並行動作可能
3. **要求の再確認**
   - ユーザーの本来の問題: NTRIP接続後に屋外検査すると失敗
   - 調査結果では「理論上は動くはず」だが、実際にはエラー発生
   - 方針転換: バックエンドAPIだけでテストして問題を切り分ける

**作成ファイル**:
| ファイル | 内容 |
|----------|------|
| [ublox-uart-config.md](../session157/ublox-uart-config.md) | u-blox PDF p.270-274抽出 |

**更新ファイル**:
| ファイル | 内容 |
|----------|------|
| [ntrip-rtk-spec-findings.md](../session156/ntrip-rtk-spec-findings.md) | 調査完了、追加調査結果反映 |

**決定事項**:
| 項目 | 決定 |
|------|------|
| 次のアプローチ | バックエンドAPIだけでNTRIP + UBXポーリングをテスト |
| ADR-013 | 不要（問題特定が先） |

**次セッション（Session 158）でやること**:
1. バックエンドAPIテスト用Makefileターゲット作成
2. 実機テスト（curlでNTRIP接続 + UBXポーリング）
3. 問題切り分け

---

## Session 158 (2026-03-13)

**概要**: NTRIP + UBXポーリング テスト準備（デバッグログ・Makefileターゲット整備）

**実施内容**:
1. **APIエンドポイント確認**
   - NTRIP API、統合API（gnss-state）の構成確認
2. **デバッグログ追加**
   - `ntrip_api.rs`: RTCM転送時のロック取得/解放タイミング
   - `gnss_state_api.rs`: 各メッセージポーリングの所要時間
3. **Makefileターゲット整備**
   - `make rtk-log/rtk-start/rtk-poll/rtk-stop`
4. **設定ファイル対応**
   - `ntrip.conf` から認証情報を読み込む仕組み

**作成ファイル**:
| ファイル | 内容 |
|----------|------|
| [ntrip.conf.example](../../prototype/m1-gnss/ntrip.conf.example) | NTRIP設定テンプレート |
| [tools/test-rtk-flow.sh](../../prototype/m1-gnss/tools/test-rtk-flow.sh) | RTK統合テストスクリプト |

**変更ファイル**:
| ファイル | 内容 |
|----------|------|
| [makefiles/api.mk](../../prototype/m1-gnss/makefiles/api.mk) | RTKデバッグテスト用ターゲット追加 |
| [gnss_state_api.rs](../../prototype/m1-gnss/backend/src/web/gnss_state_api.rs) | デバッグログ追加 |
| [ntrip_api.rs](../../prototype/m1-gnss/backend/src/web/ntrip_api.rs) | デバッグログ追加 |

**残課題**:
- 実機テスト未実施（準備のみ完了）

**次セッション（Session 159）でやること**:
1. ntrip.conf作成
2. make rtk-start/rtk-poll/rtk-stop で実機テスト
3. ログから問題特定

---
