# GNSS評価ツール アーキテクチャ設計

**作成日**: 2026-03-10
**作成元**: Session 74 要件定義
**ステータス**: 承認済み
**正式配置**: Session 75

---

## 概要

受入検査用GNSS評価ツールのアーキテクチャ設計。
スタンドアロンで動作し、複数のGNSS装置を同時に検査できる。

---

## 技術スタック

**参照**: [ADR-005](../../../adr/ADR-005-gnss-tool-tech-stack.md)（Session 75で更新）

| 層 | 技術 |
|----|------|
| バックエンド | Rust + Actix-web |
| フロントエンド | Next.js（React） |
| データベース | SQLite |
| シリアル通信 | serialport crate |
| 開発環境 | Docker + Dev Container |

---

## コンポーネント構成

```
┌─────────────────────────────────────────────────────────────┐
│                     Web Application                          │
├─────────────────────────────────────────────────────────────┤
│  ┌──────────────────────────────────────────────────────┐  │
│  │                 Next.js（React）                      │  │
│  │  ┌─────────┐  ┌─────────┐  ┌─────────┐  ┌─────────┐  │  │
│  │  │ 装置    │  │ 検査    │  │ 履歴    │  │ 比較    │  │  │
│  │  │ 一覧    │  │ 実行    │  │ 検索    │  │ 表示    │  │  │
│  │  └─────────┘  └─────────┘  └─────────┘  └─────────┘  │  │
│  └──────────────────────────────────────────────────────┘  │
│                           │                                 │
│                     HTTP / REST API                         │
│                           │                                 │
│  ┌──────────────────────────────────────────────────────┐  │
│  │                Rust + Actix-web                       │  │
│  │  ┌─────────────────┐  ┌─────────────────┐            │  │
│  │  │  Device Manager │  │  Inspection     │            │  │
│  │  │  (装置管理)     │  │  Engine         │            │  │
│  │  └────────┬────────┘  └────────┬────────┘            │  │
│  │           │                    │                      │  │
│  │  ┌────────┴────────┐  ┌────────┴────────┐            │  │
│  │  │  Serial Port    │  │  UBX Parser     │            │  │
│  │  │  (シリアル通信) │  │  (プロトコル)   │            │  │
│  │  └─────────────────┘  └─────────────────┘            │  │
│  │                                                       │  │
│  │  ┌─────────────────┐  ┌─────────────────┐            │  │
│  │  │  DB Repository  │  │  Report         │            │  │
│  │  │  (データ永続化) │  │  Generator      │            │  │
│  │  └─────────────────┘  └─────────────────┘            │  │
│  └──────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
```

---

## コンポーネント詳細

### 1. Device Manager（装置管理）

**責務**:
- USBポートの監視・装置検出
- 装置の接続状態管理
- 複数装置の並行管理

**インターフェース**:
```rust
pub trait DeviceManager {
    /// 接続された装置一覧を取得
    fn list_devices(&self) -> Vec<Device>;

    /// 装置に接続
    fn connect(&self, port: &str) -> Result<DeviceHandle>;

    /// 装置を切断
    fn disconnect(&self, handle: &DeviceHandle) -> Result<()>;
}

pub struct Device {
    pub port: String,
    pub serial_number: Option<String>,
    pub status: DeviceStatus,
}

pub enum DeviceStatus {
    Detected,      // 検出済み
    Connecting,    // 接続中
    Connected,     // 接続完了
    Inspecting,    // 検査中
    Disconnected,  // 切断
    Error(String), // エラー
}
```

---

### 2. UBX Parser（プロトコル層）

**責務**:
- UBXメッセージのエンコード/デコード
- 各メッセージタイプのパース

**既存実装**: Session 72で実装済み
- `nav_status.rs` — NAV-STATUS
- `nav_dop.rs` — NAV-DOP
- `mon_rf.rs` — MON-RF

**追加実装が必要**:
- `mon_ver.rs` — MON-VER（FWバージョン）
- `sec_uniqid.rs` — SEC-UNIQID（チップシリアル）
- `cfg_rate.rs` — CFG-RATE（出力レート）
- `cfg_prt.rs` — CFG-PRT（ポート設定）

---

### 3. Inspection Engine（検査エンジン）

**責務**:
- 検査項目の定義・実行
- 検査結果の判定（Pass/Fail）
- 検査進捗の通知

**インターフェース**:
```rust
pub struct InspectionEngine {
    items: Vec<InspectionItem>,
}

pub struct InspectionItem {
    pub name: String,
    pub expected_value: Option<String>,
    pub timeout_ms: u64,
}

pub struct InspectionResult {
    pub item_name: String,
    pub result: ResultStatus,
    pub expected_value: Option<String>,
    pub actual_value: Option<String>,
    pub inspected_at: DateTime<Utc>,
}

pub enum ResultStatus {
    Pass,
    Fail,
    Error(String),
}

impl InspectionEngine {
    /// 検査を実行
    pub async fn run(&self, device: &DeviceHandle) -> Vec<InspectionResult>;
}
```

---

### 4. DB Repository（データ永続化）

**責務**:
- SQLiteへのCRUD操作
- トランザクション管理

**テーブル構成**:
- `inspection_sessions` — 検査セッション
- `devices` — 装置マスタ
- `inspection_results` — 検査結果

---

### 5. Report Generator（レポート生成）

**責務**:
- PDF/CSV形式でのレポート出力
- 日本語フォント対応

---

### 6. Frontend（Next.js / React）

**画面構成**:

| 画面 | 機能 |
|------|------|
| 装置一覧 | 接続中の装置を表示、検査開始ボタン |
| 検査実行 | 検査進捗をリアルタイム表示 |
| 履歴検索 | 過去の検査結果を検索・表示 |
| 比較表示 | 複数装置の結果を並べて比較 |

**技術**:
- Next.js（App Router）
- React
- TypeScript
- fetch APIでバックエンドと通信

---

## データフロー

### 検査実行フロー

```
┌──────┐    ┌────────────┐    ┌────────────┐    ┌───────────┐
│ User │───>│ Frontend   │───>│ Backend    │───>│ F9P       │
│      │    │ (Next.js)  │    │ (Actix-web)│    │ (Device)  │
└──────┘    └────────────┘    └────────────┘    └───────────┘
    │              │                 │                 │
    │  1. 検査開始 │                 │                 │
    │─────────────>│                 │                 │
    │              │  2. POST /api/inspect             │
    │              │────────────────>│                 │
    │              │                 │  3. UBX request │
    │              │                 │────────────────>│
    │              │                 │  4. UBX response│
    │              │                 │<────────────────│
    │              │  5. JSON response                 │
    │              │<────────────────│                 │
    │  6. UI更新   │                 │                 │
    │<─────────────│                 │                 │
    │              │                 │  7. DB保存     │
    │              │                 │────────────────>│
    │  8. 完了通知 │                 │       (SQLite)  │
    │<─────────────│                 │                 │
```

---

## 状態遷移

### 装置状態

```
         ┌─────────────────────────────────────┐
         │                                     │
         ▼                                     │
    ┌─────────┐     ┌──────────┐     ┌─────────┴───┐
    │Detected │────>│Connecting│────>│  Connected  │
    └─────────┘     └──────────┘     └──────┬──────┘
         ▲               │                  │
         │               │                  ▼
         │               │           ┌──────────┐
         │               │           │Inspecting│
         │               │           └─────┬────┘
         │               │                 │
         │               ▼                 ▼
         │          ┌─────────┐      ┌──────────┐
         └──────────│  Error  │<─────│ Complete │
                    └─────────┘      └──────────┘
```

---

## ディレクトリ構成

**参照**: [ADR-005](../../../adr/ADR-005-gnss-tool-tech-stack.md)

```
prototype/m1-gnss/
├── .devcontainer/
│   └── devcontainer.json
├── Dockerfile.dev
├── backend/
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs
│       ├── lib.rs
│       ├── web/               # Actix-web ハンドラ
│       │   ├── mod.rs
│       │   └── handlers.rs
│       ├── device/
│       │   ├── mod.rs
│       │   ├── manager.rs     # DeviceManager
│       │   └── serial.rs      # シリアルポート操作
│       ├── ubx/
│       │   ├── mod.rs
│       │   ├── parser.rs      # 共通パーサー
│       │   ├── nav_status.rs  # (既存)
│       │   ├── nav_dop.rs     # (既存)
│       │   ├── mon_rf.rs      # (既存)
│       │   ├── mon_ver.rs     # (追加)
│       │   ├── sec_uniqid.rs  # (追加)
│       │   ├── cfg_rate.rs    # (追加)
│       │   └── cfg_prt.rs     # (追加)
│       ├── inspection/
│       │   ├── mod.rs
│       │   ├── engine.rs      # InspectionEngine
│       │   └── items.rs       # 検査項目定義
│       ├── db/
│       │   ├── mod.rs
│       │   ├── repository.rs  # DB操作
│       │   └── schema.rs      # テーブル定義
│       └── report/
│           ├── mod.rs
│           ├── pdf.rs         # PDF生成
│           └── csv.rs         # CSV生成
├── frontend/                  # Next.js
│   ├── package.json
│   ├── next.config.js
│   ├── tsconfig.json
│   └── src/
│       ├── app/
│       │   ├── page.tsx       # 装置一覧（メイン画面）
│       │   ├── inspect/
│       │   │   └── page.tsx   # 検査実行画面
│       │   ├── history/
│       │   │   └── page.tsx   # 履歴検索画面
│       │   └── compare/
│       │       └── page.tsx   # 比較表示画面
│       ├── components/
│       └── lib/
│           └── api.ts         # API通信
└── db/
    └── schema.sql
```

---

## API設計

### エンドポイント一覧

| エンドポイント | メソッド | 用途 |
|---------------|----------|------|
| /api/devices | GET | 装置一覧取得 |
| /api/devices/{port}/connect | POST | 装置接続 |
| /api/devices/{port}/disconnect | POST | 装置切断 |
| /api/devices/{port}/inspect | POST | 検査開始 |
| /api/inspections | GET | 検査結果一覧 |
| /api/inspections/{id} | GET | 検査結果詳細 |
| /api/reports/pdf | POST | PDF出力 |
| /api/reports/csv | POST | CSV出力 |

---

## 参照資料

- [15-gnss-tool-requirements.md](15-gnss-tool-requirements.md) — 要件定義
- [14-gnss-tool-needs.md](14-gnss-tool-needs.md) — 要求定義
- [ADR-005](../../../adr/ADR-005-gnss-tool-tech-stack.md) — 技術スタック

---

*作成: 2026-03-10 Session 74*
*正式配置: 2026-03-10 Session 75*
