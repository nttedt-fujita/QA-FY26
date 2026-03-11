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

// ===========================================
// ロットAPI
// ===========================================

/**
 * ロット情報
 */
export interface Lot {
  id: number;
  lot_number: string;
  expected_rate_ms: number | null;
  expected_port_in_proto: string | null;
  expected_port_out_proto: string | null;
  memo: string | null;
}

/**
 * ロット一覧レスポンス
 */
export interface LotListResponse {
  lots: Lot[];
}

/**
 * ロット作成リクエスト
 */
export interface CreateLotRequest {
  lot_number: string;
  expected_rate_ms?: number;
  expected_port_in_proto?: string;
  expected_port_out_proto?: string;
  memo?: string;
}

/**
 * ロット更新リクエスト
 */
export interface UpdateLotRequest {
  expected_rate_ms?: number;
  expected_port_in_proto?: string;
  expected_port_out_proto?: string;
  memo?: string;
}

/**
 * ロット一覧を取得
 */
export async function listLots(): Promise<LotListResponse> {
  const res = await fetch(`${API_BASE}/api/lots`);
  if (!res.ok) {
    const error: ErrorResponse = await res.json();
    throw new Error(error.error);
  }
  return res.json();
}

/**
 * ロットを作成
 */
export async function createLot(request: CreateLotRequest): Promise<Lot> {
  const res = await fetch(`${API_BASE}/api/lots`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(request),
  });
  if (!res.ok) {
    const error: ErrorResponse = await res.json();
    throw new Error(error.error);
  }
  return res.json();
}

/**
 * ロット詳細を取得
 */
export async function getLot(id: number): Promise<Lot> {
  const res = await fetch(`${API_BASE}/api/lots/${id}`);
  if (!res.ok) {
    const error: ErrorResponse = await res.json();
    throw new Error(error.error);
  }
  return res.json();
}

/**
 * ロットを更新
 */
export async function updateLot(id: number, request: UpdateLotRequest): Promise<Lot> {
  const res = await fetch(`${API_BASE}/api/lots/${id}`, {
    method: "PUT",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(request),
  });
  if (!res.ok) {
    const error: ErrorResponse = await res.json();
    throw new Error(error.error);
  }
  return res.json();
}

// ===========================================
// 検査API
// ===========================================

/**
 * 検査項目結果
 */
export interface ItemResult {
  item_name: string;
  verdict: string;
  actual_value: string | null;
  expected_value: string | null;
}

/**
 * 検査実行レスポンス
 */
export interface InspectionResponse {
  inspection_id: number;
  device_id: number;
  serial_number: string;
  overall_result: string;
  items: ItemResult[];
}

/**
 * 検査履歴（一覧用）
 */
export interface InspectionSummary {
  id: number;
  device_id: number;
  serial_number: string | null;
  inspected_at: string;
  overall_result: string | null;
}

/**
 * 検査一覧レスポンス
 */
export interface InspectionListResponse {
  inspections: InspectionSummary[];
}

/**
 * 検査実行リクエスト
 */
export interface RunInspectionRequest {
  lot_id?: number;
}

/**
 * 検査を実行
 */
export async function runInspection(request: RunInspectionRequest): Promise<InspectionResponse> {
  const res = await fetch(`${API_BASE}/api/inspections`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(request),
  });
  if (!res.ok) {
    const error: ErrorResponse = await res.json();
    throw new Error(error.error);
  }
  return res.json();
}

/**
 * 検査履歴を取得
 */
export async function listInspections(): Promise<InspectionListResponse> {
  const res = await fetch(`${API_BASE}/api/inspections`);
  if (!res.ok) {
    const error: ErrorResponse = await res.json();
    throw new Error(error.error);
  }
  return res.json();
}
