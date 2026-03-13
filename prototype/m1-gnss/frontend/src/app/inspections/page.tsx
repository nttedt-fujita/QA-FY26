"use client";

import Link from "next/link";

/**
 * 検査種別選択画面
 *
 * 屋内検査 / 屋外検査 の選択
 */
export default function InspectionsIndexPage() {
  return (
    <div className="min-h-screen bg-gray-50">
      {/* ページタイトル */}
      <div className="bg-white border-b border-gray-200">
        <div className="container mx-auto px-4 py-4">
          <h1 className="text-xl font-semibold text-gray-800">検査</h1>
          <p className="text-sm text-gray-500 mt-1">
            検査種別を選択してください
          </p>
        </div>
      </div>

      {/* メインコンテンツ */}
      <main className="container mx-auto px-4 py-8">
        <div className="grid gap-6 md:grid-cols-3 max-w-5xl mx-auto">
          {/* 屋内検査 */}
          <Link
            href="/inspections/indoor"
            className="block rounded-lg border-2 border-gray-200 bg-white p-6 hover:border-blue-400 hover:shadow-md transition-all"
          >
            <div className="mb-4 text-4xl">🏠</div>
            <h2 className="text-xl font-semibold text-gray-800 mb-2">
              屋内検査
            </h2>
            <p className="text-gray-600 text-sm mb-4">
              ZED-F9Pの基本機能を検査します。
            </p>
            <div className="text-sm text-gray-500">
              <div className="font-medium mb-1">検査項目：</div>
              <ul className="space-y-1">
                <li>• RATE - 出力レート確認</li>
                <li>• UART1 - UART1ポート設定</li>
                <li>• UART2 - UART2ポート設定</li>
                <li>• USB - USBポート設定</li>
                <li>• NAV - ナビゲーション設定</li>
              </ul>
            </div>
          </Link>

          {/* 屋外検査 */}
          <Link
            href="/inspections/outdoor"
            className="block rounded-lg border-2 border-gray-200 bg-white p-6 hover:border-blue-400 hover:shadow-md transition-all"
          >
            <div className="mb-4 text-4xl">🌳</div>
            <h2 className="text-xl font-semibold text-gray-800 mb-2">
              屋外検査
            </h2>
            <p className="text-gray-600 text-sm mb-4">
              GNSS受信性能を検査します。屋外の見晴らしの良い場所で実施してください。
            </p>
            <div className="text-sm text-gray-500">
              <div className="font-medium mb-1">検査項目：</div>
              <ul className="space-y-1">
                <li>• L1受信感度 ≥30 dBHz</li>
                <li>• L2受信率（GPS）≥50%</li>
                <li>• RTK FIX時間 ≤30秒</li>
                <li>• RTK FIX率 &gt;95%</li>
                <li>• MON-SPAN（スペクトラム）</li>
              </ul>
            </div>
          </Link>

          {/* 履歴再生 */}
          <Link
            href="/inspections/history"
            className="block rounded-lg border-2 border-gray-200 bg-white p-6 hover:border-blue-400 hover:shadow-md transition-all"
          >
            <div className="mb-4 text-4xl">📊</div>
            <h2 className="text-xl font-semibold text-gray-800 mb-2">
              履歴再生
            </h2>
            <p className="text-gray-600 text-sm mb-4">
              過去の屋外検査データを再生表示します。
            </p>
            <div className="text-sm text-gray-500">
              <div className="font-medium mb-1">機能：</div>
              <ul className="space-y-1">
                <li>• 検査結果の確認</li>
                <li>• スナップショット再生</li>
                <li>• スカイプロット表示</li>
                <li>• スペクトラム表示</li>
              </ul>
            </div>
          </Link>
        </div>
      </main>
    </div>
  );
}
