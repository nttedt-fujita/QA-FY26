# Session 220 サマリー

**日付**: 2026-03-17
**目的**: 仕様書索引整備 + 推測/事実分離ルール追加 + set-periodic-output Flash対応

---

## 実施内容

1. **ルール追加: 推測と事実の分離**
   - `17-fact-vs-hypothesis.md` を作成
   - 仮説は「未確認」と明記、事実は出典を明記

2. **仕様書索引の整備**
   - gnss/README.mdに「PDF仕様抽出状態」セクションを追加
   - 複数PDF対応（IF, IM, NTRIP, UC）
   - CLAUDE.mdはgnss/README.mdを参照するだけに簡素化

3. **PDF抽出ルール拡張**
   - `15-pdf-handling.md`に「抽出後の整備（必須）」を追加
   - 抽出したら即座に索引テーブルを更新することを強制

4. **BBR/Flash仕様確認**
   - p.223-225を抽出 → config-layers-spec.md
   - **事実確認**: BBRはバッテリーバックアップがないと電源断で消える（出典: p.224）
   - **事実確認**: Flashは永続的に保存される

5. **set-periodic-output APIをFlash対応に修正**
   - `Layer::RamAndBbr` → `Layer::RamBbrFlash` に変更
   - api.mkのコメントも更新

---

## 仕様確認結果（客観的事実）

**Configuration layers**（出典: u-blox F9 HPG 1.32 IF p.224）:

| レイヤー | 永続性 | 条件 |
|----------|--------|------|
| RAM | 電源断で消える | - |
| BBR | バッテリーバックアップ供給中は維持 | **バッテリーがないと消える** |
| Flash | 永続的 | 外部Flashメモリが必要 |

→ Session 219で発覚したBBR消失問題の原因が特定できた

---

## 変更ファイル

| ファイル | 内容 |
|----------|------|
| ~/.claude/rules/17-fact-vs-hypothesis.md | 新規: 推測と事実の分離ルール |
| ~/.claude/rules/15-pdf-handling.md | 拡張: 抽出後の整備セクション追加 |
| docs/.../gnss/README.md | 拡張: PDF仕様抽出状態セクション（複数PDF対応） |
| prototype/m1-gnss/CLAUDE.md | 簡素化: gnss/README.mdを参照 |
| set_periodic_output_api.rs | 修正: Layer::RamBbrFlashに変更 |
| api.mk | 修正: コメント更新 |

## 作成ファイル

| ファイル | 内容 |
|----------|------|
| sessions/session220/config-layers-spec.md | PDF抽出: Configuration layers (p.223-225) |

---

## 残タスク

- reset-configテスト再実行（Phase 1/2）
  - set-periodic-output（Flash対応版）で定期出力設定
  - USB抜き差し3回で維持確認
  - reset-configで消失確認

---

## 使用したコマンド

なし（実機テスト未実施）

---

## 次セッション

[session221/session-plan.md](../session221/session-plan.md)
