# セッション履歴: Session 211〜220

## Session 211 (2026-03-16)

**概要**: 古い機の定期出力問題の根本対処を検討（仕様理解）

**実施内容**:
1. CFG-CFG仕様の確認
   - u-blox F9 HPG 1.32 Interface Description p.64から抽出
   - clearMask/saveMask/loadMask の動作を確認
2. u-center メニューとUBXコマンドの対応調査
   - u-center User Guide (UBX-13005250) R30を確認
   - Receiver → Action → Save/Load/Revert Config = CFG-CFG
   - Tools → Receiver Configuration... = CFG-VALGET（設定バックアップ）
3. 設定レイヤーの仕組みを理解
   - ROM/Flash/BBR/RAMの4レイヤー
   - 起動時の優先順位: Flash > BBR > ROM → RAM
4. ドキュメント作成
   - [38-ublox-config-management.md](../../docs/missions/m1-sensor-evaluation/gnss/38-ublox-config-management.md) を作成

**作成ファイル**:
| ファイル | 内容 |
|----------|------|
| [38-ublox-config-management.md](../../docs/missions/m1-sensor-evaluation/gnss/38-ublox-config-management.md) | u-blox設定管理の仕組み解説（新規） |
| [gnss/README.md](../../docs/missions/m1-sensor-evaluation/gnss/README.md) | ドキュメント一覧に38を追加 |
| [cfg-cfg-spec.md](../session211/cfg-cfg-spec.md) | CFG-CFG仕様PDF抽出（p.63-68） |
| [ucenter-*.md](../session211/) | u-center User Guide各セクション抽出 |

**結論**: CFG-CFG（clearMask + loadMask）でBBRクリアが根本対処として有効

**次セッション**: [session212/session-plan.md](../session212/session-plan.md)

---

