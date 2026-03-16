"use client";

import { useState, useEffect, useCallback } from "react";
import {
  OutdoorInspectionResult,
  OutdoorInspectionSnapshot,
  listOutdoorResults,
  getOutdoorSnapshots,
} from "@/lib/api";
import { MonSpanComparePanel } from "@/components/MonSpanComparePanel";

/**
 * スペクトラム比較ページ
 *
 * 2つの検査データを選択し、RF波形を重ねて比較
 */
export default function SpectrumComparePage() {
  // 検査一覧
  const [results, setResults] = useState<OutdoorInspectionResult[]>([]);

  // 基準データ
  const [primaryResultId, setPrimaryResultId] = useState<number | null>(null);
  const [primarySnapshots, setPrimarySnapshots] = useState<OutdoorInspectionSnapshot[]>([]);
  const [primarySnapshotIndex, setPrimarySnapshotIndex] = useState(0);

  // 比較データ
  const [secondaryResultId, setSecondaryResultId] = useState<number | null>(null);
  const [secondarySnapshots, setSecondarySnapshots] = useState<OutdoorInspectionSnapshot[]>([]);
  const [secondarySnapshotIndex, setSecondarySnapshotIndex] = useState(0);

  // ローディング・エラー
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  // 検査一覧を取得
  const fetchResults = useCallback(async () => {
    try {
      setIsLoading(true);
      const res = await listOutdoorResults();
      const sorted = res.results.sort((a, b) => b.id - a.id);
      setResults(sorted);
    } catch (e) {
      setError(e instanceof Error ? e.message : "データの取得に失敗しました");
    } finally {
      setIsLoading(false);
    }
  }, []);

  // 初回読み込み
  useEffect(() => {
    fetchResults();
  }, [fetchResults]);

  // 基準データ選択時
  const handleSelectPrimary = async (id: number) => {
    setPrimaryResultId(id);
    setPrimarySnapshotIndex(0);
    setPrimarySnapshots([]);

    try {
      const res = await getOutdoorSnapshots(id);
      setPrimarySnapshots(res.snapshots);
    } catch (e) {
      setError(e instanceof Error ? e.message : "スナップショットの取得に失敗しました");
    }
  };

  // 比較データ選択時
  const handleSelectSecondary = async (id: number) => {
    setSecondaryResultId(id);
    setSecondarySnapshotIndex(0);
    setSecondarySnapshots([]);

    try {
      const res = await getOutdoorSnapshots(id);
      setSecondarySnapshots(res.snapshots);
    } catch (e) {
      setError(e instanceof Error ? e.message : "スナップショットの取得に失敗しました");
    }
  };

  // 選択中のデータ
  const primaryResult = results.find((r) => r.id === primaryResultId);
  const secondaryResult = results.find((r) => r.id === secondaryResultId);
  const primarySnapshot = primarySnapshots[primarySnapshotIndex];
  const secondarySnapshot = secondarySnapshots[secondarySnapshotIndex];

  // 日時フォーマット
  const formatDateTime = (isoString: string) => {
    const date = new Date(isoString);
    return date.toLocaleString("ja-JP", {
      month: "2-digit",
      day: "2-digit",
      hour: "2-digit",
      minute: "2-digit",
    });
  };

  // ラベル生成
  const createLabel = (result: OutdoorInspectionResult | undefined, snapshotIndex: number) => {
    if (!result) return "";
    const sn = result.serial_number ? result.serial_number.slice(-8) : "未登録";
    return `ID:${result.id} S/N:${sn} #${snapshotIndex + 1}`;
  };

  return (
    <div className="min-h-screen bg-gray-50">
      {/* ページタイトル */}
      <div className="bg-white border-b border-gray-200">
        <div className="container mx-auto px-4 py-4">
          <div className="flex items-center justify-between">
            <h1 className="text-xl font-semibold text-gray-800">スペクトラム比較</h1>
            <a
              href="/inspections/history"
              className="text-blue-600 hover:underline text-sm"
            >
              ← 検査履歴へ
            </a>
          </div>
          <p className="text-sm text-gray-500 mt-1">
            2つのスナップショットを選択してRF波形を比較
          </p>
        </div>
      </div>

      {/* メインコンテンツ */}
      <main className="container mx-auto px-4 py-6">
        {/* データ選択UI */}
        <div className="mb-6 grid gap-4 md:grid-cols-2">
          {/* 基準データ選択 */}
          <div className="rounded border border-blue-200 bg-blue-50 p-4">
            <h3 className="mb-3 font-medium text-blue-700">基準データ</h3>

            {/* 検査選択 */}
            <div className="mb-3">
              <label className="block text-sm text-gray-600 mb-1">検査を選択</label>
              <select
                value={primaryResultId ?? ""}
                onChange={(e) => {
                  const id = e.target.value ? Number(e.target.value) : null;
                  if (id) handleSelectPrimary(id);
                }}
                className="w-full rounded border border-gray-300 p-2 text-sm"
              >
                <option value="">選択してください</option>
                {results.map((result) => (
                  <option key={result.id} value={result.id}>
                    ID:{result.id} - {result.serial_number ? `S/N:${result.serial_number.slice(-8)}` : "S/N:未登録"} - {formatDateTime(result.inspected_at)}
                  </option>
                ))}
              </select>
            </div>

            {/* スナップショット選択 */}
            {primarySnapshots.length > 0 && (
              <div>
                <label className="block text-sm text-gray-600 mb-1">
                  スナップショット ({primarySnapshots.length}件)
                </label>
                <div className="flex items-center gap-2 flex-wrap">
                  {primarySnapshots.map((_, index) => (
                    <button
                      key={index}
                      onClick={() => setPrimarySnapshotIndex(index)}
                      className={`w-8 h-8 rounded-full text-sm font-medium transition-colors ${
                        index === primarySnapshotIndex
                          ? "bg-blue-600 text-white"
                          : "bg-white text-gray-700 hover:bg-blue-100 border border-gray-300"
                      }`}
                    >
                      {index + 1}
                    </button>
                  ))}
                </div>
              </div>
            )}
          </div>

          {/* 比較データ選択 */}
          <div className="rounded border border-orange-200 bg-orange-50 p-4">
            <h3 className="mb-3 font-medium text-orange-700">比較データ</h3>

            {/* 検査選択 */}
            <div className="mb-3">
              <label className="block text-sm text-gray-600 mb-1">検査を選択</label>
              <select
                value={secondaryResultId ?? ""}
                onChange={(e) => {
                  const id = e.target.value ? Number(e.target.value) : null;
                  if (id) handleSelectSecondary(id);
                }}
                className="w-full rounded border border-gray-300 p-2 text-sm"
              >
                <option value="">選択してください</option>
                {results.map((result) => (
                  <option key={result.id} value={result.id}>
                    ID:{result.id} - {result.serial_number ? `S/N:${result.serial_number.slice(-8)}` : "S/N:未登録"} - {formatDateTime(result.inspected_at)}
                  </option>
                ))}
              </select>
            </div>

            {/* スナップショット選択 */}
            {secondarySnapshots.length > 0 && (
              <div>
                <label className="block text-sm text-gray-600 mb-1">
                  スナップショット ({secondarySnapshots.length}件)
                </label>
                <div className="flex items-center gap-2 flex-wrap">
                  {secondarySnapshots.map((_, index) => (
                    <button
                      key={index}
                      onClick={() => setSecondarySnapshotIndex(index)}
                      className={`w-8 h-8 rounded-full text-sm font-medium transition-colors ${
                        index === secondarySnapshotIndex
                          ? "bg-orange-600 text-white"
                          : "bg-white text-gray-700 hover:bg-orange-100 border border-gray-300"
                      }`}
                    >
                      {index + 1}
                    </button>
                  ))}
                </div>
              </div>
            )}
          </div>
        </div>

        {/* 比較パネル */}
        <MonSpanComparePanel
          primary={
            primarySnapshot?.data?.mon_span
              ? {
                  label: createLabel(primaryResult, primarySnapshotIndex),
                  serialNumber: primaryResult?.serial_number ?? undefined,
                  data: primarySnapshot.data.mon_span,
                }
              : null
          }
          secondary={
            secondarySnapshot?.data?.mon_span
              ? {
                  label: createLabel(secondaryResult, secondarySnapshotIndex),
                  serialNumber: secondaryResult?.serial_number ?? undefined,
                  data: secondarySnapshot.data.mon_span,
                }
              : null
          }
        />

        {/* エラー表示 */}
        {error && (
          <div className="mt-6 rounded border border-red-200 bg-red-50 p-4 text-red-700">
            {error}
          </div>
        )}

        {/* ローディング */}
        {isLoading && (
          <div className="mt-6 rounded border border-gray-200 bg-white p-8 text-center text-gray-500">
            読み込み中...
          </div>
        )}
      </main>
    </div>
  );
}
