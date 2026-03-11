"use client";

import { useState, useEffect } from "react";
import { Lot, CreateLotRequest, UpdateLotRequest } from "@/lib/api";

interface LotFormProps {
  lot: Lot | null;
  isNew: boolean;
  onSave: (request: CreateLotRequest | UpdateLotRequest) => void;
  onCancel: () => void;
  isSaving: boolean;
}

/**
 * ロット詳細/編集フォーム
 *
 * 新規作成と編集の両方に対応
 */
export function LotForm({
  lot,
  isNew,
  onSave,
  onCancel,
  isSaving,
}: LotFormProps) {
  const [lotNumber, setLotNumber] = useState("");
  const [expectedRateMs, setExpectedRateMs] = useState<string>("");
  const [expectedPortInProto, setExpectedPortInProto] = useState("");
  const [expectedPortOutProto, setExpectedPortOutProto] = useState("");
  const [memo, setMemo] = useState("");

  // ロットが変更されたらフォームをリセット
  useEffect(() => {
    if (lot) {
      setLotNumber(lot.lot_number);
      setExpectedRateMs(lot.expected_rate_ms?.toString() || "");
      setExpectedPortInProto(lot.expected_port_in_proto || "");
      setExpectedPortOutProto(lot.expected_port_out_proto || "");
      setMemo(lot.memo || "");
    } else {
      setLotNumber("");
      setExpectedRateMs("");
      setExpectedPortInProto("");
      setExpectedPortOutProto("");
      setMemo("");
    }
  }, [lot]);

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();

    const rateMs = expectedRateMs ? parseInt(expectedRateMs, 10) : undefined;

    if (isNew) {
      const request: CreateLotRequest = {
        lot_number: lotNumber,
        expected_rate_ms: rateMs,
        expected_port_in_proto: expectedPortInProto || undefined,
        expected_port_out_proto: expectedPortOutProto || undefined,
        memo: memo || undefined,
      };
      onSave(request);
    } else {
      const request: UpdateLotRequest = {
        expected_rate_ms: rateMs,
        expected_port_in_proto: expectedPortInProto || undefined,
        expected_port_out_proto: expectedPortOutProto || undefined,
        memo: memo || undefined,
      };
      onSave(request);
    }
  };

  return (
    <div className="rounded-lg border border-gray-200 bg-white p-4">
      <h2 className="mb-4 font-bold text-gray-900">
        {isNew ? "新規ロット作成" : "ロット詳細"}
      </h2>

      <form onSubmit={handleSubmit} className="space-y-4">
        {/* ロット番号 */}
        <div>
          <label className="block text-sm font-medium text-gray-700">
            ロット番号
          </label>
          <input
            type="text"
            value={lotNumber}
            onChange={(e) => setLotNumber(e.target.value)}
            disabled={!isNew}
            required
            placeholder="例: LOT-2026-001"
            className="mt-1 w-full rounded border border-gray-300 px-3 py-2 text-sm focus:border-blue-500 focus:outline-none disabled:bg-gray-100"
          />
        </div>

        {/* 期待値: 出力レート */}
        <div>
          <label className="block text-sm font-medium text-gray-700">
            出力レート (ms)
          </label>
          <input
            type="number"
            value={expectedRateMs}
            onChange={(e) => setExpectedRateMs(e.target.value)}
            placeholder="例: 100"
            min={1}
            className="mt-1 w-full rounded border border-gray-300 px-3 py-2 text-sm focus:border-blue-500 focus:outline-none"
          />
        </div>

        {/* 期待値: ポート設定 */}
        <div className="grid grid-cols-2 gap-3">
          <div>
            <label className="block text-sm font-medium text-gray-700">
              入力プロトコル
            </label>
            <select
              value={expectedPortInProto}
              onChange={(e) => setExpectedPortInProto(e.target.value)}
              className="mt-1 w-full rounded border border-gray-300 px-3 py-2 text-sm focus:border-blue-500 focus:outline-none"
            >
              <option value="">未設定</option>
              <option value="UBX">UBX</option>
              <option value="NMEA">NMEA</option>
              <option value="RTCM3">RTCM3</option>
            </select>
          </div>
          <div>
            <label className="block text-sm font-medium text-gray-700">
              出力プロトコル
            </label>
            <select
              value={expectedPortOutProto}
              onChange={(e) => setExpectedPortOutProto(e.target.value)}
              className="mt-1 w-full rounded border border-gray-300 px-3 py-2 text-sm focus:border-blue-500 focus:outline-none"
            >
              <option value="">未設定</option>
              <option value="UBX">UBX</option>
              <option value="NMEA">NMEA</option>
              <option value="RTCM3">RTCM3</option>
            </select>
          </div>
        </div>

        {/* メモ */}
        <div>
          <label className="block text-sm font-medium text-gray-700">
            メモ
          </label>
          <textarea
            value={memo}
            onChange={(e) => setMemo(e.target.value)}
            rows={2}
            className="mt-1 w-full rounded border border-gray-300 px-3 py-2 text-sm focus:border-blue-500 focus:outline-none"
          />
        </div>

        {/* ボタン */}
        <div className="flex gap-2">
          <button
            type="submit"
            disabled={isSaving || (isNew && !lotNumber.trim())}
            className="flex-1 rounded bg-blue-500 px-4 py-2 font-bold text-white hover:bg-blue-600 disabled:opacity-50"
          >
            {isSaving ? "保存中..." : "保存"}
          </button>
          {isNew && (
            <button
              type="button"
              onClick={onCancel}
              disabled={isSaving}
              className="rounded border border-gray-300 px-4 py-2 font-bold text-gray-700 hover:bg-gray-50 disabled:opacity-50"
            >
              キャンセル
            </button>
          )}
        </div>
      </form>
    </div>
  );
}
