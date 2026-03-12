"use client";

import { useState, useEffect, useCallback } from "react";
import {
  Device,
  Lot,
  listDevices,
  listLots,
} from "@/lib/api";
import { NavSigPanel } from "@/components/NavSigPanel";
import { MonSpanPanel } from "@/components/MonSpanPanel";
import { NavStatusPanel } from "@/components/NavStatusPanel";
import { SkyPlotPanel } from "@/components/SkyPlotPanel";
import { useOutdoorInspection } from "@/hooks/useOutdoorInspection";

/**
 * 屋外検査実行画面
 *
 * L2受信率、RTK FIX率、MON-SPAN、TTFF、スカイプロット
 */
export default function OutdoorInspectionsPage() {
  // 状態
  const [lots, setLots] = useState<Lot[]>([]);
  const [devices, setDevices] = useState<Device[]>([]);
  const [selectedLotId, setSelectedLotId] = useState<number | null>(null);
  const [error, setError] = useState<string | null>(null);

  // 検査時間設定
  const [inspectionDurationSec, setInspectionDurationSec] = useState(30);

  // 屋外検査Hook
  const inspection = useOutdoorInspection();
  const isInspecting = inspection.state === "running";

  // 接続中の装置を取得
  const connectedDevice = devices.find((d) => d.status === "connected");

  // 選択中のロットを取得
  const selectedLot = lots.find((l) => l.id === selectedLotId);

  // データ取得
  const fetchData = useCallback(async () => {
    try {
      const [lotsRes, devicesRes] = await Promise.all([
        listLots(),
        listDevices(),
      ]);
      setLots(lotsRes.lots);
      setDevices(devicesRes.devices);
    } catch (e) {
      setError(e instanceof Error ? e.message : "データの取得に失敗しました");
    }
  }, []);

  // 初回読み込み
  useEffect(() => {
    fetchData();
  }, [fetchData]);

  // 検査開始
  const handleStartInspection = () => {
    inspection.start(inspectionDurationSec);
  };

  // 検査停止
  const handleStopInspection = () => {
    inspection.stop();
  };

  // 新規検査（リセット）
  const handleNewInspection = () => {
    inspection.reset();
  };

  return (
    <div className="min-h-screen bg-gray-50">
      {/* ページタイトル */}
      <div className="bg-white border-b border-gray-200">
        <div className="container mx-auto px-4 py-4">
          <div className="flex items-center justify-between">
            <h1 className="text-xl font-semibold text-gray-800">屋外検査</h1>
            <a
              href="/inspections/indoor"
              className="text-blue-600 hover:underline text-sm"
            >
              ← 屋内検査へ
            </a>
          </div>
          <p className="text-sm text-gray-500 mt-1">
            L2受信率、RTK FIX率、MON-SPAN、スカイプロット
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
              disabled={isInspecting}
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

        {/* 検査制御 */}
        <div className="mb-6 rounded border border-gray-200 bg-white p-4">
          <h3 className="mb-4 font-medium text-gray-700">検査制御</h3>
          <div className="flex items-center gap-4">
            {/* 検査時間設定 */}
            <div className="flex items-center gap-2">
              <label className="text-sm text-gray-600">検査時間:</label>
              <select
                value={inspectionDurationSec}
                onChange={(e) => setInspectionDurationSec(Number(e.target.value))}
                className="rounded border border-gray-300 p-2"
                disabled={inspection.state !== "idle"}
              >
                <option value={30}>30秒</option>
                <option value={60}>60秒</option>
                <option value={120}>120秒</option>
                <option value={300}>5分</option>
              </select>
            </div>

            {/* 開始/停止/新規ボタン */}
            {inspection.state === "idle" && (
              <button
                onClick={handleStartInspection}
                disabled={!connectedDevice}
                className="rounded bg-[#2e75b6] px-6 py-2 font-medium text-white hover:bg-[#245d92] disabled:cursor-not-allowed disabled:opacity-50"
              >
                検査開始
              </button>
            )}
            {inspection.state === "running" && (
              <button
                onClick={handleStopInspection}
                className="rounded bg-red-600 px-6 py-2 font-medium text-white hover:bg-red-700"
              >
                検査停止
              </button>
            )}
            {inspection.state === "completed" && (
              <button
                onClick={handleNewInspection}
                className="rounded bg-[#2e75b6] px-6 py-2 font-medium text-white hover:bg-[#245d92]"
              >
                新規検査
              </button>
            )}

            {/* 残り時間/サンプル数 */}
            {isInspecting && (
              <div className="flex items-center gap-4">
                <div className="flex items-center gap-2">
                  <span className="text-sm text-gray-600">残り:</span>
                  <span className="font-mono text-lg font-semibold text-blue-600">
                    {inspection.remainingTime}秒
                  </span>
                </div>
                <div className="flex items-center gap-2">
                  <span className="text-sm text-gray-600">サンプル:</span>
                  <span className="font-mono text-lg font-semibold text-gray-700">
                    {inspection.sampleCount}
                  </span>
                </div>
              </div>
            )}
          </div>
        </div>

        {/* 検査結果（完了時のみ表示） */}
        {inspection.state === "completed" && inspection.result && (
          <div
            className={`mb-6 rounded border p-4 ${
              inspection.result.judgment.is_pass
                ? "border-green-300 bg-green-50"
                : "border-red-300 bg-red-50"
            }`}
          >
            <div className="mb-4 flex items-center justify-between">
              <h3 className="font-medium text-gray-700">検査結果</h3>
              <span
                className={`rounded px-3 py-1 text-lg font-bold ${
                  inspection.result.judgment.is_pass
                    ? "bg-green-500 text-white"
                    : "bg-red-500 text-white"
                }`}
              >
                {inspection.result.judgment.is_pass ? "合格" : "不合格"}
              </span>
            </div>

            {/* 集計結果 */}
            <div className="mb-4 grid gap-4 md:grid-cols-4">
              <ResultItem
                label="L1最小C/N0"
                value={`${inspection.result.l1_min_cno.toFixed(1)} dBHz`}
                pass={inspection.result.judgment.l1_cno_pass}
                criteria="≥30 dBHz"
              />
              <ResultItem
                label="L2受信率"
                value={`${(inspection.result.l2_reception_rate * 100).toFixed(1)}%`}
                pass={inspection.result.judgment.l2_rate_pass}
                criteria="≥50%"
              />
              <ResultItem
                label="RTK FIX時間"
                value={
                  inspection.result.rtk_fix_time_ms !== null
                    ? `${(inspection.result.rtk_fix_time_ms / 1000).toFixed(1)}秒`
                    : "FIXせず"
                }
                pass={inspection.result.judgment.rtk_fix_time_pass}
                criteria="≤30秒"
              />
              <ResultItem
                label="RTK FIX率"
                value={`${(inspection.result.rtk_fix_rate * 100).toFixed(1)}%`}
                pass={inspection.result.judgment.rtk_fix_rate_pass}
                criteria=">95%"
              />
            </div>

            {/* 不合格理由 */}
            {!inspection.result.judgment.is_pass && (
              <div className="rounded bg-red-100 p-3">
                <div className="mb-1 text-sm font-medium text-red-700">
                  不合格理由:
                </div>
                <ul className="list-inside list-disc text-sm text-red-600">
                  {inspection.result.judgment.failure_reasons.map((reason, i) => (
                    <li key={i}>{reason}</li>
                  ))}
                </ul>
              </div>
            )}

            {/* サンプル数と保存ボタン */}
            <div className="mt-4 flex items-center justify-between">
              <div className="text-sm text-gray-500">
                サンプル数: {inspection.result.sample_count}
              </div>
              <div className="flex items-center gap-4">
                {/* 保存状態表示 */}
                {inspection.saveState === "saved" && (
                  <span className="text-sm text-green-600">
                    ✓ 保存済み (ID: {inspection.savedId})
                  </span>
                )}
                {inspection.saveState === "error" && (
                  <span className="text-sm text-red-600">
                    × {inspection.saveError}
                  </span>
                )}

                {/* 保存ボタン */}
                <button
                  onClick={() =>
                    inspection.saveResult(
                      undefined, // device_idは未対応（将来的にDB装置と紐付け）
                      selectedLotId ?? undefined
                    )
                  }
                  disabled={
                    inspection.saveState === "saving" ||
                    inspection.saveState === "saved"
                  }
                  className="rounded bg-[#2e75b6] px-4 py-2 text-sm font-medium text-white hover:bg-[#245d92] disabled:cursor-not-allowed disabled:opacity-50"
                >
                  {inspection.saveState === "saving"
                    ? "保存中..."
                    : inspection.saveState === "saved"
                    ? "保存済み"
                    : "結果を保存"}
                </button>
              </div>
            </div>
          </div>
        )}

        {/* 合格基準表示 */}
        <div className="mb-6 rounded border border-gray-200 bg-white p-4">
          <h3 className="mb-3 font-medium text-gray-700">合格基準（ADR-008）</h3>
          <div className="grid gap-2 md:grid-cols-4 text-sm">
            <div className="rounded bg-gray-50 p-2">
              <div className="text-gray-500">L1受信感度</div>
              <div className="font-mono">≥30 dBHz</div>
            </div>
            <div className="rounded bg-gray-50 p-2">
              <div className="text-gray-500">L2受信率（GPS）</div>
              <div className="font-mono">≥50%</div>
            </div>
            <div className="rounded bg-gray-50 p-2">
              <div className="text-gray-500">RTK FIX時間</div>
              <div className="font-mono">≤30秒</div>
            </div>
            <div className="rounded bg-gray-50 p-2">
              <div className="text-gray-500">RTK FIX率</div>
              <div className="font-mono">&gt;95%</div>
            </div>
          </div>
        </div>

        {/* NAV-STATUS（Fix状態・TTFF）パネル */}
        <div className="mb-6">
          <NavStatusPanel
            enabled={isInspecting && !!connectedDevice}
            onSample={inspection.addNavStatusSample}
          />
        </div>

        {/* スカイプロット + NAV-SIG 横並び */}
        <div className="mb-6 grid gap-4 md:grid-cols-2">
          {/* スカイプロット（NAV-SAT） */}
          <SkyPlotPanel enabled={isInspecting && !!connectedDevice} />
          {/* NAV-SIG（衛星信号） */}
          <NavSigPanel
            enabled={isInspecting && !!connectedDevice}
            onSample={inspection.addNavSigSample}
          />
        </div>

        {/* MON-SPAN（スペクトラム）パネル */}
        <div className="mb-6">
          <MonSpanPanel enabled={isInspecting && !!connectedDevice} />
        </div>

        {/* エラー表示 */}
        {error && (
          <div className="mb-6 rounded border border-red-200 bg-red-50 p-4 text-red-700">
            {error}
          </div>
        )}
      </main>
    </div>
  );
}

/**
 * 検査結果の各項目表示
 */
function ResultItem({
  label,
  value,
  pass,
  criteria,
}: {
  label: string;
  value: string;
  pass: boolean;
  criteria: string;
}) {
  return (
    <div
      className={`rounded p-3 ${
        pass ? "bg-green-100" : "bg-red-100"
      }`}
    >
      <div className="text-sm text-gray-600">{label}</div>
      <div
        className={`text-lg font-semibold ${
          pass ? "text-green-700" : "text-red-700"
        }`}
      >
        {value}
      </div>
      <div className="text-xs text-gray-500">基準: {criteria}</div>
      <div
        className={`mt-1 text-sm font-medium ${
          pass ? "text-green-600" : "text-red-600"
        }`}
      >
        {pass ? "✓ 合格" : "× 不合格"}
      </div>
    </div>
  );
}
