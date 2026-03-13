/**
 * GNSS定義の共通モジュール
 *
 * 各パネル（SkyPlot, NavSig, MonSpan）で共通利用
 */

// GNSS定義（ID、名前、色）
export const GNSS_LIST = [
  { id: 0, name: "GPS", color: "#3b82f6" },      // blue
  { id: 2, name: "Galileo", color: "#f59e0b" },  // amber
  { id: 3, name: "BeiDou", color: "#ef4444" },   // red
  { id: 6, name: "GLONASS", color: "#06b6d4" },  // cyan
  { id: 5, name: "QZSS", color: "#8b5cf6" },     // violet
  { id: 1, name: "SBAS", color: "#22c55e" },     // green
] as const;

// GNSS IDの型
export type GnssId = typeof GNSS_LIST[number]["id"];

// GNSS情報の型
export interface GnssInfo {
  id: GnssId;
  name: string;
  color: string;
}

/**
 * GNSS IDから色を取得
 */
export function getGnssColor(gnssId: number): string {
  const gnss = GNSS_LIST.find((g) => g.id === gnssId);
  return gnss?.color ?? "#6b7280"; // gray for unknown
}

/**
 * GNSS IDから名前を取得
 */
export function getGnssName(gnssId: number): string {
  const gnss = GNSS_LIST.find((g) => g.id === gnssId);
  return gnss?.name ?? "Unknown";
}

/**
 * 全GNSS IDのセットを作成
 */
export function createAllGnssSet(): Set<number> {
  return new Set(GNSS_LIST.map((g) => g.id));
}
