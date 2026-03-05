# スライド作成ワークフロー

## 目的

Markdownで内容を管理しつつ、最終的にPowerPoint（PPTX）で共有できるようにする。

---

## 選択肢の比較

| 方法 | 特徴 | 編集可能PPTX | 組織テンプレート対応 |
|------|------|-------------|-------------------|
| **Marp CLI** | Markdown → PPTX/PDF/HTML。VSCodeプレビュー可。v4.1.0で編集可能PPTX対応 | ○（--pptx-editable、LibreOffice必要） | △（CSSテーマで対応） |
| **python-pptx** | Pythonで直接PPTX生成。テンプレート挿入が得意 | ◎（ネイティブPPTX） | ◎（既存テンプレート再利用） |
| **Pandoc** | 汎用変換ツール。Markdown → PPTX | △（レイアウト制限あり） | ○（参照テンプレート指定可） |

## 推奨ワークフロー

```
① Marp形式Markdownで内容作成・レビュー
   ├── VSCode + Marp拡張でリアルタイムプレビュー
   └── 内容の修正はMarkdownで完結

② Marp CLIでPPTX出力（2通り）
   ├── marp slide.md --pptx           → 画像ベースPPTX（そのまま配布）
   └── marp slide.md --pptx-editable  → 編集可能PPTX（LibreOffice必要）

③ 必要に応じてPowerPointで微調整
```

### 補足: python-pptxを使う場合

組織テンプレートに合わせる必要がある場合や、図表を動的に生成したい場合はpython-pptxが有利。
ただし、今回は**内容固め → 共有**が目的なので、Marpで十分。

---

## セットアップ（必要な場合）

### Marp CLI
```bash
npm install -g @marp-team/marp-cli
# または
npx @marp-team/marp-cli slide.md --pptx
```

### VSCode拡張
- **Marp for VS Code** (marp-team.marp-vscode)
- Markdownファイルの先頭に `marp: true` を書くだけでプレビュー有効

### 編集可能PPTX（オプション）
```bash
# LibreOfficeが必要
sudo apt install libreoffice-impress
marp slide.md --pptx-editable
```

---

## 藤田さんの実際のワークフロー

```
[Ubuntu PC]
    │
    │  ① Marp形式.mdを作成（VSCode + Marp拡張でプレビュー）
    │
    │  ② git push
    ↓
[Windows PC]
    │
    │  ③ git pull
    │
    │  ④ .mdファイルをパワポ内のClaudeに渡す
    │
    │  ⑤ PPTX生成
    ↓
[完成] 石川さんに共有
```

### 手順詳細

1. **Ubuntu PC**: `sessions/session10/ishikawa-slide-draft.md` などを作成
2. **git push**: 変更をリポジトリにプッシュ
3. **Windows PC**: `git pull` で最新を取得
4. **パワポ内Claude**: mdファイルの内容を渡してPPTX生成を依頼
5. **微調整**: 必要に応じてPowerPointで編集

---

## 参考

- [Marp CLI GitHub](https://github.com/marp-team/marp-cli)
- [Marp CLI v4.1.0 編集可能PPTX](https://qiita.com/youtoy/items/e7168d762d3fe909d278)
- [Marp入門](https://zenn.dev/sui_water/articles/c33eab5732f0c3)
- [python-pptx テンプレート活用](https://qiita.com/a-yamagata/items/7c87b2cfd6c515e2aa65)
- [Pandoc Markdown→PPTX](https://js2iiu.com/2025/09/14/pandoc-powerpoint/)
