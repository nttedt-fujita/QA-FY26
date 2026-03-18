# AI検査サービス選定 — 中間整理メモ

**作成日**: 2026-03-18（Session 246）
**ステータス**: 検討中（方針未決定）

---

## 1. 状況の変化

| 観点 | 当初の想定 | 今日聞いた話 |
|------|-----------|-------------|
| 規模 | 個人でPoC | 1万台生産も見据える |
| アプローチ | 小規模検証 | スケーラビリティ必要 |
| 方針 | 「個人でやれる範囲」で調査 | 将来を見越したMVP構成 |

**結論**: 「すぐではないが、先を見越してスケーリングしても対応できるシステムのMVP構成」で検討する方向。

---

## 2. 調査したAWSサービスの出力仕様

### Amazon Rekognition Custom Labels

**用途**: カスタム不良検出（シルク印刷カケ、キズ等）

**出力形式**:
```json
{
  "CustomLabels": [
    {
      "Name": "シルク印刷カケ",
      "Confidence": 91.72,
      "Geometry": {
        "BoundingBox": {
          "Width": 0.78,
          "Height": 0.36,
          "Left": 0.11,
          "Top": 0.37
        }
      }
    }
  ]
}
```

**特徴**:
- BoundingBox（不良位置）が取得可能
- 信頼度スコア（0-100）
- 学習データ（画像+ラベル）が必要

**出典**: [AWS公式ドキュメント](https://docs.aws.amazon.com/rekognition/latest/APIReference/API_DetectCustomLabels.html)

---

### SageMaker（カスタムモデル）

**用途**: 高精度が必要な場合、独自モデル学習

**出力形式**:
```json
{
  "predictions": [
    {"label": "defect", "confidence": 0.95}
  ]
}
```

**特徴**:
- モデル設計次第で出力形式をカスタマイズ可能
- 学習データ（大量）が必要
- 技術的ハードル高

**出典**: [AWS公式ドキュメント](https://docs.aws.amazon.com/sagemaker/latest/dg/object-detection-in-formats.html)

---

### Amazon Bedrock（Claude / Nova）

**用途**: PoC・柔軟な判定、学習データ不要で始められる

**出力形式**:
```json
{
  "result": "不合格",
  "defects": [
    {"type": "シルク印刷カケ", "location": "左上", "confidence": "高"}
  ],
  "reason": "ロゴ部分に印刷欠けが確認されました"
}
```

**特徴**:
- 構造化出力（JSONスキーマ指定可能）
- BoundingBoxは取得不可（テキスト記述のみ）
- 信頼度は曖昧（高/中/低）
- **学習データ不要**（プロンプトベース）
- スケーラブル（AWSマネージド）

**出典**: [Claude Vision API](https://platform.claude.com/docs/en/build-with-claude/vision)

---

## 3. 比較表

| サービス | BoundingBox | 信頼度 | 学習データ | コスト | 導入難易度 | スケーラビリティ |
|---------|:-----------:|:------:|:----------:|:------:|:----------:|:----------------:|
| Rekognition Custom Labels | ✅ | 0-100 | 必要 | 中 | 中 | ✅ |
| SageMaker | ✅ | カスタム | 必要（大量） | 高 | 高 | ✅ |
| Bedrock (Claude) | ❌ | 曖昧 | **不要** | 低〜中 | 低 | ✅ |

---

## 4. スケーリングを見越したMVP構成（案）

```
[検査現場]
    │
    ▼ 画像アップロード
[S3] ─────────────────────────────┐
    │                             │
    ▼ トリガー                    │
[Lambda / Step Functions]         │
    │                             │
    ▼ 推論                        │
[Amazon Bedrock (Claude)]         │
    │                             │
    ▼ 結果保存                    │
[M3 DB] ←─────────────────────────┘
```

**この構成のメリット**:
- スケーラブル（S3/Lambda/Bedrockは全てAWSマネージド）
- MVPで始められる（小規模でも同じアーキテクチャ）
- 学習データ不要（Bedrockはプロンプトベース）
- コスト予測可能（トークン課金）

---

## 5. 次にやるべきこと

| 項目 | 内容 |
|------|------|
| 要求の整理 | そもそも何を達成したいのか（What） |
| 要件の整理 | 性能要件、コスト要件、運用要件 |
| コスト試算 | 1万台規模でのコストシミュレーション |
| サービス選定 | 要件に基づいて選択肢を絞る |

---

## 6. 参照

| 資料 | 内容 |
|------|------|
| [06_ai_visual_inspection_comparison.md](../../docs/missions/m3-incoming-inspection-db/ai-research/06_ai_visual_inspection_comparison.md) | AI外観検査サービス比較（過去調査） |
| [07_ai_integration_and_cost_analysis.md](../../docs/missions/m3-incoming-inspection-db/ai-research/07_ai_integration_and_cost_analysis.md) | AI連携設計・コスト分析 |
| [ai-inspection-m3-integration-draft.md](../session245/ai-inspection-m3-integration-draft.md) | M3連携設計たたき台 |

---

*作成: Session 246 (2026-03-18)*
