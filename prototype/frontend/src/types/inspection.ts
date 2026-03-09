// 検査セッションの型定義（バックエンドと同期）

// マスタデータ（APIレスポンスと同期）
export interface Part {
  part_id: string;
  name: string;
  category: string;
  supplier_id: string;
}

export interface InspectionItem {
  item_id: string;
  name: string;
  type: string;
}

export interface Worker {
  worker_id: string;
  name: string;
  role: string;
}

// ロット（検査画面用に拡張）
export interface LotForInspection {
  lot_id: string;
  part_id: string;
  part_name: string;
  quantity: number;
  arrival_date?: string;
}

// 検査セッション
export interface InspectionSession {
  session_id: string;
  lot_id: string;
  item_id: string;
  worker_id: string;
  ok_count: number;
  ng_count: number;
  skip_count: number;
  started_at: string;
}

// APIリクエスト
export interface StartSessionRequest {
  lot_id: string;
  item_id: string;
  worker_id: string;
}

export interface CountRequest {
  result: "ok" | "ng" | "skip";
}

// APIレスポンス
export interface StartSessionResponse {
  session_id: string;
  message: string;
}

export interface CountResponse {
  ok_count: number;
  ng_count: number;
  skip_count: number;
}

export interface UndoResponse {
  ok_count: number;
  ng_count: number;
  skip_count: number;
  undone_result: "ok" | "ng" | "skip";
}

export interface FinishSessionResponse {
  message: string;
  inspection_record_id: string;
  man_hours: number;
}
