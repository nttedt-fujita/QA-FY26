/**
 * 屋外検査 集計関数のテスト
 *
 * テーブルテスト形式（rules/06-test-style.md準拠）
 */
import { describe, it, expect } from "vitest";
import {
  calculateRtkFixRate,
  calculateRtkFixTime,
  calculateL2ReceptionRate,
  calculateMinL1Cno,
  judgeOutdoorInspection,
} from "./outdoor-inspection-calc";
import type { NavStatusSample, NavSigSample } from "@/types/outdoor-inspection";
import { OUTDOOR_INSPECTION_CRITERIA } from "@/types/outdoor-inspection";

// ===========================================
// calculateRtkFixRate
// ===========================================
describe("calculateRtkFixRate", () => {
  const testCases: Array<{
    name: string;
    samples: NavStatusSample[];
    expected: number;
    shouldSucceed: boolean;
  }> = [
    // 正常系: 全てFIX
    {
      name: "全てFIXの場合は1.0を返す",
      samples: [
        { timestamp: 1000, gps_fix: 3, carr_soln: 2, msss: 1000, ttff: 500 },
        { timestamp: 2000, gps_fix: 3, carr_soln: 2, msss: 2000, ttff: 500 },
      ],
      expected: 1.0,
      shouldSucceed: true,
    },
    // 正常系: 半分FIX
    {
      name: "半分FIXの場合は0.5を返す",
      samples: [
        { timestamp: 1000, gps_fix: 3, carr_soln: 0, msss: 1000, ttff: 500 },
        { timestamp: 2000, gps_fix: 3, carr_soln: 2, msss: 2000, ttff: 500 },
      ],
      expected: 0.5,
      shouldSucceed: true,
    },
    // 正常系: FIXなし（Floatのみ）
    {
      name: "FIXなしの場合は0.0を返す",
      samples: [
        { timestamp: 1000, gps_fix: 3, carr_soln: 0, msss: 1000, ttff: 500 },
        { timestamp: 2000, gps_fix: 3, carr_soln: 1, msss: 2000, ttff: 500 },
      ],
      expected: 0.0,
      shouldSucceed: true,
    },
    // 境界値: 空配列
    {
      name: "空配列の場合は0.0を返す",
      samples: [],
      expected: 0.0,
      shouldSucceed: true,
    },
  ];

  testCases.forEach(({ name, samples, expected, shouldSucceed }) => {
    it(name, () => {
      if (shouldSucceed) {
        expect(calculateRtkFixRate(samples)).toBe(expected);
      }
    });
  });
});

// ===========================================
// calculateRtkFixTime
// ===========================================
describe("calculateRtkFixTime", () => {
  const testCases: Array<{
    name: string;
    samples: NavStatusSample[];
    startTime: number;
    expected: number | null;
    shouldSucceed: boolean;
  }> = [
    // 正常系: FIXあり
    {
      name: "最初のFIXまでの時間を返す",
      samples: [
        { timestamp: 1000, gps_fix: 3, carr_soln: 0, msss: 1000, ttff: 500 },
        { timestamp: 2000, gps_fix: 3, carr_soln: 1, msss: 2000, ttff: 500 },
        { timestamp: 3000, gps_fix: 3, carr_soln: 2, msss: 3000, ttff: 500 },
        { timestamp: 4000, gps_fix: 3, carr_soln: 2, msss: 4000, ttff: 500 },
      ],
      startTime: 0,
      expected: 3000,
      shouldSucceed: true,
    },
    // 正常系: 即座にFIX
    {
      name: "即座にFIXした場合は最初のサンプルの時間を返す",
      samples: [
        { timestamp: 1000, gps_fix: 3, carr_soln: 2, msss: 1000, ttff: 500 },
      ],
      startTime: 0,
      expected: 1000,
      shouldSucceed: true,
    },
    // 正常系: FIXなし
    {
      name: "FIXしなかった場合はnullを返す",
      samples: [
        { timestamp: 1000, gps_fix: 3, carr_soln: 0, msss: 1000, ttff: 500 },
        { timestamp: 2000, gps_fix: 3, carr_soln: 1, msss: 2000, ttff: 500 },
      ],
      startTime: 0,
      expected: null,
      shouldSucceed: true,
    },
    // 境界値: 空配列
    {
      name: "空配列の場合はnullを返す",
      samples: [],
      startTime: 0,
      expected: null,
      shouldSucceed: true,
    },
  ];

  testCases.forEach(({ name, samples, startTime, expected, shouldSucceed }) => {
    it(name, () => {
      if (shouldSucceed) {
        expect(calculateRtkFixTime(samples, startTime)).toBe(expected);
      }
    });
  });
});

// ===========================================
// calculateL2ReceptionRate
// ===========================================
describe("calculateL2ReceptionRate", () => {
  const testCases: Array<{
    name: string;
    samples: NavSigSample[];
    expected: number;
    shouldSucceed: boolean;
  }> = [
    // 正常系: 全て100%
    {
      name: "全て100%の場合は1.0を返す",
      samples: [
        {
          timestamp: 1000,
          gps_visible_count: 10,
          gps_l2_reception_count: 10,
          gps_l2_reception_rate: 1.0,
          min_l1_cno: 35,
        },
        {
          timestamp: 2000,
          gps_visible_count: 10,
          gps_l2_reception_count: 10,
          gps_l2_reception_rate: 1.0,
          min_l1_cno: 35,
        },
      ],
      expected: 1.0,
      shouldSucceed: true,
    },
    // 正常系: 変動あり
    {
      name: "変動がある場合は平均を返す",
      samples: [
        {
          timestamp: 1000,
          gps_visible_count: 10,
          gps_l2_reception_count: 5,
          gps_l2_reception_rate: 0.5,
          min_l1_cno: 35,
        },
        {
          timestamp: 2000,
          gps_visible_count: 10,
          gps_l2_reception_count: 7,
          gps_l2_reception_rate: 0.7,
          min_l1_cno: 35,
        },
      ],
      expected: 0.6,
      shouldSucceed: true,
    },
    // 境界値: 空配列
    {
      name: "空配列の場合は0.0を返す",
      samples: [],
      expected: 0.0,
      shouldSucceed: true,
    },
  ];

  testCases.forEach(({ name, samples, expected, shouldSucceed }) => {
    it(name, () => {
      if (shouldSucceed) {
        expect(calculateL2ReceptionRate(samples)).toBeCloseTo(expected, 5);
      }
    });
  });
});

// ===========================================
// calculateMinL1Cno
// ===========================================
describe("calculateMinL1Cno", () => {
  const testCases: Array<{
    name: string;
    samples: NavSigSample[];
    expected: number;
    shouldSucceed: boolean;
  }> = [
    // 正常系: 複数サンプル
    {
      name: "最小値を返す",
      samples: [
        {
          timestamp: 1000,
          gps_visible_count: 10,
          gps_l2_reception_count: 5,
          gps_l2_reception_rate: 0.5,
          min_l1_cno: 35,
        },
        {
          timestamp: 2000,
          gps_visible_count: 10,
          gps_l2_reception_count: 5,
          gps_l2_reception_rate: 0.5,
          min_l1_cno: 28,
        },
        {
          timestamp: 3000,
          gps_visible_count: 10,
          gps_l2_reception_count: 5,
          gps_l2_reception_rate: 0.5,
          min_l1_cno: 32,
        },
      ],
      expected: 28,
      shouldSucceed: true,
    },
    // 境界値: 空配列
    {
      name: "空配列の場合は0を返す",
      samples: [],
      expected: 0,
      shouldSucceed: true,
    },
  ];

  testCases.forEach(({ name, samples, expected, shouldSucceed }) => {
    it(name, () => {
      if (shouldSucceed) {
        expect(calculateMinL1Cno(samples)).toBe(expected);
      }
    });
  });
});

// ===========================================
// judgeOutdoorInspection
// ===========================================
describe("judgeOutdoorInspection", () => {
  // 合格基準（ADR-008）
  // L1_MIN_CNO: 30dBHz以上
  // L2_RECEPTION_RATE: 50%以上
  // RTK_FIX_TIME_MS: 30秒以内
  // RTK_FIX_RATE: 95%超

  const testCases: Array<{
    name: string;
    input: {
      rtk_fix_rate: number;
      rtk_fix_time_ms: number | null;
      l2_reception_rate: number;
      l1_min_cno: number;
    };
    expectedPass: boolean;
    shouldSucceed: boolean;
  }> = [
    // 正常系: 全項目合格
    {
      name: "全項目合格の場合はis_pass=trueを返す",
      input: {
        rtk_fix_rate: 0.98, // >95%
        rtk_fix_time_ms: 5000, // <30秒
        l2_reception_rate: 0.6, // ≥50%
        l1_min_cno: 35, // ≥30dBHz
      },
      expectedPass: true,
      shouldSucceed: true,
    },
    // 正常系: L1 C/N0不合格
    {
      name: "L1 C/N0が基準未満の場合はis_pass=falseを返す",
      input: {
        rtk_fix_rate: 0.98,
        rtk_fix_time_ms: 5000,
        l2_reception_rate: 0.6,
        l1_min_cno: 25, // <30dBHz
      },
      expectedPass: false,
      shouldSucceed: true,
    },
    // 正常系: L2受信率不合格
    {
      name: "L2受信率が基準未満の場合はis_pass=falseを返す",
      input: {
        rtk_fix_rate: 0.98,
        rtk_fix_time_ms: 5000,
        l2_reception_rate: 0.4, // <50%
        l1_min_cno: 35,
      },
      expectedPass: false,
      shouldSucceed: true,
    },
    // 正常系: RTK FIX時間不合格
    {
      name: "RTK FIX時間が基準超過の場合はis_pass=falseを返す",
      input: {
        rtk_fix_rate: 0.98,
        rtk_fix_time_ms: 35000, // >30秒
        l2_reception_rate: 0.6,
        l1_min_cno: 35,
      },
      expectedPass: false,
      shouldSucceed: true,
    },
    // 正常系: RTK FIX率不合格（ちょうど95%）
    {
      name: "RTK FIX率が95%以下の場合はis_pass=falseを返す",
      input: {
        rtk_fix_rate: 0.95, // ≤95%（>95%が基準）
        rtk_fix_time_ms: 5000,
        l2_reception_rate: 0.6,
        l1_min_cno: 35,
      },
      expectedPass: false,
      shouldSucceed: true,
    },
    // 正常系: RTK FIXしなかった
    {
      name: "RTK FIXしなかった場合はis_pass=falseを返す",
      input: {
        rtk_fix_rate: 0.0,
        rtk_fix_time_ms: null,
        l2_reception_rate: 0.6,
        l1_min_cno: 35,
      },
      expectedPass: false,
      shouldSucceed: true,
    },
    // 境界値: ギリギリ合格
    {
      name: "境界値で合格の場合",
      input: {
        rtk_fix_rate: 0.951, // >95%
        rtk_fix_time_ms: 30000, // ≤30秒
        l2_reception_rate: 0.5, // ≥50%
        l1_min_cno: 30, // ≥30dBHz
      },
      expectedPass: true,
      shouldSucceed: true,
    },
  ];

  testCases.forEach(({ name, input, expectedPass, shouldSucceed }) => {
    it(name, () => {
      if (shouldSucceed) {
        const result = judgeOutdoorInspection(input);
        expect(result.is_pass).toBe(expectedPass);
      }
    });
  });

  // 不合格理由のテスト
  it("不合格の場合はfailure_reasonsに理由が含まれる", () => {
    const result = judgeOutdoorInspection({
      rtk_fix_rate: 0.9,
      rtk_fix_time_ms: 35000,
      l2_reception_rate: 0.4,
      l1_min_cno: 25,
    });

    expect(result.is_pass).toBe(false);
    expect(result.failure_reasons.length).toBe(4);
    expect(result.l1_cno_pass).toBe(false);
    expect(result.l2_rate_pass).toBe(false);
    expect(result.rtk_fix_time_pass).toBe(false);
    expect(result.rtk_fix_rate_pass).toBe(false);
  });
});
