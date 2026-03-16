"use client";

import { useState } from "react";
import { Device, blinkDevice } from "@/lib/api";

interface DeviceCardProps {
  device: Device;
  onConnect: (path: string) => void;
  onDisconnect: (path: string) => void;
  isLoading: boolean;
  /** 点滅ボタンを表示するか（検査完了後に表示） */
  showBlinkButton?: boolean;
  /** デバイスが抜かれたか */
  isDisconnected?: boolean;
}

/**
 * 装置カード
 *
 * 未接続・接続中の状態に応じた表示を行う
 */
export function DeviceCard({
  device,
  onConnect,
  onDisconnect,
  isLoading,
  showBlinkButton = false,
  isDisconnected = false,
}: DeviceCardProps) {
  const isConnected = device.status === "connected";
  const [isBlinking, setIsBlinking] = useState(false);

  // 装置タイプの判定（VID/PIDから）
  const getDeviceType = (): string => {
    if (device.vid === "1546" && device.pid === "01A9") {
      return "u-blox F9P";
    }
    if (device.vid === "0403" && device.pid === "6015") {
      return "FTDI (F9P評価ボード)";
    }
    return "不明な装置";
  };

  // LED点滅
  const handleBlink = async () => {
    setIsBlinking(true);
    try {
      await blinkDevice(device.path, 3);
    } catch (e) {
      console.error("点滅エラー:", e);
    } finally {
      setIsBlinking(false);
    }
  };

  // 抜かれた状態の表示
  if (isDisconnected) {
    return (
      <div className="rounded-lg border-2 border-dashed border-orange-400 bg-orange-50 p-4">
        <div className="flex items-center gap-2">
          <span className="text-2xl">🔌</span>
          <div>
            <h3 className="font-bold text-orange-700">抜かれました</h3>
            <p className="text-sm text-orange-600">
              シリアル: {device.f9p_serial ?? device.serial_number ?? "不明"}
            </p>
          </div>
        </div>
      </div>
    );
  }

  return (
    <div
      className={`relative rounded-lg border p-4 ${
        isConnected
          ? "border-green-500 bg-green-50"
          : "border-gray-200 bg-white"
      } ${isBlinking ? "animate-pulse ring-2 ring-yellow-400" : ""}`}
    >
      {/* 点滅中オーバーレイ */}
      {isBlinking && (
        <div className="absolute inset-0 flex items-center justify-center rounded-lg bg-yellow-100/80">
          <div className="text-center">
            <div className="text-3xl animate-bounce">💡</div>
            <p className="mt-1 font-bold text-yellow-700">点滅中...</p>
          </div>
        </div>
      )}

      {/* ヘッダー */}
      <div className="flex items-start justify-between">
        <div>
          <h3 className="font-bold text-gray-900">{device.path}</h3>
          <p className="text-sm text-gray-500">
            VID: {device.vid || "N/A"} PID: {device.pid || "N/A"} ({getDeviceType()})
          </p>
        </div>
        <StatusBadge status={device.status} />
      </div>

      {/* 接続中の追加情報 */}
      {isConnected && (
        <div className="mt-3 space-y-1 text-sm text-gray-600">
          {device.baud_rate && (
            <p>ボーレート: {device.baud_rate.toLocaleString()} bps</p>
          )}
          {(device.f9p_serial || device.serial_number) && (
            <p>シリアル: {device.f9p_serial ?? device.serial_number}</p>
          )}
        </div>
      )}

      {/* アクションボタン */}
      <div className="mt-4 space-y-2">
        {isConnected ? (
          <>
            {/* 点滅ボタン（検査完了後に表示） */}
            {showBlinkButton && (
              <button
                onClick={handleBlink}
                disabled={isLoading || isBlinking}
                className="w-full rounded bg-yellow-500 px-4 py-2 font-bold text-white hover:bg-yellow-600 disabled:opacity-50"
              >
                {isBlinking ? "点滅中..." : "💡 このデバイスを点滅"}
              </button>
            )}
            <button
              onClick={() => onDisconnect(device.path)}
              disabled={isLoading || isBlinking}
              className="w-full rounded bg-red-500 px-4 py-2 font-bold text-white hover:bg-red-600 disabled:opacity-50"
            >
              {isLoading ? "切断中..." : "切断"}
            </button>
          </>
        ) : (
          <button
            onClick={() => onConnect(device.path)}
            disabled={isLoading}
            className="w-full rounded bg-blue-500 px-4 py-2 font-bold text-white hover:bg-blue-600 disabled:opacity-50"
          >
            {isLoading ? "接続中..." : "接続"}
          </button>
        )}
      </div>
    </div>
  );
}

/**
 * 状態バッジ
 */
function StatusBadge({ status }: { status: string }) {
  const styles: Record<string, string> = {
    detected: "bg-gray-100 text-gray-600",
    connecting: "bg-yellow-100 text-yellow-700",
    connected: "bg-green-100 text-green-700",
    disconnected: "bg-gray-100 text-gray-500",
    error: "bg-red-100 text-red-700",
  };

  const labels: Record<string, string> = {
    detected: "検出済",
    connecting: "接続中",
    connected: "接続中",
    disconnected: "切断済",
    error: "エラー",
  };

  const style = styles[status] || styles.error;
  const label = labels[status] || status;

  return (
    <span className={`rounded-full px-2 py-1 text-xs font-medium ${style}`}>
      {label}
    </span>
  );
}
