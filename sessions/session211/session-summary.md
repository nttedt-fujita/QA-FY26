# Session 211 サマリー

**目的**: 古い機の定期出力問題の根本対処を検討（仕様理解）

**日付**: 2026-03-16

---

## 実施内容

1. **CFG-CFG仕様の確認**
   - u-blox F9 HPG 1.32 Interface Description p.64から抽出
   - clearMask/saveMask/loadMask の動作を確認

2. **u-center メニューとUBXコマンドの対応調査**
   - u-center User Guide (UBX-13005250) R30を確認
   - Receiver → Action → Save/Load/Revert Config = CFG-CFG
   - Tools → Receiver Configuration... = CFG-VALGET（設定バックアップ）

3. **設定レイヤーの仕組みを理解**
   - ROM/Flash/BBR/RAMの4レイヤー
   - 起動時の優先順位: Flash > BBR > ROM → RAM

4. **ドキュメント作成**
   - [38-ublox-config-management.md](../../docs/missions/m1-sensor-evaluation/gnss/38-ublox-config-management.md) を作成
   - 設定管理の仕組み、コマンド比較、図解を記載

---

## 結果

- u-blox設定管理の仕組みを理解した
- CFG-CFG（clearMask + loadMask）でBBRクリアが根本対処として有効であることを確認
- 実装検討は次セッションへ

---

## 作成・更新ファイル

| ファイル | 内容 |
|----------|------|
| [38-ublox-config-management.md](../../docs/missions/m1-sensor-evaluation/gnss/38-ublox-config-management.md) | u-blox設定管理の仕組み解説（新規） |
| [gnss/README.md](../../docs/missions/m1-sensor-evaluation/gnss/README.md) | ドキュメント一覧に38を追加 |

### 作業ファイル（sessions/session211/）

| ファイル | 内容 |
|----------|------|
| cfg-cfg-spec.md | CFG-CFG仕様PDF抽出（p.63-68） |
| ucenter-*.md | u-center User Guide各セクション抽出 |
| u-center-config/*.txt | u-centerで保存した設定ファイル（参考） |

---

## 未対応（次セッションへ）

1. CFG-CFGによる設定リセットAPIの実装
2. 代替案: 全NAVメッセージ一括無効化の検討
3. 実機での動作確認

---

## 参照

- [Session 210 summary](../session210/session-summary.md)
- u-blox F9 HPG 1.32 Interface Description (UBX-22008968) p.64
- u-center User Guide (UBX-13005250) p.24, p.64-68
