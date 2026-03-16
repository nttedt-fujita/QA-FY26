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
  /** USBシリアル番号（FTDIチップ等）- 参考用 */
  serial_number: string | null;
  /** F9Pチップのシリアル番号（UBX-SEC-UNIQID）- DB紐付け用 */
  f9p_serial: string | null;
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

/**
 * LED点滅レスポンス
 */
export interface BlinkResponse {
  path: string;
  message: string;
  send_count: number;
}

/**
 * 装置のLEDを点滅させる（物理的な識別用）
 */
export async function blinkDevice(path: string, durationSec: number = 3): Promise<BlinkResponse> {
  const encodedPath = encodeURIComponent(path);
  const res = await fetch(`${API_BASE}/api/devices/${encodedPath}/blink?duration_sec=${durationSec}`, {
    method: "POST",
  });
  if (!res.ok) {
    const error: ErrorResponse = await res.json();
    throw new Error(error.error);
  }
  return res.json();
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

// ===========================================
// 一括検査API（Phase 3: 複数台対応）
// ===========================================

/**
 * 個別デバイスの検査結果
 */
export interface DeviceInspectionResult {
  path: string;
  serial_number: string;
  overall_result: string;
  inspection_id: number;
  items: ItemResult[];
}

/**
 * 一括検査サマリー
 */
export interface BatchSummary {
  total: number;
  passed: number;
  failed: number;
}

/**
 * 一括検査レスポンス
 */
export interface BatchInspectionResponse {
  results: DeviceInspectionResult[];
  summary: BatchSummary;
}

/**
 * 一括検査リクエスト
 */
export interface BatchInspectionRequest {
  lot_id?: number;
}

/**
 * 一括検査を実行（接続中の全装置）
 */
export async function runBatchInspection(request: BatchInspectionRequest): Promise<BatchInspectionResponse> {
  const res = await fetch(`${API_BASE}/api/inspections/batch`, {
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

// ===========================================
// NAV-SIG API（衛星信号情報）
// ===========================================

/**
 * 衛星信号情報
 */
export interface NavSignal {
  gnss_id: number;     // 0=GPS, 1=SBAS, 2=Galileo, 3=BeiDou, 5=QZSS, 6=GLONASS
  sv_id: number;       // 衛星番号
  sig_id: number;      // 信号識別子
  cno: number;         // C/N0 [dBHz]
  quality_ind: number; // 品質指標（0-7）
  is_l1: boolean;      // L1帯か
  is_l2: boolean;      // L2帯か
}

/**
 * 信号統計情報
 */
export interface SignalStats {
  gps_visible_count: number;       // GPS L1可視衛星数
  gps_l2_reception_count: number;  // GPS L2受信中衛星数
  gps_l2_reception_rate: number;   // L2受信率（0.0〜1.0）
}

/**
 * NAV-SIGレスポンス
 */
export interface NavSigResponse {
  signals: NavSignal[];
  stats: SignalStats;
}

/**
 * NAV-SIG（衛星信号情報）を取得
 */
export async function getNavSig(signal?: AbortSignal): Promise<NavSigResponse> {
  const res = await fetch(`${API_BASE}/api/nav-sig`, { signal });
  if (!res.ok) {
    const error: ErrorResponse = await res.json();
    throw new Error(error.error);
  }
  return res.json();
}

// ===========================================
// MON-SPAN API（スペクトラム情報）
// ===========================================

/**
 * スペクトラムブロック情報
 */
export interface SpanBlock {
  spectrum: number[];  // 256点のスペクトラム振幅（dB）
  span: number;        // 周波数スパン（Hz）
  res: number;         // 周波数分解能（Hz）
  center: number;      // 中心周波数（Hz）
  pga: number;         // PGAゲイン（dB）
  max_amplitude: number;  // スペクトラム最大値（dB）
  avg_amplitude: number;  // スペクトラム平均値（dB）
}

/**
 * MON-SPANレスポンス
 */
export interface MonSpanResponse {
  blocks: SpanBlock[];
}

/**
 * MON-SPAN（スペクトラム情報）を取得
 */
export async function getMonSpan(signal?: AbortSignal): Promise<MonSpanResponse> {
  const res = await fetch(`${API_BASE}/api/mon-span`, { signal });
  if (!res.ok) {
    const error: ErrorResponse = await res.json();
    throw new Error(error.error);
  }
  return res.json();
}

// ===========================================
// NAV-STATUS API（Fix状態・TTFF）
// ===========================================

/**
 * NAV-STATUSレスポンス
 */
export interface NavStatusResponse {
  ttff: number;         // Time to First Fix (ms)
  msss: number;         // 起動からの経過時間 (ms)
  gps_fix: number;      // FIXタイプ（0=No fix, 2=2D, 3=3D）
  gps_fix_ok: boolean;  // 位置・速度が有効か
  is_fix_valid: boolean; // FIXが有効か
  carr_soln: number;    // RTK状態（0=なし, 1=Float, 2=Fixed）
  is_rtk_fixed: boolean;
  is_rtk_float: boolean;
}

/**
 * NAV-STATUS（Fix状態・TTFF）を取得
 */
export async function getNavStatus(signal?: AbortSignal): Promise<NavStatusResponse> {
  const res = await fetch(`${API_BASE}/api/nav-status`, { signal });
  if (!res.ok) {
    const error: ErrorResponse = await res.json();
    throw new Error(error.error);
  }
  return res.json();
}

// ===========================================
// NAV-SAT API（衛星情報・スカイプロット用）
// ===========================================

/**
 * 衛星情報
 */
export interface SatelliteInfo {
  gnss_id: number;     // 0=GPS, 1=SBAS, 2=Galileo, 3=BeiDou, 5=QZSS, 6=GLONASS
  gnss_name: string;   // GNSS名
  sv_id: number;       // 衛星番号
  cno: number;         // C/N0 [dBHz]
  elev: number;        // 仰角 [deg] (-90〜+90)
  azim: number;        // 方位角 [deg] (0〜360)
  is_used: boolean;    // ナビゲーションに使用中か
  quality_ind: number; // 品質指標（0-7）
  health: number;      // 健全性（0=unknown, 1=healthy, 2=unhealthy）
}

/**
 * GNSS別衛星数
 */
export interface GnssCounts {
  gps: number;
  sbas: number;
  galileo: number;
  beidou: number;
  qzss: number;
  glonass: number;
}

/**
 * 衛星統計情報
 */
export interface SatelliteStats {
  total_count: number;
  used_count: number;
  gnss_counts: GnssCounts;
}

/**
 * NAV-SATレスポンス
 */
export interface NavSatResponse {
  satellites: SatelliteInfo[];
  stats: SatelliteStats;
}

/**
 * NAV-SAT（衛星情報）を取得
 */
export async function getNavSat(signal?: AbortSignal): Promise<NavSatResponse> {
  const res = await fetch(`${API_BASE}/api/nav-sat`, { signal });
  if (!res.ok) {
    const error: ErrorResponse = await res.json();
    throw new Error(error.error);
  }
  return res.json();
}

// ===========================================
// GNSS State API（統合API）
// ===========================================

/**
 * NAV-PVTレスポンス
 */
export interface NavPvtResponse {
  lat: number;
  lon: number;
  fix_type: number;
  carr_soln: number;
  num_sv: number;
  h_acc: number;
  v_acc: number;
  is_rtk_fixed: boolean;
  is_rtk_float: boolean;
}

/**
 * MON-RFブロックレスポンス
 */
export interface RfBlockResponse {
  block_id: number;
  jamming_state: number;
  ant_status: number;
  ant_power: number;
  noise_per_ms: number;
  agc_cnt: number;
  cw_suppression: number;
  is_jamming_detected: boolean;
  is_antenna_ok: boolean;
}

/**
 * MON-RFレスポンス
 */
export interface MonRfResponse {
  blocks: RfBlockResponse[];
  has_jamming: boolean;
  has_critical_jamming: boolean;
}

/**
 * 統合APIレスポンス
 */
export interface GnssStateResponse {
  nav_pvt: NavPvtResponse | null;
  nav_status: NavStatusResponse | null;
  nav_sat: NavSatResponse | null;
  nav_sig: NavSigResponse | null;
  mon_span: MonSpanResponse | null;
  mon_rf: MonRfResponse | null;
  errors: string[];
}

/**
 * GNSS状態（統合API）を取得
 * @param signal AbortSignal
 * @param devicePath 装置パス（指定時はパス指定APIを使用）
 */
export async function getGnssState(signal?: AbortSignal, devicePath?: string): Promise<GnssStateResponse> {
  // パス指定があればパス指定APIを使用（Phase 3: 複数台対応）
  const url = devicePath
    ? `${API_BASE}/api/devices/${encodeURIComponent(devicePath)}/gnss-state`
    : `${API_BASE}/api/gnss-state`;

  const res = await fetch(url, { signal });
  if (!res.ok) {
    const error: ErrorResponse = await res.json();
    throw new Error(error.error);
  }
  return res.json();
}

// ===========================================
// 屋外検査結果API
// ===========================================

/**
 * 屋外検査結果
 */
export interface OutdoorInspectionResult {
  id: number;
  device_id: number | null;
  serial_number: string | null;
  lot_id: number | null;
  inspected_at: string;
  duration_sec: number;
  sample_count: number;
  rtk_fix_rate: number;
  rtk_fix_time_ms: number | null;
  l2_reception_rate: number;
  l1_min_cno: number;
  is_pass: boolean;
  l1_cno_pass: boolean;
  l2_rate_pass: boolean;
  rtk_fix_time_pass: boolean;
  rtk_fix_rate_pass: boolean;
  failure_reasons: string[];
  created_at: string;
}

/**
 * 屋外検査結果一覧レスポンス
 */
export interface OutdoorResultListResponse {
  results: OutdoorInspectionResult[];
}

/**
 * スナップショット
 */
export interface OutdoorInspectionSnapshot {
  id: number;
  timestamp_ms: number;
  data: GnssStateResponse;
}

/**
 * スナップショット一覧レスポンス
 */
export interface OutdoorSnapshotsResponse {
  inspection_id: number;
  snapshots: OutdoorInspectionSnapshot[];
}

/**
 * 屋外検査結果一覧を取得
 */
export async function listOutdoorResults(): Promise<OutdoorResultListResponse> {
  const res = await fetch(`${API_BASE}/api/outdoor-inspection-results`);
  if (!res.ok) {
    const error: ErrorResponse = await res.json();
    throw new Error(error.error);
  }
  return res.json();
}

/**
 * 屋外検査結果のスナップショットを取得
 */
export async function getOutdoorSnapshots(inspectionId: number): Promise<OutdoorSnapshotsResponse> {
  const res = await fetch(`${API_BASE}/api/outdoor-inspection-results/${inspectionId}/snapshots`);
  if (!res.ok) {
    const error: ErrorResponse = await res.json();
    throw new Error(error.error);
  }
  return res.json();
}
