"use client";

import { useState, useMemo } from "react";
import { NavSatResponse, SatelliteInfo } from "@/lib/api";

// GNSS定義（ID、名前、色）
const GNSS_LIST = [
  { id: 0, name: "GPS", color: "#3b82f6" },      // blue
  { id: 2, name: "Galileo", color: "#f59e0b" },  // amber
  { id: 3, name: "BeiDou", color: "#ef4444" },   // red
  { id: 6, name: "GLONASS", color: "#06b6d4" },  // cyan
  { id: 5, name: "QZSS", color: "#8b5cf6" },     // violet
  { id: 1, name: "SBAS", color: "#22c55e" },     // green
] as const;

/**
 * GNSS別の色を返す
 */
function getGnssColor(gnssId: number): string {
  const gnss = GNSS_LIST.find((g) => g.id === gnssId);
  return gnss?.color ?? "#6b7280"; // gray for unknown
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
  visibleGnss: Set<number>;
  size?: number;
}

/**
 * スカイプロットSVG
 */
function SkyPlotSVG({ satellites, visibleGnss, size = 300 }: SkyPlotSVGProps) {
  const center = size / 2;
  const radius = size / 2 - 30; // マージン

  // 同心円（仰角）
  const elevationCircles = [0, 30, 60, 90];

  // 方位線（北/東/南/西）
  const azimuthLines = [0, 90, 180, 270];
  const azimuthLabels = ["N", "E", "S", "W"];

  // フィルタされた衛星
  const filteredSatellites = satellites.filter(
    (sat) => sat.elev >= 0 && visibleGnss.has(sat.gnss_id)
  );

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
      {filteredSatellites.map((sat, idx) => {
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

/**
 * GNSS別の統計を計算
 */
interface GnssStats {
  gnssId: number;
  name: string;
  color: string;
  count: number;
  usedCount: number;
  avgCno: number | null;
  minCno: number | null;
  maxCno: number | null;
}

function calculateGnssStats(satellites: SatelliteInfo[]): GnssStats[] {
  return GNSS_LIST.map((gnss) => {
    const sats = satellites.filter((s) => s.gnss_id === gnss.id && s.elev >= 0);
    const usedSats = sats.filter((s) => s.is_used);
    const cnoValues = sats.filter((s) => s.cno > 0).map((s) => s.cno);

    return {
      gnssId: gnss.id,
      name: gnss.name,
      color: gnss.color,
      count: sats.length,
      usedCount: usedSats.length,
      avgCno: cnoValues.length > 0
        ? cnoValues.reduce((a, b) => a + b, 0) / cnoValues.length
        : null,
      minCno: cnoValues.length > 0 ? Math.min(...cnoValues) : null,
      maxCno: cnoValues.length > 0 ? Math.max(...cnoValues) : null,
    };
  }).filter((s) => s.count > 0); // 衛星がある GNSSのみ
}

interface SkyPlotPanelProps {
  /** NAV-SATデータ（統合APIから渡される） */
  data: NavSatResponse | null;
  /** エラーメッセージ */
  error?: string | null;
  /** 読み込み中フラグ */
  isLoading?: boolean;
  /** 装置接続フラグ */
  isConnected?: boolean;
}

/**
 * スカイプロット表示パネル
 *
 * - 極座標で衛星位置を表示
 * - GNSS種別で色分け
 * - C/N0で衛星の大きさを変化
 * - 凡例クリックでGNSSフィルタリング
 */
export function SkyPlotPanel({
  data,
  error,
  isLoading = false,
  isConnected = true,
}: SkyPlotPanelProps) {
  // フィルタ状態: 表示するGNSS IDのセット（初期値: 全表示）
  const [visibleGnss, setVisibleGnss] = useState<Set<number>>(
    () => new Set(GNSS_LIST.map((g) => g.id))
  );

  // GNSS別統計
  const gnssStats = useMemo(() => {
    if (!data) return [];
    return calculateGnssStats(data.satellites);
  }, [data]);

  // フィルタ切り替え
  const toggleGnss = (gnssId: number) => {
    setVisibleGnss((prev) => {
      const next = new Set(prev);
      if (next.has(gnssId)) {
        next.delete(gnssId);
      } else {
        next.add(gnssId);
      }
      return next;
    });
  };

  // 全選択/全解除
  const selectAll = () => setVisibleGnss(new Set(GNSS_LIST.map((g) => g.id)));
  const selectNone = () => setVisibleGnss(new Set());

  if (!isConnected) {
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
          <SkyPlotSVG
            satellites={data.satellites}
            visibleGnss={visibleGnss}
            size={280}
          />

          {/* フィルタ凡例（クリック可能） */}
          <div className="space-y-2">
            <div className="flex items-center justify-between">
              <span className="text-xs text-gray-500">クリックでフィルタ:</span>
              <div className="flex gap-2">
                <button
                  onClick={selectAll}
                  className="rounded bg-gray-100 px-2 py-0.5 text-xs text-gray-600 hover:bg-gray-200"
                >
                  全表示
                </button>
                <button
                  onClick={selectNone}
                  className="rounded bg-gray-100 px-2 py-0.5 text-xs text-gray-600 hover:bg-gray-200"
                >
                  全非表示
                </button>
              </div>
            </div>
            <div className="flex flex-wrap justify-center gap-2">
              {GNSS_LIST.map((gnss) => {
                const isVisible = visibleGnss.has(gnss.id);
                const stats = gnssStats.find((s) => s.gnssId === gnss.id);
                const hasData = stats && stats.count > 0;

                return (
                  <button
                    key={gnss.id}
                    onClick={() => toggleGnss(gnss.id)}
                    disabled={!hasData}
                    className={`flex items-center gap-1 rounded px-2 py-1 text-xs transition-all ${
                      !hasData
                        ? "cursor-not-allowed opacity-30"
                        : isVisible
                          ? "bg-gray-100 hover:bg-gray-200"
                          : "bg-gray-50 opacity-50 hover:opacity-75"
                    }`}
                  >
                    <div
                      className={`h-3 w-3 rounded-full ${!isVisible && hasData ? "opacity-30" : ""}`}
                      style={{ backgroundColor: gnss.color }}
                    />
                    <span className="text-gray-600">{gnss.name}</span>
                    {hasData && (
                      <span className="font-mono text-gray-400">({stats.count})</span>
                    )}
                  </button>
                );
              })}
            </div>
          </div>

          {/* GNSS別詳細統計 */}
          {gnssStats.length > 0 && (
            <div className="rounded bg-gray-50 p-3">
              <div className="mb-2 text-xs font-medium text-gray-600">GNSS別統計</div>
              <div className="overflow-x-auto">
                <table className="w-full text-xs">
                  <thead>
                    <tr className="text-left text-gray-500">
                      <th className="pb-1">GNSS</th>
                      <th className="pb-1 text-right">衛星数</th>
                      <th className="pb-1 text-right">使用中</th>
                      <th className="pb-1 text-right">CNO平均</th>
                      <th className="pb-1 text-right">CNO最小</th>
                      <th className="pb-1 text-right">CNO最大</th>
                    </tr>
                  </thead>
                  <tbody>
                    {gnssStats.map((stats) => (
                      <tr
                        key={stats.gnssId}
                        className={`border-t border-gray-200 ${
                          !visibleGnss.has(stats.gnssId) ? "opacity-40" : ""
                        }`}
                      >
                        <td className="py-1">
                          <div className="flex items-center gap-1">
                            <div
                              className="h-2 w-2 rounded-full"
                              style={{ backgroundColor: stats.color }}
                            />
                            <span className="font-medium">{stats.name}</span>
                          </div>
                        </td>
                        <td className="py-1 text-right font-mono">{stats.count}</td>
                        <td className="py-1 text-right font-mono">{stats.usedCount}</td>
                        <td className="py-1 text-right font-mono">
                          {stats.avgCno !== null ? stats.avgCno.toFixed(1) : "-"}
                        </td>
                        <td className="py-1 text-right font-mono">
                          {stats.minCno !== null ? stats.minCno : "-"}
                        </td>
                        <td className="py-1 text-right font-mono">
                          {stats.maxCno !== null ? stats.maxCno : "-"}
                        </td>
                      </tr>
                    ))}
                  </tbody>
                </table>
              </div>
            </div>
          )}

          {/* 全体統計（サマリー） */}
          <div className="grid grid-cols-2 gap-2 rounded bg-gray-50 p-3 text-sm">
            <div>
              <span className="text-gray-500">全衛星数: </span>
              <span className="font-mono">{data.stats.total_count}</span>
            </div>
            <div>
              <span className="text-gray-500">使用中: </span>
              <span className="font-mono">{data.stats.used_count}</span>
            </div>
          </div>
        </div>
      )}

      {!data && !error && (
        <div className="text-gray-400">データなし</div>
      )}
    </div>
  );
}
