"use client";

import { useState } from "react";
import { MonSpanResponse, SpanBlock } from "@/lib/api";

// PGAゲインの基準値（dB）
// - 正常機: 54dB程度
// - No.5異常機: 38dB（低い）
const PGA_NORMAL_THRESHOLD = 45; // 45dB未満は異常と判定

/**
 * PGAゲイン値に応じた色クラスを返す
 */
function getPgaColorClass(pga: number): string {
  if (pga >= PGA_NORMAL_THRESHOLD) return "text-green-600";
  return "text-red-600";
}

/**
 * PGAバーの幅を計算（0-60dBの範囲）
 */
function getPgaBarWidth(pga: number): number {
  return Math.min(100, Math.max(0, (pga / 60) * 100));
}

/**
 * 周波数を読みやすい形式に変換（GHz）
 */
function formatFrequency(hz: number): string {
  return (hz / 1e9).toFixed(3) + " GHz";
}

/**
 * ブロック名（L1/L2）を中心周波数から判定
 */
function getBlockName(centerHz: number): string {
  // L1: 1575.42 MHz, L2: 1227.60 MHz
  if (centerHz > 1500e6) return "L1";
  if (centerHz > 1200e6) return "L2";
  return "?";
}

interface MonSpanPanelProps {
  /** MON-SPANデータ（統合APIから渡される） */
  data: MonSpanResponse | null;
  /** エラーメッセージ */
  error?: string | null;
  /** 読み込み中フラグ */
  isLoading?: boolean;
  /** 装置接続フラグ */
  isConnected?: boolean;
}

/**
 * MON-SPAN表示パネル
 *
 * - スペクトラム波形表示（256点グラフ）
 * - PGAゲージ（正常/異常表示）
 * - L1/L2別表示
 */
export function MonSpanPanel({
  data,
  error,
  isLoading = false,
  isConnected = true,
}: MonSpanPanelProps) {
  // 全ブロックのPGA判定
  const allPgaNormal = data?.blocks.every((b) => b.pga >= PGA_NORMAL_THRESHOLD) ?? false;

  if (!isConnected) {
    return (
      <div className="rounded border border-gray-200 bg-white p-4">
        <h3 className="mb-2 font-medium text-gray-700">スペクトラム (MON-SPAN)</h3>
        <div className="text-gray-500">装置が接続されていません</div>
      </div>
    );
  }

  if (error) {
    return (
      <div className="rounded border border-red-200 bg-red-50 p-4">
        <h3 className="mb-2 font-medium text-red-700">スペクトラム (MON-SPAN)</h3>
        <div className="text-red-600">{error}</div>
      </div>
    );
  }

  return (
    <div className="rounded border border-gray-200 bg-white p-4">
      <div className="mb-4 flex items-center justify-between">
        <h3 className="font-medium text-gray-700">スペクトラム (MON-SPAN)</h3>
        <div className="flex items-center gap-2">
          {isLoading && (
            <span className="text-xs text-gray-400">更新中...</span>
          )}
          {data && (
            <span
              className={`rounded px-2 py-0.5 text-sm font-semibold ${
                allPgaNormal
                  ? "bg-green-100 text-green-700"
                  : "bg-red-100 text-red-700"
              }`}
            >
              {allPgaNormal ? "✓ 正常" : "× 異常"}
            </span>
          )}
        </div>
      </div>

      {data && data.blocks.length === 0 && (
        <div className="text-gray-500">ブロックなし</div>
      )}

      {data && data.blocks.length > 0 && (
        <div className="grid grid-cols-2 gap-4">
          {data.blocks.map((block, idx) => (
            <SpanBlockCard key={idx} block={block} />
          ))}
        </div>
      )}

      {!data && !error && (
        <div className="text-gray-400">データなし</div>
      )}
    </div>
  );
}

/**
 * スペクトラムブロックカード
 */
function SpanBlockCard({ block }: { block: SpanBlock }) {
  const [isExpanded, setIsExpanded] = useState(false);
  const blockName = getBlockName(block.center);
  const isPgaNormal = block.pga >= PGA_NORMAL_THRESHOLD;

  return (
    <>
      <div className="rounded border border-gray-100 bg-gray-50 p-3">
        {/* ヘッダー */}
        <div className="mb-2 flex items-center justify-between">
          <span className="font-medium text-gray-700">{blockName}帯</span>
          <span className="text-xs text-gray-500">
            {formatFrequency(block.center)}
          </span>
        </div>

        {/* PGAゲージ */}
        <div className="mb-3">
          <div className="mb-1 flex items-center justify-between">
            <span className="text-sm text-gray-600">PGAゲイン</span>
            <span
              className={`text-sm font-semibold ${getPgaColorClass(block.pga)}`}
            >
              {block.pga} dB
              {isPgaNormal ? " (正常)" : " (異常)"}
            </span>
          </div>
          <div className="h-3 w-full overflow-hidden rounded bg-gray-200">
            <div
              className={`h-full transition-all duration-300 ${
                isPgaNormal ? "bg-green-500" : "bg-red-500"
              }`}
              style={{ width: `${getPgaBarWidth(block.pga)}%` }}
            />
          </div>
          <div className="mt-1 flex justify-between text-xs text-gray-400">
            <span>0</span>
            <span>45 (基準)</span>
            <span>60 dB</span>
          </div>
        </div>

        {/* スペクトラム波形（クリックで拡大） */}
        <div className="mb-2">
          <div className="mb-1 flex items-center justify-between">
            <span className="text-sm text-gray-600">スペクトラム</span>
            <span className="text-xs text-gray-500">
              Max: {block.max_amplitude} dB / Avg: {block.avg_amplitude.toFixed(1)} dB
            </span>
          </div>
          <button
            onClick={() => setIsExpanded(true)}
            className="w-full cursor-pointer hover:opacity-80 transition-opacity"
            title="クリックで拡大"
          >
            <SpectrumChart spectrum={block.spectrum} maxAmplitude={block.max_amplitude} />
          </button>
          <div className="mt-1 text-center text-xs text-gray-400">
            クリックで拡大
          </div>
        </div>
      </div>

      {/* 拡大モーダル */}
      {isExpanded && (
        <div
          className="fixed inset-0 z-50 flex items-center justify-center bg-black/50"
          onClick={() => setIsExpanded(false)}
        >
          <div
            className="bg-white rounded-lg p-6 w-[90vw] h-[85vh] max-w-none"
            onClick={(e) => e.stopPropagation()}
          >
            <div className="flex items-center justify-between mb-4">
              <h3 className="text-lg font-medium">
                {blockName}帯 スペクトラム ({formatFrequency(block.center)})
              </h3>
              <button
                onClick={() => setIsExpanded(false)}
                className="text-gray-500 hover:text-gray-700 text-2xl"
              >
                ×
              </button>
            </div>
            <div className="mb-2 text-sm text-gray-600">
              Max: {block.max_amplitude} dB / Avg: {block.avg_amplitude.toFixed(1)} dB / PGA: {block.pga} dB
            </div>
            <SpectrumChart spectrum={block.spectrum} maxAmplitude={block.max_amplitude} expanded />
          </div>
        </div>
      )}
    </>
  );
}

interface SpectrumChartProps {
  spectrum: number[];
  maxAmplitude?: number;  // 最大振幅値（表示用）
  expanded?: boolean;
}

// 固定スケール: 振幅の最大値（全波形で統一）
const FIXED_AMPLITUDE_MAX = 255;

/**
 * スペクトラム波形チャート（SVG）
 */
function SpectrumChart({ spectrum, maxAmplitude, expanded = false }: SpectrumChartProps) {
  // 拡大時はマージンを確保して目盛りを表示
  const margin = expanded ? { top: 10, right: 40, bottom: 30, left: 50 } : { top: 5, right: 5, bottom: 20, left: 35 };
  const chartWidth = 256;
  const chartHeight = expanded ? 200 : 100;
  const totalWidth = chartWidth + margin.left + margin.right;
  const totalHeight = chartHeight + margin.top + margin.bottom;

  // 固定スケールを使用（全波形で同じスケール）
  const scaleMax = FIXED_AMPLITUDE_MAX;

  // パスを生成（固定スケールで描画）
  const points = spectrum.map((val, idx) => {
    const x = margin.left + (idx / 255) * chartWidth;
    const y = margin.top + chartHeight - (val / scaleMax) * chartHeight;
    return `${x},${y}`;
  });
  const pathD = `M${points.join(" L")}`;

  // Y軸メイン目盛り（固定: 0, 64, 128, 192, 255）
  const yMainTicks = expanded ? [0, 64, 128, 192, 255] : [0, 128, 255];
  // Y軸サブ目盛り（32刻み）
  const ySubTicks = expanded ? [32, 96, 160, 224] : [64, 192];
  // Y軸細分目盛り（16刻み、拡大時のみ）
  const yFineTicks = expanded ? [16, 48, 80, 112, 144, 176, 208, 240] : [];
  // X軸メイン目盛り（周波数bin: 0, 64, 128, 192, 255）
  const xMainTicks = expanded ? [0, 64, 128, 192, 255] : [0, 128, 255];
  // X軸サブ目盛り（32刻み）
  const xSubTicks = expanded ? [32, 96, 160, 224] : [64, 192];
  // X軸細分目盛り（16刻み、拡大時のみ）
  const xFineTicks = expanded ? [16, 48, 80, 112, 144, 176, 208, 240] : [];

  return (
    <div className={`overflow-hidden rounded border border-gray-200 bg-white ${expanded ? "" : "aspect-video"}`}>
      <svg
        viewBox={`0 0 ${totalWidth} ${totalHeight}`}
        className={expanded ? "w-full h-[70vh]" : "h-full w-full"}
        preserveAspectRatio={expanded ? "xMidYMid meet" : "none"}
      >
        {/* 背景 */}
        <rect
          x={margin.left}
          y={margin.top}
          width={chartWidth}
          height={chartHeight}
          fill="#fafafa"
        />

        {/* 細分グリッド線（横）- 16刻み、最も薄い */}
        {yFineTicks.map((tick) => {
          const y = margin.top + chartHeight - (tick / scaleMax) * chartHeight;
          return (
            <line
              key={`y-fine-${tick}`}
              x1={margin.left}
              y1={y}
              x2={margin.left + chartWidth}
              y2={y}
              stroke="#f9fafb"
              strokeWidth="1"
            />
          );
        })}

        {/* サブグリッド線（横）- 32刻み */}
        {ySubTicks.map((tick) => {
          const y = margin.top + chartHeight - (tick / scaleMax) * chartHeight;
          return (
            <line
              key={`y-sub-${tick}`}
              x1={margin.left}
              y1={y}
              x2={margin.left + chartWidth}
              y2={y}
              stroke="#f3f4f6"
              strokeWidth="1"
            />
          );
        })}

        {/* メイングリッド線（横） */}
        {yMainTicks.map((tick) => {
          const y = margin.top + chartHeight - (tick / scaleMax) * chartHeight;
          return (
            <line
              key={`y-main-${tick}`}
              x1={margin.left}
              y1={y}
              x2={margin.left + chartWidth}
              y2={y}
              stroke="#d1d5db"
              strokeWidth="1"
            />
          );
        })}

        {/* 細分グリッド線（縦）- 16刻み、最も薄い */}
        {xFineTicks.map((tick) => {
          const x = margin.left + (tick / 255) * chartWidth;
          return (
            <line
              key={`x-fine-${tick}`}
              x1={x}
              y1={margin.top}
              x2={x}
              y2={margin.top + chartHeight}
              stroke="#f9fafb"
              strokeWidth="1"
            />
          );
        })}

        {/* サブグリッド線（縦）- 32刻み */}
        {xSubTicks.map((tick) => {
          const x = margin.left + (tick / 255) * chartWidth;
          return (
            <line
              key={`x-sub-${tick}`}
              x1={x}
              y1={margin.top}
              x2={x}
              y2={margin.top + chartHeight}
              stroke="#f3f4f6"
              strokeWidth="1"
            />
          );
        })}

        {/* メイングリッド線（縦） */}
        {xMainTicks.map((tick) => {
          const x = margin.left + (tick / 255) * chartWidth;
          return (
            <line
              key={`x-main-${tick}`}
              x1={x}
              y1={margin.top}
              x2={x}
              y2={margin.top + chartHeight}
              stroke="#d1d5db"
              strokeWidth="1"
            />
          );
        })}

        {/* Y軸目盛りラベル（メインのみ） */}
        {yMainTicks.map((tick) => {
          const y = margin.top + chartHeight - (tick / scaleMax) * chartHeight;
          return (
            <text
              key={`y-label-${tick}`}
              x={margin.left - 5}
              y={y}
              textAnchor="end"
              dominantBaseline="middle"
              className="fill-gray-500"
              style={{ fontSize: expanded ? 10 : 8 }}
            >
              {tick}
            </text>
          );
        })}

        {/* X軸目盛りラベル（メインのみ） */}
        {xMainTicks.map((tick) => {
          const x = margin.left + (tick / 255) * chartWidth;
          return (
            <text
              key={`x-label-${tick}`}
              x={x}
              y={margin.top + chartHeight + 12}
              textAnchor="middle"
              className="fill-gray-500"
              style={{ fontSize: expanded ? 10 : 8 }}
            >
              {tick}
            </text>
          );
        })}

        {/* 軸ラベル（拡大時のみ） */}
        {expanded && (
          <>
            <text
              x={margin.left - 35}
              y={margin.top + chartHeight / 2}
              textAnchor="middle"
              dominantBaseline="middle"
              transform={`rotate(-90, ${margin.left - 35}, ${margin.top + chartHeight / 2})`}
              className="fill-gray-600"
              style={{ fontSize: 11 }}
            >
              振幅 (dB)
            </text>
            <text
              x={margin.left + chartWidth / 2}
              y={totalHeight - 5}
              textAnchor="middle"
              className="fill-gray-600"
              style={{ fontSize: 11 }}
            >
              周波数 bin
            </text>
          </>
        )}

        {/* スペクトラム波形 */}
        <path
          d={pathD}
          fill="none"
          stroke="#3b82f6"
          strokeWidth={expanded ? 1.5 : 1}
        />

        {/* 最大値ライン */}
        {maxAmplitude !== undefined && (
          <>
            <line
              x1={margin.left}
              y1={margin.top + chartHeight - (maxAmplitude / scaleMax) * chartHeight}
              x2={margin.left + chartWidth}
              y2={margin.top + chartHeight - (maxAmplitude / scaleMax) * chartHeight}
              stroke="#ef4444"
              strokeWidth="1"
              strokeDasharray="4 2"
            />
            <text
              x={margin.left + chartWidth + 3}
              y={margin.top + chartHeight - (maxAmplitude / scaleMax) * chartHeight}
              dominantBaseline="middle"
              className="fill-red-500"
              style={{ fontSize: expanded ? 10 : 8 }}
            >
              Max:{maxAmplitude}
            </text>
          </>
        )}
      </svg>
    </div>
  );
}
