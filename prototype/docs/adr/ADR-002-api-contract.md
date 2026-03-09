# ADR-002: API契約とフロントエンド/バックエンド整合性

## ステータス

承認済み

## 背景

Session 44-45でフロントエンド実装を進める中で、以下の不整合が発生した:

1. **レスポンスキー不一致**: `record_id` vs `inspection_record_id`
2. **用語不一致**: `pass/fail/waiver` vs `ok/ng/skip`
3. **型フィールド不一致**: `part_name` vs `name`
4. **オプショナルフィールド**: バックエンド `*string` がフロントエンド `string`（必須）になっていた

これらはすべてテスト実行時やビルド時に発見され、都度修正が必要だった。本番開発ではこのような手戻りを避けたい。

## 決定

### 1. API仕様を単一の信頼源（Single Source of Truth）で管理

**プロトタイプ段階**:
- バックエンドのGoの構造体をAPI仕様の正とする
- フロントエンドの型定義にコメントで対応するバックエンドファイル・構造体名を記載

**本番化時**:
- OpenAPI（Swagger）でAPI仕様を定義
- バックエンド/フロントエンドともにスキーマから型を生成

### 2. フロントエンド型定義のルール

```typescript
// バックエンド: internal/repository/master.go の Part struct と対応
export interface Part {
  part_id: string;
  name: string;          // バックエンドのフィールド名と一致させる
  supplier_id?: string;  // バックエンド *string → TypeScript optional
}
```

- コメントで対応するバックエンドファイルを明記
- バックエンドの `*T`（ポインタ）は TypeScript で `?: T`（オプショナル）
- バックエンドのJSONタグと完全一致させる

### 3. APIレスポンスキーの命名規則

| 種別 | 命名規則 | 例 |
|------|---------|-----|
| ID | `{リソース}_id` | `lot_id`, `inspection_record_id` |
| カウント | `{種別}_count` | `ok_count`, `ng_count` |
| 日時 | `{動作}_at` | `created_at`, `started_at` |
| 時間量 | `{単位付き}` | `work_time_min`（分）, `man_hours`（工数） |

### 4. 用語統一

| 概念 | バックエンド | フロントエンド | API |
|------|-------------|---------------|-----|
| 合格 | pass | ok | ok（ユーザー入力） → pass（内部） |
| 不合格 | fail | ng | ng（ユーザー入力） → fail（内部） |
| 不問 | waiver | skip | skip（ユーザー入力） → waiver（内部） |

- ユーザー入力は直感的な `ok/ng/skip` を使用
- バックエンドで `pass/fail/waiver` に正規化

### 5. テスト戦略

- **バックエンド**: 統合テストでAPIレスポンス形式を検証
- **フロントエンド**: API呼び出しモジュールの型チェック（TypeScriptビルド）
- **E2E**: 本番化時にPlaywright等で画面操作テスト

## 影響

- 新しいAPIエンドポイント追加時は、フロントエンド型定義を同時更新
- 既存APIの変更時は、フロントエンド型定義も必ず更新

## 関連

- ADR-001: エラーハンドリング方針
