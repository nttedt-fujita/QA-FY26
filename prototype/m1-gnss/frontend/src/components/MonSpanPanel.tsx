"use client";

import { useState, useEffect, useCallback } from "react";
import { MonSpanResponse, SpanBlock, getMonSpan } from "@/lib/api";

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
  /** ポーリング有効フラグ（装置接続時のみtrue） */
  enabled: boolean;
  /** ポーリング間隔（ミリ秒） */
  pollIntervalMs?: number;
}

/**
 * MON-SPAN表示パネル
 *
 * - スペクトラム波形表示（256点グラフ）
 * - PGAゲージ（正常/異常表示）
 * - L1/L2別表示
 */
export function MonSpanPanel({
  enabled,
  pollIntervalMs = 5000, // MON-SPANは取得に時間がかかるので5秒
}: MonSpanPanelProps) {
  const [data, setData] = useState<MonSpanResponse | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [isLoading, setIsLoading] = useState(false);

  // データ取得
  const fetchData = useCallback(async () => {
    if (!enabled) return;

    setIsLoading(true);
    try {
      console.log("[MON-SPAN] fetching...");
      const res = await getMonSpan();
      console.log("[MON-SPAN] success:", res.blocks.length, "blocks");
      setData(res);
      setError(null);
    } catch (e) {
      const msg = e instanceof Error ? e.message : "取得失敗";
      console.error("[MON-SPAN] error:", msg);
      setError(msg);
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

  // 全ブロックのPGA判定
  const allPgaNormal = data?.blocks.every((b) => b.pga >= PGA_NORMAL_THRESHOLD) ?? false;

  if (!enabled) {
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

      {data && data.blocks.map((block, idx) => (
        <SpanBlockCard key={idx} block={block} />
      ))}
    </div>
  );
}

/**
 * スペクトラムブロックカード
 */
function SpanBlockCard({ block }: { block: SpanBlock }) {
  const blockName = getBlockName(block.center);
  const isPgaNormal = block.pga >= PGA_NORMAL_THRESHOLD;

  return (
    <div className="mb-4 rounded border border-gray-100 bg-gray-50 p-3 last:mb-0">
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

      {/* スペクトラム波形 */}
      <div className="mb-2">
        <div className="mb-1 flex items-center justify-between">
          <span className="text-sm text-gray-600">スペクトラム</span>
          <span className="text-xs text-gray-500">
            Max: {block.max_amplitude} dB / Avg: {block.avg_amplitude.toFixed(1)} dB
          </span>
        </div>
        <SpectrumChart spectrum={block.spectrum} />
      </div>
    </div>
  );
}

/**
 * スペクトラム波形チャート（SVG）
 */
function SpectrumChart({ spectrum }: { spectrum: number[] }) {
  const width = 256;
  const height = 60;
  const maxVal = Math.max(...spectrum, 1);

  // パスを生成
  const points = spectrum.map((val, idx) => {
    const x = idx;
    const y = height - (val / maxVal) * height;
    return `${x},${y}`;
  });
  const pathD = `M${points.join(" L")}`;

  return (
    <div className="overflow-hidden rounded border border-gray-200 bg-white">
      <svg
        viewBox={`0 0 ${width} ${height}`}
        className="h-16 w-full"
        preserveAspectRatio="none"
      >
        {/* グリッド線 */}
        <line
          x1="0"
          y1={height / 2}
          x2={width}
          y2={height / 2}
          stroke="#e5e7eb"
          strokeWidth="1"
        />
        <line
          x1={width / 2}
          y1="0"
          x2={width / 2}
          y2={height}
          stroke="#e5e7eb"
          strokeWidth="1"
        />
        {/* スペクトラム波形 */}
        <path
          d={pathD}
          fill="none"
          stroke="#3b82f6"
          strokeWidth="1"
        />
      </svg>
    </div>
  );
}
