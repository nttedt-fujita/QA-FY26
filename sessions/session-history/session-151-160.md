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
