// ロットの型定義（バックエンドと同期）

export interface Lot {
  lot_id: string;
  part_id: string;
  po_number?: string;
  arrival_date?: string;
  quantity: number;
  fw_version?: string;
  hw_version?: string;
  created_at: string;
  updated_at: string;
}

export interface CreateLotRequest {
  part_id: string;
  quantity: number;
  po_number?: string;
  arrival_date?: string;
  fw_version?: string;
  hw_version?: string;
}

export interface LotsResponse {
  lots: Lot[];
}

export interface CreateLotResponse {
  lot_id: string;
  message: string;
}

export interface ErrorResponse {
  error: string;
}
