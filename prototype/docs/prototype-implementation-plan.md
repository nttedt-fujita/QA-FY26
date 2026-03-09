# プロトタイプ実装計画書

**目的**: 入力→ダッシュボードまで連携する動くデモを作成し、ヒアリングの叩き台にする

**作成日**: 2026-03-06 (Session 38)

---

## ゴール

「部品を入荷して検査記録を入力したら、ダッシュボードで集計結果が見れる」を実現する。

```
[入力画面] → [バックエンドAPI] → [PostgreSQL] → [ダッシュボード]
```

---

## 現状

| コンポーネント | 状態 | 備考 |
|---------------|------|------|
| DB設計 | ✅完成 | 8テーブル定義済み |
| バックエンドAPI | 🔶部分 | ロットCRUDのみ（検査記録なし） |
| フロントエンド | 🔶部分 | 型定義・API関数作成済み、画面は未完 |
| ダッシュボード | 🔶別物 | Streamlit、CSV読み込み（DB未連携） |

---

## 実装タスク

### Phase A: バックエンド拡張（検査記録API）

**所要目安**: 1-2セッション

#### A-1. 検査記録 CRUD API

**ファイル**: `prototype/backend/internal/handler/inspection_handler.go`

| エンドポイント | メソッド | 説明 |
|---------------|---------|------|
| `/api/v1/inspection-records` | POST | 検査記録登録 |
| `/api/v1/inspection-records` | GET | 検査記録一覧 |
| `/api/v1/inspection-records/{id}` | GET | 検査記録詳細 |

**リクエスト構造**（POST）:
```json
{
  "lot_id": "LOT-12345678",
  "item_id": "ITEM-001",
  "worker_id": "WORKER-001",
  "inspection_date": "2026-03-06",
  "sample_qty": 10,
  "result": "合格",
  "defect_qty": 0,
  "work_time_min": 30,
  "note": "特記事項なし"
}
```

**レスポンス構造**（GET list）:
```json
{
  "records": [
    {
      "record_id": "REC-12345678",
      "lot_id": "LOT-12345678",
      "part_name": "バッテリー",
      "item_name": "外観検査",
      "worker_name": "杉山",
      "inspection_date": "2026-03-06",
      "result": "合格",
      "defect_qty": 0,
      "work_time_min": 30
    }
  ]
}
```

#### A-2. マスタデータ API（参照のみ）

**ファイル**: `prototype/backend/internal/handler/master_handler.go`

| エンドポイント | メソッド | 説明 |
|---------------|---------|------|
| `/api/v1/parts` | GET | 部品一覧（ドロップダウン用） |
| `/api/v1/inspection-items` | GET | 検査項目一覧 |
| `/api/v1/workers` | GET | 作業者一覧 |

#### A-3. 集計 API（ダッシュボード用）

**ファイル**: `prototype/backend/internal/handler/stats_handler.go`

| エンドポイント | メソッド | 説明 |
|---------------|---------|------|
| `/api/v1/stats/monthly` | GET | 月別集計 |
| `/api/v1/stats/by-category` | GET | カテゴリ別集計 |
| `/api/v1/stats/by-worker` | GET | 作業者別集計 |
| `/api/v1/stats/by-part` | GET | 品名別集計 |

**クエリパラメータ**:
- `from`: 開始日（YYYY-MM-DD）
- `to`: 終了日（YYYY-MM-DD）

**レスポンス例**（月別集計）:
```json
{
  "stats": [
    {
      "month": "2026-03",
      "record_count": 45,
      "total_hours": 12.5,
      "defect_count": 3
    }
  ]
}
```

#### A-4. シードデータ作成

**ファイル**: `prototype/db/seed.sql`

デモ用のサンプルデータを投入:
- サプライヤ: 3社
- 部品: 10種類（メカ/エレキ/Api）
- 検査項目: 5種類（外観/通電/寸法/重量/動作）
- 作業者: 3名
- ロット: 20件
- 検査記録: 50件（過去3ヶ月分）

---

### Phase B: フロントエンド実装

**所要目安**: 1-2セッション

#### B-1. 画面構成

```
/                    → トップ（検査記録一覧）
/lots                → ロット一覧
/lots/new            → ロット登録
/inspection/new      → 検査記録入力 ← メイン画面
/dashboard           → ダッシュボード（集計結果）
```

#### B-2. 検査記録入力画面（メイン）

**ファイル**: `prototype/frontend/src/app/inspection/new/page.tsx`

**入力項目**:
| 項目 | 型 | 必須 | UIコンポーネント |
|------|-----|------|-----------------|
| ロット | select | ✅ | ドロップダウン（ロット一覧から選択） |
| 検査項目 | select | ✅ | ドロップダウン |
| 作業者 | select | ✅ | ドロップダウン |
| 検査日 | date | ✅ | デートピッカー（デフォルト: 今日） |
| サンプル数 | number | - | 数値入力（抜取検査用） |
| 結果 | radio | ✅ | 合格/不合格/不問 |
| 不良数 | number | - | 数値入力（不合格時のみ） |
| 工数（分） | number | - | 数値入力 |
| 備考 | textarea | - | テキストエリア |

**UX考慮**:
- ロット選択時に部品名を表示
- 結果が「不合格」のとき不良数入力を有効化
- 登録後は入力フォームをリセットして連続入力可能に

#### B-3. ダッシュボード画面

**ファイル**: `prototype/frontend/src/app/dashboard/page.tsx`

既存Streamlitダッシュボードと同等の表示:
- 月別推移（棒グラフ + 折れ線）
- カテゴリ別円グラフ
- 作業者別棒グラフ
- 品名別パレート図

**ライブラリ**: recharts または Chart.js

#### B-4. 共通コンポーネント

**ファイル**: `prototype/frontend/src/components/`

```
components/
├── Header.tsx        # ナビゲーション
├── LotSelect.tsx     # ロット選択ドロップダウン
├── ResultRadio.tsx   # 結果選択（合格/不合格/不問）
└── StatsCard.tsx     # 指標カード（件数、工数など）
```

---

### Phase C: 統合・動作確認

**所要目安**: 0.5セッション

#### C-1. CORS設定

**ファイル**: `prototype/backend/cmd/api/main.go`

フロントエンド（localhost:3000）からの呼び出しを許可:
```go
// CORS middleware
func cors(next http.Handler) http.Handler {
    return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
        w.Header().Set("Access-Control-Allow-Origin", "http://localhost:3000")
        w.Header().Set("Access-Control-Allow-Methods", "GET, POST, OPTIONS")
        w.Header().Set("Access-Control-Allow-Headers", "Content-Type")
        if r.Method == "OPTIONS" {
            w.WriteHeader(http.StatusNoContent)
            return
        }
        next.ServeHTTP(w, r)
    })
}
```

#### C-2. Docker Compose更新

フロントエンドをDocker化:
```yaml
frontend:
  build: ./frontend
  ports:
    - "3000:3000"
  environment:
    - NEXT_PUBLIC_API_URL=http://localhost:8080
```

#### C-3. 動作確認チェックリスト

- [ ] ロット登録 → 一覧に表示される
- [ ] 検査記録入力 → 一覧に表示される
- [ ] ダッシュボード → 集計結果が更新される
- [ ] データがPostgreSQLに永続化されている

---

## 実装順序（推奨）

```
1. A-4: シードデータ作成（デモ用データがないと画面が寂しい）
2. A-2: マスタデータAPI（ドロップダウンに必要）
3. A-1: 検査記録CRUD API
4. B-2: 検査記録入力画面
5. A-3: 集計API
6. B-3: ダッシュボード画面
7. C-1〜C-3: 統合・動作確認
```

---

## TDDテストシナリオ

### 検査記録API

| # | シナリオ | 期待結果 |
|---|---------|---------|
| 1 | 検査記録登録_正常 | 201 |
| 2 | 検査記録登録_必須項目欠落 | 400 |
| 3 | 検査記録登録_存在しないロット | 400 |
| 4 | 検査記録一覧取得_正常 | 200 |
| 5 | 検査記録詳細取得_正常 | 200 |
| 6 | 検査記録詳細取得_存在しない | 404 |

### 集計API

| # | シナリオ | 期待結果 |
|---|---------|---------|
| 1 | 月別集計_正常 | 200 + 月別データ |
| 2 | 月別集計_期間指定 | 200 + フィルタ済みデータ |
| 3 | カテゴリ別集計_正常 | 200 |
| 4 | 作業者別集計_正常 | 200 |

---

## 参照資料

- [DB設計](../db/schema.sql)
- [既存ダッシュボード](../../tools/incoming_inspection/dashboard.py)
- [To-Beモデル](../../docs/missions/m3-incoming-inspection-db/to-be/to-be-model.drawio)
- [ロットAPI実装](../backend/internal/handler/lot_handler.go)

---

## 次セッションでやること

1. この計画書をレビュー（追加・修正があれば）
2. シードデータ（seed.sql）作成
3. マスタデータAPI実装

---

**作成**: Session 38
**更新予定**: 実装進捗に応じて更新
