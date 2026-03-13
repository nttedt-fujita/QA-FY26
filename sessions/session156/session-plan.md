# Session 156 計画

**目的**: NTRIP/RTK分離のための仕様書調査 + TDDスキル改善

**前提**: Session 155で方針整理完了

---

## やること

| # | 作業 | 読むべきファイル |
|---|------|-----------------|
| 1 | TDDスキル改善 | ~/.claude/skills/tdd-practice/SKILL.md, sessions/session155/tdd-workflow-tradeoff-memo_20260313.md |
| 2 | PDF仕様書から目次抽出 | sessions/session155/*.pdf |
| 3 | 必要部分をmdに抽出 | （目次確認後に決定） |
| 4 | プロジェクト内仕様書を読む | docs/missions/m1-sensor-evaluation/gnss/20, 21, 22, 32番 |
| 5 | 確認項目に回答をまとめる | sessions/session155/ntrip-rtk-investigation-plan.md |

---

## 詳細

### 1. TDDスキル改善

Session 155のメモに基づいて、TDDスキルを更新する。

**変更内容**:
- 各フェーズにスコープ限定の判断基準を追加
- 「抜け漏れありますか？」→「この判断基準に照らして、優先度の高い見落としはありますか？」

### 2. PDF仕様書の抽出

Session 155に配置されたPDF:
- `u-blox-F9-HPG-1.32_InterfaceDescription_UBX-22008968.pdf` — ZED-F9P Interface Description
- `Xiao_2017_IOP_Conf._Ser.__Mater._Sci._Eng._242_012131.pdf` — 学術論文

**抽出フロー**:
1. Pythonスクリプトで目次抽出
2. 確認項目に関連するセクションを特定
3. 必要部分をmdに抽出

**確認項目（PDF確認対象）**:
- ZED-F9Pは全二重通信か
- RTCMデータの流量・頻度
- シリアルポートの読み書きタイミング

### 3. プロジェクト内仕様書

以下を読んで、確認項目に対する回答をまとめる:

- 20-ntrip-rtk-implementation.md
- 21-ntrip-protocol-spec.md
- 22-rtk-configuration.md
- 32-cfg-msgout-periodic-output.md

### 4. 成果物

- `session156/pdf-toc-extract.md` — PDF目次抽出結果
- `session156/ntrip-rtk-spec-findings.md` — 仕様書調査結果
- TDDスキル更新

---

## 参照

- [Session 155 方針整理](../session155/ntrip-rtk-investigation-plan.md)
- [TDDメモ](../session155/tdd-workflow-tradeoff-memo_20260313.md)
- [PDF: u-blox Interface Description](../session155/u-blox-F9-HPG-1.32_InterfaceDescription_UBX-22008968.pdf)
