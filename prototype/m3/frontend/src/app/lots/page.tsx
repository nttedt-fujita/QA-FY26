"use client";

import { useState, useEffect } from "react";
import { getLots, getParts } from "@/lib/api";
import { Lot } from "@/types/lot";
import { Part } from "@/types/inspection";
import Link from "next/link";

// 部品名付きロット型
interface LotWithPartName extends Lot {
  part_name: string;
}

export default function LotsPage() {
  const [lots, setLots] = useState<LotWithPartName[]>([]);
  const [parts, setParts] = useState<Part[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  // フィルター状態
  const [filterPartId, setFilterPartId] = useState<string>("");

  // データ取得
  const fetchData = async () => {
    try {
      setLoading(true);
      const [lotsData, partsData] = await Promise.all([getLots(), getParts()]);
      setParts(partsData);

      // 部品名をマッピング
      const partsMap = new Map<string, Part>(
        partsData.map((p) => [p.part_id, p])
      );
      const lotsWithNames: LotWithPartName[] = lotsData.map((lot) => {
        const part = partsMap.get(lot.part_id);
        return {
          ...lot,
          part_name: part?.name ?? lot.part_id,
        };
      });
      setLots(lotsWithNames);
      setError(null);
    } catch (e) {
      setError(e instanceof Error ? e.message : "エラーが発生しました");
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchData();
  }, []);

  // フィルター適用
  const filteredLots = lots.filter((lot) => {
    if (filterPartId && lot.part_id !== filterPartId) return false;
    return true;
  });

  // CSVエクスポート
  const exportCSV = () => {
    if (filteredLots.length === 0) return;

    const headers = [
      "ロットID",
      "部品名",
      "数量",
      "入荷日",
      "PO番号",
      "FWバージョン",
      "HWバージョン",
      "登録日",
    ];
    const rows = filteredLots.map((lot) => [
      lot.lot_id,
      lot.part_name,
      String(lot.quantity),
      lot.arrival_date ? lot.arrival_date.split("T")[0] : "",
      lot.po_number ?? "",
      lot.fw_version ?? "",
      lot.hw_version ?? "",
      lot.created_at.split("T")[0],
    ]);

    const csvContent = [
      headers.join(","),
      ...rows.map((row) => row.map((cell) => `"${cell}"`).join(",")),
    ].join("\n");

    const blob = new Blob(["\uFEFF" + csvContent], {
      type: "text/csv;charset=utf-8;",
    });
    const url = URL.createObjectURL(blob);
    const link = document.createElement("a");
    link.href = url;
    link.download = `lots_${new Date().toISOString().split("T")[0]}.csv`;
    link.click();
    URL.revokeObjectURL(url);
  };

  return (
    <div className="min-h-screen bg-zinc-50 p-8">
      <main className="max-w-6xl mx-auto">
        <div className="flex justify-between items-center mb-6">
          <h1 className="text-2xl font-bold">ロット一覧</h1>
          <div className="flex gap-2">
            <Link
              href="/"
              className="bg-green-600 text-white px-4 py-2 rounded hover:bg-green-700 text-sm"
            >
              + 新規登録
            </Link>
            <button
              onClick={exportCSV}
              disabled={filteredLots.length === 0}
              className="bg-zinc-600 text-white px-4 py-2 rounded hover:bg-zinc-700 disabled:opacity-50 text-sm"
            >
              📥 CSVエクスポート
            </button>
          </div>
        </div>

        {/* フィルター */}
        <section className="bg-white rounded-lg shadow p-4 mb-6">
          <div className="grid grid-cols-4 gap-4">
            <div>
              <label className="block text-xs font-medium mb-1 text-gray-600">
                部品
              </label>
              <select
                value={filterPartId}
                onChange={(e) => setFilterPartId(e.target.value)}
                className="w-full border rounded px-2 py-1 text-sm bg-white"
              >
                <option value="">すべて</option>
                {parts.map((p) => (
                  <option key={p.part_id} value={p.part_id}>
                    {p.name}
                  </option>
                ))}
              </select>
            </div>
            <div className="flex items-end">
              <button
                onClick={() => setFilterPartId("")}
                className="bg-zinc-200 text-zinc-700 px-4 py-1 rounded hover:bg-zinc-300 text-sm"
              >
                クリア
              </button>
            </div>
          </div>
        </section>

        {/* エラー表示 */}
        {error && (
          <div className="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded mb-4">
            {error}
          </div>
        )}

        {/* テーブル */}
        <section className="bg-white rounded-lg shadow overflow-hidden">
          {loading ? (
            <div className="p-8 text-center text-gray-500">読み込み中...</div>
          ) : filteredLots.length === 0 ? (
            <div className="p-8 text-center text-gray-500">
              ロットがありません
            </div>
          ) : (
            <div className="overflow-x-auto">
              <table className="w-full text-sm">
                <thead>
                  <tr className="border-b bg-gray-50">
                    <th className="text-left p-3">ロットID</th>
                    <th className="text-left p-3">部品名</th>
                    <th className="text-right p-3">数量</th>
                    <th className="text-left p-3">入荷日</th>
                    <th className="text-left p-3">PO番号</th>
                    <th className="text-left p-3">FW Ver</th>
                    <th className="text-left p-3">HW Ver</th>
                    <th className="text-left p-3">登録日</th>
                  </tr>
                </thead>
                <tbody>
                  {filteredLots.map((lot) => (
                    <tr key={lot.lot_id} className="border-b hover:bg-gray-50">
                      <td className="p-3 font-mono text-xs">{lot.lot_id}</td>
                      <td className="p-3">{lot.part_name}</td>
                      <td className="p-3 text-right">{lot.quantity}</td>
                      <td className="p-3">
                        {lot.arrival_date
                          ? new Date(lot.arrival_date).toLocaleDateString(
                              "ja-JP"
                            )
                          : "-"}
                      </td>
                      <td className="p-3">{lot.po_number ?? "-"}</td>
                      <td className="p-3 font-mono text-xs">
                        {lot.fw_version ?? "-"}
                      </td>
                      <td className="p-3 font-mono text-xs">
                        {lot.hw_version ?? "-"}
                      </td>
                      <td className="p-3">
                        {new Date(lot.created_at).toLocaleDateString("ja-JP")}
                      </td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>
          )}
        </section>

        {/* 件数表示 */}
        {!loading && filteredLots.length > 0 && (
          <div className="mt-4 text-sm text-gray-600">
            全 {filteredLots.length} 件
            {filterPartId && ` (フィルター適用中)`}
          </div>
        )}
      </main>
    </div>
  );
}
