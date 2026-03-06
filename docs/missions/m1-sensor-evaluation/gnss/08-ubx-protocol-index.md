# UBXプロトコル仕様書 索引（Phase 2用）

**元PDF**: u-blox F9 HPG 1.32 Interface Description (UBX-22008968)
**性格**: UBXプロトコル仕様書 — u-centerがF9Pチップと通信するときのメッセージ形式定義
**用途**: Phase 1では不要。**Phase 2（ツール設計）で使う**

---

## 主要メッセージ一覧

| メッセージ | ページ | 取得できるデータ | 評価用途 |
|-----------|--------|----------------|---------|
| UBX-NAV-PVT | p.145 | fixType, **carrSoln**, numSV, hAcc, vAcc, pDOP | 最重要。RTK状態・精度・衛星数を一括取得 |
| UBX-NAV-HPPOSLLH | p.140 | hAcc/vAcc（0.1mm精度） | mm単位の高精度測位 |
| UBX-NAV-STATUS | p.155 | gpsFix, carrSoln, **ttff**（初期FIX時間ms） | 機体準備時間の自動計測 |
| UBX-NAV-SAT | p.150 | cno (dBHz), elev, qualityInd（衛星ごと） | 受信感度の自動記録 |
| UBX-NAV-SIG | p.152 | cno (dBHz)（L1/L2信号別） | L1/L2ごとの受信感度 |
| UBX-MON-RF | p.132 | **jammingState**, noisePerMS, agcCnt | ジャミング検出・ノイズ監視 |
| UBX-MON-SPAN | p.134 | 256点スペクトラムデータ | スペアナの自動取得 |

---

## RTK FIX状態の定義

`UBX-NAV-PVT.flags` の bits 7-6（`carrSoln`）:

| 値 | 意味 |
|----|------|
| 0 | RTKなし（通常測位） |
| 1 | RTK Float（不安定） |
| **2** | **RTK Fixed（cm級精度 = 合格）** |

---

## 確認済みページ範囲

- p.132-134（UBX-MON-RF, UBX-MON-SPAN）
- p.139-158（主要NAVメッセージ）

**未読範囲**: p.1-131, p.159-305（RTCM, SPARTN, Configuration等）
→ Phase 2で必要に応じて読む
