"use client";

import { useState, useCallback, useRef, useEffect } from "react";
import type {
  NavStatusSample,
  NavSigSample,
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
 * - idle: 待機中
 * - starting: 検査開始処理中（ボタン連打防止）
 * - running: 検査実行中
 * - completing: 終了処理中（集計・保存準備）
 * - completed: 完了
 */
export type InspectionState = "idle" | "starting" | "running" | "completing" | "completed";

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
 * 保存状態
 */
export type SaveState = "idle" | "saving" | "saved" | "error";

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

  // 保存状態
  saveState: SaveState;
  saveError: string | null;
  savedId: number | null;

  // アクション
  start: (durationSec: number) => void;
  stop: () => void;
  reset: () => void;
  saveResult: (serialNumber?: string, lotId?: number) => Promise<void>;

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

  // 保存状態
  const [saveState, setSaveState] = useState<SaveState>("idle");
  const [saveError, setSaveError] = useState<string | null>(null);
  const [savedId, setSavedId] = useState<number | null>(null);

  // 検査時間（保存時に使用）
  const durationSecRef = useRef<number>(0);

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
    // 終了処理中状態に遷移（UIで「集計中...」表示）
    setState("completing");
    setRemainingTime(0);
    const summary = aggregate();
    setResult(summary);
    // 集計完了後に完了状態へ
    setState("completed");
  }, [aggregate]);

  // 検査開始
  const start = useCallback((durationSec: number) => {
    // 重複防止: idle状態でなければ開始しない
    if (state !== "idle") return;

    // 開始処理中状態に遷移（ボタン連打防止）
    setState("starting");

    // リセット
    setNavStatusSamples([]);
    setNavSigSamples([]);
    setResult(null);
    setSaveState("idle");
    setSaveError(null);
    setSavedId(null);
    startTimeRef.current = Date.now();
    durationSecRef.current = durationSec;

    // 少し遅延してrunning状態に遷移（UIの状態遷移を明確化）
    setTimeout(() => {
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
    }, 100);
  }, [state]);

  // remainingTimeが0になったら終了
  useEffect(() => {
    if (state === "running" && remainingTime === 0) {
      finishInspection();
    }
  }, [state, remainingTime, finishInspection]);

  // 検査停止（手動）
  const stop = useCallback(() => {
    // running状態でなければ停止しない
    if (state !== "running") return;
    finishInspection();
  }, [state, finishInspection]);

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
    setSaveState("idle");
    setSaveError(null);
    setSavedId(null);
    startTimeRef.current = 0;
    durationSecRef.current = 0;
  }, []);

  // 結果保存
  const saveResult = useCallback(
    async (serialNumber?: string, lotId?: number) => {
      if (state !== "completed" || !result) {
        setSaveError("検査が完了していません");
        return;
      }

      setSaveState("saving");
      setSaveError(null);

      try {
        const response = await fetch(
          "http://localhost:8080/api/outdoor-inspection-results",
          {
            method: "POST",
            headers: {
              "Content-Type": "application/json",
            },
            body: JSON.stringify({
              serial_number: serialNumber ?? null,
              lot_id: lotId ?? null,
              inspected_at: new Date(startTimeRef.current).toISOString(),
              duration_sec: durationSecRef.current,
              sample_count: result.sample_count,
              rtk_fix_rate: result.rtk_fix_rate,
              rtk_fix_time_ms: result.rtk_fix_time_ms,
              l2_reception_rate: result.l2_reception_rate,
              l1_min_cno: result.l1_min_cno,
              is_pass: result.judgment.is_pass,
              l1_cno_pass: result.judgment.l1_cno_pass,
              l2_rate_pass: result.judgment.l2_rate_pass,
              rtk_fix_time_pass: result.judgment.rtk_fix_time_pass,
              rtk_fix_rate_pass: result.judgment.rtk_fix_rate_pass,
              failure_reasons: result.judgment.failure_reasons,
            }),
          }
        );

        if (!response.ok) {
          const errorData = await response.json();
          throw new Error(errorData.error || "保存に失敗しました");
        }

        const savedData = await response.json();
        setSavedId(savedData.id);
        setSaveState("saved");
      } catch (error) {
        setSaveError(
          error instanceof Error ? error.message : "保存に失敗しました"
        );
        setSaveState("error");
      }
    },
    [state, result]
  );

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
    saveState,
    saveError,
    savedId,
    start,
    stop,
    reset,
    saveResult,
    addNavStatusSample,
    addNavSigSample,
  };
}
