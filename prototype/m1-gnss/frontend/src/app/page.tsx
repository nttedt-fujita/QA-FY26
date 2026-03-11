import { redirect } from "next/navigation";

/**
 * トップページ
 *
 * ロット管理画面にリダイレクト
 */
export default function Home() {
  redirect("/lots");
}
