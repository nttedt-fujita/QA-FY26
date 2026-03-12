"use client";

import { useState, useEffect, useRef, useCallback } from "react";
import { GnssStateResponse, getGnssState } from "@/lib/api";

export interface UseGnssStateOptions {
  /** ポーリング有効フラグ（装置接続時のみtrue） */
  enabled: boolean;
  /** ポーリング間隔（ミリ秒）、デフォルト1000ms */
  pollIntervalMs?: number;
}

export interface UseGnssStateResult {
  /** 統合APIレスポンス */
  data: GnssStateResponse | null;
  /** エラーメッセージ */
  error: string | null;
  /** 読み込み中フラグ */
  isLoading: boolean;
  /** 最終更新時刻 */
  lastUpdated: Date | null;
}

/**
 * GNSS状態（統合API）をポーリングするhook
 *
 * 個別API（nav-sat, nav-sig, nav-status, mon-span）の代わりに
 * 統合API（/api/gnss-state）を1つだけポーリングし、
 * ポーリング競合を防止する。
 */
export function useGnssState({
  enabled,
  pollIntervalMs = 1000,
}: UseGnssStateOptions): UseGnssStateResult {
  const [data, setData] = useState<GnssStateResponse | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [isLoading, setIsLoading] = useState(false);
  const [lastUpdated, setLastUpdated] = useState<Date | null>(null);

  // AbortController参照
  const abortControllerRef = useRef<AbortController | null>(null);

  // 初回取得 + ポーリング
  useEffect(() => {
    if (!enabled) {
      setData(null);
      setError(null);
      setLastUpdated(null);
      return;
    }

    // AbortController作成
    const controller = new AbortController();
    abortControllerRef.current = controller;

    const fetchData = async () => {
      setIsLoading(true);
      try {
        const res = await getGnssState(controller.signal);
        setData(res);
        setError(null);
        setLastUpdated(new Date());
        // 部分エラーがあればログ出力
        if (res.errors.length > 0) {
          console.warn("[useGnssState] partial errors:", res.errors);
        }
      } catch (e) {
        // AbortErrorは無視
        if (e instanceof Error && e.name === "AbortError") return;
        setError(e instanceof Error ? e.message : "取得失敗");
      } finally {
        setIsLoading(false);
      }
    };

    fetchData();
    const interval = setInterval(fetchData, pollIntervalMs);

    return () => {
      clearInterval(interval);
      controller.abort();
    };
  }, [enabled, pollIntervalMs]);

  return { data, error, isLoading, lastUpdated };
}
