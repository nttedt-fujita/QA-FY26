"use client";

import { useState, useEffect, useRef } from "react";
import { NavSigResponse, NavSignal, getNavSig } from "@/lib/api";

// ADR-008: L2受信率50%以上で合格
const L2_RECEPTION_THRESHOLD = 0.5;

/**
 * GNSS IDを名称に変換
 */
function gnssIdToName(gnssId: number): string {
  const mapping: Record<number, string> = {
    0: "GPS",
    1: "SBAS",
    2: "Galileo",
    3: "BeiDou",
    5: "QZSS",
    6: "GLONASS",
  };
  return mapping[gnssId] ?? `Unknown(${gnssId})`;
}

/**
 * 信号帯域を表示用文字列に変換
 */
function getBandLabel(signal: NavSignal): string {
  if (signal.is_l1) return "L1";
  if (signal.is_l2) return "L2";
  return "-";
}

/**
 * C/N0値に応じた色クラスを返す
 * 参考: 30dBHz以上で良好、20dBHz未満は弱い
 */
function getCnoColorClass(cno: number): string {
  if (cno >= 40) return "text-green-600";
  if (cno >= 30) return "text-green-500";
  if (cno >= 20) return "text-yellow-600";
  return "text-red-600";
}

/**
 * C/N0バーの幅を計算（0-50dBHzの範囲）
 */
function getCnoBarWidth(cno: number): number {
  return Math.min(100, Math.max(0, (cno / 50) * 100));
}

/**
 * C/N0バーの色クラスを返す
 */
function getCnoBarColorClass(cno: number): string {
  if (cno >= 40) return "bg-green-500";
  if (cno >= 30) return "bg-green-400";
  if (cno >= 20) return "bg-yellow-400";
  return "bg-red-400";
}

interface NavSigPanelProps {
  /** ポーリング有効フラグ（装置接続時のみtrue） */
  enabled: boolean;
  /** ポーリング間隔（ミリ秒） */
  pollIntervalMs?: number;
}

/**
 * NAV-SIG表示パネル
 *
 * - L1/L2別C/N0一覧テーブル
 * - L2受信率ゲージ（50%基準で色分け）
 * - 合格/不合格表示
 */
export function NavSigPanel({
  enabled,
  pollIntervalMs = 1000,
}: NavSigPanelProps) {
  const [data, setData] = useState<NavSigResponse | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [isLoading, setIsLoading] = useState(false);

  // AbortController参照
  const abortControllerRef = useRef<AbortController | null>(null);

  // 初回取得 + ポーリング
  useEffect(() => {
    if (!enabled) {
      setData(null);
      setError(null);
      return;
    }

    const controller = new AbortController();
    abortControllerRef.current = controller;

    const fetchData = async () => {
      setIsLoading(true);
      try {
        const res = await getNavSig(controller.signal);
        setData(res);
        setError(null);
      } catch (e) {
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

  // L1とL2で信号を分離（GPSのみ）
  const gpsL1Signals =
    data?.signals.filter((s) => s.gnss_id === 0 && s.is_l1) ?? [];
  const gpsL2Signals =
    data?.signals.filter((s) => s.gnss_id === 0 && s.is_l2) ?? [];

  // L2受信率の判定
  const l2Rate = data?.stats.gps_l2_reception_rate ?? 0;
  const isL2Pass = l2Rate >= L2_RECEPTION_THRESHOLD;

  if (!enabled) {
    return (
      <div className="rounded border border-gray-200 bg-white p-4">
        <h3 className="mb-2 font-medium text-gray-700">衛星信号 (NAV-SIG)</h3>
        <div className="text-gray-500">装置が接続されていません</div>
      </div>
    );
  }

  if (error) {
    return (
      <div className="rounded border border-red-200 bg-red-50 p-4">
        <h3 className="mb-2 font-medium text-red-700">衛星信号 (NAV-SIG)</h3>
        <div className="text-red-600">{error}</div>
      </div>
    );
  }

  return (
    <div className="rounded border border-gray-200 bg-white p-4">
      <div className="mb-4 flex items-center justify-between">
        <h3 className="font-medium text-gray-700">衛星信号 (NAV-SIG)</h3>
        {isLoading && (
          <span className="text-xs text-gray-400">更新中...</span>
        )}
      </div>

      {/* L2受信率ゲージ */}
      <div className="mb-4">
        <div className="mb-1 flex items-center justify-between">
          <span className="text-sm text-gray-600">L2受信率</span>
          <span
            className={`text-sm font-semibold ${
              isL2Pass ? "text-green-600" : "text-red-600"
            }`}
          >
            {(l2Rate * 100).toFixed(0)}% ({data?.stats.gps_l2_reception_count ?? 0}/
            {data?.stats.gps_visible_count ?? 0})
            {isL2Pass ? " ✓ 合格" : " × 不合格"}
          </span>
        </div>
        <div className="h-4 w-full overflow-hidden rounded bg-gray-200">
          <div
            className={`h-full transition-all duration-300 ${
              isL2Pass ? "bg-green-500" : "bg-red-500"
            }`}
            style={{ width: `${l2Rate * 100}%` }}
          />
          {/* 50%基準線 */}
          <div
            className="absolute h-4 w-0.5 bg-gray-400"
            style={{ left: "50%", marginTop: "-16px" }}
          />
        </div>
        <div className="mt-1 flex justify-between text-xs text-gray-400">
          <span>0%</span>
          <span>50% (基準)</span>
          <span>100%</span>
        </div>
      </div>

      {/* 信号テーブル */}
      <div className="grid gap-4 md:grid-cols-2">
        {/* L1信号 */}
        <div>
          <h4 className="mb-2 text-sm font-medium text-gray-600">
            GPS L1 ({gpsL1Signals.length}衛星)
          </h4>
          <SignalTable signals={gpsL1Signals} />
        </div>

        {/* L2信号 */}
        <div>
          <h4 className="mb-2 text-sm font-medium text-gray-600">
            GPS L2 ({gpsL2Signals.length}衛星)
          </h4>
          <SignalTable signals={gpsL2Signals} />
        </div>
      </div>
    </div>
  );
}

/**
 * 信号一覧テーブル
 */
function SignalTable({ signals }: { signals: NavSignal[] }) {
  if (signals.length === 0) {
    return <div className="text-sm text-gray-400">信号なし</div>;
  }

  // C/N0降順でソート
  const sorted = [...signals].sort((a, b) => b.cno - a.cno);

  return (
    <div className="overflow-hidden rounded border border-gray-200">
      <table className="w-full text-sm">
        <thead>
          <tr className="bg-gray-50">
            <th className="px-2 py-1 text-left">SV</th>
            <th className="px-2 py-1 text-right">C/N0</th>
            <th className="px-2 py-1 text-left">強度</th>
          </tr>
        </thead>
        <tbody>
          {sorted.map((signal, idx) => (
            <tr key={idx} className="border-t border-gray-100">
              <td className="px-2 py-1 font-mono">{signal.sv_id}</td>
              <td
                className={`px-2 py-1 text-right font-mono ${getCnoColorClass(
                  signal.cno
                )}`}
              >
                {signal.cno}
              </td>
              <td className="px-2 py-1">
                <div className="h-2 w-full overflow-hidden rounded bg-gray-200">
                  <div
                    className={`h-full ${getCnoBarColorClass(signal.cno)}`}
                    style={{ width: `${getCnoBarWidth(signal.cno)}%` }}
                  />
                </div>
              </td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}
