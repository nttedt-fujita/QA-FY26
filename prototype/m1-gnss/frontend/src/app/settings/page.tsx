"use client";

import { useState, useEffect } from "react";
import {
  NtripSettings,
  loadNtripSettings,
  saveNtripSettings,
  isNtripSettingsComplete,
} from "@/lib/ntrip-settings";

/**
 * 設定画面（NTRIP認証設定）
 */
export default function SettingsPage() {
  const [settings, setSettings] = useState<NtripSettings>({
    casterUrl: "",
    port: 2101,
    mountpoint: "",
    username: "",
    password: "",
  });
  const [isSaved, setIsSaved] = useState(false);
  const [errors, setErrors] = useState<Record<string, string>>({});

  // 初回読み込み
  useEffect(() => {
    const loaded = loadNtripSettings();
    setSettings(loaded);
  }, []);

  // バリデーション
  const validate = (): boolean => {
    const newErrors: Record<string, string> = {};

    if (!settings.casterUrl.trim()) {
      newErrors.casterUrl = "キャスターURLを入力してください";
    }

    if (!settings.port || settings.port < 1 || settings.port > 65535) {
      newErrors.port = "ポートは1〜65535の範囲で入力してください";
    }

    if (!settings.mountpoint.trim()) {
      newErrors.mountpoint = "マウントポイントを入力してください";
    }

    if (!settings.username.trim()) {
      newErrors.username = "ユーザーIDを入力してください";
    }

    if (!settings.password.trim()) {
      newErrors.password = "パスワードを入力してください";
    }

    setErrors(newErrors);
    return Object.keys(newErrors).length === 0;
  };

  // 保存
  const handleSave = () => {
    if (!validate()) {
      return;
    }

    saveNtripSettings(settings);
    setIsSaved(true);
    // 3秒後にメッセージを消す
    setTimeout(() => setIsSaved(false), 3000);
  };

  // 入力変更
  const handleChange = (field: keyof NtripSettings, value: string | number) => {
    setSettings((prev) => ({ ...prev, [field]: value }));
    setIsSaved(false);
    // エラーをクリア
    if (errors[field]) {
      setErrors((prev) => {
        const newErrors = { ...prev };
        delete newErrors[field];
        return newErrors;
      });
    }
  };

  const isComplete = isNtripSettingsComplete(settings);

  return (
    <div className="px-4 py-6">
      <div className="mb-6">
        <h2 className="text-xl font-bold text-gray-900">設定</h2>
        <p className="mt-1 text-sm text-gray-500">
          NTRIP接続に必要な認証情報を設定します
        </p>
      </div>

      {/* NTRIP設定フォーム */}
      <div className="rounded-lg border border-gray-200 bg-white p-6">
        <h3 className="mb-4 text-lg font-medium text-gray-900">NTRIP認証設定</h3>

        <div className="space-y-4">
          {/* キャスターURL */}
          <div>
            <label
              htmlFor="casterUrl"
              className="block text-sm font-medium text-gray-700"
            >
              キャスターURL <span className="text-red-500">*</span>
            </label>
            <input
              type="text"
              id="casterUrl"
              value={settings.casterUrl}
              onChange={(e) => handleChange("casterUrl", e.target.value)}
              placeholder="例: ntrip.jenoba.jp"
              className={`mt-1 block w-full rounded-md border px-3 py-2 shadow-sm focus:outline-none focus:ring-1 ${
                errors.casterUrl
                  ? "border-red-300 focus:border-red-500 focus:ring-red-500"
                  : "border-gray-300 focus:border-blue-500 focus:ring-blue-500"
              }`}
            />
            {errors.casterUrl && (
              <p className="mt-1 text-sm text-red-600">{errors.casterUrl}</p>
            )}
          </div>

          {/* ポート */}
          <div>
            <label
              htmlFor="port"
              className="block text-sm font-medium text-gray-700"
            >
              ポート <span className="text-red-500">*</span>
            </label>
            <input
              type="number"
              id="port"
              value={settings.port}
              onChange={(e) => handleChange("port", parseInt(e.target.value) || 0)}
              placeholder="例: 2101"
              min={1}
              max={65535}
              className={`mt-1 block w-32 rounded-md border px-3 py-2 shadow-sm focus:outline-none focus:ring-1 ${
                errors.port
                  ? "border-red-300 focus:border-red-500 focus:ring-red-500"
                  : "border-gray-300 focus:border-blue-500 focus:ring-blue-500"
              }`}
            />
            {errors.port && (
              <p className="mt-1 text-sm text-red-600">{errors.port}</p>
            )}
          </div>

          {/* マウントポイント */}
          <div>
            <label
              htmlFor="mountpoint"
              className="block text-sm font-medium text-gray-700"
            >
              マウントポイント <span className="text-red-500">*</span>
            </label>
            <input
              type="text"
              id="mountpoint"
              value={settings.mountpoint}
              onChange={(e) => handleChange("mountpoint", e.target.value)}
              placeholder="例: TOKYO_RTCM3"
              className={`mt-1 block w-full rounded-md border px-3 py-2 shadow-sm focus:outline-none focus:ring-1 ${
                errors.mountpoint
                  ? "border-red-300 focus:border-red-500 focus:ring-red-500"
                  : "border-gray-300 focus:border-blue-500 focus:ring-blue-500"
              }`}
            />
            {errors.mountpoint && (
              <p className="mt-1 text-sm text-red-600">{errors.mountpoint}</p>
            )}
            <p className="mt-1 text-xs text-gray-500">
              NTRIPサービス提供者から提供されるストリーム名
            </p>
          </div>

          {/* ユーザーID */}
          <div>
            <label
              htmlFor="username"
              className="block text-sm font-medium text-gray-700"
            >
              ユーザーID <span className="text-red-500">*</span>
            </label>
            <input
              type="text"
              id="username"
              value={settings.username}
              onChange={(e) => handleChange("username", e.target.value)}
              placeholder="例: user123"
              className={`mt-1 block w-full rounded-md border px-3 py-2 shadow-sm focus:outline-none focus:ring-1 ${
                errors.username
                  ? "border-red-300 focus:border-red-500 focus:ring-red-500"
                  : "border-gray-300 focus:border-blue-500 focus:ring-blue-500"
              }`}
            />
            {errors.username && (
              <p className="mt-1 text-sm text-red-600">{errors.username}</p>
            )}
          </div>

          {/* パスワード */}
          <div>
            <label
              htmlFor="password"
              className="block text-sm font-medium text-gray-700"
            >
              パスワード <span className="text-red-500">*</span>
            </label>
            <input
              type="password"
              id="password"
              value={settings.password}
              onChange={(e) => handleChange("password", e.target.value)}
              placeholder="••••••••"
              className={`mt-1 block w-full rounded-md border px-3 py-2 shadow-sm focus:outline-none focus:ring-1 ${
                errors.password
                  ? "border-red-300 focus:border-red-500 focus:ring-red-500"
                  : "border-gray-300 focus:border-blue-500 focus:ring-blue-500"
              }`}
            />
            {errors.password && (
              <p className="mt-1 text-sm text-red-600">{errors.password}</p>
            )}
          </div>
        </div>

        {/* 保存ボタン */}
        <div className="mt-6 flex items-center gap-4">
          <button
            onClick={handleSave}
            className="rounded-md bg-blue-600 px-4 py-2 text-sm font-medium text-white hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2"
          >
            保存
          </button>

          {isSaved && (
            <span className="text-sm text-green-600">✓ 保存しました</span>
          )}
        </div>

        {/* 状態表示 */}
        <div className="mt-6 border-t border-gray-200 pt-4">
          <div className="flex items-center gap-2">
            <span className="text-sm text-gray-500">設定状態:</span>
            {isComplete ? (
              <span className="inline-flex items-center rounded-full bg-green-100 px-2.5 py-0.5 text-xs font-medium text-green-800">
                設定完了
              </span>
            ) : (
              <span className="inline-flex items-center rounded-full bg-yellow-100 px-2.5 py-0.5 text-xs font-medium text-yellow-800">
                未設定
              </span>
            )}
          </div>
        </div>
      </div>

      {/* 説明 */}
      <div className="mt-6 rounded-lg border border-blue-100 bg-blue-50 p-4">
        <h4 className="text-sm font-medium text-blue-800">NTRIPとは</h4>
        <p className="mt-1 text-sm text-blue-700">
          NTRIP（Networked Transport of RTCM via Internet
          Protocol）は、RTK補正データをインターネット経由で受信するためのプロトコルです。
          ジェノバなどのRTKサービスに契約すると、これらの認証情報が提供されます。
        </p>
      </div>
    </div>
  );
}
