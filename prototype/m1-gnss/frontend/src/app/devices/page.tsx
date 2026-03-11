"use client";

import { useState, useEffect, useCallback } from "react";
import { Device, listDevices, connectDevice, disconnectDevice } from "@/lib/api";
import { DeviceCard } from "@/components/DeviceCard";

/**
 * 装置接続画面
 *
 * モックアップ: sessions/session92/ui-mockup-phase1.drawio の②画面
 */
export default function DevicesPage() {
  const [devices, setDevices] = useState<Device[]>([]);
  const [isLoading, setIsLoading] = useState(false);
  const [isScanning, setIsScanning] = useState(false);
  const [error, setError] = useState<string | null>(null);

  // 装置一覧を取得
  const fetchDevices = useCallback(async () => {
    setIsScanning(true);
    setError(null);
    try {
      const response = await listDevices();
      setDevices(response.devices);
    } catch (e) {
      setError(e instanceof Error ? e.message : "装置の取得に失敗しました");
    } finally {
      setIsScanning(false);
    }
  }, []);

  // 初回読み込み
  useEffect(() => {
    fetchDevices();
  }, [fetchDevices]);

  // 接続
  const handleConnect = async (path: string) => {
    setIsLoading(true);
    setError(null);
    try {
      await connectDevice(path);
      await fetchDevices(); // 状態を更新
    } catch (e) {
      setError(e instanceof Error ? e.message : "接続に失敗しました");
    } finally {
      setIsLoading(false);
    }
  };

  // 切断
  const handleDisconnect = async (path: string) => {
    setIsLoading(true);
    setError(null);
    try {
      await disconnectDevice(path);
      await fetchDevices(); // 状態を更新
    } catch (e) {
      setError(e instanceof Error ? e.message : "切断に失敗しました");
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <div className="min-h-screen bg-gray-50">
      {/* ヘッダー */}
      <header className="bg-[#5b9bd5] text-white">
        <div className="container mx-auto px-4 py-4">
          <h1 className="text-xl font-bold">装置接続</h1>
        </div>
      </header>

      {/* タブ（仮） */}
      <nav className="border-b border-gray-200 bg-white">
        <div className="container mx-auto flex px-4">
          <a
            href="/"
            className="border-b-2 border-transparent px-4 py-3 text-gray-500 hover:text-gray-700"
          >
            ロット
          </a>
          <a
            href="/devices"
            className="border-b-2 border-[#5b9bd5] px-4 py-3 font-medium text-[#5b9bd5]"
          >
            装置
          </a>
          <a
            href="/inspection"
            className="border-b-2 border-transparent px-4 py-3 text-gray-500 hover:text-gray-700"
          >
            検査
          </a>
        </div>
      </nav>

      {/* メインコンテンツ */}
      <main className="container mx-auto px-4 py-6">
        {/* スキャンボタン */}
        <div className="mb-6">
          <button
            onClick={fetchDevices}
            disabled={isScanning}
            className="rounded bg-[#5b9bd5] px-6 py-2 font-medium text-white hover:bg-[#4a8bc4] disabled:opacity-50"
          >
            {isScanning ? "スキャン中..." : "🔍 スキャン"}
          </button>
        </div>

        {/* エラー表示 */}
        {error && (
          <div className="mb-6 rounded border border-red-200 bg-red-50 p-4 text-red-700">
            {error}
          </div>
        )}

        {/* 装置一覧 */}
        <div className="space-y-4">
          <h2 className="text-lg font-medium text-gray-700">
            検出された装置 ({devices.length})
          </h2>

          {devices.length === 0 ? (
            <div className="rounded border border-gray-200 bg-white p-8 text-center text-gray-500">
              装置が検出されませんでした。
              <br />
              USBケーブルを接続して「スキャン」を押してください。
            </div>
          ) : (
            <div className="grid gap-4 md:grid-cols-2">
              {devices.map((device) => (
                <DeviceCard
                  key={device.path}
                  device={device}
                  onConnect={handleConnect}
                  onDisconnect={handleDisconnect}
                  isLoading={isLoading}
                />
              ))}
            </div>
          )}
        </div>
      </main>
    </div>
  );
}
