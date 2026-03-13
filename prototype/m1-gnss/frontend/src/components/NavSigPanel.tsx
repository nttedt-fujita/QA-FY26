"use client";

import { useMemo } from "react";
import { NavSigResponse, NavSignal } from "@/lib/api";
import { GNSS_LIST, getGnssColor, getGnssName } from "@/lib/gnss-constants";

// ADR-008: L2受信率50%以上で合格
const L2_RECEPTION_THRESHOLD = 0.5;

/**
 * C/N0値に応じた色クラスを返す
 * 参考: 30dBHz以上で良好、20dBHz未満は弱い
 */
function getCnoColorClass(cno: number): string {
  if (cno >= 40) return "text-green-600";
  if (cno >= 30) return "text-green-500";
  if (cno >= 20) return "text-yellow-600";
  return "text-red-600";
}

/**
 * C/N0バーの幅を計算（0-50dBHzの範囲）
 */
function getCnoBarWidth(cno: number): number {
  return Math.min(100, Math.max(0, (cno / 50) * 100));
}

/**
 * C/N0バーの色クラスを返す
 */
function getCnoBarColorClass(cno: number): string {
  if (cno >= 40) return "bg-green-500";
  if (cno >= 30) return "bg-green-400";
  if (cno >= 20) return "bg-yellow-400";
  return "bg-red-400";
}

interface NavSigPanelProps {
  /** NAV-SIGデータ（統合APIから渡される） */
  data: NavSigResponse | null;
  /** エラーメッセージ */
  error?: string | null;
  /** 読み込み中フラグ */
  isLoading?: boolean;
  /** 装置接続フラグ */
  isConnected?: boolean;
  /** 選択中のGNSS IDセット（親コンポーネントから渡される） */
  selectedGnss: Set<number>;
}

/**
 * GNSS×周波数帯の統計情報
 */
interface GnssFreqStats {
  gnssId: number;
  gnssName: string;
  color: string;
  l1Count: number;
  l2Count: number;
  l1AvgCno: number | null;
  l2AvgCno: number | null;
  l2Rate: number; // l2Count / l1Count
}

/**
 * NAV-SIG表示パネル
 *
 * - L1/L2別C/N0一覧テーブル
 * - L2受信率ゲージ（50%基準で色分け）
 * - 合格/不合格表示
 * - GNSS×周波数帯統計
 */
export function NavSigPanel({
  data,
  error,
  isLoading = false,
  isConnected = true,
  selectedGnss,
}: NavSigPanelProps) {
  // 選択GNSSの信号をフィルタリング
  const filteredSignals = useMemo(() => {
    if (!data) return [];
    return data.signals.filter((s) => selectedGnss.has(s.gnss_id));
  }, [data, selectedGnss]);

  // L1とL2で信号を分離（選択GNSSのみ）
  const l1Signals = filteredSignals.filter((s) => s.is_l1);
  const l2Signals = filteredSignals.filter((s) => s.is_l2);

  // GNSS×周波数帯統計を計算
  const gnssFreqStats = useMemo((): GnssFreqStats[] => {
    if (!data) return [];

    return GNSS_LIST.map((gnss) => {
      const gnssSignals = data.signals.filter((s) => s.gnss_id === gnss.id);
      const l1 = gnssSignals.filter((s) => s.is_l1);
      const l2 = gnssSignals.filter((s) => s.is_l2);
      const l1Cnos = l1.filter((s) => s.cno > 0).map((s) => s.cno);
      const l2Cnos = l2.filter((s) => s.cno > 0).map((s) => s.cno);

      return {
        gnssId: gnss.id,
        gnssName: gnss.name,
        color: gnss.color,
        l1Count: l1.length,
        l2Count: l2.length,
        l1AvgCno:
          l1Cnos.length > 0
            ? l1Cnos.reduce((a, b) => a + b, 0) / l1Cnos.length
            : null,
        l2AvgCno:
          l2Cnos.length > 0
            ? l2Cnos.reduce((a, b) => a + b, 0) / l2Cnos.length
            : null,
        l2Rate: l1.length > 0 ? l2.length / l1.length : 0,
      };
    }).filter((s) => s.l1Count > 0 || s.l2Count > 0);
  }, [data]);

  // L2受信率の判定（GPS固定、ADR-008）
  const l2Rate = data?.stats.gps_l2_reception_rate ?? 0;
  const isL2Pass = l2Rate >= L2_RECEPTION_THRESHOLD;

  // 選択GNSSの合計L2受信率（参考表示用）
  const selectedL2Stats = useMemo(() => {
    const filtered = gnssFreqStats.filter((s) => selectedGnss.has(s.gnssId));
    const totalL1 = filtered.reduce((sum, s) => sum + s.l1Count, 0);
    const totalL2 = filtered.reduce((sum, s) => sum + s.l2Count, 0);
    return {
      l1Count: totalL1,
      l2Count: totalL2,
      rate: totalL1 > 0 ? totalL2 / totalL1 : 0,
    };
  }, [gnssFreqStats, selectedGnss]);

  if (!isConnected) {
    return (
      <div className="rounded border border-gray-200 bg-white p-4">
        <h3 className="mb-2 font-medium text-gray-700">衛星信号 (NAV-SIG)</h3>
        <div className="text-gray-500">装置が接続されていません</div>
      </div>
    );
  }

  if (error) {
    return (
      <div className="rounded border border-red-200 bg-red-50 p-4">
        <h3 className="mb-2 font-medium text-red-700">衛星信号 (NAV-SIG)</h3>
        <div className="text-red-600">{error}</div>
      </div>
    );
  }

  return (
    <div className="rounded border border-gray-200 bg-white p-4">
      <div className="mb-4 flex items-center justify-between">
        <h3 className="font-medium text-gray-700">衛星信号 (NAV-SIG)</h3>
        {isLoading && (
          <span className="text-xs text-gray-400">更新中...</span>
        )}
      </div>

      {data && (
        <>
          {/* L2受信率ゲージ */}
          <div className="mb-4">
            <div className="mb-1 flex items-center justify-between">
              <span className="text-sm text-gray-600">L2受信率</span>
              <span
                className={`text-sm font-semibold ${
                  isL2Pass ? "text-green-600" : "text-red-600"
                }`}
              >
                {(l2Rate * 100).toFixed(0)}% ({data.stats.gps_l2_reception_count}/
                {data.stats.gps_visible_count})
                {isL2Pass ? " ✓ 合格" : " × 不合格"}
              </span>
            </div>
            <div className="h-4 w-full overflow-hidden rounded bg-gray-200">
              <div
                className={`h-full transition-all duration-300 ${
                  isL2Pass ? "bg-green-500" : "bg-red-500"
                }`}
                style={{ width: `${l2Rate * 100}%` }}
              />
              {/* 50%基準線 */}
              <div
                className="absolute h-4 w-0.5 bg-gray-400"
                style={{ left: "50%", marginTop: "-16px" }}
              />
            </div>
            <div className="mt-1 flex justify-between text-xs text-gray-400">
              <span>0%</span>
              <span>50% (基準)</span>
              <span>100%</span>
            </div>
            {/* 選択GNSSの参考L2受信率 */}
            {selectedL2Stats.l1Count > 0 && !selectedGnss.has(0) && (
              <div className="mt-2 text-xs text-gray-500">
                選択GNSS: {(selectedL2Stats.rate * 100).toFixed(0)}%
                ({selectedL2Stats.l2Count}/{selectedL2Stats.l1Count})
              </div>
            )}
            {selectedL2Stats.l1Count > 0 && selectedGnss.has(0) && selectedGnss.size > 1 && (
              <div className="mt-2 text-xs text-gray-500">
                選択GNSS合計: {(selectedL2Stats.rate * 100).toFixed(0)}%
                ({selectedL2Stats.l2Count}/{selectedL2Stats.l1Count})
              </div>
            )}
          </div>

          {/* GNSS×周波数帯統計テーブル */}
          {gnssFreqStats.length > 0 && (
            <div className="mb-4 rounded bg-gray-50 p-3">
              <div className="mb-2 text-xs font-medium text-gray-600">
                GNSS×周波数帯統計
              </div>
              <div className="overflow-x-auto">
                <table className="w-full text-xs">
                  <thead>
                    <tr className="text-left text-gray-500">
                      <th className="pb-1">GNSS</th>
                      <th className="pb-1 text-right">L1衛星</th>
                      <th className="pb-1 text-right">L2衛星</th>
                      <th className="pb-1 text-right">L1 CNO</th>
                      <th className="pb-1 text-right">L2 CNO</th>
                      <th className="pb-1 text-right">L2受信率</th>
                    </tr>
                  </thead>
                  <tbody>
                    {gnssFreqStats.map((stats) => (
                      <tr
                        key={stats.gnssId}
                        className={`border-t border-gray-200 ${
                          !selectedGnss.has(stats.gnssId) ? "opacity-40" : ""
                        }`}
                      >
                        <td className="py-1">
                          <div className="flex items-center gap-1">
                            <div
                              className="h-2 w-2 rounded-full"
                              style={{ backgroundColor: stats.color }}
                            />
                            <span className="font-medium">{stats.gnssName}</span>
                          </div>
                        </td>
                        <td className="py-1 text-right font-mono">
                          {stats.l1Count}
                        </td>
                        <td className="py-1 text-right font-mono">
                          {stats.l2Count > 0 ? stats.l2Count : "-"}
                        </td>
                        <td className="py-1 text-right font-mono">
                          {stats.l1AvgCno !== null
                            ? stats.l1AvgCno.toFixed(1)
                            : "-"}
                        </td>
                        <td className="py-1 text-right font-mono">
                          {stats.l2AvgCno !== null
                            ? stats.l2AvgCno.toFixed(1)
                            : "-"}
                        </td>
                        <td className="py-1 text-right font-mono">
                          {stats.l1Count > 0
                            ? `${(stats.l2Rate * 100).toFixed(0)}%`
                            : "-"}
                        </td>
                      </tr>
                    ))}
                    {/* 合計行（選択GNSSのみ） */}
                    {gnssFreqStats.filter((s) => selectedGnss.has(s.gnssId))
                      .length > 1 && (
                      <tr className="border-t-2 border-gray-300 font-semibold">
                        <td className="py-1">合計</td>
                        <td className="py-1 text-right font-mono">
                          {gnssFreqStats
                            .filter((s) => selectedGnss.has(s.gnssId))
                            .reduce((sum, s) => sum + s.l1Count, 0)}
                        </td>
                        <td className="py-1 text-right font-mono">
                          {gnssFreqStats
                            .filter((s) => selectedGnss.has(s.gnssId))
                            .reduce((sum, s) => sum + s.l2Count, 0) || "-"}
                        </td>
                        <td className="py-1 text-right font-mono">-</td>
                        <td className="py-1 text-right font-mono">-</td>
                        <td className="py-1 text-right font-mono">
                          {(() => {
                            const filtered = gnssFreqStats.filter((s) =>
                              selectedGnss.has(s.gnssId)
                            );
                            const totalL1 = filtered.reduce(
                              (sum, s) => sum + s.l1Count,
                              0
                            );
                            const totalL2 = filtered.reduce(
                              (sum, s) => sum + s.l2Count,
                              0
                            );
                            return totalL1 > 0
                              ? `${((totalL2 / totalL1) * 100).toFixed(0)}%`
                              : "-";
                          })()}
                        </td>
                      </tr>
                    )}
                  </tbody>
                </table>
              </div>
            </div>
          )}

          {/* 信号テーブル（選択GNSS） */}
          <div className="grid gap-4 md:grid-cols-2">
            {/* L1信号 */}
            <div>
              <h4 className="mb-2 text-sm font-medium text-gray-600">
                L1信号 ({l1Signals.length}衛星)
              </h4>
              <SignalTable signals={l1Signals} />
            </div>

            {/* L2信号 */}
            <div>
              <h4 className="mb-2 text-sm font-medium text-gray-600">
                L2信号 ({l2Signals.length}衛星)
              </h4>
              <SignalTable signals={l2Signals} />
            </div>
          </div>
        </>
      )}

      {!data && !error && (
        <div className="text-gray-400">データなし</div>
      )}
    </div>
  );
}

/**
 * GNSS別にグループ化した信号
 */
interface GnssGroup {
  gnssId: number;
  gnssName: string;
  color: string;
  signals: NavSignal[];
}

/**
 * 信号一覧テーブル（GNSS別グルーピング）
 */
function SignalTable({ signals }: { signals: NavSignal[] }) {
  if (signals.length === 0) {
    return <div className="text-sm text-gray-400">信号なし</div>;
  }

  // GNSS別にグループ化し、各グループ内はC/N0降順でソート
  const groups: GnssGroup[] = useMemo(() => {
    const groupMap = new Map<number, NavSignal[]>();

    for (const signal of signals) {
      const existing = groupMap.get(signal.gnss_id);
      if (existing) {
        existing.push(signal);
      } else {
        groupMap.set(signal.gnss_id, [signal]);
      }
    }

    // GNSS_LISTの順番でグループをソート
    const gnssOrder = GNSS_LIST.map((g) => g.id);
    const sortedGroups: GnssGroup[] = [];

    for (const gnssId of gnssOrder) {
      const groupSignals = groupMap.get(gnssId);
      if (groupSignals && groupSignals.length > 0) {
        // グループ内はC/N0降順でソート
        groupSignals.sort((a, b) => b.cno - a.cno);
        sortedGroups.push({
          gnssId,
          gnssName: getGnssName(gnssId),
          color: getGnssColor(gnssId),
          signals: groupSignals,
        });
      }
    }

    return sortedGroups;
  }, [signals]);

  return (
    <div className="overflow-hidden rounded border border-gray-200">
      <table className="w-full text-sm">
        <thead>
          <tr className="bg-gray-50">
            <th className="w-1 px-0"></th>
            <th className="px-2 py-1 text-left">SV</th>
            <th className="px-2 py-1 text-right">C/N0</th>
            <th className="px-2 py-1 text-left">強度</th>
          </tr>
        </thead>
        <tbody>
          {groups.map((group) => (
            group.signals.map((signal, idx) => (
              <tr
                key={`${group.gnssId}-${signal.sv_id}`}
                className={`border-t ${idx === 0 ? "border-gray-300" : "border-gray-100"}`}
              >
                {/* GNSS色インジケーター */}
                <td
                  className="w-1 px-0"
                  style={{ backgroundColor: group.color }}
                  title={group.gnssName}
                />
                <td className="px-2 py-1 font-mono">
                  {idx === 0 && (
                    <span
                      className="mr-1 text-xs font-medium"
                      style={{ color: group.color }}
                    >
                      {group.gnssName}
                    </span>
                  )}
                  {signal.sv_id}
                </td>
                <td
                  className={`px-2 py-1 text-right font-mono ${getCnoColorClass(
                    signal.cno
                  )}`}
                >
                  {signal.cno}
                </td>
                <td className="px-2 py-1">
                  <div className="h-2 w-full overflow-hidden rounded bg-gray-200">
                    <div
                      className={`h-full ${getCnoBarColorClass(signal.cno)}`}
                      style={{ width: `${getCnoBarWidth(signal.cno)}%` }}
                    />
                  </div>
                </td>
              </tr>
            ))
          ))}
        </tbody>
      </table>
    </div>
  );
}
