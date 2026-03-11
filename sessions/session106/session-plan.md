# Session 106 計画

**目的**: MON-SPAN仕様確認 + 屋外検査要求整理

---

## 背景

Session 105で屋外検査項目を調査したが、以下が未確認:
- MON-SPANの詳細仕様（128MHz/256点の取得方法）
- 要求（What）の整理が不十分

---

## やること

### 1. u-blox仕様書確認（MON-SPAN）

**確認すべき点**:
- メッセージ構造（Class/ID、ペイロード形式）
- RF1（L1帯）とRF2（L2帯）の選択方法
- 256点が128MHzをどうカバーするか（分解能）
- ポーリングか自動出力か

**参照**: UBX-22008968 p.134

### 2. 屋外検査の要求整理

**確認すべき点**:
1. 小板橋さん・末永さんヒアリング結果（07-cross-sheet-findings.mdの問い）
2. 合格基準の確定状況
3. Phase 1（屋内検査）と屋外検査の範囲分担

**参照**:
- [07-cross-sheet-findings.md](../../docs/missions/m1-sensor-evaluation/gnss/07-cross-sheet-findings.md)
- [sessions/session16/gnss-hearing-koitabashi-01.md](../session16/gnss-hearing-koitabashi-01.md)

---

## 成果物

- MON-SPAN仕様メモ
- 屋外検査要求整理（What）

---

## 参照資料

- [Session 105 サマリー](../session105/session-summary.md)
- [08-ubx-protocol-index.md](../../docs/missions/m1-sensor-evaluation/gnss/08-ubx-protocol-index.md)

---

*計画作成: 2026-03-11 Session 105終了時*
