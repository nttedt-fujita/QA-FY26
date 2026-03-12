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
