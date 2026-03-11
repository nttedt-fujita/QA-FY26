"use client";

import { useState, useEffect, useCallback } from "react";
import {
  Lot,
  listLots,
  createLot,
  updateLot,
  CreateLotRequest,
  UpdateLotRequest,
} from "@/lib/api";
import { LotList } from "@/components/LotList";
import { LotForm } from "@/components/LotForm";

/**
 * ロット管理画面
 *
 * レイアウト:
 * ┌──────────────────┬────────────────────┐
 * │ ロット一覧        │ ロット詳細         │
 * │                  │                    │
 * │ • LOT-2026-001   │ ロット番号:        │
 * │   ✓ 選択中       │ [LOT-2026-001  ]   │
 * │                  │                    │
 * │ • LOT-2026-002   │ 期待値:            │
 * │                  │ 出力レート: [100ms] │
 * │                  │ ポート設定: [UBX]   │
 * │                  │                    │
 * │ [+ 新規作成]     │ [保存]             │
 * └──────────────────┴────────────────────┘
 */
export default function LotsPage() {
  const [lots, setLots] = useState<Lot[]>([]);
  const [selectedLot, setSelectedLot] = useState<Lot | null>(null);
  const [isNew, setIsNew] = useState(false);
  const [isLoading, setIsLoading] = useState(true);
  const [isSaving, setIsSaving] = useState(false);
  const [error, setError] = useState<string | null>(null);

  // ロット一覧を取得
  const fetchLots = useCallback(async () => {
    try {
      setIsLoading(true);
      setError(null);
      const response = await listLots();
      setLots(response.lots);
    } catch (e) {
      setError(e instanceof Error ? e.message : "ロット取得に失敗しました");
    } finally {
      setIsLoading(false);
    }
  }, []);

  useEffect(() => {
    fetchLots();
  }, [fetchLots]);

  // ロット選択
  const handleSelect = (lot: Lot) => {
    setSelectedLot(lot);
    setIsNew(false);
  };

  // 新規作成モード
  const handleNewLot = () => {
    setSelectedLot(null);
    setIsNew(true);
  };

  // キャンセル
  const handleCancel = () => {
    setIsNew(false);
    setSelectedLot(null);
  };

  // 保存
  const handleSave = async (request: CreateLotRequest | UpdateLotRequest) => {
    try {
      setIsSaving(true);
      setError(null);

      if (isNew) {
        const created = await createLot(request as CreateLotRequest);
        await fetchLots();
        setSelectedLot(created);
        setIsNew(false);
      } else if (selectedLot) {
        const updated = await updateLot(
          selectedLot.id,
          request as UpdateLotRequest
        );
        await fetchLots();
        setSelectedLot(updated);
      }
    } catch (e) {
      setError(e instanceof Error ? e.message : "保存に失敗しました");
    } finally {
      setIsSaving(false);
    }
  };

  return (
    <div className="p-6">
      <h1 className="mb-6 text-2xl font-bold text-gray-900">ロット管理</h1>

      {/* エラー表示 */}
      {error && (
        <div className="mb-4 rounded-lg border border-red-200 bg-red-50 p-3 text-sm text-red-700">
          {error}
        </div>
      )}

      {/* 2カラムレイアウト */}
      <div className="grid grid-cols-2 gap-6">
        {/* 左: ロット一覧 */}
        <div className="rounded-lg border border-gray-200 bg-white p-4">
          {isLoading ? (
            <p className="text-sm text-gray-500">読み込み中...</p>
          ) : (
            <LotList
              lots={lots}
              selectedLotId={selectedLot?.id ?? null}
              onSelect={handleSelect}
              onNewLot={handleNewLot}
            />
          )}
        </div>

        {/* 右: ロット詳細/編集 */}
        <div>
          {isNew || selectedLot ? (
            <LotForm
              lot={selectedLot}
              isNew={isNew}
              onSave={handleSave}
              onCancel={handleCancel}
              isSaving={isSaving}
            />
          ) : (
            <div className="flex h-full items-center justify-center rounded-lg border border-dashed border-gray-300 bg-gray-50 p-8">
              <p className="text-center text-gray-500">
                ロットを選択するか、
                <br />
                新規作成してください
              </p>
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
