"use client";

import Link from "next/link";
import { usePathname } from "next/navigation";

const tabs = [
  { href: "/lots", label: "ロット" },
  { href: "/devices", label: "装置" },
  { href: "/inspections", label: "検査" },
];

/**
 * タブナビゲーション
 *
 * モックアップに合わせたタブ形式のナビゲーション
 */
export function Navigation() {
  const pathname = usePathname();

  return (
    <nav className="border-b border-gray-200 bg-white">
      <div className="mx-auto max-w-7xl px-4">
        <div className="flex space-x-1">
          {tabs.map((tab) => {
            const isActive = pathname === tab.href;
            return (
              <Link
                key={tab.href}
                href={tab.href}
                className={`px-4 py-3 text-sm font-medium transition-colors ${
                  isActive
                    ? "border-b-2 border-blue-500 text-blue-600"
                    : "text-gray-500 hover:text-gray-700"
                }`}
              >
                {tab.label}
              </Link>
            );
          })}
        </div>
      </div>
    </nav>
  );
}
