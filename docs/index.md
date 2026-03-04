# QA-FY26 ドキュメントインデックス

**最終更新**: 2026-03-04 (Session 2)

---

## ミッション別ドキュメント

| ミッション | 担当 | 概要 | ディレクトリ |
|-----------|------|------|-------------|
| **M1** | ふじた | [AirGrow搭載センサー評価手法策定（Lidar/GNSS）](missions/m1-sensor-evaluation/README.md) | `missions/m1-sensor-evaluation/` |
| **M2** | ふじた | [点群データ検証方法策定](missions/m2-pointcloud-verification/README.md) | `missions/m2-pointcloud-verification/` |
| **M3** | ふじた | [受入検査データベース化](missions/m3-incoming-inspection-db/README.md) | `missions/m3-incoming-inspection-db/` |
| **M4** | ふじた | [工程不良データベース化](missions/m4-defect-db/README.md) | `missions/m4-defect-db/` |

---

## QA基礎知識

| ドキュメント | 内容 | 作成Session |
|-------------|------|-------------|
| [QA基礎知識](qa-knowledge/qa-fundamentals.md) | QA/QCの違い、ISO 9001、AQL、FMEA等 | Session 1 |
| [自社QA/QC活動の整理](qa-knowledge/company-qa-qc.md) | 部品購入型メーカーとしての品質管理体制 | Session 2 |

---

## 優先順位・計画

| ドキュメント | 内容 | 作成Session |
|-------------|------|-------------|
| [ミッション優先順位分析](../sessions/session2/mission-priority-analysis.md) | 4ミッションの優先順位検討 | Session 2 |

---

## ディレクトリ構成

```
docs/
├── index.md              ← このファイル（全体インデックス）
├── missions/
│   ├── m1-sensor-evaluation/     ← M1: センサー評価
│   │   └── README.md
│   ├── m2-pointcloud-verification/ ← M2: 点群検証
│   │   └── README.md
│   ├── m3-incoming-inspection-db/  ← M3: 受入検査DB
│   │   └── README.md
│   └── m4-defect-db/              ← M4: 工程不良DB
│       └── README.md
├── qa-knowledge/
│   ├── qa-fundamentals.md         ← QA基礎知識
│   └── company-qa-qc.md           ← 自社QA/QC整理
sessions/
├── session-history/               ← セッション履歴
├── session1/                      ← Session 1資料
└── session2/                      ← Session 2資料
```
