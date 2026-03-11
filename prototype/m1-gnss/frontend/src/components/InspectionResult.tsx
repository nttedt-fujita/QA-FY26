"use client";

import { InspectionResponse } from "@/lib/api";

interface InspectionResultProps {
  result: InspectionResponse;
}

/**
 * 検査項目名を日本語に変換
 */
function itemNameToJapanese(itemName: string): string {
  const mapping: Record<string, string> = {
    communication: "通信疎通",
    fw: "FWバージョン",
    serial: "シリアル番号",
    rate: "出力レート",
    port: "ポート設定",
  };
  return mapping[itemName] || itemName;
}

/**
 * 判定結果に応じた色を返す
 */
function getVerdictColor(verdict: string): string {
  switch (verdict) {
    case "Pass":
      return "text-green-600";
    case "Fail":
      return "text-red-600";
    case "Error":
      return "text-yellow-600";
    default:
      return "text-gray-600";
  }
}

/**
 * 判定結果を日本語に変換
 */
function verdictToJapanese(verdict: string): string {
  switch (verdict) {
    case "Pass":
      return "合格";
    case "Fail":
      return "不合格";
    case "Error":
      return "エラー";
    default:
      return verdict;
  }
}

/**
 * 検査結果表示コンポーネント
 */
export function InspectionResult({ result }: InspectionResultProps) {
  const isPass = result.overall_result === "Pass";

  return (
    <div className="border rounded-lg p-4 bg-white">
      <h3 className="text-lg font-semibold mb-4">検査結果</h3>

      {/* シリアル番号 */}
      <div className="mb-4 text-sm text-gray-600">
        シリアル番号: {result.serial_number}
      </div>

      {/* 結果テーブル */}
      <table className="w-full border-collapse mb-4">
        <thead>
          <tr className="bg-gray-100">
            <th className="border p-2 text-left">項目</th>
            <th className="border p-2 text-left">期待値</th>
            <th className="border p-2 text-left">実測値</th>
            <th className="border p-2 text-center">判定</th>
          </tr>
        </thead>
        <tbody>
          {result.items.map((item, index) => (
            <tr key={index}>
              <td className="border p-2">{itemNameToJapanese(item.item_name)}</td>
              <td className="border p-2 text-gray-600">
                {item.expected_value || "-"}
              </td>
              <td className="border p-2">{item.actual_value || "-"}</td>
              <td
                className={`border p-2 text-center font-semibold ${getVerdictColor(
                  item.verdict
                )}`}
              >
                {verdictToJapanese(item.verdict)}
              </td>
            </tr>
          ))}
        </tbody>
      </table>

      {/* 総合判定 */}
      <div
        className={`text-center p-3 rounded-lg ${
          isPass ? "bg-green-100 text-green-800" : "bg-red-100 text-red-800"
        }`}
      >
        <span className="text-lg font-bold">
          総合判定: {isPass ? "✅ 合格" : "❌ 不合格"}
        </span>
      </div>
    </div>
  );
}
