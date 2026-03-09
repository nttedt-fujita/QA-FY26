import type { Metadata } from "next";
import { Geist, Geist_Mono } from "next/font/google";
import Link from "next/link";
import "./globals.css";

const geistSans = Geist({
  variable: "--font-geist-sans",
  subsets: ["latin"],
});

const geistMono = Geist_Mono({
  variable: "--font-geist-mono",
  subsets: ["latin"],
});

export const metadata: Metadata = {
  title: "受入検査DB",
  description: "受入検査プロトタイプ",
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="ja">
      <body
        className={`${geistSans.variable} ${geistMono.variable} antialiased`}
      >
        {/* 共通ナビゲーション */}
        <nav className="bg-zinc-800 text-white">
          <div className="max-w-6xl mx-auto px-4 flex items-center h-12">
            <Link href="/" className="font-bold mr-8">
              🏭 受入検査
            </Link>
            <div className="flex gap-1">
              <Link
                href="/"
                className="px-4 py-2 rounded hover:bg-zinc-700 text-sm"
              >
                ロット
              </Link>
              <Link
                href="/inspection"
                className="px-4 py-2 rounded hover:bg-zinc-700 text-sm bg-green-600"
              >
                検査入力
              </Link>
              <Link
                href="/records"
                className="px-4 py-2 rounded hover:bg-zinc-700 text-sm"
              >
                一覧
              </Link>
              <Link
                href="/dashboard"
                className="px-4 py-2 rounded hover:bg-zinc-700 text-sm"
              >
                ダッシュボード
              </Link>
            </div>
          </div>
        </nav>
        {children}
      </body>
    </html>
  );
}
