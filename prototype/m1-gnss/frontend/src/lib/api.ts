/**
 * バックエンドAPI呼び出し
 */

const API_BASE = process.env.NEXT_PUBLIC_API_URL || "http://localhost:8080";

/**
 * 装置情報
 */
export interface Device {
  path: string;
  vid: string | null;
  pid: string | null;
  serial_number: string | null;
  status: string;
  baud_rate: number | null;
}

/**
 * 装置一覧レスポンス
 */
export interface DeviceListResponse {
  devices: Device[];
}

/**
 * 接続成功レスポンス
 */
export interface ConnectResponse {
  path: string;
  baud_rate: number;
  message: string;
}

/**
 * エラーレスポンス
 */
export interface ErrorResponse {
  error: string;
  code: string;
}

/**
 * 装置一覧を取得
 */
export async function listDevices(): Promise<DeviceListResponse> {
  const res = await fetch(`${API_BASE}/api/devices`);
  if (!res.ok) {
    const error: ErrorResponse = await res.json();
    throw new Error(error.error);
  }
  return res.json();
}

/**
 * 装置に接続（自動検出）
 */
export async function connectDevice(path: string): Promise<ConnectResponse> {
  const encodedPath = encodeURIComponent(path);
  const res = await fetch(`${API_BASE}/api/devices/${encodedPath}/connect`, {
    method: "POST",
  });
  if (!res.ok) {
    const error: ErrorResponse = await res.json();
    throw new Error(error.error);
  }
  return res.json();
}

/**
 * 装置を切断
 */
export async function disconnectDevice(path: string): Promise<void> {
  const encodedPath = encodeURIComponent(path);
  const res = await fetch(`${API_BASE}/api/devices/${encodedPath}/disconnect`, {
    method: "POST",
  });
  if (!res.ok) {
    const error: ErrorResponse = await res.json();
    throw new Error(error.error);
  }
}
