"use client";

import { Lot } from "@/lib/api";

interface LotListProps {
  lots: Lot[];
  selectedLotId: number | null;
  onSelect: (lot: Lot) => void;
  onNewLot: () => void;
}

/**
 * ロット一覧コンポーネント
 *
 * ロットの一覧表示と選択状態を管理
 */
export function LotList({
  lots,
  selectedLotId,
  onSelect,
  onNewLot,
}: LotListProps) {
  return (
    <div className="h-full flex flex-col">
      <div className="flex items-center justify-between border-b border-gray-200 pb-3">
        <h2 className="font-bold text-gray-900">ロット一覧</h2>
        <button
          onClick={onNewLot}
          className="rounded bg-blue-500 px-3 py-1 text-sm font-bold text-white hover:bg-blue-600"
        >
          + 新規作成
        </button>
      </div>

      <div className="mt-3 flex-1 overflow-y-auto">
        {lots.length === 0 ? (
          <p className="text-sm text-gray-500">ロットがありません</p>
        ) : (
          <ul className="space-y-2">
            {lots.map((lot) => (
              <LotItem
                key={lot.id}
                lot={lot}
                isSelected={lot.id === selectedLotId}
                onSelect={onSelect}
              />
            ))}
          </ul>
        )}
      </div>
    </div>
  );
}

interface LotItemProps {
  lot: Lot;
  isSelected: boolean;
  onSelect: (lot: Lot) => void;
}

/**
 * ロット項目
 */
function LotItem({ lot, isSelected, onSelect }: LotItemProps) {
  return (
    <li>
      <button
        onClick={() => onSelect(lot)}
        className={`w-full rounded-lg border p-3 text-left transition-colors ${
          isSelected
            ? "border-blue-500 bg-blue-50"
            : "border-gray-200 bg-white hover:bg-gray-50"
        }`}
      >
        <div className="flex items-center justify-between">
          <span className="font-medium text-gray-900">{lot.lot_number}</span>
          {isSelected && (
            <span className="text-xs text-blue-600">✓ 選択中</span>
          )}
        </div>
        {lot.expected_rate_ms && (
          <p className="mt-1 text-xs text-gray-500">
            出力レート: {lot.expected_rate_ms}ms
          </p>
        )}
      </button>
    </li>
  );
}
