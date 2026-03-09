"use client";

import Link from "next/link";
import { usePathname } from "next/navigation";

const navItems = [
  { href: "/", label: "ロット" },
  { href: "/inspection", label: "検査入力" },
  { href: "/records", label: "一覧" },
  { href: "/dashboard", label: "ダッシュボード" },
];

export function Navigation() {
  const pathname = usePathname();

  // 現在のページかどうかを判定
  const isActive = (href: string) => {
    if (href === "/") {
      return pathname === "/";
    }
    return pathname.startsWith(href);
  };

  return (
    <nav className="bg-zinc-800 text-white">
      <div className="max-w-6xl mx-auto px-4 flex items-center h-12">
        <Link href="/" className="font-bold mr-8">
          🏭 受入検査
        </Link>
        <div className="flex gap-1">
          {navItems.map((item) => (
            <Link
              key={item.href}
              href={item.href}
              className={`px-4 py-2 rounded text-sm transition-colors ${
                isActive(item.href)
                  ? "bg-green-600 hover:bg-green-700"
                  : "hover:bg-zinc-700"
              }`}
            >
              {item.label}
            </Link>
          ))}
        </div>
      </div>
    </nav>
  );
}
