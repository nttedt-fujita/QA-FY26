# 進捗管理方法の調査・検討結果

**作成**: Session 279 (2026-03-23)

---

## 1. Web調査で得られた知見

### 1.1 SSOT（Single Source of Truth）の原則

**出典**: [Atlassian](https://www.atlassian.com/work-management/knowledge-sharing/documentation/building-a-single-source-of-truth-ssot-for-your-team), [Wikipedia](https://en.wikipedia.org/wiki/Single_source_of_truth)

| 原則 | 説明 |
|------|------|
| **マスターは1箇所** | 各データは1箇所でのみ編集（mastered） |
| **リンクで参照** | 他からはリンク・参照で済ませる（コピペ禁止） |
| **自動同期** | 変更があれば関係者に通知される仕組み |
| **データオーナーシップ** | 誰が何を管理するか明確に |

**重要**: 「to reuse means to link or reference, not copy and paste」

---

### 1.2 Issue Tracker + Documentation の使い分け

**出典**: [LinearとNotionの実践例](https://alaniswright.com/blog/how-we-are-using-linear-and-notion-to-manage-our-product-backlog-and-project-work/), [Nuclino比較](https://www.nuclino.com/solutions/linear-vs-notion)

| ツール | 役割 | 管理する情報 |
|--------|------|------------|
| **Linear** | アクション（タスク） | 何をやるか、進捗状態 |
| **Notion/Docs** | 知識（ドキュメント） | なぜやるか、どうやったか |

**ポイント**: 両者をリンクで繋ぎ、情報を重複させない

---

### 1.3 Living Documentation / Docs as Code

**出典**: [CodeLucky](https://codelucky.com/agile-documentation-living-approach/), [Google Style Guide](https://google.github.io/styleguide/docguide/best_practices.html), [Agile Modeling](https://agilemodeling.com/essays/agiledocumentation.htm)

| 原則 | 説明 |
|------|------|
| **Just Barely Good Enough** | 軽量に保つ、必要最小限 |
| **Change with Code** | コード変更と同時にドキュメント更新 |
| **Eliminate Duplication** | 重複を排除、1箇所に集約 |
| **Automation** | 自動生成・CI/CD連携で陳腐化防止 |

---

### 1.4 コンテキストスイッチングの軽減

**出典**: [Hatica](https://www.hatica.io/blog/context-switching-killing-developer-productivity/), [Trunk](https://trunk.io/learn/context-switching-in-software-engineering-how-developers-lose-productivity)

- 1回のコンテキストスイッチで**認知能力の20%を消費**
- 情報が散らばっていると「調査プロジェクト」になってしまう
- **ドキュメントを整備して依存を減らす**ことが重要

---

## 2. 既存ルールとの整合性確認

### 2.1 既に採用している原則

| 原則 | 該当ルール | 状態 |
|------|-----------|------|
| 二重管理の禁止 | `14-document-management.md` セクション5 | ✅ 採用済み |
| Curation over Creation | `14-document-management.md` セクション6 | ✅ 採用済み |
| 情報配置の判断フロー | `07-information-routing.md` | ✅ 採用済み |
| Living Documentationの知識分類 | `07-information-routing.md` | ✅ 採用済み |
| ADRによる設計判断記録 | `10-adr-enforcement.md` | ✅ 採用済み |

### 2.2 採用していない・不足している原則

| 原則 | 状態 | 問題点 |
|------|------|--------|
| **SSOTの明確化** | ❌ 不足 | Linear / progress.md / session-history の役割が曖昧 |
| **進捗管理の自動更新** | ❌ 不足 | progress.md が手動更新で放置されている |
| **再開時に読むべきファイル** | ❌ 不足 | どこから再開すればいいか不明確 |

---

## 3. 現状の問題点

### 3.1 docs/の状態

| 項目 | 状態 | 問題 |
|------|------|------|
| `docs/index.md` | Session 47で止まっている | 古い（45セッション遅れ） |
| `docs/progress.md` | Session 234で止まっている | 古い（45セッション遅れ） |
| `missions/m1-sensor-evaluation/gnss/` | 46ファイル | 多すぎる（番号管理はOK） |
| sessions/への参照 | index.mdに残っている | 正式配置が完了していない |

### 3.2 進捗管理の分散

```
現状:
┌─────────────┐  ┌─────────────┐  ┌─────────────┐
│   Linear    │  │ progress.md │  │session-hist.│
│ （タスク）   │  │ （俯瞰）     │  │ （記録）     │
└─────────────┘  └─────────────┘  └─────────────┘
      ↓               ↓               ↓
   最新         Session 234       最新
                  で停止
```

**問題**: 3つが独立して存在し、同期されていない

---

## 4. 改善案

### 4.1 SSOTの明確化

| 情報の種類 | SSOT（正） | 他からの参照方法 |
|-----------|-----------|----------------|
| **タスク状態** | Linear | progress.mdからリンク |
| **セッション記録** | session-history | progress.mdからリンク |
| **俯瞰ビュー** | progress.md | なし（ハブとして機能） |

### 4.2 progress.md の役割再定義

**現状**: 各ミッションの「状況」を文章で書いている
**改善後**: **ハブ（リンク集）として機能**させる

```markdown
# ミッション進捗

## M1: GNSS評価ツール

**状態**: 開発中（Phase 3）
**Linear**: [M1-GNSS Project](リンク)
**最新セッション**: [Session 229 summary](リンク)
**再開時に読む**: [docs/missions/m1-sensor-evaluation/gnss/README.md](リンク)
```

### 4.3 更新ルール（セッション終了時）

**IMPORTANT**: 以下を `session-management` スキルに追加

```
セッション終了時:
1. session-history に追記
2. progress.md の該当ミッションを更新
   - 最新セッションへのリンク
   - 状態が変わった場合のみ状態を更新
```

**ポイント**: 詳細は書かない、リンクだけ更新

---

## 5. メンテナンスフリーな運用の実現

### 5.1 原則

| 原則 | 実現方法 |
|------|----------|
| **書く量を減らす** | 詳細はリンク先に、progress.mdは最小限 |
| **更新タイミングを明確に** | セッション終了時のみ |
| **更新内容を限定** | リンクと状態のみ（説明文は書かない） |

### 5.2 docs/index.md の扱い

**選択肢**:

| 選択肢 | メリット | デメリット |
|--------|---------|-----------|
| A: 廃止 | メンテナンス不要 | 新規参加者が困る |
| B: 最小化 | 構造だけ示す | 更新は必要 |
| C: 現状維持 | - | 更新されない |

**推奨**: B（最小化）— ディレクトリ構造とREADME.mdへのリンクのみ

---

## 6. 次のアクション

| # | 作業 | 優先度 |
|---|------|--------|
| 1 | progress.md を「ハブ形式」に書き換え | 高 |
| 2 | session-management スキルに更新ルール追加 | 高 |
| 3 | docs/index.md を最小化 | 中 |
| 4 | 必要に応じて progress-management スキル作成 | 低 |

---

## 出典

| トピック | URL |
|----------|-----|
| SSOT原則 | https://www.atlassian.com/work-management/knowledge-sharing/documentation/building-a-single-source-of-truth-ssot-for-your-team |
| SSOT定義 | https://en.wikipedia.org/wiki/Single_source_of_truth |
| Linear + Notion使い分け | https://alaniswright.com/blog/how-we-are-using-linear-and-notion-to-manage-our-product-backlog-and-project-work/ |
| Living Documentation | https://codelucky.com/agile-documentation-living-approach/ |
| Docs as Code | https://google.github.io/styleguide/docguide/best_practices.html |
| コンテキストスイッチング | https://www.hatica.io/blog/context-switching-killing-developer-productivity/ |
