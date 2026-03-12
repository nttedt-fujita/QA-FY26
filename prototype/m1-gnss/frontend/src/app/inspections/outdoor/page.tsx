"use client";

import { useState, useEffect, useCallback, useRef } from "react";
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

  // 検査制御
  const [isInspecting, setIsInspecting] = useState(false);
  const [inspectionDurationSec, setInspectionDurationSec] = useState(30);
  const [remainingTime, setRemainingTime] = useState(0);
  const inspectionTimerRef = useRef<NodeJS.Timeout | null>(null);

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
    setIsInspecting(true);
    setRemainingTime(inspectionDurationSec);

    // カウントダウン
    inspectionTimerRef.current = setInterval(() => {
      setRemainingTime((prev) => {
        if (prev <= 1) {
          // 検査終了
          if (inspectionTimerRef.current) {
            clearInterval(inspectionTimerRef.current);
            inspectionTimerRef.current = null;
          }
          setIsInspecting(false);
          return 0;
        }
        return prev - 1;
      });
    }, 1000);
  };

  // 検査停止
  const handleStopInspection = () => {
    if (inspectionTimerRef.current) {
      clearInterval(inspectionTimerRef.current);
      inspectionTimerRef.current = null;
    }
    setIsInspecting(false);
    setRemainingTime(0);
  };

  // ページ離脱時にタイマークリア
  useEffect(() => {
    return () => {
      if (inspectionTimerRef.current) {
        clearInterval(inspectionTimerRef.current);
      }
    };
  }, []);

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
                disabled={isInspecting}
              >
                <option value={30}>30秒</option>
                <option value={60}>60秒</option>
                <option value={120}>120秒</option>
                <option value={300}>5分</option>
              </select>
            </div>

            {/* 開始/停止ボタン */}
            {!isInspecting ? (
              <button
                onClick={handleStartInspection}
                disabled={!connectedDevice}
                className="rounded bg-[#2e75b6] px-6 py-2 font-medium text-white hover:bg-[#245d92] disabled:cursor-not-allowed disabled:opacity-50"
              >
                検査開始
              </button>
            ) : (
              <button
                onClick={handleStopInspection}
                className="rounded bg-red-600 px-6 py-2 font-medium text-white hover:bg-red-700"
              >
                検査停止
              </button>
            )}

            {/* 残り時間 */}
            {isInspecting && (
              <div className="flex items-center gap-2">
                <span className="text-sm text-gray-600">残り:</span>
                <span className="font-mono text-lg font-semibold text-blue-600">
                  {remainingTime}秒
                </span>
              </div>
            )}
          </div>
        </div>

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
          <NavStatusPanel enabled={isInspecting && !!connectedDevice} />
        </div>

        {/* スカイプロット + NAV-SIG 横並び */}
        <div className="mb-6 grid gap-4 md:grid-cols-2">
          {/* スカイプロット（NAV-SAT） */}
          <SkyPlotPanel enabled={isInspecting && !!connectedDevice} />
          {/* NAV-SIG（衛星信号） */}
          <NavSigPanel enabled={isInspecting && !!connectedDevice} />
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
