"use client";

import { useState, useEffect, useCallback, useRef } from "react";
import { Device, listDevices, connectDevice, disconnectDevice } from "@/lib/api";
import { DeviceCard } from "@/components/DeviceCard";

/** 抜かれたデバイス情報 */
interface DisconnectedDevice {
  path: string;
  f9p_serial: string | null;
  serial_number: string | null;
}

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
  // 抜かれたデバイスを追跡
  const [disconnectedDevices, setDisconnectedDevices] = useState<DisconnectedDevice[]>([]);
  // 前回の接続中デバイスを記憶
  const previousConnectedRef = useRef<Map<string, Device>>(new Map());

  // 装置一覧を取得
  const fetchDevices = useCallback(async () => {
    setIsScanning(true);
    setError(null);
    try {
      const response = await listDevices();
      const newDevices = response.devices;

      // 抜かれたデバイスを検出
      const currentPaths = new Set(newDevices.map(d => d.path));
      const previousConnected = previousConnectedRef.current;

      // 以前接続中だったが今はリストにないデバイスを検出
      const newlyDisconnected: DisconnectedDevice[] = [];
      previousConnected.forEach((device, path) => {
        if (!currentPaths.has(path)) {
          newlyDisconnected.push({
            path,
            f9p_serial: device.f9p_serial,
            serial_number: device.serial_number,
          });
        }
      });

      if (newlyDisconnected.length > 0) {
        setDisconnectedDevices(prev => [...prev, ...newlyDisconnected]);
      }

      // 現在の接続中デバイスを記憶
      const newConnected = new Map<string, Device>();
      newDevices.forEach(d => {
        if (d.status === "connected") {
          newConnected.set(d.path, d);
        }
      });
      previousConnectedRef.current = newConnected;

      setDevices(newDevices);
    } catch (e) {
      setError(e instanceof Error ? e.message : "装置の取得に失敗しました");
    } finally {
      setIsScanning(false);
    }
  }, []);

  // 初回読み込み + 定期ポーリング（2秒ごと）
  useEffect(() => {
    fetchDevices();
    const interval = setInterval(fetchDevices, 2000);
    return () => clearInterval(interval);
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

  // 抜かれたデバイス表示をクリア
  const clearDisconnected = (path: string) => {
    setDisconnectedDevices(prev => prev.filter(d => d.path !== path));
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
            href="/inspections"
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

        {/* 抜かれたデバイスの通知 */}
        {disconnectedDevices.length > 0 && (
          <div className="mb-6 space-y-2">
            {disconnectedDevices.map((d) => (
              <div
                key={d.path}
                className="flex items-center justify-between rounded border-2 border-orange-400 bg-orange-50 p-3"
              >
                <div className="flex items-center gap-2">
                  <span className="text-xl">🔌</span>
                  <div>
                    <span className="font-bold text-orange-700">抜かれました: </span>
                    <span className="text-orange-600">
                      {d.f9p_serial ?? d.serial_number ?? d.path}
                    </span>
                  </div>
                </div>
                <button
                  onClick={() => clearDisconnected(d.path)}
                  className="text-orange-500 hover:text-orange-700"
                >
                  ✕
                </button>
              </div>
            ))}
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
                  showBlinkButton={device.status === "connected"}
                />
              ))}
            </div>
          )}
        </div>
      </main>
    </div>
  );
}
