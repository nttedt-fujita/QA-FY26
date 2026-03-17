# Session 220 計画

**目的**: 仕様書索引整備 + 推測/事実分離ルール追加 + reset-configテスト再実行

**前提**: Session 219でUSB抜き差し時にBBR設定が消える問題発覚。原因は仮説段階。

---

## やること

| # | 作業 | 読むべきファイル | 参照コマンド |
|---|------|-----------------|--------------|
| 1 | ルール追加: 推測と事実の分離 | ~/.claude/rules/ | - |
| 2 | 仕様書索引の整備（CLAUDE.mdに追記） | prototype/m1-gnss/CLAUDE.md | - |
| 3 | BBR/Flash仕様確認（p.223抽出） | sources/u-blox PDF | - |
| 4 | set-periodic-output APIをFlash対応に修正 | set_periodic_output_api.rs | - |
| 5 | reset-configテスト再実行（Phase 1/2） | - | make connect, make set-periodic-output, make message-scan, make reset-config |

---

## 詳細

### 1. ルール追加: 推測と事実の分離

新規ルール `17-fact-vs-hypothesis.md` を作成:
- 仮説は「未確認」「仮説」と明記する
- 仕様書で確認した事実は出典を明記する
- 「〜のはず」「〜だろう」は使わない

### 2. 仕様書索引の整備

M1-GNSS CLAUDE.mdに以下を追記:
- CFG-VALSET仕様: sessions/session214/cfg-valget-spec.md (p.96-98)
- CFG-CFG仕様: sessions/session211/cfg-cfg-spec.md (p.63-68)
- Configuration layers: 要抽出 (p.223)

### 3. BBR/Flash仕様確認

u-blox F9 HPG 1.32 Interface Description p.223「Configuration layers」を抽出:
- BBRの永続性条件を確認
- バッテリーバックアップの要件を確認

### 4. set-periodic-output API修正

`Layer::RamBbrFlash` (0x07) を使用してFlashにも保存するよう修正。

### 5. reset-configテスト再実行

Session 219の計画を継続:
- Phase 1: 定期出力設定 → USB抜き差し3回で維持確認
- Phase 2: reset-config → USB抜き差し3回で消失確認

---

## 参照

- [Session 219 summary](../session219/session-summary.md)
- [CFG-VALSET仕様](../session214/cfg-valget-spec.md)
- [38-ublox-config-management.md](../../docs/missions/m1-sensor-evaluation/gnss/38-ublox-config-management.md)
