/**
 * 屋外検査 ドメインモデル
 *
 * 参照: sessions/session123/outdoor-inspection-domain-model.md
 */

// ===========================================
// 検査サンプル（一時データ、FE保持）
// ===========================================

/**
 * NAV-STATUSのサンプル
 */
export interface NavStatusSample {
  timestamp: number; // ミリ秒エポック
  gps_fix: number; // Fix種別
  carr_soln: number; // RTK状態 (0=なし, 1=Float, 2=Fixed)
  msss: number; // 起動からの経過時間(ms)
  ttff: number; // TTFF(ms)
}

/**
 * NAV-SIGのサンプル
 */
export interface NavSigSample {
  timestamp: number;
  gps_visible_count: number; // GPS可視衛星数
  gps_l2_reception_count: number; // GPS L2受信衛星数
  gps_l2_reception_rate: number; // L2受信率 (0.0-1.0)
  min_l1_cno: number; // L1最小C/N0
}

/**
 * 検査中の蓄積データ
 */
export interface OutdoorInspectionSamples {
  inspectionId: string; // 検査ID（UUID）
  startedAt: number; // 検査開始時刻
  durationSec: number; // 設定した検査時間
  navStatusSamples: NavStatusSample[];
  navSigSamples: NavSigSample[];
}

// ===========================================
// 検査結果（集計後）
// ===========================================

/**
 * 合否判定結果
 */
export interface OutdoorInspectionJudgment {
  is_pass: boolean; // 総合判定（全項目合格なら合格）

  // 各項目の判定
  l1_cno_pass: boolean; // L1受信感度 ≥30dBHz
  l2_rate_pass: boolean; // L2受信率 ≥50%
  rtk_fix_time_pass: boolean; // RTK FIX時間 ≤30秒
  rtk_fix_rate_pass: boolean; // RTK FIX率 >95%

  // 判定理由（不合格の場合に詳細を記載）
  failure_reasons: string[];
}

/**
 * 屋外検査結果（集計後、DB保存対象）
 */
export interface OutdoorInspectionResult {
  // 識別
  id: string; // 検査結果ID（UUID）
  device_id?: number; // デバイスID（装置画面で登録済みの場合）
  lot_id?: number; // ロットID（選択した場合）

  // 検査情報
  inspected_at: string; // 検査日時（ISO 8601）
  duration_sec: number; // 検査時間（秒）
  sample_count: number; // 総サンプル数

  // 集計結果
  rtk_fix_rate: number; // RTK FIX率 (0.0-1.0)
  rtk_fix_time_ms: number | null; // RTK FIX時間（ms）。FIXしなかった場合はnull
  l2_reception_rate: number; // L2受信率（平均）(0.0-1.0)
  l1_min_cno: number; // L1最小C/N0（dBHz）

  // 判定結果
  judgment: OutdoorInspectionJudgment;
}

// ===========================================
// 合格基準（ADR-008）
// ===========================================

export const OUTDOOR_INSPECTION_CRITERIA = {
  L1_MIN_CNO: 30, // dBHz以上
  L2_RECEPTION_RATE: 0.5, // 50%以上
  RTK_FIX_TIME_MS: 30000, // 30秒以内
  RTK_FIX_RATE: 0.95, // 95%超
} as const;
