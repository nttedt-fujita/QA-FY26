"use client";

import { useState, useEffect, useCallback } from "react";
import {
  Device,
  Lot,
  InspectionResponse,
  InspectionSummary,
  listDevices,
  listLots,
  runInspection,
  listInspections,
} from "@/lib/api";
import { InspectionResult } from "@/components/InspectionResult";
import { NavSigPanel } from "@/components/NavSigPanel";
import { MonSpanPanel } from "@/components/MonSpanPanel";
import { NavStatusPanel } from "@/components/NavStatusPanel";
import { SkyPlotPanel } from "@/components/SkyPlotPanel";

/**
 * 検査実行画面
 *
 * モックアップ: sessions/session92/ui-design-phase1.md の「3. 検査実行画面」
 */
export default function InspectionsPage() {
  // 状態
  const [lots, setLots] = useState<Lot[]>([]);
  const [devices, setDevices] = useState<Device[]>([]);
  const [selectedLotId, setSelectedLotId] = useState<number | null>(null);
  const [inspectionResult, setInspectionResult] =
    useState<InspectionResponse | null>(null);
  const [inspectionHistory, setInspectionHistory] = useState<
    InspectionSummary[]
  >([]);

  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  // 接続中の装置を取得
  const connectedDevice = devices.find((d) => d.status === "connected");

  // 選択中のロットを取得
  const selectedLot = lots.find((l) => l.id === selectedLotId);

  // データ取得
  const fetchData = useCallback(async () => {
    try {
      const [lotsRes, devicesRes, inspectionsRes] = await Promise.all([
        listLots(),
        listDevices(),
        listInspections(),
      ]);
      setLots(lotsRes.lots);
      setDevices(devicesRes.devices);
      setInspectionHistory(inspectionsRes.inspections);
    } catch (e) {
      setError(e instanceof Error ? e.message : "データの取得に失敗しました");
    }
  }, []);

  // 初回読み込み
  useEffect(() => {
    fetchData();
  }, [fetchData]);

  // 検査実行
  const handleRunInspection = async () => {
    if (!connectedDevice) {
      setError("装置が接続されていません");
      return;
    }

    setIsLoading(true);
    setError(null);
    setInspectionResult(null);

    try {
      const result = await runInspection({
        lot_id: selectedLotId ?? undefined,
      });
      setInspectionResult(result);
      // 履歴を更新
      const inspectionsRes = await listInspections();
      setInspectionHistory(inspectionsRes.inspections);
    } catch (e) {
      setError(e instanceof Error ? e.message : "検査に失敗しました");
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <div className="min-h-screen bg-gray-50">
      {/* メインコンテンツ */}
      <main className="container mx-auto px-4 py-6">
        {/* 現在の状態 */}
        <div className="mb-6 grid gap-4 md:grid-cols-2">
          {/* ロット選択 */}
          <div className="rounded border border-gray-200 bg-white p-4">
            <h3 className="mb-2 font-medium text-gray-700">ロット選択</h3>
            <select
              value={selectedLotId ?? ""}
              onChange={(e) =>
                setSelectedLotId(e.target.value ? Number(e.target.value) : null)
              }
              className="w-full rounded border border-gray-300 p-2"
            >
              <option value="">（ロットなし）</option>
              {lots.map((lot) => (
                <option key={lot.id} value={lot.id}>
                  {lot.lot_number}
                </option>
              ))}
            </select>
            {selectedLot && (
              <div className="mt-2 text-sm text-gray-600">
                <div>
                  出力レート: {selectedLot.expected_rate_ms ?? "-"} ms
                </div>
                <div>
                  ポートIN: {selectedLot.expected_port_in_proto ?? "-"}
                </div>
                <div>
                  ポートOUT: {selectedLot.expected_port_out_proto ?? "-"}
                </div>
              </div>
            )}
          </div>

          {/* 接続中の装置 */}
          <div className="rounded border border-gray-200 bg-white p-4">
            <h3 className="mb-2 font-medium text-gray-700">接続中の装置</h3>
            {connectedDevice ? (
              <div>
                <div className="font-mono text-sm">{connectedDevice.path}</div>
                <div className="text-sm text-gray-600">
                  ボーレート: {connectedDevice.baud_rate ?? "-"} bps
                </div>
                <div className="mt-1 inline-block rounded bg-green-100 px-2 py-1 text-sm text-green-800">
                  接続中
                </div>
              </div>
            ) : (
              <div className="text-gray-500">
                装置が接続されていません。
                <a
                  href="/devices"
                  className="ml-2 text-blue-600 hover:underline"
                >
                  装置画面へ
                </a>
              </div>
            )}
          </div>
        </div>

        {/* 検査開始ボタン */}
        <div className="mb-6">
          <button
            onClick={handleRunInspection}
            disabled={isLoading || !connectedDevice}
            className="rounded bg-[#2e75b6] px-8 py-3 text-lg font-medium text-white hover:bg-[#245d92] disabled:cursor-not-allowed disabled:opacity-50"
          >
            {isLoading ? "検査中..." : "検査開始"}
          </button>
        </div>

        {/* NAV-STATUS（Fix状態・TTFF）パネル */}
        <div className="mb-6">
          <NavStatusPanel enabled={!!connectedDevice} />
        </div>

        {/* スカイプロット + NAV-SIG 横並び */}
        <div className="mb-6 grid gap-4 md:grid-cols-2">
          {/* スカイプロット（NAV-SAT） */}
          <SkyPlotPanel enabled={!!connectedDevice} />
          {/* NAV-SIG（衛星信号） */}
          <NavSigPanel enabled={!!connectedDevice} />
        </div>

        {/* MON-SPAN（スペクトラム）パネル */}
        <div className="mb-6">
          <MonSpanPanel enabled={!!connectedDevice} />
        </div>

        {/* エラー表示 */}
        {error && (
          <div className="mb-6 rounded border border-red-200 bg-red-50 p-4 text-red-700">
            {error}
          </div>
        )}

        {/* 検査結果 */}
        {inspectionResult && (
          <div className="mb-6">
            <InspectionResult result={inspectionResult} />
          </div>
        )}

        {/* 検査履歴 */}
        <div className="rounded border border-gray-200 bg-white p-4">
          <h3 className="mb-4 text-lg font-medium text-gray-700">検査履歴</h3>
          {inspectionHistory.length === 0 ? (
            <div className="text-center text-gray-500">検査履歴がありません</div>
          ) : (
            <table className="w-full border-collapse">
              <thead>
                <tr className="bg-gray-100">
                  <th className="border p-2 text-left">ID</th>
                  <th className="border p-2 text-left">シリアル番号</th>
                  <th className="border p-2 text-left">検査日時</th>
                  <th className="border p-2 text-center">結果</th>
                </tr>
              </thead>
              <tbody>
                {inspectionHistory.map((inspection) => (
                  <tr key={inspection.id}>
                    <td className="border p-2">{inspection.id}</td>
                    <td className="border p-2 font-mono text-sm">
                      {inspection.serial_number || "-"}
                    </td>
                    <td className="border p-2">{inspection.inspected_at}</td>
                    <td
                      className={`border p-2 text-center font-semibold ${
                        inspection.overall_result === "Pass"
                          ? "text-green-600"
                          : inspection.overall_result === "Fail"
                          ? "text-red-600"
                          : "text-gray-600"
                      }`}
                    >
                      {inspection.overall_result || "-"}
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          )}
        </div>
      </main>
    </div>
  );
}
