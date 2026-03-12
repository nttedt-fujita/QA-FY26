"use client";

import { useState, useCallback, useRef, useEffect } from "react";
import type {
  NavStatusSample,
  NavSigSample,
  OutdoorInspectionSamples,
  OutdoorInspectionResult,
  OutdoorInspectionJudgment,
} from "@/types/outdoor-inspection";
import {
  calculateRtkFixRate,
  calculateRtkFixTime,
  calculateL2ReceptionRate,
  calculateMinL1Cno,
  judgeOutdoorInspection,
} from "@/lib/outdoor-inspection-calc";

/**
 * 検査状態
 */
export type InspectionState = "idle" | "running" | "completed";

/**
 * 検査結果（簡易版、DB保存前）
 */
export interface InspectionSummary {
  rtk_fix_rate: number;
  rtk_fix_time_ms: number | null;
  l2_reception_rate: number;
  l1_min_cno: number;
  sample_count: number;
  judgment: OutdoorInspectionJudgment;
}

/**
 * useOutdoorInspectionの戻り値
 */
export interface UseOutdoorInspectionReturn {
  // 状態
  state: InspectionState;
  remainingTime: number;
  sampleCount: number;

  // 結果（検査完了時のみ）
  result: InspectionSummary | null;

  // アクション
  start: (durationSec: number) => void;
  stop: () => void;
  reset: () => void;

  // サンプル追加（パネルから呼び出される）
  addNavStatusSample: (sample: Omit<NavStatusSample, "timestamp">) => void;
  addNavSigSample: (sample: Omit<NavSigSample, "timestamp">) => void;
}

/**
 * 屋外検査のサンプル蓄積・集計を行うHook
 */
export function useOutdoorInspection(): UseOutdoorInspectionReturn {
  // 検査状態
  const [state, setState] = useState<InspectionState>("idle");
  const [remainingTime, setRemainingTime] = useState(0);

  // サンプル蓄積
  const [navStatusSamples, setNavStatusSamples] = useState<NavStatusSample[]>(
    []
  );
  const [navSigSamples, setNavSigSamples] = useState<NavSigSample[]>([]);

  // 検査開始時刻
  const startTimeRef = useRef<number>(0);

  // タイマー
  const timerRef = useRef<NodeJS.Timeout | null>(null);

  // 結果
  const [result, setResult] = useState<InspectionSummary | null>(null);

  // サンプル数
  const sampleCount = navStatusSamples.length;

  // 集計処理
  const aggregate = useCallback((): InspectionSummary => {
    const rtkFixRate = calculateRtkFixRate(navStatusSamples);
    const rtkFixTime = calculateRtkFixTime(navStatusSamples, startTimeRef.current);
    const l2ReceptionRate = calculateL2ReceptionRate(navSigSamples);
    const l1MinCno = calculateMinL1Cno(navSigSamples);

    const judgment = judgeOutdoorInspection({
      rtk_fix_rate: rtkFixRate,
      rtk_fix_time_ms: rtkFixTime,
      l2_reception_rate: l2ReceptionRate,
      l1_min_cno: l1MinCno,
    });

    return {
      rtk_fix_rate: rtkFixRate,
      rtk_fix_time_ms: rtkFixTime,
      l2_reception_rate: l2ReceptionRate,
      l1_min_cno: l1MinCno,
      sample_count: navStatusSamples.length,
      judgment,
    };
  }, [navStatusSamples, navSigSamples]);

  // 検査終了処理
  const finishInspection = useCallback(() => {
    if (timerRef.current) {
      clearInterval(timerRef.current);
      timerRef.current = null;
    }
    setState("completed");
    setRemainingTime(0);
    const summary = aggregate();
    setResult(summary);
  }, [aggregate]);

  // 検査開始
  const start = useCallback((durationSec: number) => {
    // リセット
    setNavStatusSamples([]);
    setNavSigSamples([]);
    setResult(null);
    startTimeRef.current = Date.now();
    setState("running");
    setRemainingTime(durationSec);

    // カウントダウン
    timerRef.current = setInterval(() => {
      setRemainingTime((prev) => {
        if (prev <= 1) {
          // ここで直接finishInspectionを呼ぶと状態が古い可能性があるので、
          // useEffectで監視する
          return 0;
        }
        return prev - 1;
      });
    }, 1000);
  }, []);

  // remainingTimeが0になったら終了
  useEffect(() => {
    if (state === "running" && remainingTime === 0) {
      finishInspection();
    }
  }, [state, remainingTime, finishInspection]);

  // 検査停止（手動）
  const stop = useCallback(() => {
    finishInspection();
  }, [finishInspection]);

  // リセット
  const reset = useCallback(() => {
    if (timerRef.current) {
      clearInterval(timerRef.current);
      timerRef.current = null;
    }
    setState("idle");
    setRemainingTime(0);
    setNavStatusSamples([]);
    setNavSigSamples([]);
    setResult(null);
    startTimeRef.current = 0;
  }, []);

  // サンプル追加
  const addNavStatusSample = useCallback(
    (sample: Omit<NavStatusSample, "timestamp">) => {
      if (state !== "running") return;
      setNavStatusSamples((prev) => [
        ...prev,
        { ...sample, timestamp: Date.now() },
      ]);
    },
    [state]
  );

  const addNavSigSample = useCallback(
    (sample: Omit<NavSigSample, "timestamp">) => {
      if (state !== "running") return;
      setNavSigSamples((prev) => [...prev, { ...sample, timestamp: Date.now() }]);
    },
    [state]
  );

  // クリーンアップ
  useEffect(() => {
    return () => {
      if (timerRef.current) {
        clearInterval(timerRef.current);
      }
    };
  }, []);

  return {
    state,
    remainingTime,
    sampleCount,
    result,
    start,
    stop,
    reset,
    addNavStatusSample,
    addNavSigSample,
  };
}
