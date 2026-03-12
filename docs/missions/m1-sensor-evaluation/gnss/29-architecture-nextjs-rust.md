# Next.js / Rust アーキテクチャ解説

**作成日**: 2026-03-12
**対象**: M1-B GNSS評価ツール

---

## 概要

GNSS評価ツールは**Next.js（フロントエンド）+ Rust（バックエンド）**の構成で、HTTPのJSON APIで通信します。

```
┌─────────────────────────────────────────────────────────────┐
│                      ブラウザ                                │
│  ┌────────────────────────────────────────────────────────┐ │
│  │              Next.js (App Router)                      │ │
│  │  ┌────────────┐  ┌────────────┐  ┌────────────┐       │ │
│  │  │ ホーム画面  │  │ 装置画面   │  │ 検査画面   │       │ │
│  │  └────────────┘  └────────────┘  └────────────┘       │ │
│  │                      │                                  │ │
│  │              fetch() / JSON                            │ │
│  └────────────────────────│────────────────────────────────┘ │
└──────────────────────────│──────────────────────────────────┘
                           │ HTTP
                           ▼
┌─────────────────────────────────────────────────────────────┐
│              Rust バックエンド (Actix-web)                   │
│  ┌────────────────────────────────────────────────────────┐ │
│  │  /api/devices    /api/lots    /api/inspections         │ │
│  │  /api/nav-sig    /api/mon-span  /api/nav-status        │ │
│  │  /api/nav-sat                                          │ │
│  └────────────────────────────────────────────────────────┘ │
│                      │                                       │
│              ┌───────┴───────┐                              │
│              │ DeviceManager │                              │
│              └───────┬───────┘                              │
│                      │ Serial (USB)                         │
└──────────────────────│──────────────────────────────────────┘
                       ▼
               ┌──────────────┐
               │  ZED-F9P    │
               │ (USB接続)   │
               └──────────────┘
```

---

## Next.js App Router の役割

### App Router とは？

Next.js 13以降の新しいルーティングシステム。`app/` ディレクトリ配下のファイル構造がそのままURLルートになります。

```
frontend/src/app/
├── page.tsx              → /
├── devices/page.tsx      → /devices
├── lots/page.tsx         → /lots
├── inspections/
│   ├── page.tsx          → /inspections
│   ├── indoor/page.tsx   → /inspections/indoor
│   └── outdoor/page.tsx  → /inspections/outdoor
└── layout.tsx            → 共通レイアウト
```

### Server Components vs Client Components

App Routerでは2種類のコンポーネントがあります：

| 種類 | 実行場所 | 特徴 |
|------|---------|------|
| Server Components | サーバー（ビルド時/リクエスト時） | デフォルト、データ取得可能 |
| Client Components | ブラウザ | `"use client"`指定、useState/useEffect使用可能 |

**本プロジェクトでは**:
- ページコンポーネント: `"use client"` → **Client Components**
- 理由: ポーリング、状態管理、イベントハンドリングが必要

```tsx
// "use client" がないとServer Component
// "use client" があるとClient Component
"use client";

export default function InspectionsPage() {
  const [data, setData] = useState(null);  // ← これはClient Componentでしか使えない
  // ...
}
```

### バックエンド処理はできる？

**できます**。Server ComponentsやAPI Routes (`app/api/`)でバックエンド的な処理が可能です：

```tsx
// app/api/hello/route.ts
export async function GET() {
  // サーバーサイドで実行される
  return Response.json({ message: "Hello" });
}
```

**ただし本プロジェクトでは使っていません**。理由：

1. **シリアルポートアクセス**: Node.jsでも可能だがRustのほうが得意
2. **パフォーマンス**: UBXパース等のバイナリ処理はRustが高速
3. **型安全性**: Rustの強い型システムで安全にバイナリを扱える
4. **既存資産**: Rustバックエンドが既に実装済み

---

## Rustバックエンドの役割

### 責務

| 機能 | 説明 |
|------|------|
| シリアルポート管理 | ZED-F9PへのUSB接続・切断 |
| UBXプロトコル | バイナリメッセージの送受信・パース |
| 検査ロジック | 合格/不合格判定 |
| データ蓄積 | SQLite保存 |
| REST API | Actix-webでJSON API提供 |

### API一覧

| エンドポイント | メソッド | 説明 |
|---------------|---------|------|
| `/api/devices` | GET | 装置一覧 |
| `/api/devices/{path}/connect` | POST | 接続 |
| `/api/devices/{path}/disconnect` | POST | 切断 |
| `/api/lots` | GET/POST | ロット一覧/作成 |
| `/api/inspections` | GET/POST | 検査履歴/実行 |
| `/api/nav-sig` | GET | NAV-SIG（衛星信号） |
| `/api/mon-span` | GET | MON-SPAN（スペクトラム） |
| `/api/nav-status` | GET | NAV-STATUS（Fix状態・TTFF） |
| `/api/nav-sat` | GET | NAV-SAT（スカイプロット用） |

---

## 通信フロー例

### 検査実行

```
1. ブラウザ: 「検査開始」ボタンクリック
2. Next.js: fetch("http://localhost:8080/api/inspections", { method: "POST" })
3. Rust: DeviceManager.run_inspection()
4. Rust: ZED-F9Pにpollリクエスト送信
5. ZED-F9P: UBXレスポンス返却
6. Rust: パース → 判定 → DB保存
7. Rust: JSON応答返却
8. Next.js: 結果を画面に表示
```

### ポーリング（NAV-SIG等）

```
1. ブラウザ: setInterval(fetchData, 1000) でポーリング開始
2. 毎秒: fetch("http://localhost:8080/api/nav-sig")
3. Rust: ZED-F9PにNAV-SIGをpoll
4. ZED-F9P: NAV-SIGレスポンス
5. Rust: パース → 統計計算 → JSON返却
6. Next.js: 画面更新
```

---

## なぜこの構成にしたか

### 選択肢の比較

| 構成 | メリット | デメリット |
|------|---------|-----------|
| **Next.js + Rust** | 型安全、高速、UIとロジック分離 | 2言語必要 |
| Next.js単体 | 1言語で完結 | シリアルポート扱いにくい |
| Electron + Rust | デスクトップアプリ | 配布が複雑 |
| Rust単体（WASM） | 1言語 | UI開発が大変 |

### 採用理由（ADR-005）

1. **Rustの強み**: UBXバイナリパース、シリアルポート、パフォーマンス
2. **Next.jsの強み**: UI開発の速さ、React/Tailwindのエコシステム
3. **分離のメリット**: FE/BE独立開発、テスト容易

---

## ディレクトリ構成

```
prototype/m1-gnss/
├── backend/           ← Rust (Actix-web)
│   ├── src/
│   │   ├── main.rs
│   │   ├── web/       ← APIハンドラー
│   │   ├── ubx/       ← UBXパーサー
│   │   ├── device/    ← DeviceManager
│   │   └── inspection/ ← 検査ロジック
│   └── Cargo.toml
│
├── frontend/          ← Next.js (App Router)
│   ├── src/
│   │   ├── app/       ← ページコンポーネント
│   │   ├── components/ ← 共通コンポーネント
│   │   └── lib/       ← API呼び出し
│   └── package.json
│
└── Makefile           ← 開発用コマンド
```

---

## 開発時の起動

```bash
# ターミナル1: バックエンド
cd prototype/m1-gnss
make backend-dev  # cargo run

# ターミナル2: フロントエンド
make frontend-dev  # npm run dev
```

ブラウザで `http://localhost:3000` にアクセス。

---

## まとめ

- **Next.js App Router**: ブラウザで動くUI、Client Componentsでポーリング
- **Rust Actix-web**: シリアルポート・UBXパース・検査ロジック
- **通信**: HTTP JSON API（CORS許可設定済み）
- **分離理由**: 各言語の得意分野を活かす

---

*作成: Session 122（2026-03-12）*
