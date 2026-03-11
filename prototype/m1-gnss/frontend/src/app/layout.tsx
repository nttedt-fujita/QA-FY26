import type { Metadata } from "next";
import { Geist, Geist_Mono } from "next/font/google";
import "./globals.css";
import { Navigation } from "@/components/Navigation";

const geistSans = Geist({
  variable: "--font-geist-sans",
  subsets: ["latin"],
});

const geistMono = Geist_Mono({
  variable: "--font-geist-mono",
  subsets: ["latin"],
});

export const metadata: Metadata = {
  title: "GNSS評価ツール",
  description: "F9P GNSS受信機の屋内検査ツール",
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="ja">
      <body
        className={`${geistSans.variable} ${geistMono.variable} antialiased bg-gray-50`}
      >
        <div className="min-h-screen">
          {/* ヘッダー */}
          <header className="border-b border-gray-200 bg-white">
            <div className="mx-auto max-w-7xl px-4 py-4">
              <h1 className="text-xl font-bold text-gray-900">GNSS評価ツール</h1>
            </div>
          </header>

          {/* タブナビゲーション */}
          <Navigation />

          {/* メインコンテンツ */}
          <main className="mx-auto max-w-7xl">{children}</main>
        </div>
      </body>
    </html>
  );
}
