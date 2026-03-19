# ワークショップ資料 印刷準備チェックリスト

**作成日**: 2026-03-19 (Session 263)
**対象**: SIPOCワークショップ（田原さん・杉山さん）
**実施日**: （ワークショップ実施日を記入）

---

## 印刷資料リスト

### 📄 資料1: ヒアリング項目リスト

**ファイル**: [sessions/session262/sipoc-workshop-hearing-items.md](../session262/sipoc-workshop-hearing-items.md)

- [ ] **印刷済み**
- **印刷枚数**: 3部（藤田、田原さん、杉山さん）
- **用途**: ワークショップの流れ説明、初期ヒアリング項目参照
- **印刷設定**: A4、両面印刷推奨（約8ページ）

---

### 📄 資料2: Excelから抽出した情報

**ファイル**: [sessions/session262/pre-extracted-info-from-excel.md](../session262/pre-extracted-info-from-excel.md)

- [ ] **印刷済み**
- **印刷枚数**: 3部（藤田、田原さん、杉山さん）
- **用途**: Excelから抽出した情報の確認、具体例の参照（行番号・品番を示しながら確認）
- **印刷設定**: A4、両面印刷推奨（約10ページ）

---

### 📄 資料3: SIPOCテンプレート

**ファイル**: [sessions/session262/sipoc-iqc-template.drawio](../session262/sipoc-iqc-template.drawio)

- [ ] **PDF/PNG出力済み**
- [ ] **印刷済み**
- **印刷枚数**: 1部（参照用）
- **用途**: SIPOCの5要素（Suppliers, Inputs, Process, Outputs, Customers）の説明
- **印刷設定**: A3サイズ推奨（A4だと文字が小さい）

**PDF/PNG出力方法**:
1. Draw.ioで `sipoc-iqc-template.drawio` を開く
2. メニュー: File → Export as → PNG (または PDF)
3. 出力先: `sessions/session262/sipoc-iqc-template.drawio.png`

---

## デジタル資料準備

### 💻 写真: 現場の部品と梱包

**場所**: [sessions/session260/Photos/](../session260/Photos/)

- [ ] **ノートPC or タブレットで表示準備完了**
- **ファイル**:
  - `parts1.jpg` — 小物部品（グロメット・ナット）
  - `parts2.jpg` — 小物部品（白い緩衝材）
  - `parts3.jpg` — 大物部品（発泡スチロール型枠）
- **用途**: 「この部品はExcelのどの品番ですか？」と確認する際に使用

---

### 💻 Excel: 受入検査作業集計

**場所**: `docs/excel-orgn/受入検査作業集計.xlsx`

- [ ] **ノートPCで開けることを確認済み**
- **用途**: ワークショップ中に具体的な行番号を参照する際に使用
  - 例: 「行27の『インサート逆入れあり』はこれですね」と画面で見せる

---

## 会場準備

### ホワイトボード or 模造紙

- [ ] **ホワイトボード確保済み**
- [ ] **模造紙準備済み**（ホワイトボードがない場合）
- **サイズ**: A1サイズ以上推奨（SIPOC 5列を書くため）

---

### 付箋

- [ ] **5色以上の付箋準備済み**
- **用途**:
  - 色1: Suppliers（供給者）
  - 色2: Inputs（入力）
  - 色3: Process（プロセス）
  - 色4: Outputs（出力）
  - 色5: Customers（顧客）
- **枚数**: 各色50枚程度

---

### マーカー

- [ ] **3-5色のマーカー準備済み**
- **用途**: ホワイトボードに書き込み、付箋の文字書き

---

### マスキングテープ

- [ ] **マスキングテープ準備済み**
- **用途**: 模造紙を壁に貼る場合

---

## 記録準備

### カメラ（スマホ）

- [ ] **充電済み**
- [ ] **ストレージ空き容量確認済み**
- **用途**: ホワイトボードの写真撮影（ワークショップ後のデジタル化に使用）

---

### ノート

- [ ] **ノート準備済み**
- **用途**: 追加ヒアリング項目のメモ
  - 「ここ詳しく聞かないと」という箇所
  - 認識のズレが発見された箇所
  - 曖昧な返答があった箇所

---

## 印刷チェックリスト完了確認

- [ ] 資料1（ヒアリング項目）印刷済み
- [ ] 資料2（Excel抽出情報）印刷済み
- [ ] 資料3（SIPOCテンプレート）印刷済み
- [ ] 写真（parts1-3）表示準備完了
- [ ] Excel（受入検査作業集計）開けることを確認
- [ ] ホワイトボード or 模造紙準備完了
- [ ] 付箋・マーカー・テープ準備完了
- [ ] カメラ充電・ノート準備完了

---

## 補足: 印刷失敗時の対応

### Markdownファイルの印刷方法

**方法1: ブラウザで開いて印刷**
1. VSCodeでMarkdownファイルを開く
2. プレビュー表示（Ctrl+Shift+V）
3. プレビュー画面から印刷

**方法2: PDFに変換してから印刷**
1. VSCodeで Markdown PDF 拡張機能をインストール
2. Markdownファイルを右クリック → "Markdown PDF: Export (pdf)"
3. 生成されたPDFを印刷

**方法3: pandoc を使用**
```bash
pandoc sipoc-workshop-hearing-items.md -o sipoc-workshop-hearing-items.pdf
```

---

### Draw.ioファイルの印刷方法

**方法1: Draw.io Desktopアプリで開く**
1. Draw.io Desktop をインストール（https://github.com/jgraph/drawio-desktop/releases）
2. `.drawio` ファイルを開く
3. File → Export as → PNG (または PDF)
4. 生成されたPNG/PDFを印刷

**方法2: draw.io Webアプリで開く**
1. https://app.diagrams.net/ にアクセス
2. "Open Existing Diagram" → ファイルを選択
3. File → Export as → PNG (または PDF)
4. 生成されたファイルを印刷

---

*作成: Session 263 (2026-03-19)*
