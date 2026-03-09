// API呼び出し関数

import { Lot, CreateLotRequest, LotsResponse, CreateLotResponse } from "@/types/lot";

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
