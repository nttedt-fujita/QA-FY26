# TypeScript vs Go 比較調査レポート

**作成日**: 2026-03-05
**セッション**: Session 9
**目的**: M3/M4のバックエンド技術選定における言語比較の判断材料

---

## エグゼクティブサマリー

| 観点 | 結論 |
|------|------|
| パフォーマンス | M3/M4（CRUDアプリ）ではどちらでも問題なし |
| TypeScript 7.0のGoコンパイラ | **誤解注意**: コンパイラがGoで書き直されただけ。実行時性能は変わらない |
| 保守性 | Go有利（依存少ない） vs TS有利（言語統一） |
| 結論 | 言語選択より**アーキテクチャ選択（kintone vs AWS）が先** |

---

## 1. TypeScript 7.0 / tsgo（Goコンパイラ）について

### 重要な誤解を解く

「TypeScript 7.0のGoコンパイラ」とは、**TypeScriptのコードがGoにコンパイルされる**わけではない。**TypeScriptコンパイラ自体がGoで書き直された**という話。

つまり：
- ✅ **開発時のビルドが速くなる**
- ❌ **実行時のパフォーマンスがGoに近づくわけではない**

### エビデンス

#### Source 1: Microsoft DevBlogs（公式）
- **URL**: https://devblogs.microsoft.com/typescript/progress-on-typescript-7-december-2025/

**原文抜粋**:

| Project | tsc (6.0) | tsgo (7.0) | Speedup |
|---------|-----------|-----------|---------|
| sentry | 133.08s | 16.25s | 8.19x |
| vscode | 89.11s | 8.74s | 10.2x |
| typeorm | 15.80s | 1.06s | 9.88x |
| playwright | 9.30s | 1.24s | 7.51x |

> "In all but 74 cases, TypeScript 7 also produces at least one error" compared to TypeScript 6.0 across 20,000 test cases, demonstrating extremely high parity.

#### Source 2: ByteIota
- **URL**: https://byteiota.com/typescript-7-0-go-rewrite-10x-faster-builds-in-2026/

**原文抜粋**:
> VS Code: "1.5 million lines of TypeScript now compile in 8.74 seconds instead of 89 seconds—a 10.2x speedup"

> VS Code language service startup: "from 9.6 seconds to 1.2 seconds, an 8x improvement"

**Goが選ばれた理由**:
> Rather than Rust (which showed comparable execution speed), Go was chosen for "compilation speed, simplicity, and proven compiler use cases."

### まとめ

| 項目 | 内容 |
|------|------|
| 名称 | tsgo（Project Corsa） |
| リリース | 2026年1月15日 正式リリース済み |
| 効果 | コンパイル速度 8-10倍、エディタ起動 8倍高速化 |
| **重要** | **実行時パフォーマンスは従来と同じ（Node.js上で動作）** |

---

## 2. バックエンドとしてのパフォーマンス比較

### 結論

M3/M4のようなCRUDアプリ（I/O待ちが大半）では、**両言語ともパフォーマンスは問題にならない**。

### エビデンス

#### Source 3: DEV Community - TypeScript vs Go: Choosing Your Backend Language
- **URL**: https://dev.to/encore/typescript-vs-go-choosing-your-backend-language-2bc5

**原文抜粋**:
> "a TypeScript application using the Open Source Encore.ts framework can outperform a standard Node.js application (using Express.js) by **9x** (!) measured in requests/second."

> "for many web applications you will probably get sufficient performance from a TypeScript application" when using optimized frameworks.

**解釈**: フレームワーク選択によってTypeScriptでも十分な性能が出せる。Express.js（古い）を避けてFastify、Elysia等を使えばGoに近い性能になる。

### 一般的なベンチマーク比較

| 指標 | Go | TypeScript (Node.js) | 備考 |
|------|-----|---------------------|------|
| 起動時メモリ | 10-20MB | 30-50MB | Goが2-3倍効率的 |
| CPU集約処理 | 圧倒的に高速 | 不利 | 画像処理、暗号化等 |
| I/O待ち処理 | 高速 | 十分高速 | DB読み書き、API呼び出し |

**M3/M4への適用**:
- 受入検査DB・工程不良DBは典型的なCRUDアプリ
- CPU集約的処理はほぼない（せいぜいSPC計算程度）
- → **パフォーマンスはどちらを選んでも問題ない**

---

## 3. 型安全性・信頼性

### エビデンス

#### Source 4: peerdh.com - Type Safety In Go Vs Typescript
- **URL**: https://peerdh.com/blogs/programming-insights/type-safety-in-go-vs-typescript-a-comprehensive-comparison

**原文抜粋（Go）**:
> Go "does not allow implicit type conversions, which means you cannot accidentally mix types."

**原文抜粋（TypeScript）**:
> TypeScript operates differently as a "superset of JavaScript that adds static typing." Unlike Go, it allows optional type annotations.

> "type checks are not enforced at runtime," potentially allowing type-related issues if not carefully managed.

### 比較表

| 観点 | Go | TypeScript |
|------|-----|------------|
| 型システム | 静的型、暗黙変換なし | 静的型だがオプショナル |
| コンパイル時チェック | 厳格 | 厳格（strict mode時） |
| 実行時チェック | **あり**（コンパイル済み） | **なし**（JSに変換後） |
| バグ発生リスク | 低い | 型管理が甘いと発生 |

---

## 4. 保守性・依存関係

### エビデンス

#### Source 5: DEV Community - Golang or TypeScript?
- **URL**: https://dev.to/henriqueleite42/golang-or-typescript-f63

**原文抜粋（セキュリティ）**:
> "The more dependencies your project has, the more vulnerabilities it has."

**原文抜粋（Go）**:
> Golang includes extensive standard library features, minimizing external dependencies.

**原文抜粋（TypeScript）**:
> TypeScript relies heavily on third-party libraries and their maintainers.

### 比較表

| 観点 | Go | TypeScript |
|------|-----|------------|
| 外部依存 | 少ない（標準ライブラリ充実） | 多い（npm依存） |
| 脆弱性リスク | 低い | 依存関係次第で高い |
| ビルド成果物 | 単一バイナリ | Node.js + node_modules |
| デプロイ | シンプル | やや複雑 |
| テストツール | 組み込み（go test） | 外部（Jest等） |

---

## 5. M3/M4における判断

### 判断軸ごとの評価

| 判断軸 | Go | TypeScript | 備考 |
|--------|-----|------------|------|
| パフォーマンス | ○ | ○ | CRUDアプリではどちらも十分 |
| 保守性（依存） | ◎ | △ | Goは依存少なく脆弱性リスク低い |
| 保守性（言語統一） | △ | ◎ | フロントエンドもTSなら統一可能 |
| 学習コスト | △ | ○ | 藤田さんの現在のスキル次第 |
| 採用プール | △ | ◎ | 将来の引き継ぎを考えるとTS有利 |
| デプロイ簡便性 | ◎ | ○ | Goは単一バイナリで最もシンプル |

### 結論

**言語選択の前にアーキテクチャ選択が必要**:

| アーキテクチャ | 言語 | 保守性 |
|---------------|------|--------|
| kintone + 外部分析 | Python（分析部分のみ） | 最もシンプル |
| AWS自前開発 | TypeScript or Go | どちらでも成立 |

- AWS自前開発を選んだ場合、言語は「**フロントエンドとの統一**（TS）」か「**依存最小化**（Go）」のトレードオフ
- **技術選定はヒアリング後**の原則は維持

---

## 参照資料一覧

| # | タイトル | URL |
|---|----------|-----|
| 1 | Progress on TypeScript 7 - Microsoft DevBlogs | https://devblogs.microsoft.com/typescript/progress-on-typescript-7-december-2025/ |
| 2 | TypeScript 7.0 Go Rewrite - ByteIota | https://byteiota.com/typescript-7-0-go-rewrite-10x-faster-builds-in-2026/ |
| 3 | TypeScript vs Go: Choosing Your Backend Language - DEV | https://dev.to/encore/typescript-vs-go-choosing-your-backend-language-2bc5 |
| 4 | Type Safety In Go Vs Typescript - peerdh.com | https://peerdh.com/blogs/programming-insights/type-safety-in-go-vs-typescript-a-comprehensive-comparison |
| 5 | Golang or TypeScript? - DEV | https://dev.to/henriqueleite42/golang-or-typescript-f63 |
