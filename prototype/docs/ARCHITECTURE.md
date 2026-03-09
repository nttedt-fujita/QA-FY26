# プロトタイプ アーキテクチャ

**作成日**: 2026-03-09（Session 50）
**ステータス**: プロトタイプ段階

---

## 概要

M3（受入検査DB化）のプロトタイプ。ヒアリング前の仮説検証用として作成。
修正前提のKISS原則で実装されている。

### 図解

詳細な図は [architecture-diagrams.drawio](./architecture-diagrams.drawio) を参照。

| ページ | 内容 |
|--------|------|
| コンポーネント構成 | Frontend/Backend/DBの3層構成と各モジュールの関係 |
| ER図 | 8テーブルのエンティティ関係図 |
| データフロー | 検査記録の登録フロー（カウンター方式） |

---

## 技術スタック

| レイヤー | 技術 | 理由 |
|----------|------|------|
| フロントエンド | Next.js 15 (App Router) + TypeScript + Tailwind CSS | モダンなReact開発 |
| バックエンド | Go 1.22 (標準ライブラリのみ) | 軽量・高速・単一バイナリ |
| データベース | PostgreSQL 15 | 信頼性・JSON対応 |
| コンテナ | Docker Compose | 開発環境の統一 |

### 技術選定の経緯（Session 34）

当初はTypeScript + Lambdaを検討したが、以下の理由でGoに変更:

- **パフォーマンス**: Goが圧倒的に有利
- **依存関係**: 標準ライブラリ充実、npm依存地獄を回避
- **脆弱性リスク**: Goの方が低い
- **デプロイ**: 単一バイナリでシンプル

---

## ディレクトリ構造

```
prototype/
├── backend/
│   ├── cmd/
│   │   └── api/
│   │       └── main.go          # エントリーポイント
│   └── internal/
│       ├── handler/             # HTTPハンドラー（プレゼンテーション層）
│       │   ├── lot_handler.go
│       │   ├── master_handler.go
│       │   ├── inspection_session_handler.go
│       │   ├── inspection_record_handler.go
│       │   └── dashboard_handler.go
│       ├── repository/          # データベース操作（インフラ層）
│       │   ├── db.go            # DB接続
│       │   ├── lot.go
│       │   ├── master.go
│       │   ├── inspection_record.go
│       │   └── dashboard.go
│       └── session/             # 検査セッション状態管理
│           └── session.go       # インメモリ状態管理
├── frontend/
│   └── src/
│       ├── app/                 # Next.js App Router
│       │   ├── page.tsx         # ロット登録画面
│       │   ├── lots/            # ロット一覧
│       │   ├── inspection/      # カウンター画面
│       │   ├── records/         # 検査記録一覧
│       │   └── dashboard/       # ダッシュボード
│       ├── components/          # 共通コンポーネント
│       │   └── Navigation.tsx
│       ├── lib/
│       │   └── api.ts           # API呼び出しユーティリティ
│       └── types/               # 型定義
├── db/
│   ├── schema.sql               # スキーマ定義
│   └── seed.sql                 # シードデータ
├── docs/
│   ├── ARCHITECTURE.md          # ← このファイル
│   ├── adr/                     # Architecture Decision Records
│   ├── demo-guide.md
│   └── quickstart.md
└── makefiles/                   # Makefileモジュール
```

---

## コンポーネント構成

```
┌─────────────────────────────────────────────────────────────┐
│                        Frontend                              │
│  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────────────┐│
│  │ロット登録 │ │カウンター│ │検査一覧  │ │ダッシュボード    ││
│  │  page    │ │  page    │ │  page    │ │     page         ││
│  └────┬─────┘ └────┬─────┘ └────┬─────┘ └────────┬─────────┘│
│       │            │            │                │          │
│       └────────────┴────────────┴────────────────┘          │
│                              │                               │
│                        api.ts（fetch）                       │
└──────────────────────────────┼───────────────────────────────┘
                               │ HTTP (JSON)
┌──────────────────────────────┼───────────────────────────────┐
│                        Backend                               │
│  ┌───────────────────────────┴─────────────────────────────┐ │
│  │                    main.go (Router)                     │ │
│  │         http.ServeMux + CORSミドルウェア                 │ │
│  └───────────────────────────┬─────────────────────────────┘ │
│                              │                               │
│  ┌──────────────┬────────────┼────────────┬───────────────┐  │
│  │ LotHandler   │MasterHandler│SessionHandler│DashboardHandler│
│  └──────┬───────┴──────┬──────┴──────┬──────┴──────┬──────┘  │
│         │              │             │             │         │
│  ┌──────┴───────┬──────┴──────┬──────┴──────┬──────┴───────┐ │
│  │LotRepository │PartRepository│RecordRepository│DashboardRepository│
│  └──────────────┴──────────────┴──────────────┴──────────────┘│
│                              │                               │
│                         pgx/v5                               │
└──────────────────────────────┼───────────────────────────────┘
                               │
                    ┌──────────┴──────────┐
                    │    PostgreSQL       │
                    │  (Docker Container) │
                    └─────────────────────┘
```

---

## レイヤー構成

現在は**2層構造**（handler + repository）を採用。

```
handler/    ← プレゼンテーション層（HTTPリクエスト処理）
    ↓
repository/ ← インフラ層（DB操作）※ドメイン層と混在
```

### なぜ2層なのか（Service層・Domain層がない理由）

**意図的な省略**です。詳細は [architecture-concerns.md](./architecture-concerns.md) 参照。

| 理由 | 説明 |
|------|------|
| ドメインが未確定 | ヒアリング前で要件が流動的 |
| 変更コストを最小化 | 過度な抽象化は手戻りコストを増やす |
| TDDで保証 | テストがあれば安全にリファクタリング可能 |
| YAGNI原則 | 今必要なものだけ作る |

### 本番化時のリファクタリング計画

```
# Phase 2（ヒアリング後）
internal/
├── handler/    ← プレゼンテーション層
├── domain/     ← ドメイン層（エンティティ、バリューオブジェクト）
└── repository/ ← インフラ層

# Phase 3（本格開発）
internal/
├── handler/    ← プレゼンテーション層
├── usecase/    ← アプリケーション層（ユースケース）
├── domain/     ← ドメイン層
└── repository/ ← インフラ層
```

---

## インターフェースについて

### 現状

リポジトリを呼び出すところに**インターフェースを噛ませていない**。

```go
// 現状: 具象型に直接依存
type LotHandler struct {
    repo *LotRepository  // 具象型
}
```

### なぜインターフェースがないのか

1. **プロトタイプ段階**: モックが不要（統合テストで十分）
2. **実装が1つ**: PostgreSQL以外のバックエンドを想定していない
3. **変更コスト**: インターフェース追加は後からでも可能（Goは構造的部分型）

### インターフェースが必要になるタイミング

| シナリオ | 対応 |
|----------|------|
| ユニットテストでDBをモックしたい | インターフェースを定義し、handler側で宣言 |
| 別のDB（SQLite等）に対応したい | インターフェースを定義し、実装を差し替え |
| ドメイン層を分離したい | repository → domain への依存逆転が必要 |

### 本番化時の変更例

```go
// handler/lot_handler.go
type LotRepository interface {
    Create(ctx context.Context, lot *domain.Lot) error
    FindByID(ctx context.Context, id string) (*domain.Lot, error)
    FindAll(ctx context.Context) ([]*domain.Lot, error)
}

type LotHandler struct {
    repo LotRepository  // インターフェース
}
```

---

## API設計

### エンドポイント一覧

| メソッド | パス | 説明 |
|----------|------|------|
| **ロット** |||
| POST | /api/v1/lots | ロット作成 |
| GET | /api/v1/lots | ロット一覧 |
| GET | /api/v1/lots/{id} | ロット詳細 |
| **マスタ** |||
| GET | /api/v1/parts | 部品一覧 |
| GET | /api/v1/inspection-items | 検査項目一覧 |
| GET | /api/v1/workers | 作業者一覧 |
| **検査セッション** |||
| POST | /api/v1/inspection-sessions | セッション開始 |
| POST | /api/v1/inspection-sessions/{id}/count | カウント追加 |
| DELETE | /api/v1/inspection-sessions/{id}/count | カウント取消 |
| POST | /api/v1/inspection-sessions/{id}/finish | セッション終了 |
| **検査記録** |||
| GET | /api/v1/inspection-records | 検査記録一覧 |
| **ダッシュボード** |||
| GET | /api/v1/dashboard/summary | KPI概要 |
| GET | /api/v1/dashboard/monthly | 月別統計 |
| GET | /api/v1/dashboard/top-defects | 不良トップ |
| GET | /api/v1/dashboard/items | 検査項目別 |
| GET | /api/v1/dashboard/recent | 直近記録 |
| GET | /api/v1/dashboard/suppliers | サプライヤー別 |

### API契約

ADR-002で定義: [ADR-002-api-contract.md](./adr/ADR-002-api-contract.md)

---

## データフロー

### 検査記録の登録フロー（カウンター方式）

```
1. ロット登録
   Frontend → POST /api/v1/lots → LotHandler → LotRepository → DB

2. 検査セッション開始
   Frontend → POST /api/v1/inspection-sessions
            → InspectionSessionHandler
            → インメモリセッション作成（session.Manager）

3. カウント追加（合格/不合格/スキップ）
   Frontend → POST /api/v1/inspection-sessions/{id}/count
            → session.AddCount()（インメモリ更新）

4. セッション終了
   Frontend → POST /api/v1/inspection-sessions/{id}/finish
            → InspectionRecordRepository.Create() → DB保存
            → session.Manager.Clear()

5. ダッシュボード表示
   Frontend → GET /api/v1/dashboard/*
            → DashboardHandler → DashboardRepository → DB
```

---

## データベース設計

### ER図（簡易版）

```
┌─────────────┐       ┌─────────────┐       ┌──────────────────┐
│ suppliers   │──────<│   parts     │──────<│     lots         │
│ サプライヤ  │ 1:N   │   部品      │ 1:N   │   ロット         │
└─────────────┘       └─────────────┘       └────────┬─────────┘
                                                     │
                      ┌─────────────────────┐        │ 1:N
                      │ inspection_items    │        │
                      │ 検査項目            │        │
                      └────────┬────────────┘        │
                               │                     │
                               │ N:1           N:1   │
                               ▼                     ▼
                      ┌────────────────────────────────┐
                      │      inspection_records        │
                      │          検査記録              │
                      └────────────────┬───────────────┘
                                       │
                           ┌───────────┴───────────┐
                           │ 1:N                   │ 1:N
                           ▼                       ▼
                   ┌───────────────┐       ┌───────────────┐
                   │ defect_reports│       │    waivers    │
                   │ 不良レポート  │       │  不問判定     │
                   └───────────────┘       └───────────────┘
```

### テーブル一覧

詳細は [schema.sql](../db/schema.sql) 参照。

| テーブル | 説明 | 分類 |
|----------|------|------|
| suppliers | サプライヤ | マスタ |
| parts | 部品 | マスタ |
| inspection_items | 検査項目 | マスタ |
| workers | 作業者 | マスタ |
| lots | ロット（入荷単位） | トランザクション |
| inspection_records | 検査記録 | トランザクション |
| defect_reports | 不良レポート（8D対応） | トランザクション |
| waivers | 不問判定 | トランザクション |

---

## 開発環境

### 起動方法

```bash
cd prototype
make up          # 全サービス起動
make logs        # ログ確認
make down        # 停止
```

### コマンド一覧

```bash
make help        # 全コマンド表示
make demo-flow   # デモフロー表示
```

詳細は [quickstart.md](./quickstart.md) 参照。

---

## ADR（設計判断記録）

| ADR | タイトル | 内容 |
|-----|---------|------|
| [ADR-001](./adr/ADR-001-error-handling.md) | エラーハンドリング方針 | プロトタイプは最小限、本番化時にcockroachdb/errors導入 |
| [ADR-002](./adr/ADR-002-api-contract.md) | API契約とFE/BE整合性 | 型定義の整合性ルール |
| [ADR-003](./adr/ADR-003-lot-list-view.md) | ロット一覧画面 | 画面設計の判断 |

---

## 今後の拡張

### Phase 2（ヒアリング後）

- バリューオブジェクトの定義（LotID等）
- 集約の特定（ロット→検査記録の関係）
- ドメインサービスの抽出

### Phase 3（本格開発）

- クリーンアーキテクチャ（4層構造）
- ユースケース層の導入
- M4（工程不良DB）との連携
- 認証・認可

詳細は [architecture-concerns.md](./architecture-concerns.md) 参照。

---

## 参照資料

- [quickstart.md](./quickstart.md) — 起動手順
- [demo-guide.md](./demo-guide.md) — デモ手順
- [implementation-plan.md](./implementation-plan.md) — 実装計画
- [implementation-result.md](./implementation-result.md) — 実装結果
- [architecture-concerns.md](./architecture-concerns.md) — 懸念点・将来対応
