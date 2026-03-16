# Session 214 サマリー

**日付**: 2026-03-17
**目的**: CFG-CFG仕様の再確認 + 実装修正

---

## 実施内容

### 1. CFG-CFG仕様の再確認

Session 211で抽出した仕様書（p.64）を再確認:

**重要な発見**:
- `deviceMask` は `clearMask` / `saveMask` にのみ適用される
- `loadMask` には適用されない（残っている下位レイヤーから再構築）

### 2. CFG-VALGET仕様の確認

p.95-96を新たに抽出し、レイヤー別読み出し方法を確認:
- リクエスト時に `layer` フィールドで指定（0=RAM, 1=BBR, 2=Flash, 7=Default）

### 3. 問題の原因特定

Session 213で効かなかった理由:
- 現在の実装: `deviceMask = 0x01` (BBRのみ)
- 設定がFlashに保存されていたため、BBRをクリアしても効果なし

### 4. 実装修正

`reset_config_to_default()` を修正:
- `deviceMask` を `0x03` (BBR + Flash) に変更
- テスト修正、全4件パス

---

## 変更ファイル

| ファイル | 内容 |
|----------|------|
| [cfg_cfg.rs](../../prototype/m1-gnss/backend/src/ubx/cfg_cfg.rs) | deviceMaskをBBR+Flashに変更 |

## 作成ファイル

| ファイル | 内容 |
|----------|------|
| [cfg-valget-spec.md](cfg-valget-spec.md) | CFG-VALGET仕様（p.96-98抽出） |
| [cfg-valget-request-spec.md](cfg-valget-request-spec.md) | CFG-VALGETリクエスト仕様（p.95抽出） |

---

## 残タスク

- [ ] 実機テスト（古い機でreset-config APIが効くか確認）

---

## 次セッションでやりたいこと（ユーザーからの提案）

1. **PDF仕様書のインデックス作成** - リポジトリ内のPDFごとに目次的なファイルを作成
2. **全体の進捗整理** - M1の作業範囲、M3/M4のAI見積作成時期

---

## 参照

- [session-plan.md](session-plan.md)
- [38-ublox-config-management.md](../../docs/missions/m1-sensor-evaluation/gnss/38-ublox-config-management.md)
