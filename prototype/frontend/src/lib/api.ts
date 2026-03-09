// API呼び出し関数

import { Lot, CreateLotRequest, LotsResponse, CreateLotResponse } from "@/types/lot";
import {
  Part,
  InspectionItem,
  Worker,
  LotForInspection,
  StartSessionRequest,
  StartSessionResponse,
  CountRequest,
  CountResponse,
  UndoResponse,
  FinishSessionResponse,
} from "@/types/inspection";

const API_BASE = process.env.NEXT_PUBLIC_API_URL || "http://localhost:8080";

// ロット一覧を取得
export async function getLots(): Promise<Lot[]> {
  const res = await fetch(`${API_BASE}/api/v1/lots`);
  if (!res.ok) {
    throw new Error("ロット一覧の取得に失敗しました");
  }
  const data: LotsResponse = await res.json();
  return data.lots ?? [];
}

// ロットを登録
export async function createLot(req: CreateLotRequest): Promise<CreateLotResponse> {
  const res = await fetch(`${API_BASE}/api/v1/lots`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify(req),
  });
  if (!res.ok) {
    const err = await res.json();
    throw new Error(err.error || "ロットの登録に失敗しました");
  }
  return res.json();
}

// ロット詳細を取得
export async function getLot(lotId: string): Promise<Lot> {
  const res = await fetch(`${API_BASE}/api/v1/lots/${lotId}`);
  if (!res.ok) {
    if (res.status === 404) {
      throw new Error("ロットが見つかりません");
    }
    throw new Error("ロットの取得に失敗しました");
  }
  return res.json();
}

// === マスタデータAPI ===

// 部品一覧を取得
export async function getParts(): Promise<Part[]> {
  const res = await fetch(`${API_BASE}/api/v1/parts`);
  if (!res.ok) {
    throw new Error("部品一覧の取得に失敗しました");
  }
  const data = await res.json();
  return data.parts ?? [];
}

// 検査項目一覧を取得
export async function getInspectionItems(): Promise<InspectionItem[]> {
  const res = await fetch(`${API_BASE}/api/v1/inspection-items`);
  if (!res.ok) {
    throw new Error("検査項目一覧の取得に失敗しました");
  }
  const data = await res.json();
  return data.inspection_items ?? [];
}

// 作業者一覧を取得
export async function getWorkers(): Promise<Worker[]> {
  const res = await fetch(`${API_BASE}/api/v1/workers`);
  if (!res.ok) {
    throw new Error("作業者一覧の取得に失敗しました");
  }
  const data = await res.json();
  return data.workers ?? [];
}

// 検査用ロット一覧を取得（部品名付き）
export async function getLotsForInspection(): Promise<LotForInspection[]> {
  const [lotsRes, partsRes] = await Promise.all([
    fetch(`${API_BASE}/api/v1/lots`),
    fetch(`${API_BASE}/api/v1/parts`),
  ]);
  if (!lotsRes.ok || !partsRes.ok) {
    throw new Error("データの取得に失敗しました");
  }
  const lotsData = await lotsRes.json();
  const partsData = await partsRes.json();
  const partsMap = new Map<string, Part>(
    (partsData.parts ?? []).map((p: Part) => [p.part_id, p])
  );
  return (lotsData.lots ?? []).map((lot: Lot) => ({
    lot_id: lot.lot_id,
    part_id: lot.part_id,
    part_name: partsMap.get(lot.part_id)?.name ?? lot.part_id,
    quantity: lot.quantity,
    arrival_date: lot.arrival_date,
  }));
}

// === 検査セッションAPI ===

// 検査セッション開始
export async function startInspectionSession(
  req: StartSessionRequest
): Promise<StartSessionResponse> {
  const res = await fetch(`${API_BASE}/api/v1/inspection-sessions`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(req),
  });
  if (!res.ok) {
    const err = await res.json();
    throw new Error(err.error || "検査セッションの開始に失敗しました");
  }
  return res.json();
}

// カウント追加
export async function addCount(
  sessionId: string,
  req: CountRequest
): Promise<CountResponse> {
  const res = await fetch(
    `${API_BASE}/api/v1/inspection-sessions/${sessionId}/count`,
    {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify(req),
    }
  );
  if (!res.ok) {
    const err = await res.json();
    throw new Error(err.error || "カウントの追加に失敗しました");
  }
  return res.json();
}

// カウント取り消し
export async function undoCount(sessionId: string): Promise<UndoResponse> {
  const res = await fetch(
    `${API_BASE}/api/v1/inspection-sessions/${sessionId}/count`,
    { method: "DELETE" }
  );
  if (!res.ok) {
    const err = await res.json();
    throw new Error(err.error || "取り消しに失敗しました");
  }
  return res.json();
}

// 検査セッション終了
export async function finishInspectionSession(
  sessionId: string
): Promise<FinishSessionResponse> {
  const res = await fetch(
    `${API_BASE}/api/v1/inspection-sessions/${sessionId}/finish`,
    { method: "POST" }
  );
  if (!res.ok) {
    const err = await res.json();
    throw new Error(err.error || "検査終了に失敗しました");
  }
  return res.json();
}

// === 検査記録一覧API ===

export interface InspectionRecordWithDetails {
  record_id: string;
  lot_id: string;
  part_name: string;
  item_name: string;
  worker_name?: string;
  inspection_date: string;
  sample_qty?: number;
  result: string;
  defect_qty: number;
  work_time_min?: number;
  note?: string;
  created_at: string;
}

export interface InspectionRecordsResponse {
  records: InspectionRecordWithDetails[];
  total: number;
  limit: number;
  offset: number;
}

export interface ListFilter {
  date_from?: string;
  date_to?: string;
  lot_id?: string;
  part_id?: string;
  result?: string;
  limit?: number;
  offset?: number;
}

// 検査記録一覧を取得
export async function getInspectionRecords(
  filter?: ListFilter
): Promise<InspectionRecordsResponse> {
  const params = new URLSearchParams();
  if (filter?.date_from) params.set("date_from", filter.date_from);
  if (filter?.date_to) params.set("date_to", filter.date_to);
  if (filter?.lot_id) params.set("lot_id", filter.lot_id);
  if (filter?.part_id) params.set("part_id", filter.part_id);
  if (filter?.result) params.set("result", filter.result);
  if (filter?.limit) params.set("limit", String(filter.limit));
  if (filter?.offset) params.set("offset", String(filter.offset));

  const url = `${API_BASE}/api/v1/inspection-records?${params.toString()}`;
  const res = await fetch(url);
  if (!res.ok) {
    throw new Error("検査記録一覧の取得に失敗しました");
  }
  return res.json();
}

// === ダッシュボードAPI ===

export interface SummaryStats {
  total_inspections: number;
  total_lots: number;
  pass_rate: number;
  avg_work_time: number;
}

export interface MonthlyStats {
  month: string;
  total: number;
  pass_qty: number;
  fail_qty: number;
  skip_qty: number;
  pass_rate: number;
}

export interface PartDefectRate {
  part_id: string;
  part_name: string;
  total_qty: number;
  defect_qty: number;
  defect_rate: number;
}

// サマリー情報を取得
export async function getDashboardSummary(): Promise<SummaryStats> {
  const res = await fetch(`${API_BASE}/api/v1/dashboard/summary`);
  if (!res.ok) {
    throw new Error("サマリー情報の取得に失敗しました");
  }
  return res.json();
}

// 月別統計を取得
export async function getDashboardMonthly(
  months: number = 6
): Promise<MonthlyStats[]> {
  const res = await fetch(`${API_BASE}/api/v1/dashboard/monthly?months=${months}`);
  if (!res.ok) {
    throw new Error("月別統計の取得に失敗しました");
  }
  const data = await res.json();
  return data.monthly_stats ?? [];
}

// 不良トップ部品を取得
export async function getDashboardTopDefects(
  limit: number = 5,
  months: number = 3
): Promise<PartDefectRate[]> {
  const res = await fetch(
    `${API_BASE}/api/v1/dashboard/top-defects?limit=${limit}&months=${months}`
  );
  if (!res.ok) {
    throw new Error("不良トップ部品の取得に失敗しました");
  }
  const data = await res.json();
  return data.top_defects ?? [];
}

// 検査項目別統計
export interface ItemStats {
  item_id: string;
  item_name: string;
  total: number;
  pass_qty: number;
  fail_qty: number;
  pass_rate: number;
}

export async function getDashboardItemStats(): Promise<ItemStats[]> {
  const res = await fetch(`${API_BASE}/api/v1/dashboard/items`);
  if (!res.ok) {
    throw new Error("検査項目別統計の取得に失敗しました");
  }
  const data = await res.json();
  return data.item_stats ?? [];
}

// 直近の検査記録
export interface RecentRecord {
  record_id: string;
  lot_id: string;
  part_name: string;
  item_name: string;
  result: string;
  inspection_date: string;
}

export async function getDashboardRecent(limit: number = 5): Promise<RecentRecord[]> {
  const res = await fetch(`${API_BASE}/api/v1/dashboard/recent?limit=${limit}`);
  if (!res.ok) {
    throw new Error("直近の検査記録の取得に失敗しました");
  }
  const data = await res.json();
  return data.recent_records ?? [];
}

// サプライヤー別不良率
export interface SupplierDefectRate {
  supplier_id: string;
  supplier_name: string;
  total_qty: number;
  defect_qty: number;
  defect_rate: number;
}

export async function getDashboardSuppliers(): Promise<SupplierDefectRate[]> {
  const res = await fetch(`${API_BASE}/api/v1/dashboard/suppliers`);
  if (!res.ok) {
    throw new Error("サプライヤー別不良率の取得に失敗しました");
  }
  const data = await res.json();
  return data.supplier_defects ?? [];
}
