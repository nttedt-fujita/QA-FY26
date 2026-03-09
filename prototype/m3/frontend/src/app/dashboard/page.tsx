"use client";

import { useState, useEffect } from "react";
import {
  getDashboardSummary,
  getDashboardMonthly,
  getDashboardTopDefects,
  getDashboardItemStats,
  getDashboardRecent,
  getDashboardSuppliers,
  SummaryStats,
  MonthlyStats,
  PartDefectRate,
  ItemStats,
  RecentRecord,
  SupplierDefectRate,
} from "@/lib/api";

export default function DashboardPage() {
  const [summary, setSummary] = useState<SummaryStats | null>(null);
  const [monthly, setMonthly] = useState<MonthlyStats[]>([]);
  const [topDefects, setTopDefects] = useState<PartDefectRate[]>([]);
  const [itemStats, setItemStats] = useState<ItemStats[]>([]);
  const [recentRecords, setRecentRecords] = useState<RecentRecord[]>([]);
  const [supplierDefects, setSupplierDefects] = useState<SupplierDefectRate[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  // データ取得
  const fetchData = async () => {
    try {
      setLoading(true);
      const [
        summaryData,
        monthlyData,
        defectsData,
        itemData,
        recentData,
        supplierData,
      ] = await Promise.all([
        getDashboardSummary(),
        getDashboardMonthly(6),
        getDashboardTopDefects(5, 3),
        getDashboardItemStats(),
        getDashboardRecent(5),
        getDashboardSuppliers(),
      ]);
      setSummary(summaryData);
      setMonthly(monthlyData);
      setTopDefects(defectsData);
      setItemStats(itemData);
      setRecentRecords(recentData);
      setSupplierDefects(supplierData);
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

  // 月別グラフの最大値（バー表示用）
  const maxTotal = Math.max(...monthly.map((m) => m.total), 1);

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

  return (
    <div className="min-h-screen bg-zinc-50 p-8">
      <main className="max-w-6xl mx-auto">
        <h1 className="text-2xl font-bold mb-6">ダッシュボード</h1>

        {/* エラー表示 */}
        {error && (
          <div className="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded mb-4">
            {error}
          </div>
        )}

        {loading ? (
          <div className="text-center text-gray-500 py-8">読み込み中...</div>
        ) : (
          <>
            {/* KPIカード */}
            <section className="grid grid-cols-4 gap-4 mb-6">
              <div className="bg-white rounded-lg shadow p-6">
                <div className="text-sm text-gray-500 mb-1">総検査数</div>
                <div className="text-3xl font-bold text-zinc-800">
                  {summary?.total_inspections ?? 0}
                </div>
              </div>
              <div className="bg-white rounded-lg shadow p-6">
                <div className="text-sm text-gray-500 mb-1">総ロット数</div>
                <div className="text-3xl font-bold text-zinc-800">
                  {summary?.total_lots ?? 0}
                </div>
              </div>
              <div className="bg-white rounded-lg shadow p-6">
                <div className="text-sm text-gray-500 mb-1">合格率</div>
                <div className="text-3xl font-bold text-green-600">
                  {summary?.pass_rate.toFixed(1) ?? 0}%
                </div>
              </div>
              <div className="bg-white rounded-lg shadow p-6">
                <div className="text-sm text-gray-500 mb-1">平均工数</div>
                <div className="text-3xl font-bold text-zinc-800">
                  {summary?.avg_work_time.toFixed(1) ?? 0}
                  <span className="text-sm font-normal text-gray-500 ml-1">分</span>
                </div>
              </div>
            </section>

            {/* 直近の検査記録 */}
            <section className="bg-white rounded-lg shadow p-6 mb-6">
              <h2 className="text-lg font-semibold mb-4">直近の検査記録</h2>
              {recentRecords.length === 0 ? (
                <div className="text-gray-500 text-center py-4">
                  検査記録がありません
                </div>
              ) : (
                <div className="overflow-x-auto">
                  <table className="w-full text-sm">
                    <thead>
                      <tr className="border-b bg-gray-50">
                        <th className="text-left p-2">検査日</th>
                        <th className="text-left p-2">ロットID</th>
                        <th className="text-left p-2">部品</th>
                        <th className="text-left p-2">検査項目</th>
                        <th className="text-center p-2">結果</th>
                      </tr>
                    </thead>
                    <tbody>
                      {recentRecords.map((r) => (
                        <tr key={r.record_id} className="border-b">
                          <td className="p-2">
                            {new Date(r.inspection_date).toLocaleDateString("ja-JP")}
                          </td>
                          <td className="p-2 font-mono text-xs">{r.lot_id}</td>
                          <td className="p-2">{r.part_name}</td>
                          <td className="p-2">{r.item_name}</td>
                          <td className="p-2 text-center">
                            <span
                              className={`px-2 py-1 rounded text-xs font-medium ${getResultBadge(
                                r.result
                              )}`}
                            >
                              {r.result}
                            </span>
                          </td>
                        </tr>
                      ))}
                    </tbody>
                  </table>
                </div>
              )}
            </section>

            <div className="grid grid-cols-2 gap-6 mb-6">
              {/* 月別グラフ */}
              <section className="bg-white rounded-lg shadow p-6">
                <h2 className="text-lg font-semibold mb-4">月別検査件数（過去6ヶ月）</h2>
                {monthly.length === 0 ? (
                  <div className="text-gray-500 text-center py-8">
                    データがありません
                  </div>
                ) : (
                  <div className="space-y-3">
                    {monthly
                      .slice()
                      .reverse()
                      .map((m) => (
                        <div key={m.month} className="flex items-center gap-3">
                          <div className="w-16 text-sm text-gray-600">{m.month}</div>
                          <div className="flex-1">
                            <div className="h-6 bg-gray-100 rounded overflow-hidden flex">
                              <div
                                className="bg-green-500 h-full"
                                style={{
                                  width: `${(m.pass_qty / maxTotal) * 100}%`,
                                }}
                                title={`合格: ${m.pass_qty}`}
                              />
                              <div
                                className="bg-red-500 h-full"
                                style={{
                                  width: `${(m.fail_qty / maxTotal) * 100}%`,
                                }}
                                title={`不合格: ${m.fail_qty}`}
                              />
                              <div
                                className="bg-gray-400 h-full"
                                style={{
                                  width: `${(m.skip_qty / maxTotal) * 100}%`,
                                }}
                                title={`不問: ${m.skip_qty}`}
                              />
                            </div>
                          </div>
                          <div className="w-12 text-right text-sm font-medium">
                            {m.total}
                          </div>
                        </div>
                      ))}
                    <div className="flex justify-center gap-6 mt-4 text-xs">
                      <span className="flex items-center gap-1">
                        <span className="w-3 h-3 bg-green-500 rounded" /> 合格
                      </span>
                      <span className="flex items-center gap-1">
                        <span className="w-3 h-3 bg-red-500 rounded" /> 不合格
                      </span>
                      <span className="flex items-center gap-1">
                        <span className="w-3 h-3 bg-gray-400 rounded" /> 不問
                      </span>
                    </div>
                  </div>
                )}
              </section>

              {/* 検査項目別件数 */}
              <section className="bg-white rounded-lg shadow p-6">
                <h2 className="text-lg font-semibold mb-4">検査項目別件数</h2>
                {itemStats.length === 0 ? (
                  <div className="text-gray-500 text-center py-8">
                    データがありません
                  </div>
                ) : (
                  <div className="overflow-x-auto">
                    <table className="w-full text-sm">
                      <thead>
                        <tr className="border-b bg-gray-50">
                          <th className="text-left p-2">検査項目</th>
                          <th className="text-right p-2">件数</th>
                          <th className="text-right p-2">合格</th>
                          <th className="text-right p-2">不合格</th>
                          <th className="text-right p-2">合格率</th>
                        </tr>
                      </thead>
                      <tbody>
                        {itemStats.map((s) => (
                          <tr key={s.item_id} className="border-b">
                            <td className="p-2">{s.item_name}</td>
                            <td className="p-2 text-right font-medium">{s.total}</td>
                            <td className="p-2 text-right text-green-600">{s.pass_qty}</td>
                            <td className="p-2 text-right text-red-600">{s.fail_qty}</td>
                            <td className="p-2 text-right">{s.pass_rate.toFixed(1)}%</td>
                          </tr>
                        ))}
                      </tbody>
                    </table>
                  </div>
                )}
              </section>
            </div>

            <div className="grid grid-cols-2 gap-6">
              {/* 不良トップ5 */}
              <section className="bg-white rounded-lg shadow p-6">
                <h2 className="text-lg font-semibold mb-4">
                  部品別不良トップ5（過去3ヶ月）
                </h2>
                {topDefects.length === 0 ? (
                  <div className="text-gray-500 text-center py-8">
                    不良データがありません
                  </div>
                ) : (
                  <div className="overflow-x-auto">
                    <table className="w-full text-sm">
                      <thead>
                        <tr className="border-b bg-gray-50">
                          <th className="text-left p-2">#</th>
                          <th className="text-left p-2">部品</th>
                          <th className="text-right p-2">検査数</th>
                          <th className="text-right p-2">不良数</th>
                          <th className="text-right p-2">不良率</th>
                        </tr>
                      </thead>
                      <tbody>
                        {topDefects.map((d, i) => (
                          <tr key={d.part_id} className="border-b">
                            <td className="p-2 text-gray-500">{i + 1}</td>
                            <td className="p-2">{d.part_name}</td>
                            <td className="p-2 text-right">{d.total_qty}</td>
                            <td className="p-2 text-right text-red-600 font-medium">
                              {d.defect_qty}
                            </td>
                            <td className="p-2 text-right">
                              {d.defect_rate.toFixed(1)}%
                            </td>
                          </tr>
                        ))}
                      </tbody>
                    </table>
                  </div>
                )}
              </section>

              {/* サプライヤー別不良率 */}
              <section className="bg-white rounded-lg shadow p-6">
                <h2 className="text-lg font-semibold mb-4">サプライヤー別実績</h2>
                {supplierDefects.length === 0 ? (
                  <div className="text-gray-500 text-center py-8">
                    データがありません
                  </div>
                ) : (
                  <div className="overflow-x-auto">
                    <table className="w-full text-sm">
                      <thead>
                        <tr className="border-b bg-gray-50">
                          <th className="text-left p-2">サプライヤー</th>
                          <th className="text-right p-2">検査数</th>
                          <th className="text-right p-2">不良数</th>
                          <th className="text-right p-2">不良率</th>
                        </tr>
                      </thead>
                      <tbody>
                        {supplierDefects.map((s) => (
                          <tr key={s.supplier_id} className="border-b">
                            <td className="p-2">{s.supplier_name}</td>
                            <td className="p-2 text-right">{s.total_qty}</td>
                            <td className="p-2 text-right text-red-600 font-medium">
                              {s.defect_qty}
                            </td>
                            <td className="p-2 text-right">
                              {s.defect_rate.toFixed(1)}%
                            </td>
                          </tr>
                        ))}
                      </tbody>
                    </table>
                  </div>
                )}
              </section>
            </div>
          </>
        )}
      </main>
    </div>
  );
}
