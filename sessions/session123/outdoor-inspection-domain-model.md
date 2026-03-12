# 屋外検査 ドメインモデル設計

**作成日**: 2026-03-12 Session 123
**目的**: 屋外検査結果のデータ構造と処理フローを設計する

---

## 1. 要求の整理

### What（変えられない要求）

1. 屋外検査で「合格/不合格」を判定したい
2. 判定根拠となる数値を記録・表示したい
3. 屋内検査と屋外検査を同一デバイスに紐づけたい

### How（変えられる要件）

- 集計方法（平均/最終値/中央値）
- サンプリング間隔
- 保存形式・タイミング

---

## 2. 検査フローの整理

```
[検査前]
  ↓
  デバイス接続確認
  ロット選択（任意）
  ↓
[検査開始]
  ↓
  指定時間（30秒〜5分）のポーリング開始
  - NAV-STATUS: 1秒間隔で取得
  - NAV-SIG: 1秒間隔で取得
  - MON-SPAN: 1秒間隔で取得
  ↓
  サンプル蓄積（FE React状態）
  ↓
[検査終了]
  ↓
  集計処理
  - RTK FIX率 = carrSoln=2のサンプル数 / 総サンプル数
  - L2受信率 = 全サンプルの平均値
  - RTK FIX時間 = 最初のcarrSoln=2サンプルの時刻 - 検査開始時刻
  - L1最小C/N0 = 全サンプル中の最小値（異常検出用）
  ↓
  合否判定（ADR-008基準）
  ↓
  結果表示
  ↓
[結果保存]（任意）
  ↓
  DBに保存
```

---

## 3. データモデル

### 3.1 検査サンプル（一時データ、FE保持）

```typescript
// NAV-STATUSのサンプル
interface NavStatusSample {
  timestamp: number;      // ミリ秒エポック
  gps_fix: number;        // Fix種別
  carr_soln: number;      // RTK状態 (0=なし, 1=Float, 2=Fixed)
  msss: number;           // 起動からの経過時間(ms)
  ttff: number;           // TTFF(ms)
}

// NAV-SIGのサンプル
interface NavSigSample {
  timestamp: number;
  gps_visible_count: number;      // GPS可視衛星数
  gps_l2_reception_count: number; // GPS L2受信衛星数
  gps_l2_reception_rate: number;  // L2受信率 (0.0-1.0)
  min_l1_cno: number;             // L1最小C/N0
}

// 検査中の蓄積データ
interface OutdoorInspectionSamples {
  inspectionId: string;           // 検査ID（UUID）
  startedAt: number;              // 検査開始時刻
  durationSec: number;            // 設定した検査時間
  navStatusSamples: NavStatusSample[];
  navSigSamples: NavSigSample[];
}
```

### 3.2 検査結果（集計後、DB保存対象）

```typescript
interface OutdoorInspectionResult {
  // 識別
  id: string;                     // 検査結果ID（UUID）
  device_id?: number;             // デバイスID（装置画面で登録済みの場合）
  lot_id?: number;                // ロットID（選択した場合）

  // 検査情報
  inspected_at: string;           // 検査日時（ISO 8601）
  duration_sec: number;           // 検査時間（秒）
  sample_count: number;           // 総サンプル数

  // 集計結果
  rtk_fix_rate: number;           // RTK FIX率 (0.0-1.0)
  rtk_fix_time_ms: number | null; // RTK FIX時間（ms）。FIXしなかった場合はnull
  l2_reception_rate: number;      // L2受信率（平均）(0.0-1.0)
  l1_min_cno: number;             // L1最小C/N0（dBHz）

  // 判定結果
  judgment: OutdoorInspectionJudgment;
}

interface OutdoorInspectionJudgment {
  is_pass: boolean;               // 総合判定（全項目合格なら合格）

  // 各項目の判定
  l1_cno_pass: boolean;           // L1受信感度 ≥30dBHz
  l2_rate_pass: boolean;          // L2受信率 ≥50%
  rtk_fix_time_pass: boolean;     // RTK FIX時間 ≤30秒
  rtk_fix_rate_pass: boolean;     // RTK FIX率 >95%

  // 判定理由（不合格の場合に詳細を記載）
  failure_reasons: string[];
}
```

---

## 4. 集計ロジック

### 4.1 RTK FIX率

```typescript
function calculateRtkFixRate(samples: NavStatusSample[]): number {
  if (samples.length === 0) return 0;
  const fixedCount = samples.filter(s => s.carr_soln === 2).length;
  return fixedCount / samples.length;
}
```

### 4.2 RTK FIX時間

```typescript
function calculateRtkFixTime(
  samples: NavStatusSample[],
  inspectionStartTime: number
): number | null {
  const firstFixed = samples.find(s => s.carr_soln === 2);
  if (!firstFixed) return null;
  return firstFixed.timestamp - inspectionStartTime;
}
```

### 4.3 L2受信率（平均）

```typescript
function calculateL2ReceptionRate(samples: NavSigSample[]): number {
  if (samples.length === 0) return 0;
  const sum = samples.reduce((acc, s) => acc + s.gps_l2_reception_rate, 0);
  return sum / samples.length;
}
```

### 4.4 L1最小C/N0

```typescript
function calculateMinL1Cno(samples: NavSigSample[]): number {
  if (samples.length === 0) return 0;
  return Math.min(...samples.map(s => s.min_l1_cno));
}
```

---

## 5. 合否判定ロジック

```typescript
// ADR-008基準
const CRITERIA = {
  L1_MIN_CNO: 30,           // dBHz以上
  L2_RECEPTION_RATE: 0.5,   // 50%以上
  RTK_FIX_TIME_MS: 30000,   // 30秒以内
  RTK_FIX_RATE: 0.95,       // 95%超
};

function judgeOutdoorInspection(result: OutdoorInspectionResult): OutdoorInspectionJudgment {
  const failure_reasons: string[] = [];

  // L1受信感度
  const l1_cno_pass = result.l1_min_cno >= CRITERIA.L1_MIN_CNO;
  if (!l1_cno_pass) {
    failure_reasons.push(`L1最小C/N0: ${result.l1_min_cno}dBHz < 基準${CRITERIA.L1_MIN_CNO}dBHz`);
  }

  // L2受信率
  const l2_rate_pass = result.l2_reception_rate >= CRITERIA.L2_RECEPTION_RATE;
  if (!l2_rate_pass) {
    failure_reasons.push(`L2受信率: ${(result.l2_reception_rate * 100).toFixed(1)}% < 基準50%`);
  }

  // RTK FIX時間
  const rtk_fix_time_pass = result.rtk_fix_time_ms !== null &&
                             result.rtk_fix_time_ms <= CRITERIA.RTK_FIX_TIME_MS;
  if (!rtk_fix_time_pass) {
    if (result.rtk_fix_time_ms === null) {
      failure_reasons.push(`RTK FIX時間: FIXしなかった`);
    } else {
      failure_reasons.push(`RTK FIX時間: ${result.rtk_fix_time_ms / 1000}秒 > 基準30秒`);
    }
  }

  // RTK FIX率
  const rtk_fix_rate_pass = result.rtk_fix_rate > CRITERIA.RTK_FIX_RATE;
  if (!rtk_fix_rate_pass) {
    failure_reasons.push(`RTK FIX率: ${(result.rtk_fix_rate * 100).toFixed(1)}% ≤ 基準95%`);
  }

  const is_pass = l1_cno_pass && l2_rate_pass && rtk_fix_time_pass && rtk_fix_rate_pass;

  return {
    is_pass,
    l1_cno_pass,
    l2_rate_pass,
    rtk_fix_time_pass,
    rtk_fix_rate_pass,
    failure_reasons,
  };
}
```

---

## 6. 屋内/屋外検査の紐づけ

### 現状のデータモデル（屋内検査）

現在、屋内検査結果は `inspection_results` テーブルに保存されている:

```sql
-- 既存（屋内検査）
CREATE TABLE inspection_results (
  id INTEGER PRIMARY KEY,
  device_id INTEGER REFERENCES devices(id),
  lot_id INTEGER REFERENCES lots(id),
  inspected_at TEXT,
  rate_ok BOOLEAN,
  uart1_ok BOOLEAN,
  uart2_ok BOOLEAN,
  usb_ok BOOLEAN,
  nav_ok BOOLEAN,
  is_pass BOOLEAN
);
```

### 設計選択肢

| 案 | 内容 | メリット | デメリット |
|----|------|---------|-----------|
| A: 別テーブル | `outdoor_inspection_results` 新規作成 | シンプル、独立性高い | 両方の結果を取得するのに2クエリ必要 |
| B: 統合テーブル | `inspection_results` に屋外用カラム追加 | 1テーブルで完結 | カラムが増えて複雑に |
| C: 検査種別分離 | `inspection_results` + `inspection_type` カラム | 拡張性高い | 既存スキーマ変更が必要 |

**推奨: 案A（別テーブル）**

理由:
- 屋内と屋外は検査項目が完全に異なる
- プロトタイプ段階ではシンプルさ優先
- 後から統合も可能

### 提案スキーマ

```sql
-- 屋外検査結果
CREATE TABLE outdoor_inspection_results (
  id INTEGER PRIMARY KEY,
  device_id INTEGER REFERENCES devices(id),
  lot_id INTEGER REFERENCES lots(id),
  inspected_at TEXT NOT NULL,           -- ISO 8601
  duration_sec INTEGER NOT NULL,        -- 検査時間（秒）
  sample_count INTEGER NOT NULL,        -- サンプル数

  -- 集計結果
  rtk_fix_rate REAL NOT NULL,           -- RTK FIX率 (0.0-1.0)
  rtk_fix_time_ms INTEGER,              -- RTK FIX時間（ms）、nullはFIXしなかった
  l2_reception_rate REAL NOT NULL,      -- L2受信率 (0.0-1.0)
  l1_min_cno REAL NOT NULL,             -- L1最小C/N0 (dBHz)

  -- 判定結果
  is_pass BOOLEAN NOT NULL,
  l1_cno_pass BOOLEAN NOT NULL,
  l2_rate_pass BOOLEAN NOT NULL,
  rtk_fix_time_pass BOOLEAN NOT NULL,
  rtk_fix_rate_pass BOOLEAN NOT NULL,
  failure_reasons TEXT,                 -- JSON配列（不合格理由）

  created_at TEXT DEFAULT CURRENT_TIMESTAMP
);
```

---

## 7. 実装順序

1. **FE: サンプル蓄積Hook作成** (`useOutdoorInspection.ts`)
   - 検査開始/停止
   - サンプル蓄積
   - 集計処理
   - 判定処理

2. **FE: 検査結果表示コンポーネント** (`OutdoorInspectionResultPanel.tsx`)
   - 集計結果表示
   - 合否判定表示
   - 不合格理由表示

3. **BE: DBスキーマ追加** (`outdoor_inspection_results`)

4. **BE: 保存API** (`POST /api/outdoor-inspection-results`)

5. **FE: 保存ボタン追加**

---

## 8. 未決定事項

| 項目 | 選択肢 | 決定 |
|------|--------|------|
| L2受信率の集計方法 | 平均 / 最終値 / 中央値 | **平均**（変動を平滑化） |
| L1 C/N0の集計方法 | 最小 / 平均 / 最終値 | **最小**（異常検出重視） |
| サンプリング間隔 | 100ms / 500ms / 1000ms | **1000ms**（現行のまま） |

---

## 参照資料

- [ADR-008](../../docs/adr/m1/ADR-008-outdoor-inspection-criteria.md) — 合格基準
- [remaining-tasks.md](./remaining-tasks.md) — 残タスク一覧
