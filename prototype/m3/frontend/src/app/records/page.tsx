"use client";

import { useState, useEffect } from "react";
import {
  getInspectionRecords,
  getParts,
  InspectionRecordWithDetails,
  ListFilter,
} from "@/lib/api";
import { Part } from "@/types/inspection";

export default function RecordsPage() {
  const [records, setRecords] = useState<InspectionRecordWithDetails[]>([]);
  const [parts, setParts] = useState<Part[]>([]);
  const [total, setTotal] = useState(0);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  // フィルター状態
  const [filter, setFilter] = useState<ListFilter>({
    limit: 20,
    offset: 0,
  });

  // データ取得
  const fetchData = async () => {
    try {
      setLoading(true);
      const [recordsData, partsData] = await Promise.all([
        getInspectionRecords(filter),
        getParts(),
      ]);
      setRecords(recordsData.records ?? []);
      setTotal(recordsData.total);
      setParts(partsData);
      setError(null);
    } catch (e) {
      setError(e instanceof Error ? e.message : "エラーが発生しました");
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchData();
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [filter]);

  // ページネーション
  const currentPage = Math.floor((filter.offset ?? 0) / (filter.limit ?? 20)) + 1;
  const totalPages = Math.ceil(total / (filter.limit ?? 20));

  const goToPage = (page: number) => {
    const newOffset = (page - 1) * (filter.limit ?? 20);
    setFilter({ ...filter, offset: newOffset });
  };

  // 結果のバッジ色
  const getResultBadge = (result: string) => {
    switch (result) {
      case "合格":
        return "bg-green-100 text-green-800";
      case "不合格":
        return "bg-red-100 text-red-800";
      case "不問":
        return "bg-gray-100 text-gray-800";
      default:
        return "bg-gray-100 text-gray-600";
    }
  };

  // CSVエクスポート
  const exportCSV = () => {
    if (records.length === 0) return;

    const headers = [
      "記録ID",
      "ロットID",
      "部品名",
      "検査項目",
      "作業者",
      "検査日",
      "結果",
      "不良数",
      "工数(分)",
      "備考",
    ];
    const rows = records.map((r) => [
      r.record_id,
      r.lot_id,
      r.part_name,
      r.item_name,
      r.worker_name ?? "",
      r.inspection_date.split("T")[0],
      r.result,
      String(r.defect_qty),
      r.work_time_min?.toFixed(1) ?? "",
      r.note ?? "",
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
    link.download = `inspection_records_${new Date().toISOString().split("T")[0]}.csv`;
    link.click();
    URL.revokeObjectURL(url);
  };

  return (
    <div className="min-h-screen bg-zinc-50 p-8">
      <main className="max-w-6xl mx-auto">
        <div className="flex justify-between items-center mb-6">
          <h1 className="text-2xl font-bold">検査記録一覧</h1>
          <button
            onClick={exportCSV}
            disabled={records.length === 0}
            className="bg-zinc-600 text-white px-4 py-2 rounded hover:bg-zinc-700 disabled:opacity-50 text-sm"
          >
            📥 CSVエクスポート
          </button>
        </div>

        {/* フィルター */}
        <section className="bg-white rounded-lg shadow p-4 mb-6">
          <div className="grid grid-cols-5 gap-4">
            <div>
              <label className="block text-xs font-medium mb-1 text-gray-600">
                日付（From）
              </label>
              <input
                type="date"
                value={filter.date_from ?? ""}
                onChange={(e) =>
                  setFilter({
                    ...filter,
                    date_from: e.target.value || undefined,
                    offset: 0,
                  })
                }
                className="w-full border rounded px-2 py-1 text-sm"
              />
            </div>
            <div>
              <label className="block text-xs font-medium mb-1 text-gray-600">
                日付（To）
              </label>
              <input
                type="date"
                value={filter.date_to ?? ""}
                onChange={(e) =>
                  setFilter({
                    ...filter,
                    date_to: e.target.value || undefined,
                    offset: 0,
                  })
                }
                className="w-full border rounded px-2 py-1 text-sm"
              />
            </div>
            <div>
              <label className="block text-xs font-medium mb-1 text-gray-600">
                部品
              </label>
              <select
                value={filter.part_id ?? ""}
                onChange={(e) =>
                  setFilter({
                    ...filter,
                    part_id: e.target.value || undefined,
                    offset: 0,
                  })
                }
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
            <div>
              <label className="block text-xs font-medium mb-1 text-gray-600">
                結果
              </label>
              <select
                value={filter.result ?? ""}
                onChange={(e) =>
                  setFilter({
                    ...filter,
                    result: e.target.value || undefined,
                    offset: 0,
                  })
                }
                className="w-full border rounded px-2 py-1 text-sm bg-white"
              >
                <option value="">すべて</option>
                <option value="合格">合格</option>
                <option value="不合格">不合格</option>
                <option value="不問">不問</option>
              </select>
            </div>
            <div className="flex items-end">
              <button
                onClick={() =>
                  setFilter({ limit: 20, offset: 0 })
                }
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
          ) : records.length === 0 ? (
            <div className="p-8 text-center text-gray-500">
              検査記録がありません
            </div>
          ) : (
            <>
              <div className="overflow-x-auto">
                <table className="w-full text-sm">
                  <thead>
                    <tr className="border-b bg-gray-50">
                      <th className="text-left p-3">検査日</th>
                      <th className="text-left p-3">ロットID</th>
                      <th className="text-left p-3">部品</th>
                      <th className="text-left p-3">検査項目</th>
                      <th className="text-left p-3">作業者</th>
                      <th className="text-center p-3">結果</th>
                      <th className="text-right p-3">不良数</th>
                      <th className="text-right p-3">工数(分)</th>
                      <th className="text-left p-3">備考</th>
                    </tr>
                  </thead>
                  <tbody>
                    {records.map((r) => (
                      <tr
                        key={r.record_id}
                        className="border-b hover:bg-gray-50"
                      >
                        <td className="p-3">
                          {new Date(r.inspection_date).toLocaleDateString("ja-JP")}
                        </td>
                        <td className="p-3 font-mono text-xs">{r.lot_id}</td>
                        <td className="p-3">{r.part_name}</td>
                        <td className="p-3">{r.item_name}</td>
                        <td className="p-3">{r.worker_name ?? "-"}</td>
                        <td className="p-3 text-center">
                          <span
                            className={`px-2 py-1 rounded text-xs font-medium ${getResultBadge(
                              r.result
                            )}`}
                          >
                            {r.result}
                          </span>
                        </td>
                        <td className="p-3 text-right">
                          {r.defect_qty > 0 ? (
                            <span className="text-red-600 font-medium">
                              {r.defect_qty}
                            </span>
                          ) : (
                            "-"
                          )}
                        </td>
                        <td className="p-3 text-right">
                          {r.work_time_min?.toFixed(1) ?? "-"}
                        </td>
                        <td className="p-3 text-gray-500 text-xs truncate max-w-[150px]">
                          {r.note ?? "-"}
                        </td>
                      </tr>
                    ))}
                  </tbody>
                </table>
              </div>

              {/* ページネーション */}
              <div className="flex justify-between items-center p-4 border-t bg-gray-50">
                <div className="text-sm text-gray-600">
                  全 {total} 件中 {(filter.offset ?? 0) + 1} -{" "}
                  {Math.min((filter.offset ?? 0) + (filter.limit ?? 20), total)} 件を表示
                </div>
                <div className="flex gap-2">
                  <button
                    onClick={() => goToPage(currentPage - 1)}
                    disabled={currentPage === 1}
                    className="px-3 py-1 border rounded text-sm disabled:opacity-50 disabled:cursor-not-allowed hover:bg-gray-100"
                  >
                    前へ
                  </button>
                  <span className="px-3 py-1 text-sm">
                    {currentPage} / {totalPages || 1}
                  </span>
                  <button
                    onClick={() => goToPage(currentPage + 1)}
                    disabled={currentPage >= totalPages}
                    className="px-3 py-1 border rounded text-sm disabled:opacity-50 disabled:cursor-not-allowed hover:bg-gray-100"
                  >
                    次へ
                  </button>
                </div>
              </div>
            </>
          )}
        </section>
      </main>
    </div>
  );
}
