# Session 246 サマリー

**日付**: 2026-03-18
**目的**: M3/M4ドメインモデル（概念図）の新規作成 + AI検査サービス選定の議論

---

## 実施内容

1. **M3ドメインモデル（概念図）の作成**
   - 配置: [docs/missions/m3-incoming-inspection-db/domain/domain-model.drawio](../../docs/missions/m3-incoming-inspection-db/domain/domain-model.drawio)
   - 内容:
     - エンティティ: サプライヤ、部品、ロット、検査記録、不良レポート、不問判定
     - ビジネスルール4項目を図中に記載
     - M4との連携（lot_id）を明示

2. **M4ドメインモデル（概念図）の作成**
   - 配置: [docs/missions/m4-defect-db/domain/domain-model.drawio](../../docs/missions/m4-defect-db/domain/domain-model.drawio)
   - 内容:
     - 工程不良記録エンティティ
     - 不良コード体系（3階層: EL/ME/SW/SE）
     - 原因コード体系（4M1E: Man/Machine/Material/Method/Environment）
     - M3との連携（lot_id）を明示

3. **to-be-model.drawioのリネーム実行**
   - 変更: `to-be-model.drawio` → `er-diagram.drawio`
   - 理由: 名前（モデル）と実態（ER図）の不一致を解消
   - 参照更新: [architecture-concerns.md](../../prototype/m3/docs/architecture-concerns.md)

4. **AI検査サービス選定の議論**
   - AWS各サービスの出力仕様を調査（Rekognition Custom Labels、SageMaker、Bedrock）
   - 新たな状況: 「1万台生産を見据えたスケーラビリティ」が必要
   - 方針: 要求・要件・コストを整理してからサービス選定する
   - 中間ファイル: [ai-service-selection-notes.md](ai-service-selection-notes.md)

---

## 作成/変更ファイル

| ファイル | 変更内容 |
|----------|----------|
| [docs/.../m3/.../domain/domain-model.drawio](../../docs/missions/m3-incoming-inspection-db/domain/domain-model.drawio) | 新規作成 |
| [docs/.../m4/.../domain/domain-model.drawio](../../docs/missions/m4-defect-db/domain/domain-model.drawio) | 新規作成 |
| [docs/.../m3/.../to-be/er-diagram.drawio](../../docs/missions/m3-incoming-inspection-db/to-be/er-diagram.drawio) | リネーム（旧: to-be-model.drawio） |
| [prototype/m3/docs/architecture-concerns.md](../../prototype/m3/docs/architecture-concerns.md) | 参照更新 |
| [ai-service-selection-notes.md](ai-service-selection-notes.md) | 新規作成（中間ファイル） |

---

## ドキュメント構成（更新後）

### M3

```
docs/missions/m3-incoming-inspection-db/
├── domain/
│   └── domain-model.drawio    ← 新規（概念図）
├── to-be/
│   └── er-diagram.drawio      ← リネーム（ER図）
└── as-is/
    └── as-is-model.drawio     ← 現状維持
```

### M4

```
docs/missions/m4-defect-db/
├── domain/
│   └── domain-model.drawio    ← 新規（概念図）
└── to-be/
    └── （スキーマ設計時にER図を作成）
```

---

## Living Documentation観点の整理

| 図 | 目的 | 読む人 | 更新タイミング |
|----|------|--------|---------------|
| **ドメインモデル** | ビジネス概念の理解 | 開発者 + 業務担当者 | 概念が変わったとき |
| **ER図** | DB実装の設計 | 開発者 | スキーマが変わったとき |
| **schema.sql** | 実装（コード） | 開発者 | 実装時 |

---

## 残った課題

| 課題 | 優先度 | 備考 |
|------|:------:|------|
| AI検査サービス選定 | 高 | 要求・要件・コストを整理してから判断 |
| ER図の自動生成検討 | 低 | 長期的なメンテコスト削減 |

---

## 次セッションでやること

1. **AI検査の要求整理**（What: 何を達成したいか）
2. **要件整理**（性能、コスト、運用）
3. **コスト試算**（1万台規模）
4. **サービス選定**

---

## 参照

- [Session 245: ドメインモデル vs ER図解説](../session245/domain-model-vs-er-diagram.md)
- [Session 246 計画](session-plan.md)
- [AI外観検査サービス比較](../../docs/missions/m3-incoming-inspection-db/ai-research/06_ai_visual_inspection_comparison.md)

---

*作成: Session 246 (2026-03-18)*
