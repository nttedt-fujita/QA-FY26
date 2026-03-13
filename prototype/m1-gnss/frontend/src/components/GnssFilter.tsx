"use client";

import { GNSS_LIST, createAllGnssSet } from "@/lib/gnss-constants";

interface GnssFilterProps {
  /** 選択中のGNSS IDセット */
  selectedGnss: Set<number>;
  /** GNSS選択変更時のコールバック */
  onGnssChange: (gnss: Set<number>) => void;
  /** データがあるGNSS IDセット（オプション） */
  availableGnss?: Set<number>;
  /** 各GNSSの衛星数（オプション） */
  gnssCounts?: Map<number, number>;
}

/**
 * GNSSフィルタコンポーネント
 *
 * 各パネルで共通利用するGNSSフィルタUI
 * - 凡例ボタン（クリックでトグル）
 * - 全選択/全解除ボタン
 */
export function GnssFilter({
  selectedGnss,
  onGnssChange,
  availableGnss,
  gnssCounts,
}: GnssFilterProps) {
  // フィルタ切り替え
  const toggleGnss = (gnssId: number) => {
    const next = new Set(selectedGnss);
    if (next.has(gnssId)) {
      next.delete(gnssId);
    } else {
      next.add(gnssId);
    }
    onGnssChange(next);
  };

  // 全選択
  const selectAll = () => onGnssChange(createAllGnssSet());

  // 全解除
  const selectNone = () => onGnssChange(new Set());

  return (
    <div className="space-y-2">
      <div className="flex items-center justify-between">
        <span className="text-xs text-gray-500">GNSSフィルタ:</span>
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
          const isVisible = selectedGnss.has(gnss.id);
          const hasData = availableGnss ? availableGnss.has(gnss.id) : true;
          const count = gnssCounts?.get(gnss.id);

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
              {count !== undefined && (
                <span className="font-mono text-gray-400">({count})</span>
              )}
            </button>
          );
        })}
      </div>
    </div>
  );
}
