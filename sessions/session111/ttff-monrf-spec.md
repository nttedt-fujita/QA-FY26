# TTFF・MON-RF仕様調査結果

**セッション**: 111
**作成日**: 2026-03-11
**参照PDF**: UBX-22008968 (u-blox F9 HPG 1.32)

---

## 1. TTFF測定（NAV-STATUS）

### 仕様

| メッセージ | Class/ID | 関連フィールド |
|-----------|----------|---------------|
| NAV-STATUS | 0x01 0x03 | ttff, msss |

**ペイロード（16バイト）**:

| Offset | Type | Name | Unit | Description |
|--------|------|------|------|-------------|
| 0 | U4 | iTOW | ms | GPSタイム |
| 4 | U1 | gpsFix | - | Fix種別（0=no fix, 2=2D, 3=3D, etc.） |
| 5 | X1 | flags | - | gpsFixOk, diffSoln等 |
| 6 | X1 | fixStat | - | diffCorr, carrSolnValid |
| 7 | X1 | flags2 | - | spoofDetState, carrSoln等 |
| **8** | **U4** | **ttff** | **ms** | **Time to First Fix（ミリ秒）** |
| 12 | U4 | msss | ms | Milliseconds since Startup/Reset |

### TTFFの取得方法

1. **電源ON/リセット後**、NAV-STATUSをポーリング
2. `gpsFix >= 2`（2D-fix以上）かつ `gpsFixOk=1` を確認
3. `ttff` フィールドを読み取り → **ミリ秒単位でTTFF取得**

### Cold/Warm/Hot Startの違い

| 種別 | 条件 | 業界標準 |
|------|------|---------|
| Cold Start | エフェメリス・アルマナック・位置・時刻なし | 2-4分 |
| Warm Start | アルマナック・おおよその位置・時刻あり | 30-45秒 |
| Hot Start | エフェメリス・位置・時刻あり（短時間停止） | ≤22秒 |

### 屋外検査での用途

- 優先度: **中**（O6）
- 測定手順が確立されていない（ヒアリング必要）
- Cold Startの再現が難しい（メモリクリアが必要）

---

## 2. ジャミング検出（MON-RF）

### 仕様

| メッセージ | Class/ID | 繰り返し単位 |
|-----------|----------|-------------|
| MON-RF | 0x0a 0x38 | 24バイト/ブロック |

**ヘッダ（4バイト）**:

| Offset | Type | Name | Description |
|--------|------|------|-------------|
| 0 | U1 | version | メッセージバージョン (0x00) |
| 1 | U1 | nBlocks | RFブロック数 |
| 2 | U1[2] | reserved0 | 予約 |

**繰り返しグループ（nBlocks回、各24バイト）**:

| Offset | Type | Name | Description |
|--------|------|------|-------------|
| 4+n×24 | U1 | blockId | 0=L1, 1=L2/L5 |
| 5+n×24 | X1 | flags | **jammingState** (bits 1..0) |
| 6+n×24 | U1 | antStatus | アンテナ状態 (0=INIT, 2=OK, 3=SHORT, 4=OPEN) |
| 7+n×24 | U1 | antPower | アンテナ電源 (0=OFF, 1=ON) |
| 8+n×24 | U4 | postStatus | POST状態ワード |
| 16+n×24 | U2 | noisePerMS | ノイズレベル |
| 18+n×24 | U2 | agcCnt | AGCモニタ (0-8191) |
| 20+n×24 | U1 | cwSuppression | CW干渉抑制レベル (0-255) |
| 21+n×24 | I1 | ofsI | I成分インバランス |
| 22+n×24 | U1 | magI | I成分マグニチュード |
| 23+n×24 | I1 | ofsQ | Q成分インバランス |
| 24+n×24 | U1 | magQ | Q成分マグニチュード |

### jammingState（重要）

| 値 | 意味 | 対応 |
|---|------|------|
| 0 | unknown/disabled | 機能無効または不明 |
| 1 | **ok** | ジャミングなし |
| 2 | **warning** | 干渉あり、Fixは正常 |
| 3 | **critical** | 干渉あり、Fixできない |

### 屋外検査での用途

- 優先度: **低**（O7）
- 異常検出の補助情報（正常時は常に1）
- L1/L2それぞれのブロックでjammingStateを確認可能

---

## 3. 実装難易度

| 機能 | 難易度 | 理由 |
|------|-------|------|
| NAV-STATUS ttff取得 | **低** | 既存パーサー（nav_status.rs）にttffフィールド追加のみ |
| MON-RFパーサー | **低** | 単純な構造、繰り返し解析のみ |
| TTFF測定手順 | **中** | Cold/Warm/Hot Startの再現方法が課題 |

---

## 4. 参照

- [ubx-nav-status-pages.md](../session64/ubx-nav-status-pages.md) — NAV-STATUS仕様（PDF抽出）
- [outdoor-inspection-needs.md](../session106/outdoor-inspection-needs.md) — 要求整理
- [ADR-008](../../docs/adr/m1/ADR-008-outdoor-inspection-criteria.md) — 屋外検査合格基準
