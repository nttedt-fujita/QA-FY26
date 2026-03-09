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
