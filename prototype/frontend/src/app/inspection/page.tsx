"use client";

import { useState, useEffect, useCallback } from "react";
import {
  LotForInspection,
  InspectionItem,
  Worker,
} from "@/types/inspection";
import {
  getLotsForInspection,
  getInspectionItems,
  getWorkers,
  startInspectionSession,
  addCount,
  undoCount,
  finishInspectionSession,
} from "@/lib/api";

// 検査セッションの状態
interface SessionState {
  sessionId: string;
  lotId: string;
  lotName: string;
  itemName: string;
  okCount: number;
  ngCount: number;
  skipCount: number;
  startedAt: Date;
}

export default function InspectionPage() {
  // マスタデータ
  const [lots, setLots] = useState<LotForInspection[]>([]);
  const [inspectionItems, setInspectionItems] = useState<InspectionItem[]>([]);
  const [workers, setWorkers] = useState<Worker[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  // フォーム状態
  const [selectedLotId, setSelectedLotId] = useState("");
  const [selectedItemId, setSelectedItemId] = useState("");
  const [selectedWorkerId, setSelectedWorkerId] = useState("");

  // セッション状態
  const [session, setSession] = useState<SessionState | null>(null);
  const [submitting, setSubmitting] = useState(false);

  // マスタデータ取得
  useEffect(() => {
    const fetchMasterData = async () => {
      try {
        setLoading(true);
        const [lotsData, itemsData, workersData] = await Promise.all([
          getLotsForInspection(),
          getInspectionItems(),
          getWorkers(),
        ]);
        setLots(lotsData);
        setInspectionItems(itemsData);
        setWorkers(workersData);
        setError(null);
      } catch (e) {
        setError(e instanceof Error ? e.message : "データ取得に失敗しました");
      } finally {
        setLoading(false);
      }
    };
    fetchMasterData();
  }, []);

  // 検査開始
  const handleStart = async () => {
    if (!selectedLotId || !selectedItemId || !selectedWorkerId) {
      setError("ロット、検査項目、作業者を選択してください");
      return;
    }
    try {
      setSubmitting(true);
      setError(null);
      const res = await startInspectionSession({
        lot_id: selectedLotId,
        item_id: selectedItemId,
        worker_id: selectedWorkerId,
      });
      const lot = lots.find((l) => l.lot_id === selectedLotId);
      const item = inspectionItems.find((i) => i.item_id === selectedItemId);
      setSession({
        sessionId: res.session_id,
        lotId: selectedLotId,
        lotName: lot?.part_name ?? selectedLotId,
        itemName: item?.name ?? selectedItemId,
        okCount: 0,
        ngCount: 0,
        skipCount: 0,
        startedAt: new Date(),
      });
    } catch (e) {
      setError(e instanceof Error ? e.message : "検査開始に失敗しました");
    } finally {
      setSubmitting(false);
    }
  };

  // カウント追加
  const handleCount = useCallback(
    async (result: "ok" | "ng" | "skip") => {
      if (!session) return;
      try {
        setError(null);
        const res = await addCount(session.sessionId, { result });
        setSession({
          ...session,
          okCount: res.ok_count,
          ngCount: res.ng_count,
          skipCount: res.skip_count,
        });
      } catch (e) {
        setError(e instanceof Error ? e.message : "カウント追加に失敗しました");
      }
    },
    [session]
  );

  // 取り消し
  const handleUndo = async () => {
    if (!session) return;
    try {
      setError(null);
      const res = await undoCount(session.sessionId);
      setSession({
        ...session,
        okCount: res.ok_count,
        ngCount: res.ng_count,
        skipCount: res.skip_count,
      });
    } catch (e) {
      setError(e instanceof Error ? e.message : "取り消しに失敗しました");
    }
  };

  // 検査終了
  const handleFinish = async () => {
    if (!session) return;
    try {
      setSubmitting(true);
      setError(null);
      const res = await finishInspectionSession(session.sessionId);
      // セッションをリセット
      setSession(null);
      setSelectedLotId("");
      setSelectedItemId("");
      // 完了メッセージ（簡易的にalert）
      alert(
        `検査完了\n工数: ${res.man_hours}分\n記録ID: ${res.inspection_record_id}`
      );
    } catch (e) {
      setError(e instanceof Error ? e.message : "検査終了に失敗しました");
    } finally {
      setSubmitting(false);
    }
  };

  // キーボードショートカット
  useEffect(() => {
    if (!session) return;
    const handleKeyDown = (e: KeyboardEvent) => {
      // フォーム要素にフォーカスがある場合は無視
      if (
        document.activeElement?.tagName === "INPUT" ||
        document.activeElement?.tagName === "SELECT"
      ) {
        return;
      }
      switch (e.key.toLowerCase()) {
        case "o":
        case "1":
          handleCount("ok");
          break;
        case "n":
        case "2":
          handleCount("ng");
          break;
        case "s":
        case "3":
          handleCount("skip");
          break;
        case "u":
        case "z":
          handleUndo();
          break;
      }
    };
    window.addEventListener("keydown", handleKeyDown);
    return () => window.removeEventListener("keydown", handleKeyDown);
  }, [session, handleCount]);

  if (loading) {
    return (
      <div className="min-h-screen bg-zinc-50 flex items-center justify-center">
        <p className="text-gray-500">読み込み中...</p>
      </div>
    );
  }

  return (
    <div className="min-h-screen bg-zinc-50">
      <main className="max-w-2xl mx-auto p-6">
        {/* エラー表示 */}
        {error && (
          <div className="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded mb-4">
            {error}
          </div>
        )}

        {/* 検査開始前: 選択フォーム */}
        {!session && (
          <section className="bg-white rounded-lg shadow p-6">
            <h2 className="text-lg font-semibold mb-4">検査対象を選択</h2>
            <div className="space-y-4">
              {/* ロット選択 */}
              <div>
                <label className="block text-sm font-medium mb-1">
                  ロット <span className="text-red-500">*</span>
                </label>
                <select
                  value={selectedLotId}
                  onChange={(e) => setSelectedLotId(e.target.value)}
                  className="w-full border rounded px-3 py-2 bg-gray-50"
                >
                  <option value="">-- 選択してください --</option>
                  {lots.map((lot) => (
                    <option key={lot.lot_id} value={lot.lot_id}>
                      {lot.lot_id} ({lot.part_name} {lot.quantity}個)
                    </option>
                  ))}
                </select>
              </div>

              {/* 検査項目選択 */}
              <div>
                <label className="block text-sm font-medium mb-1">
                  検査項目 <span className="text-red-500">*</span>
                </label>
                <select
                  value={selectedItemId}
                  onChange={(e) => setSelectedItemId(e.target.value)}
                  className="w-full border rounded px-3 py-2 bg-gray-50"
                >
                  <option value="">-- 選択してください --</option>
                  {inspectionItems.map((item) => (
                    <option key={item.item_id} value={item.item_id}>
                      {item.name}
                    </option>
                  ))}
                </select>
              </div>

              {/* 作業者選択 */}
              <div>
                <label className="block text-sm font-medium mb-1">
                  作業者 <span className="text-red-500">*</span>
                </label>
                <select
                  value={selectedWorkerId}
                  onChange={(e) => setSelectedWorkerId(e.target.value)}
                  className="w-full border rounded px-3 py-2 bg-gray-50"
                >
                  <option value="">-- 選択してください --</option>
                  {workers.map((worker) => (
                    <option key={worker.worker_id} value={worker.worker_id}>
                      {worker.name}
                    </option>
                  ))}
                </select>
              </div>

              {/* 検査開始ボタン */}
              <button
                onClick={handleStart}
                disabled={submitting}
                className="w-full bg-green-600 text-white py-3 rounded-lg font-bold text-lg hover:bg-green-700 disabled:opacity-50"
              >
                {submitting ? "開始中..." : "検査開始"}
              </button>
            </div>
          </section>
        )}

        {/* 検査中: カウンター画面 */}
        {session && (
          <div className="space-y-4">
            {/* 検査中ラベル */}
            <div className="bg-green-100 border border-green-400 text-green-800 px-4 py-2 rounded-lg text-center font-semibold">
              検査中: {session.lotName} / {session.itemName}
            </div>

            {/* カウンター表示 */}
            <div className="bg-white rounded-lg shadow p-4">
              <div className="grid grid-cols-3 gap-4">
                {/* 合格カウント */}
                <div className="bg-green-50 border-2 border-green-400 rounded-lg p-4 text-center">
                  <div className="text-sm text-green-700 font-semibold">
                    合格
                  </div>
                  <div className="text-4xl font-bold text-green-600">
                    {session.okCount}
                  </div>
                </div>
                {/* 不合格カウント */}
                <div className="bg-red-50 border-2 border-red-400 rounded-lg p-4 text-center">
                  <div className="text-sm text-red-700 font-semibold">
                    不合格
                  </div>
                  <div className="text-4xl font-bold text-red-600">
                    {session.ngCount}
                  </div>
                </div>
                {/* 不問カウント */}
                <div className="bg-yellow-50 border-2 border-yellow-400 rounded-lg p-4 text-center">
                  <div className="text-sm text-yellow-700 font-semibold">
                    不問
                  </div>
                  <div className="text-4xl font-bold text-yellow-600">
                    {session.skipCount}
                  </div>
                </div>
              </div>
            </div>

            {/* 入力ボタン（大きく押しやすく） */}
            <div className="grid grid-cols-3 gap-4">
              <button
                onClick={() => handleCount("ok")}
                className="bg-green-500 hover:bg-green-600 text-white py-8 rounded-lg font-bold text-2xl shadow-lg active:scale-95 transition-transform"
              >
                合格 ✓
              </button>
              <button
                onClick={() => handleCount("ng")}
                className="bg-red-500 hover:bg-red-600 text-white py-8 rounded-lg font-bold text-2xl shadow-lg active:scale-95 transition-transform"
              >
                不合格 ✗
              </button>
              <button
                onClick={() => handleCount("skip")}
                className="bg-yellow-500 hover:bg-yellow-600 text-white py-8 rounded-lg font-bold text-2xl shadow-lg active:scale-95 transition-transform"
              >
                不問
              </button>
            </div>

            {/* 操作ボタン */}
            <div className="flex gap-4">
              <button
                onClick={handleUndo}
                className="flex-1 bg-gray-200 hover:bg-gray-300 text-gray-700 py-3 rounded-lg font-semibold"
              >
                ↩ 取り消し
              </button>
              <button
                onClick={handleFinish}
                disabled={submitting}
                className="flex-1 bg-blue-500 hover:bg-blue-600 text-white py-3 rounded-lg font-bold disabled:opacity-50"
              >
                {submitting ? "保存中..." : "検査終了 →"}
              </button>
            </div>

            {/* キーボードショートカットヘルプ */}
            <div className="text-center text-xs text-gray-400">
              キーボード: O/1=合格, N/2=不合格, S/3=不問, U/Z=取り消し
            </div>
          </div>
        )}
      </main>
    </div>
  );
}
