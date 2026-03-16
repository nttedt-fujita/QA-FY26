"use client";

import { useState } from "react";
import { MonSpanResponse, SpanBlock } from "@/lib/api";
import { SpectrumChart } from "./MonSpanPanel";

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
  if (centerHz > 1500e6) return "L1";
  if (centerHz > 1200e6) return "L2";
  return "?";
}

/**
 * 帯域ごとの色設定
 * - 基準側: L1=青、L2=緑
 * - 比較側: L1=オレンジ、L2=紫
 * - 転送表示（点線）: 元の色のまま
 */
const BAND_COLORS = {
  L1: {
    primary: "#3b82f6",      // 青（基準側の実線）
    primaryDashed: "#60a5fa", // 明るい青（基準側からの点線）
    secondary: "#f97316",     // オレンジ（比較側の実線）
    secondaryDashed: "#fb923c", // 明るいオレンジ（比較側からの点線）
  },
  L2: {
    primary: "#22c55e",      // 緑（基準側の実線）
    primaryDashed: "#4ade80", // 明るい緑（基準側からの点線）
    secondary: "#a855f7",     // 紫（比較側の実線）
    secondaryDashed: "#c084fc", // 明るい紫（比較側からの点線）
  },
} as const;

interface CompareData {
  /** データラベル（シリアル番号等） */
  label: string;
  /** シリアル番号 */
  serialNumber?: string;
  /** MON-SPANデータ */
  data: MonSpanResponse;
}

interface MonSpanComparePanelProps {
  /** 基準データ */
  primary: CompareData | null;
  /** 比較データ */
  secondary: CompareData | null;
}

/**
 * MON-SPAN比較パネル
 *
 * 縦: L1/L2、横: 基準/比較 のレイアウト
 */
export function MonSpanComparePanel({
  primary,
  secondary,
}: MonSpanComparePanelProps) {
  const [expandedCell, setExpandedCell] = useState<{band: string, type: "primary" | "secondary"} | null>(null);

  // 同一機体かどうか判定
  const isSameDevice =
    primary?.serialNumber &&
    secondary?.serialNumber &&
    primary.serialNumber === secondary.serialNumber;

  // ブロックをL1/L2でグルーピング
  const bandNames = ["L1", "L2"] as const;

  // 各帯域のブロックを取得
  const getBlock = (data: MonSpanResponse | undefined, bandName: string): SpanBlock | undefined => {
    return data?.blocks.find((b) => getBlockName(b.center) === bandName);
  };

  if (!primary && !secondary) {
    return (
      <div className="rounded border border-gray-200 bg-white p-4">
        <h3 className="mb-2 font-medium text-gray-700">スペクトラム比較</h3>
        <div className="text-gray-500">データを選択してください</div>
      </div>
    );
  }

  return (
    <div className="rounded border border-gray-200 bg-white p-4">
      <div className="mb-4 flex items-center justify-between">
        <h3 className="font-medium text-gray-700">スペクトラム比較</h3>
        {primary && secondary && (
          <span
            className={`rounded px-2 py-0.5 text-sm font-medium ${
              isSameDevice
                ? "bg-blue-100 text-blue-700"
                : "bg-yellow-100 text-yellow-700"
            }`}
          >
            {isSameDevice ? "同一機体" : "別機体"}
          </span>
        )}
      </div>

      {/* ヘッダー行: 基準 / 比較 */}
      <div className="grid grid-cols-[80px_1fr_1fr] gap-2 mb-2">
        <div /> {/* 空セル */}
        <div className="rounded bg-gray-100 p-2 text-center">
          <span className="font-medium text-gray-700">基準</span>
          {primary && (
            <div className="text-xs text-gray-500 font-mono truncate">{primary.label}</div>
          )}
        </div>
        <div className="rounded bg-gray-100 p-2 text-center">
          <span className="font-medium text-gray-700">比較</span>
          {secondary && (
            <div className="text-xs text-gray-500 font-mono truncate">{secondary.label}</div>
          )}
        </div>
      </div>

      {/* L1/L2 行 */}
      {bandNames.map((bandName) => {
        const primaryBlock = getBlock(primary?.data, bandName);
        const secondaryBlock = getBlock(secondary?.data, bandName);
        const colors = BAND_COLORS[bandName];

        return (
          <div key={bandName} className="grid grid-cols-[80px_1fr_1fr] gap-2 mb-2">
            {/* 帯域ラベル */}
            <div
              className="flex flex-col items-center justify-center rounded p-2"
              style={{ backgroundColor: bandName === "L1" ? "#eff6ff" : "#f0fdf4" }}
            >
              <span
                className="font-bold text-lg"
                style={{ color: colors.primary }}
              >
                {bandName}
              </span>
              {primaryBlock && (
                <span className="text-xs text-gray-500">
                  {formatFrequency(primaryBlock.center)}
                </span>
              )}
            </div>

            {/* 基準データ波形（基準=実線、比較=点線） */}
            <div className="rounded border border-gray-100 bg-gray-50 p-2">
              {primaryBlock ? (
                <>
                  <div className="mb-1 text-xs text-gray-600">
                    PGA {primaryBlock.pga}dB / Max {primaryBlock.max_amplitude}
                  </div>
                  <button
                    onClick={() => setExpandedCell({ band: bandName, type: "primary" })}
                    className="w-full cursor-pointer hover:opacity-80 transition-opacity"
                    title="クリックで拡大"
                  >
                    <SpectrumChart
                      spectrum={primaryBlock.spectrum}
                      maxAmplitude={primaryBlock.max_amplitude}
                      compareSpectrum={secondaryBlock?.spectrum}
                      compareMaxAmplitude={secondaryBlock?.max_amplitude}
                      strokeColor={colors.primary}
                      compareStrokeColor={colors.secondaryDashed}
                    />
                  </button>
                </>
              ) : (
                <div className="text-gray-400 text-sm text-center py-4">データなし</div>
              )}
            </div>

            {/* 比較データ波形（比較=実線、基準=点線） */}
            <div className="rounded border border-gray-100 bg-gray-50 p-2">
              {secondaryBlock ? (
                <>
                  <div className="mb-1 text-xs text-gray-600">
                    PGA {secondaryBlock.pga}dB / Max {secondaryBlock.max_amplitude}
                  </div>
                  <button
                    onClick={() => setExpandedCell({ band: bandName, type: "secondary" })}
                    className="w-full cursor-pointer hover:opacity-80 transition-opacity"
                    title="クリックで拡大"
                  >
                    <SpectrumChart
                      spectrum={secondaryBlock.spectrum}
                      maxAmplitude={secondaryBlock.max_amplitude}
                      compareSpectrum={primaryBlock?.spectrum}
                      compareMaxAmplitude={primaryBlock?.max_amplitude}
                      strokeColor={colors.secondary}
                      compareStrokeColor={colors.primaryDashed}
                    />
                  </button>
                </>
              ) : (
                <div className="text-gray-400 text-sm text-center py-4">データなし</div>
              )}
            </div>
          </div>
        );
      })}

      {/* 凡例 */}
      <div className="mt-4 flex flex-wrap gap-4 text-sm justify-center">
        <div className="flex items-center gap-2">
          <div className="w-6 h-0.5" style={{ backgroundColor: BAND_COLORS.L1.primary }} />
          <span className="text-gray-700">L1 基準</span>
        </div>
        <div className="flex items-center gap-2">
          <div className="w-6 h-0.5" style={{ backgroundColor: BAND_COLORS.L1.secondary }} />
          <span className="text-gray-700">L1 比較</span>
        </div>
        <div className="flex items-center gap-2">
          <div className="w-6 h-0.5" style={{ backgroundColor: BAND_COLORS.L2.primary }} />
          <span className="text-gray-700">L2 基準</span>
        </div>
        <div className="flex items-center gap-2">
          <div className="w-6 h-0.5" style={{ backgroundColor: BAND_COLORS.L2.secondary }} />
          <span className="text-gray-700">L2 比較</span>
        </div>
      </div>

      {/* 拡大モーダル */}
      {expandedCell && (
        <div
          className="fixed inset-0 z-50 flex items-center justify-center bg-black/50"
          onClick={() => setExpandedCell(null)}
        >
          <div
            className="bg-white rounded-lg p-6 w-[90vw] h-[85vh] max-w-none"
            onClick={(e) => e.stopPropagation()}
          >
            {(() => {
              const primaryBlock = getBlock(primary?.data, expandedCell.band);
              const secondaryBlock = getBlock(secondary?.data, expandedCell.band);
              const mainBlock = expandedCell.type === "primary" ? primaryBlock : secondaryBlock;
              const compareBlock = expandedCell.type === "primary" ? secondaryBlock : primaryBlock;
              const label = expandedCell.type === "primary" ? primary?.label : secondary?.label;
              const colors = BAND_COLORS[expandedCell.band as keyof typeof BAND_COLORS];

              return (
                <>
                  <div className="flex items-center justify-between mb-4">
                    <h3 className="text-lg font-medium">
                      {expandedCell.band}帯 - {expandedCell.type === "primary" ? "基準" : "比較"}
                      {label && <span className="ml-2 text-gray-500 font-mono text-sm">({label})</span>}
                    </h3>
                    <button
                      onClick={() => setExpandedCell(null)}
                      className="text-gray-500 hover:text-gray-700 text-2xl"
                    >
                      ×
                    </button>
                  </div>
                  {mainBlock && (
                    <>
                      <div className="mb-2 text-sm text-gray-600">
                        {formatFrequency(mainBlock.center)} / PGA {mainBlock.pga}dB / Max {mainBlock.max_amplitude}
                      </div>
                      <SpectrumChart
                        spectrum={mainBlock.spectrum}
                        maxAmplitude={mainBlock.max_amplitude}
                        compareSpectrum={compareBlock?.spectrum}
                        compareMaxAmplitude={compareBlock?.max_amplitude}
                        strokeColor={expandedCell.type === "primary" ? colors.primary : colors.secondary}
                        compareStrokeColor={expandedCell.type === "primary" ? colors.secondaryDashed : colors.primaryDashed}
                        primaryLabel={expandedCell.type === "primary" ? "基準" : "比較"}
                        compareLabel={expandedCell.type === "primary" ? "比較" : "基準"}
                        expanded
                      />
                    </>
                  )}
                </>
              );
            })()}
          </div>
        </div>
      )}
    </div>
  );
}
