/**
 * NTRIP設定の型定義とローカルストレージ操作
 */

// NTRIP設定の型
export interface NtripSettings {
  casterUrl: string; // キャスターURL（例: ntrip.jenoba.jp）
  port: number; // ポート（例: 2101）
  mountpoint: string; // マウントポイント（例: TOKYO_RTCM3）
  username: string; // ユーザーID
  password: string; // パスワード
}

// ローカルストレージのキー
const STORAGE_KEY = "gnss-eval-ntrip-settings";

// デフォルト値（空の設定）
const DEFAULT_SETTINGS: NtripSettings = {
  casterUrl: "",
  port: 2101, // NTRIPの一般的なデフォルトポート
  mountpoint: "",
  username: "",
  password: "",
};

/**
 * ローカルストレージからNTRIP設定を取得
 */
export function loadNtripSettings(): NtripSettings {
  if (typeof window === "undefined") {
    return DEFAULT_SETTINGS;
  }

  try {
    const stored = localStorage.getItem(STORAGE_KEY);
    if (!stored) {
      return DEFAULT_SETTINGS;
    }
    const parsed = JSON.parse(stored);
    // 型の整合性を保証するためにマージ
    return { ...DEFAULT_SETTINGS, ...parsed };
  } catch {
    // パースエラー時はデフォルト値を返す
    return DEFAULT_SETTINGS;
  }
}

/**
 * NTRIP設定をローカルストレージに保存
 */
export function saveNtripSettings(settings: NtripSettings): void {
  if (typeof window === "undefined") {
    return;
  }
  localStorage.setItem(STORAGE_KEY, JSON.stringify(settings));
}

/**
 * NTRIP設定が入力済みかどうかを確認
 * 全ての必須項目が入力されているかを返す
 */
export function isNtripSettingsComplete(settings: NtripSettings): boolean {
  return (
    settings.casterUrl.trim() !== "" &&
    settings.port > 0 &&
    settings.port <= 65535 &&
    settings.mountpoint.trim() !== "" &&
    settings.username.trim() !== "" &&
    settings.password.trim() !== ""
  );
}
