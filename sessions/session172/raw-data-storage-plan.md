# 屋外検査 生データ保存機能 設計計画書

**作成**: Session 172 (2026-03-13)
**ステータス**: ドラフト（レビュー待ち）

---

## 0. 確認した資料・コード一覧

| 種類 | ファイル | 確認内容 |
|------|---------|---------|
| ドメインモデル | [19-gnss-unified-domain-model.md](../../docs/missions/m1-sensor-evaluation/gnss/19-gnss-unified-domain-model.md) | 統合ドメインモデル、エンティティ構造 |
| DB設計 | [prototype/m1-gnss/db/schema.sql](../../prototype/m1-gnss/db/schema.sql) | 既存テーブル構造 |
| 設計資料 | [session105/outdoor-inspection-design.md](../session105/outdoor-inspection-design.md) | 屋外検査機能設計 |
| 要求整理 | [session106/outdoor-inspection-needs.md](../session106/outdoor-inspection-needs.md) | 屋外検査の要求（What） |
| BE型定義 | [repository/types.rs](../../prototype/m1-gnss/backend/src/repository/types.rs) | OutdoorInspectionResult構造体 |
| BE API | [outdoor_inspection_api.rs](../../prototype/m1-gnss/backend/src/web/outdoor_inspection_api.rs) | 現在のAPI実装 |
| FE Hook | [useOutdoorInspection.ts](../../prototype/m1-gnss/frontend/src/hooks/useOutdoorInspection.ts) | 検査フック |
| FE型定義 | [outdoor-inspection.ts](../../prototype/m1-gnss/frontend/src/types/outdoor-inspection.ts) | サンプル型定義 |
| FE API | [api.ts](../../prototype/m1-gnss/frontend/src/lib/api.ts) | GnssStateResponse定義 |

---

## 1. 背景・目的

### 1.1 現状の問題

現在、屋外検査では集計結果のみDBに保存している。

| 保存されている | 保存されていない |
|--------------|----------------|
| rtk_fix_rate, l1_min_cno 等の集計値 | 各時点のスカイプロット、NAV-SIG等の生データ |

**結果**:
- 検査後にスカイプロット等を再表示できない
- BEだけでテストしても結果の良し悪しが可視化できない
- デバッグ時にFEを立ち上げる必要がある

### 1.2 目的

検査中の生データ（GnssStateResponse）を保存し、後から再生できるようにする。

**期待する効果**:
1. 検査結果をDBから選択→スカイプロット等で再表示
2. BEテスト時も後から結果を確認可能
3. 問題発生時のデバッグが容易に

---

## 2. 既存設計の確認（重要）

### 2.1 既存のDBスキーマ

**既に詳細データ用のテーブルが存在する**（schema.sql）:

```
outdoor_measurements       ← 屋外計測セッション（場所・天候等含む）
  ├── nav_pvt_records      ← NAV-PVTの時系列
  ├── nav_status_records   ← NAV-STATUSの時系列
  ├── nav_dop_records      ← NAV-DOPの時系列
  ├── satellites           ← NAV-SATの時系列
  ├── signals              ← NAV-SIGの時系列
  ├── mon_span_records     ← MON-SPANの時系列
  └── mon_rf_records       ← MON-RFの時系列
```

### 2.2 現在の実装状況

| テーブル | スキーマ | 実装 | 備考 |
|---------|---------|------|------|
| outdoor_inspection_results | ✅ あり | ✅ 使用中 | 集計結果のみ |
| outdoor_measurements | ✅ あり | ❌ 未使用 | 詳細計測セッション |
| nav_pvt_records等 | ✅ あり | ❌ 未使用 | 時系列データ |

### 2.3 2つのテーブルの関係

ドメインモデル（19-gnss-unified-domain-model.md）によると:

- **outdoor_measurements**: 詳細な計測セッション（研究・分析用）
  - 場所、天候、基地局情報等の条件
  - UBXメッセージの時系列データ
  - 評価結果（判定、所見）

- **outdoor_inspection_results**: 検査合否判定（運用用）
  - ADR-008の合格基準に基づく集計
  - 合格/不合格の判定

**現状の乖離**: スキーマは詳細設計されているが、実装は集計結果のみ。

---

## 3. 設計オプション

### オプションA: 新規スナップショットテーブル（シンプル案）

```sql
CREATE TABLE outdoor_inspection_snapshots (
    id INTEGER PRIMARY KEY,
    inspection_id INTEGER NOT NULL,  -- FK → outdoor_inspection_results
    timestamp INTEGER NOT NULL,
    snapshot TEXT NOT NULL,          -- GnssStateResponse全体のJSON
    ...
);
```

| メリット | デメリット |
|---------|-----------|
| 実装がシンプル | 既存設計との乖離 |
| 必要なデータだけ保存 | 二重設計（outdoor_measurementsと重複） |

### オプションB: 既存のoutdoor_measurements系を活用

```
outdoor_inspection_results
  └── measurement_id (FK) → outdoor_measurements
                               ├── nav_pvt_records
                               ├── satellites
                               └── ...
```

| メリット | デメリット |
|---------|-----------|
| 既存設計との整合性 | 実装量が多い |
| 正規化されたデータ構造 | テーブル間の結合が必要 |

### オプションC: ハイブリッド（推奨案）

`outdoor_inspection_results`にスナップショット用の参照を追加:

```sql
-- 新テーブル
CREATE TABLE outdoor_inspection_snapshots (
    id INTEGER PRIMARY KEY,
    inspection_id INTEGER NOT NULL,
    timestamp INTEGER NOT NULL,
    snapshot TEXT NOT NULL,
    FOREIGN KEY (inspection_id) REFERENCES outdoor_inspection_results(id)
);
```

| メリット | デメリット |
|---------|-----------|
| 最小限の変更 | outdoor_measurementsとの二重管理 |
| 後方互換性維持 | JSONパースが必要 |
| 将来的にoutdoor_measurementsに統合可能 | - |

---

## 4. 推奨: オプションC（ハイブリッド）

### 4.1 理由

1. **最小限の変更**: 現在のoutdoor_inspection_resultsを壊さない
2. **スカイプロット再生に必要十分**: JSON形式でGnssStateResponseを丸ごと保存
3. **将来の統合余地**: outdoor_measurementsへの移行パスを残す
4. **工数**: 1-2セッションで完了可能

### 4.2 将来の統合について

outdoor_measurementsは「研究・分析用」の詳細計測を想定している。
現時点では「検査結果の再生」が目的なので、シンプルなスナップショットで十分。

将来的に詳細な分析が必要になったら:
1. スナップショットデータをoutdoor_measurements系にマイグレーション
2. または、両方を並行運用

---

## 5. 詳細設計（オプションC採用時）

### 5.1 DBスキーマ

```sql
-- 新テーブル: 屋外検査スナップショット
CREATE TABLE outdoor_inspection_snapshots (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    inspection_id INTEGER NOT NULL,
    timestamp INTEGER NOT NULL,          -- ミリ秒エポック
    snapshot TEXT NOT NULL,              -- GnssStateResponseのJSON
    created_at TEXT DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (inspection_id) REFERENCES outdoor_inspection_results(id)
);

CREATE INDEX idx_snapshots_inspection ON outdoor_inspection_snapshots(inspection_id);
```

### 5.2 データサイズ見積もり

- 1スナップショット: 約5-10KB（衛星数による）
- 30秒検査（1秒間隔）: 約150-300KB
- 許容範囲

### 5.3 API変更

**POST /api/outdoor-inspection-results**（拡張）:

```json
{
  "serial_number": "...",
  "rtk_fix_rate": 0.95,
  ...,
  "snapshots": [
    {
      "timestamp": 1710000000000,
      "data": { /* GnssStateResponse */ }
    }
  ]
}
```

**GET /api/outdoor-inspection-results/{id}/snapshots**（新規）:

```json
{
  "inspection_id": 1,
  "snapshots": [...]
}
```

### 5.4 FE変更

1. **useOutdoorInspection**: `GnssStateResponse`のスナップショットを蓄積
2. **saveResult**: スナップショットも送信
3. **再生機能**: 既存パネルを流用、データソースを切り替え

---

## 6. 実装計画

### Phase 1: BE（生データ保存）

| # | タスク | 成果物 |
|---|--------|--------|
| 1-1 | DBマイグレーション | outdoor_inspection_snapshots テーブル |
| 1-2 | リポジトリ実装 | insert/get_snapshots |
| 1-3 | POST API拡張 | snapshots受け取り・保存 |
| 1-4 | GET API追加 | snapshots取得 |

### Phase 2: FE（生データ送信）

| # | タスク | 成果物 |
|---|--------|--------|
| 2-1 | useOutdoorInspection拡張 | スナップショット蓄積 |
| 2-2 | saveResult拡張 | スナップショット送信 |

### Phase 3: 再生機能

| # | タスク | 成果物 |
|---|--------|--------|
| 3-1 | 検査結果一覧画面 | DBの検査一覧を表示 |
| 3-2 | 再生モード | 選択した検査を各パネルで再生 |

---

## 7. 確認事項（次回レビュー時）

1. **オプションC（ハイブリッド）で進めてよいか？**
   - outdoor_measurementsを今は使わない判断

2. **スナップショットの保存間隔**: 1秒でよいか？

3. **データ保持期間**: 古いスナップショットの削除ポリシーは必要か？

4. **再生UI**: スライダーで時系列を動かす？ or 自動再生？

5. **Phase分割**: Phase 1-2を先に実装し、Phase 3は後でもよいか？

---

## 8. 参照

- [19-gnss-unified-domain-model.md](../../docs/missions/m1-sensor-evaluation/gnss/19-gnss-unified-domain-model.md) - ドメインモデル
- [schema.sql](../../prototype/m1-gnss/db/schema.sql) - DBスキーマ
- [ADR-008](../../docs/adr/m1/ADR-008-outdoor-inspection-criteria.md) - 合格基準
