# Session 143 計画

**目的**: 統合API実装（REST vs GraphQL検討 + TDD Phase 2〜4）

**前提**: Session 142でTDD Phase 0〜1完了（仕様設計）

---

## やること

| # | 作業 | 読むべきファイル |
|---|------|-----------------|
| 1 | REST API vs GraphQL の解説 | - |
| 2 | TDD Phase 2: テストシナリオリスト作成 | session142/session-summary.md |
| 3 | TDD Phase 3: テストコード作成（Red） | nav_sat_api.rs（テスト参考） |
| 4 | TDD Phase 4: 実装（Green） | 既存APIファイル |
| 5 | main.rsにAPI登録 | main.rs |

---

## 詳細

### 1. REST API vs GraphQL の解説

**ユーザーからの質問**:
> 統合APIってなんかかなりでかいし情報の種類多いけどREST APIみたいなんじゃなくてGraphQLのほうがいいとかあるのかな

**解説すべき観点**:
- REST vs GraphQL の特徴比較
- 統合APIのユースケースでどちらが適切か
- 実装コストの違い
- 現状のActix-webでの対応

### 2. TDD Phase 2: テストシナリオリスト

Session 142で決定した振る舞いに基づいてテストシナリオを作成:
- 正常系: 全データ取得成功
- 部分失敗: 一部メッセージのみ取得
- エラー系: 未接続、全失敗

### 3-4. TDD Phase 3-4: テスト・実装

`gnss_state_api.rs` を新規作成:
- 既存APIのパターンを踏襲
- 6メッセージを順次ポーリング
- 部分失敗時はerrorsに追加

### 5. main.rsにAPI登録

`gnss_state_api::configure` を追加

---

## 参照

- [Session 142 サマリー](../session142/session-summary.md)
- [ADR-012: 統合API方針](../../docs/adr/m1/ADR-012-periodic-output-and-unified-api.md)
- [TDDスキル](../../.claude/skills/tdd-practice/SKILL.md)
