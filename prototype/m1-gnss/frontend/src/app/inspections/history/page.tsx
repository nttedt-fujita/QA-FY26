"use client";

import { useState, useEffect, useCallback } from "react";
import {
  OutdoorInspectionResult,
  OutdoorInspectionSnapshot,
  listOutdoorResults,
  getOutdoorSnapshots,
  GnssStateResponse,
} from "@/lib/api";
import { NavSigPanel } from "@/components/NavSigPanel";
import { MonSpanPanel } from "@/components/MonSpanPanel";
import { NavStatusPanel } from "@/components/NavStatusPanel";
import { SkyPlotPanel } from "@/components/SkyPlotPanel";

/**
 * 検査履歴再生画面
 *
 * DBに保存されたスナップショットを再生表示
 */
export default function InspectionHistoryPage() {
  // 検査一覧
  const [results, setResults] = useState<OutdoorInspectionResult[]>([]);
  const [selectedResultId, setSelectedResultId] = useState<number | null>(null);

  // スナップショット
  const [snapshots, setSnapshots] = useState<OutdoorInspectionSnapshot[]>([]);
  const [selectedSnapshotIndex, setSelectedSnapshotIndex] = useState(0);

  // ローディング・エラー
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  // 検査一覧を取得
  const fetchResults = useCallback(async () => {
    try {
      setIsLoading(true);
      const res = await listOutdoorResults();
      // 新しい順にソート
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

  // 検査選択時にスナップショットを取得
  const handleSelectResult = async (id: number) => {
    setSelectedResultId(id);
    setSelectedSnapshotIndex(0);
    setSnapshots([]);

    try {
      setIsLoading(true);
      const res = await getOutdoorSnapshots(id);
      setSnapshots(res.snapshots);
    } catch (e) {
      setError(e instanceof Error ? e.message : "スナップショットの取得に失敗しました");
    } finally {
      setIsLoading(false);
    }
  };

  // 選択中の検査
  const selectedResult = results.find((r) => r.id === selectedResultId);

  // 選択中のスナップショット
  const selectedSnapshot = snapshots[selectedSnapshotIndex];
  const snapshotData: GnssStateResponse | null = selectedSnapshot?.data ?? null;

  // タイムスタンプをフォーマット
  const formatTime = (timestampMs: number) => {
    const date = new Date(timestampMs);
    return date.toLocaleTimeString("ja-JP");
  };

  // 検査日時をフォーマット
  const formatDateTime = (isoString: string) => {
    const date = new Date(isoString);
    return date.toLocaleString("ja-JP", {
      month: "2-digit",
      day: "2-digit",
      hour: "2-digit",
      minute: "2-digit",
    });
  };

  return (
    <div className="min-h-screen bg-gray-50">
      {/* ページタイトル */}
      <div className="bg-white border-b border-gray-200">
        <div className="container mx-auto px-4 py-4">
          <div className="flex items-center justify-between">
            <h1 className="text-xl font-semibold text-gray-800">検査履歴再生</h1>
            <a
              href="/inspections/outdoor"
              className="text-blue-600 hover:underline text-sm"
            >
              ← 屋外検査へ
            </a>
          </div>
          <p className="text-sm text-gray-500 mt-1">
            保存されたスナップショットを再生表示
          </p>
        </div>
      </div>

      {/* メインコンテンツ */}
      <main className="container mx-auto px-4 py-6">
        {/* 検査選択 */}
        <div className="mb-6 rounded border border-gray-200 bg-white p-4">
          <h3 className="mb-3 font-medium text-gray-700">検査選択</h3>
          <select
            value={selectedResultId ?? ""}
            onChange={(e) => {
              const id = e.target.value ? Number(e.target.value) : null;
              if (id) handleSelectResult(id);
            }}
            className="w-full rounded border border-gray-300 p-2"
            disabled={isLoading}
          >
            <option value="">検査を選択してください</option>
            {results.map((result) => (
              <option key={result.id} value={result.id}>
                ID:{result.id} - {formatDateTime(result.inspected_at)} -{" "}
                {result.is_pass ? "合格" : "不合格"} ({result.sample_count}サンプル)
              </option>
            ))}
          </select>
        </div>

        {/* 選択中の検査情報 */}
        {selectedResult && (
          <div
            className={`mb-6 rounded border p-4 ${
              selectedResult.is_pass
                ? "border-green-300 bg-green-50"
                : "border-red-300 bg-red-50"
            }`}
          >
            <div className="flex items-center justify-between mb-3">
              <h3 className="font-medium text-gray-700">検査ID: {selectedResult.id}</h3>
              <span
                className={`rounded px-3 py-1 font-bold ${
                  selectedResult.is_pass
                    ? "bg-green-500 text-white"
                    : "bg-red-500 text-white"
                }`}
              >
                {selectedResult.is_pass ? "合格" : "不合格"}
              </span>
            </div>
            <div className="grid gap-2 md:grid-cols-4 text-sm">
              <div>
                <span className="text-gray-500">L1最小C/N0:</span>{" "}
                <span className="font-mono">{selectedResult.l1_min_cno.toFixed(1)} dBHz</span>
              </div>
              <div>
                <span className="text-gray-500">L2受信率:</span>{" "}
                <span className="font-mono">{(selectedResult.l2_reception_rate * 100).toFixed(1)}%</span>
              </div>
              <div>
                <span className="text-gray-500">RTK FIX時間:</span>{" "}
                <span className="font-mono">
                  {selectedResult.rtk_fix_time_ms !== null
                    ? `${(selectedResult.rtk_fix_time_ms / 1000).toFixed(1)}秒`
                    : "FIXせず"}
                </span>
              </div>
              <div>
                <span className="text-gray-500">RTK FIX率:</span>{" "}
                <span className="font-mono">{(selectedResult.rtk_fix_rate * 100).toFixed(1)}%</span>
              </div>
            </div>
          </div>
        )}

        {/* タイムライン */}
        {snapshots.length > 0 && (
          <div className="mb-6 rounded border border-gray-200 bg-white p-4">
            <h3 className="mb-3 font-medium text-gray-700">
              タイムライン ({snapshots.length}件)
            </h3>
            <div className="flex items-center gap-2 flex-wrap mb-3">
              {snapshots.map((snapshot, index) => (
                <button
                  key={snapshot.id}
                  onClick={() => setSelectedSnapshotIndex(index)}
                  className={`w-8 h-8 rounded-full text-sm font-medium transition-colors ${
                    index === selectedSnapshotIndex
                      ? "bg-blue-600 text-white"
                      : "bg-gray-200 text-gray-700 hover:bg-gray-300"
                  }`}
                >
                  {index + 1}
                </button>
              ))}
            </div>
            {selectedSnapshot && (
              <div className="text-sm text-gray-600">
                <span className="font-mono">
                  {formatTime(selectedSnapshot.timestamp_ms)}
                </span>
                <span className="ml-2">
                  (サンプル {selectedSnapshotIndex + 1}/{snapshots.length})
                </span>
              </div>
            )}
          </div>
        )}

        {/* スナップショット未選択時 */}
        {!selectedResultId && (
          <div className="mb-6 rounded border border-gray-200 bg-white p-8 text-center text-gray-500">
            検査を選択するとスナップショットが表示されます
          </div>
        )}

        {/* スナップショットがない場合 */}
        {selectedResultId && snapshots.length === 0 && !isLoading && (
          <div className="mb-6 rounded border border-yellow-200 bg-yellow-50 p-4 text-yellow-700">
            この検査にはスナップショットが保存されていません
          </div>
        )}

        {/* パネル表示（スナップショット選択時） */}
        {snapshotData && (
          <>
            {/* NAV-STATUS（Fix状態・TTFF）パネル */}
            <div className="mb-6">
              <NavStatusPanel
                data={snapshotData.nav_status ?? null}
                error={null}
                isLoading={false}
                isConnected={true}
              />
            </div>

            {/* スカイプロット + NAV-SIG 横並び */}
            <div className="mb-6 grid gap-4 md:grid-cols-2">
              {/* スカイプロット（NAV-SAT） */}
              <SkyPlotPanel
                data={snapshotData.nav_sat ?? null}
                error={null}
                isLoading={false}
                isConnected={true}
              />
              {/* NAV-SIG（衛星信号） */}
              <NavSigPanel
                data={snapshotData.nav_sig ?? null}
                error={null}
                isLoading={false}
                isConnected={true}
              />
            </div>

            {/* MON-SPAN（スペクトラム）パネル */}
            <div className="mb-6">
              <MonSpanPanel
                data={snapshotData.mon_span ?? null}
                error={null}
                isLoading={false}
                isConnected={true}
              />
            </div>
          </>
        )}

        {/* エラー表示 */}
        {error && (
          <div className="mb-6 rounded border border-red-200 bg-red-50 p-4 text-red-700">
            {error}
          </div>
        )}

        {/* ローディング */}
        {isLoading && (
          <div className="mb-6 rounded border border-gray-200 bg-white p-8 text-center text-gray-500">
            読み込み中...
          </div>
        )}
      </main>
    </div>
  );
}
