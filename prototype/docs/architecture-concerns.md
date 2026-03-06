# アーキテクチャ懸念点・将来対応

**作成日**: 2026-03-06（Session 37）
**ステータス**: プロトタイプ段階のため意図的に簡素化

---

## 現状の設計

### コード構造

```
internal/
├── handler/    ← プレゼンテーション層（HTTPハンドラー）
└── repository/ ← インフラ層（DB操作）※ドメイン層と混在
```

### 採用しているパターン

| パターン | 状態 | 備考 |
|----------|------|------|
| リポジトリパターン | ✅ 採用 | 単純なCRUD操作 |
| handler/repository分離 | ✅ 採用 | 2層構造 |

---

## 未適用のDDD戦術的パターン

### 1. バリューオブジェクト（Value Object）

**現状**: `lot_id`, `part_id` などは単なる `string`

**将来対応**:
```go
// 例: LotID をバリューオブジェクトとして定義
type LotID struct {
    value string
}

func NewLotID(s string) (LotID, error) {
    if !strings.HasPrefix(s, "LOT-") {
        return LotID{}, errors.New("LotIDは'LOT-'で始まる必要があります")
    }
    return LotID{value: s}, nil
}
```

**対応タイミング**: ヒアリング後、IDの形式・制約が確定してから

---

### 2. 集約（Aggregate）

**現状**: エンティティ間の境界が曖昧

**課題**:
- ロット（Lot）が集約ルートかどうか不明確
- 検査記録（InspectionRecord）はロットの一部か、独立か

**将来対応**:
```
Lot（集約ルート）
├── InspectionRecord（子エンティティ）
└── DefectReport（子エンティティ）
```

**対応タイミング**: M3/M4のドメインが確定してから

---

### 3. ドメインサービス

**現状**: ビジネスロジックがhandlerに混在

**将来対応**:
- 検査結果の判定ロジック → `InspectionService`
- 不良レポートの8Dフロー → `DefectReportService`

**対応タイミング**: ビジネスルールが明確になってから

---

### 4. ドメイン層の分離（クリーンアーキテクチャ）

**現状**: 2層構造（handler + repository）

**将来対応**:
```
internal/
├── handler/    ← プレゼンテーション層
├── usecase/    ← アプリケーション層（ユースケース）
├── domain/     ← ドメイン層（エンティティ、バリューオブジェクト、ドメインサービス）
└── repository/ ← インフラ層（DB操作）
```

**対応タイミング**: プロトタイプ検証後、本格開発移行時

---

## To-Beドメインモデルの課題

### M4（工程不良DB）の未設計

**現状**: M3（受入検査DB）のみモデル化

**将来対応**:
- M4専用エンティティの設計
  - 工程（ManufacturingProcess）
  - 工程内不良（ProcessDefect）
  - 製造ライン（ProductionLine）
- M3-M4連携の詳細設計
  - 部品不良 → 工程不良の因果関係

**対応タイミング**: M4ヒアリング後

---

## リファクタリング計画

### Phase 1（現在）: プロトタイプ

- シンプルなCRUD
- 2層構造
- TDDで振る舞いを保証

### Phase 2（ヒアリング後）: ドメイン層導入

- バリューオブジェクトの定義
- 集約の特定
- ドメインサービスの抽出

### Phase 3（本格開発）: クリーンアーキテクチャ

- 4層構造への移行
- ユースケース層の導入
- 依存性の方向を制御

---

## 判断の理由

**なぜ今は簡素化しているか**:

1. **ドメインが未確定**: ヒアリング前で要件が流動的
2. **変更コストを最小化**: 過度な抽象化は手戻りコストを増やす
3. **TDDで保証**: テストがあれば安全にリファクタリング可能
4. **YAGNI原則**: 今必要なものだけ作る

---

## 参考資料

- [To-Beドメインモデル](../../docs/missions/m3-incoming-inspection-db/to-be/to-be-model.drawio)
- [品質管理フレームワーク調査](../../sessions/session25/quality-framework-research.md)
