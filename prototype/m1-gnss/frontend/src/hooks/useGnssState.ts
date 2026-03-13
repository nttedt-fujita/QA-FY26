"use client";

import { useState, useEffect, useRef } from "react";
import { GnssStateResponse, getGnssState } from "@/lib/api";

export interface UseGnssStateOptions {
  /** ポーリング有効フラグ（装置接続時のみtrue） */
  enabled: boolean;
  /** レスポンス後の遅延（ミリ秒）、デフォルト0 */
  delayAfterResponseMs?: number;
}

export interface UseGnssStateResult {
  /** 統合APIレスポンス */
  data: GnssStateResponse | null;
  /** エラーメッセージ */
  error: string | null;
  /** 読み込み中フラグ（初回ロード用） */
  isLoading: boolean;
  /** リクエスト中フラグ（レスポンス駆動の状態管理用） */
  isFetching: boolean;
  /** 最終更新時刻 */
  lastUpdated: Date | null;
  /** リクエスト回数（デバッグ用） */
  requestCount: number;
}

/**
 * GNSS状態（統合API）をレスポンス駆動でポーリングするhook
 *
 * レスポンス駆動方式:
 * - リクエスト → レスポンス受信 → 次のリクエスト → ...
 * - BEの処理速度に自動追従し、キュー滞留を防止
 * - 固定間隔ポーリングより効率的
 */
export function useGnssState({
  enabled,
  delayAfterResponseMs = 0,
}: UseGnssStateOptions): UseGnssStateResult {
  const [data, setData] = useState<GnssStateResponse | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [isLoading, setIsLoading] = useState(false);
  const [isFetching, setIsFetching] = useState(false);
  const [lastUpdated, setLastUpdated] = useState<Date | null>(null);
  const [requestCount, setRequestCount] = useState(0);

  // enabledをrefで保持（ループ内で最新値を参照するため）
  const enabledRef = useRef(enabled);
  enabledRef.current = enabled;

  // ループ制御用
  const mountedRef = useRef(true);

  // レスポンス駆動ポーリング
  useEffect(() => {
    mountedRef.current = true;

    // 非活性時はデータクリア
    if (!enabled) {
      setData(null);
      setError(null);
      setLastUpdated(null);
      setIsFetching(false);
      return;
    }

    const controller = new AbortController();

    const fetchLoop = async () => {
      while (mountedRef.current && enabledRef.current) {
        setIsFetching(true);
        setIsLoading(requestCount === 0);

        try {
          const res = await getGnssState(controller.signal);

          // abort済みなら無視
          if (!mountedRef.current) return;

          setData(res);
          setError(null);
          setLastUpdated(new Date());
          setRequestCount((prev) => prev + 1);

          // 部分エラーがあればログ出力
          if (res.errors.length > 0) {
            console.warn("[useGnssState] partial errors:", res.errors);
          }
        } catch (e) {
          // AbortErrorは無視
          if (e instanceof Error && e.name === "AbortError") return;
          if (!mountedRef.current) return;
          setError(e instanceof Error ? e.message : "取得失敗");
        } finally {
          if (mountedRef.current) {
            setIsFetching(false);
            setIsLoading(false);
          }
        }

        // enabledが変わっていたらループ終了
        if (!enabledRef.current) break;

        // 次のリクエストまでの遅延（オプション）
        if (delayAfterResponseMs > 0) {
          await new Promise((resolve) => setTimeout(resolve, delayAfterResponseMs));
        }
      }
    };

    fetchLoop();

    return () => {
      mountedRef.current = false;
      controller.abort();
    };
  }, [enabled, delayAfterResponseMs]);

  // リクエストカウントのリセット（enabled変更時）
  useEffect(() => {
    if (!enabled) {
      setRequestCount(0);
    }
  }, [enabled]);

  return { data, error, isLoading, isFetching, lastUpdated, requestCount };
}
