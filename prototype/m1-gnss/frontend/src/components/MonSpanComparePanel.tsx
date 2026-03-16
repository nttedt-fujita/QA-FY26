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
 * 2つのデータを重ねて表示
 */
export function MonSpanComparePanel({
  primary,
  secondary,
}: MonSpanComparePanelProps) {
  const [expandedBlock, setExpandedBlock] = useState<string | null>(null);

  // 同一機体かどうか判定
  const isSameDevice =
    primary?.serialNumber &&
    secondary?.serialNumber &&
    primary.serialNumber === secondary.serialNumber;

  // ブロックをL1/L2でグルーピング
  const blockNames = ["L1", "L2"];

  // 各帯域のブロックを取得
  const getBlock = (data: MonSpanResponse | undefined, blockName: string): SpanBlock | undefined => {
    return data?.blocks.find((b) => getBlockName(b.center) === blockName);
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

      {/* データ情報 */}
      <div className="mb-4 grid grid-cols-2 gap-4 text-sm">
        <div className="rounded bg-blue-50 p-2">
          <span className="font-medium text-blue-700">基準: </span>
          {primary ? (
            <span className="font-mono">{primary.label}</span>
          ) : (
            <span className="text-gray-400">未選択</span>
          )}
        </div>
        <div className="rounded bg-orange-50 p-2">
          <span className="font-medium text-orange-700">比較: </span>
          {secondary ? (
            <span className="font-mono">{secondary.label}</span>
          ) : (
            <span className="text-gray-400">未選択</span>
          )}
        </div>
      </div>

      {/* L1/L2 横並び */}
      <div className="grid grid-cols-2 gap-4">
        {blockNames.map((blockName) => {
          const primaryBlock = getBlock(primary?.data, blockName);
          const secondaryBlock = getBlock(secondary?.data, blockName);

          // 両方ない場合はスキップ
          if (!primaryBlock && !secondaryBlock) {
            return (
              <div key={blockName} className="rounded border border-gray-100 bg-gray-50 p-3">
                <span className="font-medium text-gray-400">{blockName}帯: データなし</span>
              </div>
            );
          }

          return (
            <div key={blockName} className="rounded border border-gray-100 bg-gray-50 p-3">
              {/* ヘッダー */}
              <div className="mb-2 flex items-center justify-between">
                <span className="font-medium text-gray-700">{blockName}帯</span>
                {primaryBlock && (
                  <span className="text-xs text-gray-500">
                    {formatFrequency(primaryBlock.center)}
                  </span>
                )}
              </div>

              {/* PGA情報 */}
              <div className="mb-2 text-xs text-gray-600">
                {primaryBlock && (
                  <div>
                    <span className="text-blue-600">基準:</span> PGA {primaryBlock.pga}dB / Max {primaryBlock.max_amplitude}
                  </div>
                )}
                {secondaryBlock && (
                  <div>
                    <span className="text-orange-600">比較:</span> PGA {secondaryBlock.pga}dB / Max {secondaryBlock.max_amplitude}
                  </div>
                )}
              </div>

              {/* 波形（クリックで拡大） */}
              <button
                onClick={() => setExpandedBlock(blockName)}
                className="w-full cursor-pointer hover:opacity-80 transition-opacity"
                title="クリックで拡大"
              >
                <SpectrumChart
                  spectrum={primaryBlock?.spectrum ?? []}
                  maxAmplitude={primaryBlock?.max_amplitude}
                  compareSpectrum={secondaryBlock?.spectrum}
                  compareMaxAmplitude={secondaryBlock?.max_amplitude}
                  primaryLabel={primary?.label}
                  compareLabel={secondary?.label}
                />
              </button>
              <div className="mt-1 text-center text-xs text-gray-400">
                クリックで拡大
              </div>
            </div>
          );
        })}
      </div>

      {/* 拡大モーダル */}
      {expandedBlock && (
        <div
          className="fixed inset-0 z-50 flex items-center justify-center bg-black/50"
          onClick={() => setExpandedBlock(null)}
        >
          <div
            className="bg-white rounded-lg p-6 w-[90vw] h-[85vh] max-w-none"
            onClick={(e) => e.stopPropagation()}
          >
            <div className="flex items-center justify-between mb-4">
              <h3 className="text-lg font-medium">
                {expandedBlock}帯 スペクトラム比較
              </h3>
              <button
                onClick={() => setExpandedBlock(null)}
                className="text-gray-500 hover:text-gray-700 text-2xl"
              >
                ×
              </button>
            </div>

            {/* 凡例 */}
            <div className="mb-4 flex gap-6 text-sm">
              <div className="flex items-center gap-2">
                <div className="w-6 h-0.5 bg-blue-500" />
                <span className="text-gray-700">{primary?.label || "基準"}</span>
              </div>
              <div className="flex items-center gap-2">
                <div className="w-6 h-0.5 bg-orange-500" style={{ borderStyle: "dashed" }} />
                <span className="text-gray-700">{secondary?.label || "比較"}</span>
              </div>
            </div>

            {(() => {
              const primaryBlock = getBlock(primary?.data, expandedBlock);
              const secondaryBlock = getBlock(secondary?.data, expandedBlock);
              return (
                <SpectrumChart
                  spectrum={primaryBlock?.spectrum ?? []}
                  maxAmplitude={primaryBlock?.max_amplitude}
                  compareSpectrum={secondaryBlock?.spectrum}
                  compareMaxAmplitude={secondaryBlock?.max_amplitude}
                  primaryLabel={primary?.label}
                  compareLabel={secondary?.label}
                  expanded
                />
              );
            })()}
          </div>
        </div>
      )}
    </div>
  );
}
