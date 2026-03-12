"use client";

import { useState, useEffect, useCallback } from "react";
import { NavStatusResponse, getNavStatus } from "@/lib/api";

/**
 * Fix種別を表示用文字列に変換
 */
function fixTypeToString(gpsFix: number): string {
  const mapping: Record<number, string> = {
    0: "No Fix",
    1: "Dead Reckoning",
    2: "2D Fix",
    3: "3D Fix",
    4: "GNSS + DR",
    5: "Time Only",
  };
  return mapping[gpsFix] ?? `Unknown(${gpsFix})`;
}

/**
 * RTK状態を表示用文字列に変換
 */
function rtkStatusToString(carrSoln: number): string {
  const mapping: Record<number, string> = {
    0: "なし",
    1: "Float",
    2: "Fixed",
  };
  return mapping[carrSoln] ?? `Unknown(${carrSoln})`;
}

/**
 * ミリ秒を「分:秒.ミリ秒」形式に変換
 */
function msToTimeString(ms: number): string {
  if (ms === 0) return "-";
  const totalSec = Math.floor(ms / 1000);
  const minutes = Math.floor(totalSec / 60);
  const seconds = totalSec % 60;
  const millis = ms % 1000;
  if (minutes > 0) {
    return `${minutes}分${seconds}.${millis.toString().padStart(3, "0")}秒`;
  }
  return `${seconds}.${millis.toString().padStart(3, "0")}秒`;
}

/**
 * Fix状態に応じた色クラスを返す
 */
function getFixColorClass(isFixValid: boolean, isRtkFixed: boolean): string {
  if (isRtkFixed) return "text-green-600";
  if (isFixValid) return "text-blue-600";
  return "text-red-600";
}

/**
 * Fix状態に応じたバッジクラスを返す
 */
function getFixBadgeClass(isFixValid: boolean, isRtkFixed: boolean, isRtkFloat: boolean): string {
  if (isRtkFixed) return "bg-green-100 text-green-700";
  if (isRtkFloat) return "bg-yellow-100 text-yellow-700";
  if (isFixValid) return "bg-blue-100 text-blue-700";
  return "bg-red-100 text-red-700";
}

interface NavStatusPanelProps {
  /** ポーリング有効フラグ（装置接続時のみtrue） */
  enabled: boolean;
  /** ポーリング間隔（ミリ秒） */
  pollIntervalMs?: number;
}

/**
 * NAV-STATUS表示パネル
 *
 * - TTFF（Time to First Fix）表示
 * - 起動からの経過時間
 * - Fix状態・RTK状態
 */
export function NavStatusPanel({
  enabled,
  pollIntervalMs = 1000,
}: NavStatusPanelProps) {
  const [data, setData] = useState<NavStatusResponse | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [isLoading, setIsLoading] = useState(false);

  // データ取得
  const fetchData = useCallback(async () => {
    if (!enabled) return;

    setIsLoading(true);
    try {
      const res = await getNavStatus();
      setData(res);
      setError(null);
    } catch (e) {
      setError(e instanceof Error ? e.message : "取得失敗");
    } finally {
      setIsLoading(false);
    }
  }, [enabled]);

  // 初回取得 + ポーリング
  useEffect(() => {
    if (!enabled) {
      setData(null);
      setError(null);
      return;
    }

    fetchData();
    const interval = setInterval(fetchData, pollIntervalMs);
    return () => clearInterval(interval);
  }, [enabled, pollIntervalMs, fetchData]);

  if (!enabled) {
    return (
      <div className="rounded border border-gray-200 bg-white p-4">
        <h3 className="mb-2 font-medium text-gray-700">Fix状態・TTFF (NAV-STATUS)</h3>
        <div className="text-gray-500">装置が接続されていません</div>
      </div>
    );
  }

  if (error) {
    return (
      <div className="rounded border border-red-200 bg-red-50 p-4">
        <h3 className="mb-2 font-medium text-red-700">Fix状態・TTFF (NAV-STATUS)</h3>
        <div className="text-red-600">{error}</div>
      </div>
    );
  }

  return (
    <div className="rounded border border-gray-200 bg-white p-4">
      <div className="mb-4 flex items-center justify-between">
        <h3 className="font-medium text-gray-700">Fix状態・TTFF (NAV-STATUS)</h3>
        {isLoading && (
          <span className="text-xs text-gray-400">更新中...</span>
        )}
      </div>

      {data && (
        <div className="space-y-4">
          {/* Fix状態 */}
          <div className="flex items-center justify-between">
            <span className="text-sm text-gray-600">Fix状態</span>
            <div className="flex items-center gap-2">
              <span
                className={`rounded px-2 py-0.5 text-sm font-medium ${getFixBadgeClass(
                  data.is_fix_valid,
                  data.is_rtk_fixed,
                  data.is_rtk_float
                )}`}
              >
                {fixTypeToString(data.gps_fix)}
                {data.is_rtk_fixed && " (RTK Fixed)"}
                {data.is_rtk_float && " (RTK Float)"}
              </span>
            </div>
          </div>

          {/* TTFF */}
          <div className="flex items-center justify-between">
            <span className="text-sm text-gray-600">TTFF (Time to First Fix)</span>
            <span
              className={`font-mono text-lg ${getFixColorClass(
                data.is_fix_valid,
                data.is_rtk_fixed
              )}`}
            >
              {msToTimeString(data.ttff)}
            </span>
          </div>

          {/* 起動からの経過時間 */}
          <div className="flex items-center justify-between">
            <span className="text-sm text-gray-600">起動からの経過時間</span>
            <span className="font-mono text-gray-700">
              {msToTimeString(data.msss)}
            </span>
          </div>

          {/* 詳細情報（折りたたみ可能にしても良い） */}
          <div className="mt-4 rounded bg-gray-50 p-3">
            <div className="grid grid-cols-2 gap-2 text-sm">
              <div>
                <span className="text-gray-500">gps_fix: </span>
                <span className="font-mono">{data.gps_fix}</span>
              </div>
              <div>
                <span className="text-gray-500">gps_fix_ok: </span>
                <span className="font-mono">{data.gps_fix_ok ? "true" : "false"}</span>
              </div>
              <div>
                <span className="text-gray-500">carr_soln: </span>
                <span className="font-mono">{rtkStatusToString(data.carr_soln)}</span>
              </div>
              <div>
                <span className="text-gray-500">is_fix_valid: </span>
                <span className="font-mono">{data.is_fix_valid ? "true" : "false"}</span>
              </div>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}
