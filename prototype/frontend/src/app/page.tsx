"use client";

import { useState, useEffect } from "react";
import { Lot, CreateLotRequest } from "@/types/lot";
import { getLots, createLot } from "@/lib/api";

export default function Home() {
  const [lots, setLots] = useState<Lot[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  // フォーム状態
  const [formData, setFormData] = useState<CreateLotRequest>({
    part_id: "",
    quantity: 1,
  });
  const [submitting, setSubmitting] = useState(false);

  // ロット一覧を取得
  const fetchLots = async () => {
    try {
      setLoading(true);
      const data = await getLots();
      setLots(data);
      setError(null);
    } catch (e) {
      setError(e instanceof Error ? e.message : "エラーが発生しました");
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchLots();
  }, []);

  // ロット登録
  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!formData.part_id || formData.quantity < 1) {
      setError("部品IDと数量は必須です");
      return;
    }

    try {
      setSubmitting(true);
      setError(null);
      await createLot(formData);
      // フォームリセット
      setFormData({ part_id: "", quantity: 1 });
      // 一覧を再取得
      await fetchLots();
    } catch (e) {
      setError(e instanceof Error ? e.message : "登録に失敗しました");
    } finally {
      setSubmitting(false);
    }
  };

  return (
    <div className="min-h-screen bg-zinc-50 p-8">
      <main className="max-w-4xl mx-auto">
        <h1 className="text-2xl font-bold mb-8">受入検査DB - ロット管理</h1>

        {/* エラー表示 */}
        {error && (
          <div className="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded mb-4">
            {error}
          </div>
        )}

        {/* ロット登録フォーム */}
        <section className="bg-white rounded-lg shadow p-6 mb-8">
          <h2 className="text-lg font-semibold mb-4">ロット登録</h2>
          <form onSubmit={handleSubmit} className="grid grid-cols-2 gap-4">
            <div>
              <label className="block text-sm font-medium mb-1">
                部品ID <span className="text-red-500">*</span>
              </label>
              <input
                type="text"
                value={formData.part_id}
                onChange={(e) =>
                  setFormData({ ...formData, part_id: e.target.value })
                }
                className="w-full border rounded px-3 py-2"
                placeholder="PART-001"
                required
              />
            </div>
            <div>
              <label className="block text-sm font-medium mb-1">
                数量 <span className="text-red-500">*</span>
              </label>
              <input
                type="number"
                value={formData.quantity}
                onChange={(e) =>
                  setFormData({ ...formData, quantity: parseInt(e.target.value) || 1 })
                }
                className="w-full border rounded px-3 py-2"
                min="1"
                required
              />
            </div>
            <div>
              <label className="block text-sm font-medium mb-1">発注番号</label>
              <input
                type="text"
                value={formData.po_number || ""}
                onChange={(e) =>
                  setFormData({ ...formData, po_number: e.target.value || undefined })
                }
                className="w-full border rounded px-3 py-2"
                placeholder="PO-2026-001"
              />
            </div>
            <div>
              <label className="block text-sm font-medium mb-1">入荷日</label>
              <input
                type="date"
                value={formData.arrival_date || ""}
                onChange={(e) =>
                  setFormData({ ...formData, arrival_date: e.target.value || undefined })
                }
                className="w-full border rounded px-3 py-2"
              />
            </div>
            <div>
              <label className="block text-sm font-medium mb-1">FWバージョン</label>
              <input
                type="text"
                value={formData.fw_version || ""}
                onChange={(e) =>
                  setFormData({ ...formData, fw_version: e.target.value || undefined })
                }
                className="w-full border rounded px-3 py-2"
                placeholder="v1.2.3"
              />
            </div>
            <div>
              <label className="block text-sm font-medium mb-1">HWバージョン</label>
              <input
                type="text"
                value={formData.hw_version || ""}
                onChange={(e) =>
                  setFormData({ ...formData, hw_version: e.target.value || undefined })
                }
                className="w-full border rounded px-3 py-2"
                placeholder="rev.A"
              />
            </div>
            <div className="col-span-2">
              <button
                type="submit"
                disabled={submitting}
                className="bg-blue-600 text-white px-6 py-2 rounded hover:bg-blue-700 disabled:opacity-50"
              >
                {submitting ? "登録中..." : "登録"}
              </button>
            </div>
          </form>
        </section>

        {/* ロット一覧 */}
        <section className="bg-white rounded-lg shadow p-6">
          <h2 className="text-lg font-semibold mb-4">ロット一覧</h2>
          {loading ? (
            <p className="text-gray-500">読み込み中...</p>
          ) : lots.length === 0 ? (
            <p className="text-gray-500">ロットがありません</p>
          ) : (
            <div className="overflow-x-auto">
              <table className="w-full text-sm">
                <thead>
                  <tr className="border-b bg-gray-50">
                    <th className="text-left p-2">ロットID</th>
                    <th className="text-left p-2">部品ID</th>
                    <th className="text-right p-2">数量</th>
                    <th className="text-left p-2">発注番号</th>
                    <th className="text-left p-2">入荷日</th>
                    <th className="text-left p-2">FW/HW</th>
                    <th className="text-left p-2">登録日時</th>
                  </tr>
                </thead>
                <tbody>
                  {lots.map((lot) => (
                    <tr key={lot.lot_id} className="border-b hover:bg-gray-50">
                      <td className="p-2 font-mono text-xs">{lot.lot_id}</td>
                      <td className="p-2">{lot.part_id}</td>
                      <td className="p-2 text-right">{lot.quantity}</td>
                      <td className="p-2">{lot.po_number || "-"}</td>
                      <td className="p-2">
                        {lot.arrival_date
                          ? new Date(lot.arrival_date).toLocaleDateString("ja-JP")
                          : "-"}
                      </td>
                      <td className="p-2 text-xs">
                        {lot.fw_version || "-"} / {lot.hw_version || "-"}
                      </td>
                      <td className="p-2 text-xs text-gray-500">
                        {new Date(lot.created_at).toLocaleString("ja-JP")}
                      </td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>
          )}
        </section>
      </main>
    </div>
  );
}
