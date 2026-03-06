# Session 17 確認ファイル一覧

**目的**: Phase 1（検証項目の妥当性検証）の準備として、技術資料と現状データを確認した。

---

## プロジェクト内ドキュメント

| ファイル | 内容 | 確認目的 |
|---------|------|---------|
| [session16/session-summary.md](../session16/session-summary.md) | Session 16サマリー | 前回の作業内容・引き継ぎ確認 |
| [session16/m1b-gnss-roadmap.md](../session16/m1b-gnss-roadmap.md) | M1-Bロードマップ | 現在地とPhase 1の内容確認 |
| [session16/gnss-hearing-koitabashi-01.md](../session16/gnss-hearing-koitabashi-01.md) | ヒアリング結果 | 小板橋さんの指摘事項確認 |

---

## 今セッションで新たに読み込んだファイル

### PDFファイル

| ファイル | 内容 | 読み込み範囲 |
|---------|------|------------|
| `session17/1-14-app-note-ublox.pdf` (631KB) | UBX Interface Description 目次（p.1-14） | 全ページ（目次のみ） |
| `session17/アプリケーションノート_u-blox-F9-HPG-1.32_InterfaceDescription_UBX-22008968.pdf` (3.6MB) | UBX Interface Description 本文（305ページ） | p.132-134, p.139-158（主要メッセージ定義部分） |

**読み込んだ主要セクション**:
- UBX-NAV-PVT（p.145-148）: fixType, carrSoln, numSV, hAcc, vAcc
- UBX-NAV-HPPOSLLH（p.140-141）: 高精度測位（0.1mm精度）
- UBX-NAV-SAT（p.150-151）: 衛星ごとのC/N0・品質情報
- UBX-NAV-SIG（p.152-153）: 信号ごとのC/N0（L1/L2別）
- UBX-NAV-STATUS（p.155-156）: RTK状態・ttff
- UBX-MON-RF（p.132-133）: ジャミング検出・ノイズ
- UBX-MON-SPAN（p.134）: スペクトラムアナライザ

**未読範囲**: p.1-131, p.159-305（RTCM, SPARTN, Configuration等）

### Excelファイル

| ファイル | 内容 | 確認範囲 |
|---------|------|---------|
| `session17/小峰無線GPS確認.xlsx` (23MB) | 小板橋さんのGNSS評価記録 | 全シートのテキストデータ（画像88枚は未読） |

**読み込んだシート**:
- 内部設定: FW版数・PROTVER確認記録
- 受信感度: C/N0測定データ（3回分・機体別・衛星別・L1/L2別）
- 受信感度_仰角ごとの整理: 仰角帯別C/N0整理
- 電池確認: バックアップ電池電圧測定
- Spectrum Analyze: 測定条件・機体一覧（スペアナ波形画像は未読）
- 20260218: 飛行試験結果（テキストデータ部分）

**未確認**（画像88枚）:
- スペアナ波形（Spectrum Analyze: 17枚）
- 飛行中ロググラフ（20260218: 15枚）
- 受信感度スクリーンショット（受信感度: 40枚）
- MAGずれ量（Mag確認: 5枚）
- その他（電池確認: 6枚、No.02: 4枚、受信感度_仰角: 1枚）

---

## 分析結果

→ [pdf-excel-analysis.md](pdf-excel-analysis.md) に詳細記録
