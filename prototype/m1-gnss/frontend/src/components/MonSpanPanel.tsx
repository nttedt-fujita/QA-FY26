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
  // 比較用（オプション）
  compareSpectrum?: number[];
  compareMaxAmplitude?: number;
  compareLabel?: string;  // 比較データのラベル（凡例用）
  primaryLabel?: string;  // 基準データのラベル（凡例用）
  // 色指定（オプション）
  strokeColor?: string;         // メイン波形の色
  compareStrokeColor?: string;  // 比較波形の色
}

interface SpectrumChartSingleProps {
  spectrum: number[];
  maxAmplitude?: number;
  expanded?: boolean;
  strokeColor?: string;  // 波形の色
  isDashed?: boolean;    // 点線にするか
}

// 固定スケール: dB表示（0〜64 dB）
// spectrum値は0.25dB単位なので、spectrum * 0.25 = dB
// 出典: ZED-F9P Integration Manual p.83 "256 spectrum data points (0.25 dB units)"
const FIXED_DB_MAX = 64;
// spectrum生値の最大（255 * 0.25 = 63.75 dB）
const SPECTRUM_TO_DB = 0.25;

/**
 * スペクトラム波形チャート（SVG）
 *
 * 比較モード: compareSpectrumを渡すと2つの波形を重ねて表示
 */
export function SpectrumChart({
  spectrum,
  maxAmplitude,
  expanded = false,
  compareSpectrum,
  compareMaxAmplitude,
  compareLabel,
  primaryLabel,
  strokeColor = "#3b82f6",        // デフォルト: 青
  compareStrokeColor = "#f97316", // デフォルト: オレンジ
}: SpectrumChartProps) {
  const isCompareMode = compareSpectrum && compareSpectrum.length > 0;
  // 拡大時はマージンを確保して目盛りを表示
  const margin = expanded ? { top: 10, right: 40, bottom: 30, left: 50 } : { top: 5, right: 5, bottom: 20, left: 35 };
  const chartWidth = 256;
  const chartHeight = expanded ? 200 : 100;
  const totalWidth = chartWidth + margin.left + margin.right;
  const totalHeight = chartHeight + margin.top + margin.bottom;

  // dB単位での固定スケール（0〜64 dB）
  const scaleMaxDb = FIXED_DB_MAX;

  // パスを生成（spectrum値をdBに変換して描画）
  const points = spectrum.map((val, idx) => {
    const x = margin.left + (idx / 255) * chartWidth;
    const dbVal = val * SPECTRUM_TO_DB;  // spectrum → dB変換
    const y = margin.top + chartHeight - (dbVal / scaleMaxDb) * chartHeight;
    return `${x},${y}`;
  });
  const pathD = `M${points.join(" L")}`;

  // Y軸メイン目盛り（dB単位: 0, 16, 32, 48, 64）
  const yMainTicks = expanded ? [0, 16, 32, 48, 64] : [0, 32, 64];
  // Y軸サブ目盛り（8dB刻み）
  const ySubTicks = expanded ? [8, 24, 40, 56] : [16, 48];
  // Y軸細分目盛り（4dB刻み、拡大時のみ）
  const yFineTicks = expanded ? [4, 12, 20, 28, 36, 44, 52, 60] : [];
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
          const y = margin.top + chartHeight - (tick / scaleMaxDb) * chartHeight;
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
          const y = margin.top + chartHeight - (tick / scaleMaxDb) * chartHeight;
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
          const y = margin.top + chartHeight - (tick / scaleMaxDb) * chartHeight;
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
          const y = margin.top + chartHeight - (tick / scaleMaxDb) * chartHeight;
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

        {/* スペクトラム波形（基準） */}
        <path
          d={pathD}
          fill="none"
          stroke={strokeColor}
          strokeWidth={expanded ? 1.5 : 1}
        />

        {/* 比較波形（オプション） */}
        {isCompareMode && (() => {
          const comparePoints = compareSpectrum.map((val, idx) => {
            const x = margin.left + (idx / 255) * chartWidth;
            const dbVal = val * SPECTRUM_TO_DB;  // spectrum → dB変換
            const y = margin.top + chartHeight - (dbVal / scaleMaxDb) * chartHeight;
            return `${x},${y}`;
          });
          const comparePathD = `M${comparePoints.join(" L")}`;
          return (
            <path
              d={comparePathD}
              fill="none"
              stroke={compareStrokeColor}
              strokeWidth={expanded ? 1.5 : 1}
              strokeDasharray={expanded ? "6 3" : "4 2"}
            />
          );
        })()}

        {/* 凡例（比較モード時のみ） */}
        {isCompareMode && expanded && (
          <g>
            {/* 基準 */}
            <line
              x1={margin.left + chartWidth - 120}
              y1={margin.top + 15}
              x2={margin.left + chartWidth - 100}
              y2={margin.top + 15}
              stroke={strokeColor}
              strokeWidth="2"
            />
            <text
              x={margin.left + chartWidth - 95}
              y={margin.top + 15}
              dominantBaseline="middle"
              className="fill-gray-700"
              style={{ fontSize: 10 }}
            >
              {primaryLabel || "基準"}
            </text>
            {/* 比較 */}
            <line
              x1={margin.left + chartWidth - 120}
              y1={margin.top + 30}
              x2={margin.left + chartWidth - 100}
              y2={margin.top + 30}
              stroke={compareStrokeColor}
              strokeWidth="2"
              strokeDasharray="6 3"
            />
            <text
              x={margin.left + chartWidth - 95}
              y={margin.top + 30}
              dominantBaseline="middle"
              className="fill-gray-700"
              style={{ fontSize: 10 }}
            >
              {compareLabel || "比較"}
            </text>
          </g>
        )}

        {/* 比較最大値ライン（比較波形の色に合わせる） */}
        {isCompareMode && compareMaxAmplitude !== undefined && (() => {
          const compareMaxDb = compareMaxAmplitude * SPECTRUM_TO_DB;
          return (
            <line
              x1={margin.left}
              y1={margin.top + chartHeight - (compareMaxDb / scaleMaxDb) * chartHeight}
              x2={margin.left + chartWidth}
              y2={margin.top + chartHeight - (compareMaxDb / scaleMaxDb) * chartHeight}
              stroke={compareStrokeColor}
              strokeWidth="1"
              strokeDasharray="2 2"
              opacity="0.7"
            />
          );
        })()}

        {/* 最大値ライン（波形の色に合わせる） */}
        {maxAmplitude !== undefined && (() => {
          const maxDb = maxAmplitude * SPECTRUM_TO_DB;
          return (
            <>
              <line
                x1={margin.left}
                y1={margin.top + chartHeight - (maxDb / scaleMaxDb) * chartHeight}
                x2={margin.left + chartWidth}
                y2={margin.top + chartHeight - (maxDb / scaleMaxDb) * chartHeight}
                stroke={strokeColor}
                strokeWidth="1"
                strokeDasharray="4 2"
                opacity="0.7"
              />
              <text
                x={margin.left + chartWidth + 3}
                y={margin.top + chartHeight - (maxDb / scaleMaxDb) * chartHeight}
                dominantBaseline="middle"
                fill={strokeColor}
                style={{ fontSize: expanded ? 10 : 8 }}
              >
                Max:{maxDb.toFixed(1)}dB
              </text>
            </>
          );
        })()}
      </svg>
    </div>
  );
}

/**
 * 単一波形チャート（比較画面用）
 *
 * 色と点線を指定可能
 */
export function SpectrumChartSingle({
  spectrum,
  maxAmplitude,
  expanded = false,
  strokeColor = "#3b82f6",
  isDashed = false,
}: SpectrumChartSingleProps) {
  const margin = expanded ? { top: 10, right: 40, bottom: 30, left: 50 } : { top: 5, right: 5, bottom: 20, left: 35 };
  const chartWidth = 256;
  const chartHeight = expanded ? 200 : 80;
  const totalWidth = chartWidth + margin.left + margin.right;
  const totalHeight = chartHeight + margin.top + margin.bottom;

  // dB単位での固定スケール（0〜64 dB）
  const scaleMaxDb = FIXED_DB_MAX;

  // パスを生成（spectrum値をdBに変換して描画）
  const points = spectrum.map((val, idx) => {
    const x = margin.left + (idx / 255) * chartWidth;
    const dbVal = val * SPECTRUM_TO_DB;  // spectrum → dB変換
    const y = margin.top + chartHeight - (dbVal / scaleMaxDb) * chartHeight;
    return `${x},${y}`;
  });
  const pathD = points.length > 0 ? `M${points.join(" L")}` : "";

  // 目盛り（dB単位）
  const yMainTicks = expanded ? [0, 16, 32, 48, 64] : [0, 32, 64];
  const xMainTicks = expanded ? [0, 64, 128, 192, 255] : [0, 128, 255];

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

        {/* グリッド線（横） */}
        {yMainTicks.map((tick) => {
          const y = margin.top + chartHeight - (tick / scaleMaxDb) * chartHeight;
          return (
            <line
              key={`y-${tick}`}
              x1={margin.left}
              y1={y}
              x2={margin.left + chartWidth}
              y2={y}
              stroke="#e5e7eb"
              strokeWidth="1"
            />
          );
        })}

        {/* グリッド線（縦） */}
        {xMainTicks.map((tick) => {
          const x = margin.left + (tick / 255) * chartWidth;
          return (
            <line
              key={`x-${tick}`}
              x1={x}
              y1={margin.top}
              x2={x}
              y2={margin.top + chartHeight}
              stroke="#e5e7eb"
              strokeWidth="1"
            />
          );
        })}

        {/* Y軸目盛りラベル */}
        {yMainTicks.map((tick) => {
          const y = margin.top + chartHeight - (tick / scaleMaxDb) * chartHeight;
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

        {/* X軸目盛りラベル */}
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

        {/* 波形 */}
        {pathD && (
          <path
            d={pathD}
            fill="none"
            stroke={strokeColor}
            strokeWidth={expanded ? 1.5 : 1}
            strokeDasharray={isDashed ? (expanded ? "6 3" : "4 2") : undefined}
          />
        )}

        {/* 最大値ライン（波形の色に合わせる） */}
        {maxAmplitude !== undefined && (() => {
          const maxDb = maxAmplitude * SPECTRUM_TO_DB;
          return (
            <>
              <line
                x1={margin.left}
                y1={margin.top + chartHeight - (maxDb / scaleMaxDb) * chartHeight}
                x2={margin.left + chartWidth}
                y2={margin.top + chartHeight - (maxDb / scaleMaxDb) * chartHeight}
                stroke={strokeColor}
                strokeWidth="1"
                strokeDasharray="4 2"
                opacity="0.7"
              />
              {expanded && (
                <text
                  x={margin.left + chartWidth + 3}
                  y={margin.top + chartHeight - (maxDb / scaleMaxDb) * chartHeight}
                  dominantBaseline="middle"
                  fill={strokeColor}
                  style={{ fontSize: 10 }}
                >
                  Max:{maxDb.toFixed(1)}dB
                </text>
              )}
            </>
          );
        })()}
      </svg>
    </div>
  );
}
