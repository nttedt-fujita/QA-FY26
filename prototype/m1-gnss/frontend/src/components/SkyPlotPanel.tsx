"use client";

import { useState, useEffect, useCallback, useRef } from "react";
import { NavSatResponse, SatelliteInfo, getNavSat } from "@/lib/api";

/**
 * GNSS別の色を返す
 */
function getGnssColor(gnssId: number): string {
  const colors: Record<number, string> = {
    0: "#3b82f6", // GPS: blue
    1: "#22c55e", // SBAS: green
    2: "#f59e0b", // Galileo: amber
    3: "#ef4444", // BeiDou: red
    5: "#8b5cf6", // QZSS: violet
    6: "#06b6d4", // GLONASS: cyan
  };
  return colors[gnssId] ?? "#6b7280"; // gray for unknown
}

/**
 * C/N0に基づいて衛星の大きさを返す
 */
function getSatelliteRadius(cno: number): number {
  // 20〜50 dBHz → 3〜8px
  if (cno < 20) return 3;
  if (cno > 50) return 8;
  return 3 + ((cno - 20) / 30) * 5;
}

/**
 * 仰角・方位角を極座標のx, yに変換
 * 中心が天頂（elev=90°）、外周が水平線（elev=0°）
 */
function polarToCartesian(
  elev: number,
  azim: number,
  centerX: number,
  centerY: number,
  radius: number
): { x: number; y: number } {
  // 仰角が90°なら中心、0°なら外周
  const r = ((90 - elev) / 90) * radius;
  // 方位角: 北=上(0°)、東=右(90°)
  const angle = ((azim - 90) * Math.PI) / 180;
  return {
    x: centerX + r * Math.cos(angle),
    y: centerY + r * Math.sin(angle),
  };
}

interface SkyPlotSVGProps {
  satellites: SatelliteInfo[];
  size?: number;
}

/**
 * スカイプロットSVG
 */
function SkyPlotSVG({ satellites, size = 300 }: SkyPlotSVGProps) {
  const center = size / 2;
  const radius = size / 2 - 30; // マージン

  // 同心円（仰角）
  const elevationCircles = [0, 30, 60, 90];

  // 方位線（北/東/南/西）
  const azimuthLines = [0, 90, 180, 270];
  const azimuthLabels = ["N", "E", "S", "W"];

  return (
    <svg width={size} height={size} className="mx-auto">
      {/* 背景 */}
      <circle cx={center} cy={center} r={radius} fill="#f8fafc" stroke="#e2e8f0" strokeWidth={1} />

      {/* 同心円（仰角30°/60°） */}
      {elevationCircles.slice(1, 3).map((elev) => (
        <circle
          key={`elev-${elev}`}
          cx={center}
          cy={center}
          r={((90 - elev) / 90) * radius}
          fill="none"
          stroke="#e2e8f0"
          strokeWidth={1}
          strokeDasharray="4 4"
        />
      ))}

      {/* 方位線 */}
      {azimuthLines.map((azim, idx) => {
        const end = polarToCartesian(0, azim, center, center, radius);
        return (
          <g key={`azim-${azim}`}>
            <line
              x1={center}
              y1={center}
              x2={end.x}
              y2={end.y}
              stroke="#e2e8f0"
              strokeWidth={1}
            />
            {/* ラベル */}
            <text
              x={polarToCartesian(-5, azim, center, center, radius).x}
              y={polarToCartesian(-5, azim, center, center, radius).y}
              textAnchor="middle"
              dominantBaseline="middle"
              className="fill-gray-500 text-xs"
            >
              {azimuthLabels[idx]}
            </text>
          </g>
        );
      })}

      {/* 仰角ラベル */}
      <text
        x={center + 5}
        y={center - ((90 - 60) / 90) * radius - 5}
        className="fill-gray-400 text-[10px]"
      >
        60°
      </text>
      <text
        x={center + 5}
        y={center - ((90 - 30) / 90) * radius - 5}
        className="fill-gray-400 text-[10px]"
      >
        30°
      </text>

      {/* 衛星 */}
      {satellites
        .filter((sat) => sat.elev >= 0) // 地平線下は非表示
        .map((sat, idx) => {
          const pos = polarToCartesian(sat.elev, sat.azim, center, center, radius);
          const r = getSatelliteRadius(sat.cno);
          const color = getGnssColor(sat.gnss_id);
          return (
            <g key={`sat-${sat.gnss_id}-${sat.sv_id}-${idx}`}>
              <circle
                cx={pos.x}
                cy={pos.y}
                r={r}
                fill={sat.is_used ? color : "none"}
                stroke={color}
                strokeWidth={sat.is_used ? 0 : 1.5}
              />
              {/* 衛星番号ラベル（大きい衛星のみ） */}
              {r >= 5 && (
                <text
                  x={pos.x}
                  y={pos.y + r + 10}
                  textAnchor="middle"
                  className="fill-gray-600 text-[9px]"
                >
                  {sat.gnss_name[0]}{sat.sv_id}
                </text>
              )}
            </g>
          );
        })}

      {/* 中心点（天頂） */}
      <circle cx={center} cy={center} r={2} fill="#94a3b8" />
    </svg>
  );
}

interface SkyPlotPanelProps {
  /** ポーリング有効フラグ */
  enabled: boolean;
  /** ポーリング間隔（ミリ秒） */
  pollIntervalMs?: number;
}

/**
 * スカイプロット表示パネル
 *
 * - 極座標で衛星位置を表示
 * - GNSS種別で色分け
 * - C/N0で衛星の大きさを変化
 */
export function SkyPlotPanel({
  enabled,
  pollIntervalMs = 2000,
}: SkyPlotPanelProps) {
  const [data, setData] = useState<NavSatResponse | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [isLoading, setIsLoading] = useState(false);

  // データ取得
  const fetchData = useCallback(async () => {
    if (!enabled) return;

    setIsLoading(true);
    try {
      const res = await getNavSat();
      setData(res);
      setError(null);
    } catch (e) {
      setError(e instanceof Error ? e.message : "取得失敗");
    } finally {
      setIsLoading(false);
    }
  }, [enabled]);

  // AbortController参照
  const abortControllerRef = useRef<AbortController | null>(null);

  // 初回取得 + ポーリング
  useEffect(() => {
    if (!enabled) {
      setData(null);
      setError(null);
      return;
    }

    abortControllerRef.current = new AbortController();

    fetchData();
    const interval = setInterval(fetchData, pollIntervalMs);

    return () => {
      clearInterval(interval);
      abortControllerRef.current?.abort();
    };
  }, [enabled, pollIntervalMs, fetchData]);

  if (!enabled) {
    return (
      <div className="rounded border border-gray-200 bg-white p-4">
        <h3 className="mb-2 font-medium text-gray-700">スカイプロット (NAV-SAT)</h3>
        <div className="text-gray-500">装置が接続されていません</div>
      </div>
    );
  }

  if (error) {
    return (
      <div className="rounded border border-red-200 bg-red-50 p-4">
        <h3 className="mb-2 font-medium text-red-700">スカイプロット (NAV-SAT)</h3>
        <div className="text-red-600">{error}</div>
      </div>
    );
  }

  return (
    <div className="rounded border border-gray-200 bg-white p-4">
      <div className="mb-4 flex items-center justify-between">
        <h3 className="font-medium text-gray-700">スカイプロット (NAV-SAT)</h3>
        {isLoading && (
          <span className="text-xs text-gray-400">更新中...</span>
        )}
      </div>

      {data && (
        <div className="space-y-4">
          {/* スカイプロットSVG */}
          <SkyPlotSVG satellites={data.satellites} size={280} />

          {/* 凡例 */}
          <div className="flex flex-wrap justify-center gap-3 text-xs">
            {[
              { id: 0, name: "GPS" },
              { id: 2, name: "Galileo" },
              { id: 3, name: "BeiDou" },
              { id: 6, name: "GLONASS" },
              { id: 5, name: "QZSS" },
              { id: 1, name: "SBAS" },
            ].map((gnss) => (
              <div key={gnss.id} className="flex items-center gap-1">
                <div
                  className="h-3 w-3 rounded-full"
                  style={{ backgroundColor: getGnssColor(gnss.id) }}
                />
                <span className="text-gray-600">{gnss.name}</span>
              </div>
            ))}
          </div>

          {/* 統計 */}
          <div className="mt-2 grid grid-cols-2 gap-2 rounded bg-gray-50 p-3 text-sm">
            <div>
              <span className="text-gray-500">全衛星数: </span>
              <span className="font-mono">{data.stats.total_count}</span>
            </div>
            <div>
              <span className="text-gray-500">使用中: </span>
              <span className="font-mono">{data.stats.used_count}</span>
            </div>
            <div>
              <span className="text-gray-500">GPS: </span>
              <span className="font-mono">{data.stats.gnss_counts.gps}</span>
            </div>
            <div>
              <span className="text-gray-500">GLONASS: </span>
              <span className="font-mono">{data.stats.gnss_counts.glonass}</span>
            </div>
            <div>
              <span className="text-gray-500">Galileo: </span>
              <span className="font-mono">{data.stats.gnss_counts.galileo}</span>
            </div>
            <div>
              <span className="text-gray-500">BeiDou: </span>
              <span className="font-mono">{data.stats.gnss_counts.beidou}</span>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}
