/**
 * 屋外検査 集計関数
 *
 * 参照: sessions/session123/outdoor-inspection-domain-model.md
 */
import type {
  NavStatusSample,
  NavSigSample,
  OutdoorInspectionJudgment,
} from "@/types/outdoor-inspection";
import { OUTDOOR_INSPECTION_CRITERIA } from "@/types/outdoor-inspection";

/**
 * RTK FIX率を計算
 *
 * carr_soln=2（Fixed）のサンプル割合を返す
 */
export function calculateRtkFixRate(samples: NavStatusSample[]): number {
  if (samples.length === 0) return 0;
  const fixedCount = samples.filter((s) => s.carr_soln === 2).length;
  return fixedCount / samples.length;
}

/**
 * RTK FIX時間を計算
 *
 * 検査開始から最初のFIX（carr_soln=2）までの時間（ms）を返す
 * FIXしなかった場合はnullを返す
 */
export function calculateRtkFixTime(
  samples: NavStatusSample[],
  inspectionStartTime: number
): number | null {
  const firstFixed = samples.find((s) => s.carr_soln === 2);
  if (!firstFixed) return null;
  return firstFixed.timestamp - inspectionStartTime;
}

/**
 * L2受信率の平均を計算
 */
export function calculateL2ReceptionRate(samples: NavSigSample[]): number {
  if (samples.length === 0) return 0;
  const sum = samples.reduce((acc, s) => acc + s.gps_l2_reception_rate, 0);
  return sum / samples.length;
}

/**
 * L1 C/N0の最小値を計算
 */
export function calculateMinL1Cno(samples: NavSigSample[]): number {
  if (samples.length === 0) return 0;
  return Math.min(...samples.map((s) => s.min_l1_cno));
}

/**
 * 合否判定の入力
 */
interface JudgmentInput {
  rtk_fix_rate: number;
  rtk_fix_time_ms: number | null;
  l2_reception_rate: number;
  l1_min_cno: number;
}

/**
 * 屋外検査の合否を判定
 *
 * ADR-008基準:
 * - L1受信感度: ≥30dBHz
 * - L2受信率: ≥50%
 * - RTK FIX時間: ≤30秒
 * - RTK FIX率: >95%
 */
export function judgeOutdoorInspection(
  input: JudgmentInput
): OutdoorInspectionJudgment {
  const failureReasons: string[] = [];

  // L1受信感度
  const l1CnoPass = input.l1_min_cno >= OUTDOOR_INSPECTION_CRITERIA.L1_MIN_CNO;
  if (!l1CnoPass) {
    failureReasons.push(
      `L1最小C/N0: ${input.l1_min_cno}dBHz < 基準${OUTDOOR_INSPECTION_CRITERIA.L1_MIN_CNO}dBHz`
    );
  }

  // L2受信率
  const l2RatePass =
    input.l2_reception_rate >= OUTDOOR_INSPECTION_CRITERIA.L2_RECEPTION_RATE;
  if (!l2RatePass) {
    failureReasons.push(
      `L2受信率: ${(input.l2_reception_rate * 100).toFixed(1)}% < 基準50%`
    );
  }

  // RTK FIX時間
  const rtkFixTimePass =
    input.rtk_fix_time_ms !== null &&
    input.rtk_fix_time_ms <= OUTDOOR_INSPECTION_CRITERIA.RTK_FIX_TIME_MS;
  if (!rtkFixTimePass) {
    if (input.rtk_fix_time_ms === null) {
      failureReasons.push(`RTK FIX時間: FIXしなかった`);
    } else {
      failureReasons.push(
        `RTK FIX時間: ${input.rtk_fix_time_ms / 1000}秒 > 基準30秒`
      );
    }
  }

  // RTK FIX率（>95%、=95%は不合格）
  const rtkFixRatePass =
    input.rtk_fix_rate > OUTDOOR_INSPECTION_CRITERIA.RTK_FIX_RATE;
  if (!rtkFixRatePass) {
    failureReasons.push(
      `RTK FIX率: ${(input.rtk_fix_rate * 100).toFixed(1)}% ≤ 基準95%`
    );
  }

  const isPass = l1CnoPass && l2RatePass && rtkFixTimePass && rtkFixRatePass;

  return {
    is_pass: isPass,
    l1_cno_pass: l1CnoPass,
    l2_rate_pass: l2RatePass,
    rtk_fix_time_pass: rtkFixTimePass,
    rtk_fix_rate_pass: rtkFixRatePass,
    failure_reasons: failureReasons,
  };
}
