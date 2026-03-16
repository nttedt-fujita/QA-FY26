"use client";

import { useState, useEffect, useCallback } from "react";
import {
  Device,
  Lot,
  InspectionSummary,
  BatchInspectionResponse,
  DeviceInspectionResult,
  listDevices,
  listLots,
  runBatchInspection,
  listInspections,
  blinkDevice,
} from "@/lib/api";

/**
 * 屋内検査実行画面（Phase 3: 複数台対応）
 *
 * 5項目: RATE, UART1, UART2, USB, NAV
 */
export default function IndoorInspectionsPage() {
  // 状態
  const [lots, setLots] = useState<Lot[]>([]);
  const [devices, setDevices] = useState<Device[]>([]);
  const [selectedLotId, setSelectedLotId] = useState<number | null>(null);
  const [batchResult, setBatchResult] = useState<BatchInspectionResponse | null>(null);
  const [inspectionHistory, setInspectionHistory] = useState<InspectionSummary[]>([]);
  const [blinkingPaths, setBlinkingPaths] = useState<Set<string>>(new Set());

  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  // 接続中の装置一覧（複数台対応）
  const connectedDevices = devices.filter((d) => d.status === "connected");

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

  // 一括検査実行
  const handleRunBatchInspection = async () => {
    if (connectedDevices.length === 0) {
      setError("装置が接続されていません");
      return;
    }

    setIsLoading(true);
    setError(null);
    setBatchResult(null);

    try {
      const result = await runBatchInspection({
        lot_id: selectedLotId ?? undefined,
      });
      setBatchResult(result);
      // 履歴を更新
      const inspectionsRes = await listInspections();
      setInspectionHistory(inspectionsRes.inspections);
    } catch (e) {
      setError(e instanceof Error ? e.message : "検査に失敗しました");
    } finally {
      setIsLoading(false);
    }
  };

  // LED点滅
  const handleBlink = async (path: string) => {
    setBlinkingPaths((prev) => new Set(prev).add(path));
    try {
      await blinkDevice(path, 3);
    } catch (e) {
      console.error("点滅エラー:", e);
    } finally {
      setBlinkingPaths((prev) => {
        const next = new Set(prev);
        next.delete(path);
        return next;
      });
    }
  };

  // パス名の短縮表示（/dev/ttyUSB0 → ttyUSB0）
  const shortPath = (path: string) => path.replace("/dev/", "");

  return (
    <div className="min-h-screen bg-gray-50">
      {/* ページタイトル */}
      <div className="bg-white border-b border-gray-200">
        <div className="container mx-auto px-4 py-4">
          <div className="flex items-center justify-between">
            <h1 className="text-xl font-semibold text-gray-800">屋内検査</h1>
            <a
              href="/inspections/outdoor"
              className="text-blue-600 hover:underline text-sm"
            >
              屋外検査へ →
            </a>
          </div>
          <p className="text-sm text-gray-500 mt-1">
            RATE, UART1, UART2, USB, NAV の5項目を検査
          </p>
        </div>
      </div>

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
            <h3 className="mb-2 font-medium text-gray-700">
              接続中の装置 ({connectedDevices.length}台)
            </h3>
            {connectedDevices.length > 0 ? (
              <div className="space-y-2">
                {connectedDevices.map((device) => (
                  <div
                    key={device.path}
                    className="flex items-center justify-between rounded bg-gray-50 p-2"
                  >
                    <div>
                      <div className="font-mono text-sm">{shortPath(device.path)}</div>
                      <div className="text-xs text-gray-500">
                        {device.f9p_serial || device.serial_number || "-"}
                      </div>
                    </div>
                    <div className="inline-block rounded bg-green-100 px-2 py-1 text-xs text-green-800">
                      接続中
                    </div>
                  </div>
                ))}
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
            onClick={handleRunBatchInspection}
            disabled={isLoading || connectedDevices.length === 0}
            className="rounded bg-[#2e75b6] px-8 py-3 text-lg font-medium text-white hover:bg-[#245d92] disabled:cursor-not-allowed disabled:opacity-50"
          >
            {isLoading
              ? "検査中..."
              : connectedDevices.length === 1
              ? "検査開始"
              : `全台検査開始 (${connectedDevices.length}台)`}
          </button>
        </div>

        {/* 検査項目一覧 */}
        <div className="mb-6 rounded border border-gray-200 bg-white p-4">
          <h3 className="mb-4 text-lg font-medium text-gray-700">検査項目</h3>
          <div className="grid gap-2 md:grid-cols-5">
            {["RATE", "UART1", "UART2", "USB", "NAV"].map((item) => (
              <div
                key={item}
                className="rounded border border-gray-200 bg-gray-50 p-3 text-center"
              >
                <span className="font-mono text-sm text-gray-700">{item}</span>
              </div>
            ))}
          </div>
        </div>

        {/* エラー表示 */}
        {error && (
          <div className="mb-6 rounded border border-red-200 bg-red-50 p-4 text-red-700">
            {error}
          </div>
        )}

        {/* 一括検査結果 */}
        {batchResult && (
          <div className="mb-6 rounded border border-gray-200 bg-white p-4">
            <h3 className="mb-4 text-lg font-medium text-gray-700">
              検査結果 ({batchResult.summary.total}台中 {batchResult.summary.passed}台合格)
            </h3>
            <div className="space-y-3">
              {batchResult.results.map((result) => (
                <BatchResultCard
                  key={result.path}
                  result={result}
                  isBlinking={blinkingPaths.has(result.path)}
                  onBlink={() => handleBlink(result.path)}
                />
              ))}
            </div>
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

/**
 * 検査項目名を日本語に変換
 */
function itemNameToJapanese(itemName: string): string {
  const mapping: Record<string, string> = {
    communication: "通信疎通",
    fw: "FWバージョン",
    serial: "シリアル番号",
    rate: "出力レート",
    port: "ポート設定",
  };
  return mapping[itemName] || itemName;
}

/**
 * 判定結果に応じた色を返す
 */
function getVerdictColor(verdict: string): string {
  switch (verdict) {
    case "Pass":
      return "text-green-600";
    case "Fail":
      return "text-red-600";
    case "Error":
      return "text-yellow-600";
    default:
      return "text-gray-600";
  }
}

/**
 * 判定結果を日本語に変換
 */
function verdictToJapanese(verdict: string): string {
  switch (verdict) {
    case "Pass":
      return "合格";
    case "Fail":
      return "不合格";
    case "Error":
      return "エラー";
    default:
      return verdict;
  }
}

/**
 * 一括検査結果カード
 */
function BatchResultCard({
  result,
  isBlinking,
  onBlink,
}: {
  result: DeviceInspectionResult;
  isBlinking: boolean;
  onBlink: () => void;
}) {
  const isPassed = result.overall_result === "Pass";
  const shortPath = result.path.replace("/dev/", "");

  return (
    <div
      className={`rounded border p-4 ${
        isPassed
          ? "border-green-200 bg-green-50"
          : "border-red-200 bg-red-50"
      }`}
    >
      {/* ヘッダー: デバイス情報 + 総合判定 + 点滅ボタン */}
      <div className="flex items-center justify-between mb-3">
        <div className="flex items-center gap-3">
          <span className="text-2xl">{isPassed ? "✅" : "❌"}</span>
          <div>
            <div className="font-mono text-sm font-medium">
              {shortPath} / {result.serial_number}
            </div>
            <div
              className={`text-sm font-semibold ${
                isPassed ? "text-green-700" : "text-red-700"
              }`}
            >
              {isPassed ? "合格" : "不合格"}
            </div>
          </div>
        </div>
        {/* 点滅ボタン */}
        <button
          onClick={onBlink}
          disabled={isBlinking}
          className={`rounded px-3 py-2 text-sm font-medium transition-colors ${
            isBlinking
              ? "animate-pulse bg-yellow-400 text-yellow-900"
              : "bg-yellow-100 text-yellow-800 hover:bg-yellow-200"
          }`}
        >
          💡 {isBlinking ? "点滅中..." : "点滅"}
        </button>
      </div>

      {/* 項目詳細テーブル */}
      {result.items.length > 0 && (
        <table className="w-full border-collapse text-sm">
          <thead>
            <tr className="bg-white/50">
              <th className="border border-gray-300 p-2 text-left">項目</th>
              <th className="border border-gray-300 p-2 text-left">期待値</th>
              <th className="border border-gray-300 p-2 text-left">実測値</th>
              <th className="border border-gray-300 p-2 text-center w-20">判定</th>
            </tr>
          </thead>
          <tbody>
            {result.items.map((item, index) => (
              <tr key={index} className="bg-white/30">
                <td className="border border-gray-300 p-2">
                  {itemNameToJapanese(item.item_name)}
                </td>
                <td className="border border-gray-300 p-2 text-gray-600">
                  {item.expected_value || "-"}
                </td>
                <td className="border border-gray-300 p-2">
                  {item.actual_value || "-"}
                </td>
                <td
                  className={`border border-gray-300 p-2 text-center font-semibold ${getVerdictColor(
                    item.verdict
                  )}`}
                >
                  {verdictToJapanese(item.verdict)}
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      )}
    </div>
  );
}
