# 品質保証・品質管理 用語・略語・著名人 整理ノート

> **目的**: QA組織立ち上げにあたり、品質関連の概念・略語・役割・著名人を体系的に整理する
> **ステータス**: 初期整理段階。深掘りが必要な項目は `[TODO]` でマーク

---

## 1. 概念の階層構造

```
品質マネジメント（QM: Quality Management）
├── 品質保証（QA: Quality Assurance）
│   → プロセス志向・予防的
│   → 「良いものが作られる仕組み」を担保する
│
└── 品質管理（QC: Quality Control）
    → プロダクト志向・検出的
    → 「基準を満たしているか」を検査・測定する
```

- ソフトウェア文脈では頭に **S** がついて **SQA / SQC / SQM** となるが構造は同じ

---

## 2. 主要な略語・用語一覧

### 2.1 組織・プロセス系

| 略語 | 正式名称 | 概要 |
|------|----------|------|
| QM | Quality Management | 品質マネジメント全体の上位概念 |
| QMS | Quality Management System | ISO 9001等で定義される品質マネジメントの仕組み |
| TQM | Total Quality Management | 組織全体で品質を追求する経営手法 |
| TQC | Total Quality Control | TQMの原型。日本発の全社的品質管理 |
| CMMI | Capability Maturity Model Integration | プロセス成熟度を5段階で評価するモデル |
| SEPG | Software Engineering Process Group | CMMI等のプロセス改善推進グループ |
| SPI | Software Process Improvement | ソフトウェアプロセス改善活動全般 |

### 2.2 手法・ツール系

| 略語 | 正式名称 | 概要 |
|------|----------|------|
| PDCA | Plan-Do-Check-Act | デミングが広めた改善サイクル |
| V&V | Verification & Validation | 「正しく作っているか」と「正しいものを作っているか」の2軸 |
| SQuaRE | ISO/IEC 25000シリーズ | ソフトウェア品質の測定・評価の国際標準 |

[TODO] FMEA、FTA、SPC、QFDなど製造業由来の手法も整理する

### 2.3 役割系

| 略語・名称 | 概要 |
|------------|------|
| QAエンジニア / SQAエンジニア | プロセスと品質保証を担う |
| QCエンジニア | 検査・テスト寄りの役割 |
| SET (Software Engineer in Test) | テスト自動化・テスタビリティの開発に重点 |
| SDET (Software Development Engineer in Test) | SETとほぼ同義。Microsoft発祥の呼称 |
| TEM / Test Manager | テスト活動全体の計画・管理 |

[TODO] QAマネージャー、QAリード、QAアーキテクトなど組織階層ごとの役割定義を深掘り

---

## 3. 「SQM」の多義性 — 同じ略語・異なる文脈

| 文脈 | SQMの意味 | 使われる領域 |
|------|-----------|-------------|
| ソフトウェア開発 | **S**oftware **Q**uality **M**anagement | IT・ソフトウェア全般 |
| サプライチェーン・製造業 | **S**upplier **Q**uality **M**anagement | 自動車、半導体、医療機器、ドローン製造など |
| 通信・ITサービス | **S**ervice **Q**uality **M**anagement | ITIL、通信事業者など |

> **注意**: QA組織のスコープ定義時に「自分たちのSQMはどの意味か」を明確にする必要がある。
> ドローン事業の場合、ハードウェアサプライチェーン側とソフトウェア側の両方が視野に入りうる。

[TODO] Supplier Quality Managementの具体的なプラクティス（受入検査、サプライヤー監査、SQA計画など）を整理

---

## 4. 著名人

### 4.1 品質マネジメントの父祖たち（製造業起源）

| 人物 | 主な貢献 |
|------|----------|
| **W. Edwards Deming** | PDCAサイクル、統計的品質管理を日本に伝えた。デミング賞の由来 |
| **Joseph Juran** | 品質三部作（品質計画・品質管理・品質改善）のフレームワーク |
| **Philip Crosby** | 「Quality is Free（品質はタダ）」の著者。ZD（ゼロ・ディフェクト）運動 |

[TODO] Armand Feigenbaum（TQCの提唱者）、Walter Shewhart（統計的品質管理の元祖、管理図の考案者）も追加調査

### 4.2 日本の品質の巨人たち

| 人物 | 主な貢献 |
|------|----------|
| **石川馨（かおる）** | 特性要因図（フィッシュボーン・ダイアグラム）の考案者。TQCを日本に根付かせた |
| **田口玄一** | タグチメソッド（品質工学）の創始者。ロバスト設計の概念を確立 |
| **新郷重夫** | ポカヨケ（mistake-proofing）の概念を体系化 |

[TODO] 日本の品質管理の歴史的文脈（戦後の統計的品質管理導入〜QCサークル運動〜TQC/TQM）を整理

### 4.3 ソフトウェア品質 — 国際

| 人物 | 主な貢献 |
|------|----------|
| **Watts Humphrey** | CMMの父。PSP（Personal Software Process）/TSP（Team Software Process）の提唱者 |
| **Gerald Weinberg** | 「プログラミングの心理学」など。人間系の品質に着目 |
| **Capers Jones** | ソフトウェアメトリクスと品質測定の先駆者 |

[TODO] Michael Bolton、James Bach（Context-Driven Testing）、Cem Kaner（テスト技法）なども調査

### 4.4 日本のソフトウェア品質・テスト分野

| 人物 | 主な貢献 |
|------|----------|
| **西康晴** | 電通大教授。JaSST/ASTERの中心人物。日本のテストコミュニティの牽引者 |
| **秋山浩一** | 組み合わせテスト技法「HAYST法」の考案者 |
| **湯本剛** | テスト設計・テストプロセスの実践家として著名 |
| **鷲崎弘宜** | 早稲田大学教授。ソフトウェア品質の学術面のリーダー |

[TODO] 各人物の代表的な書籍・論文・講演をリストアップ

---

## 5. 次のセッションで深掘りしたい項目

- [ ] 製造業由来の品質手法（FMEA、FTA、SPC、QFD等）のソフトウェアへの応用
- [ ] Supplier Quality Managementの具体的プラクティス
- [ ] QA組織の役割定義（マネージャー/リード/アーキテクト等）の業界標準的パターン
- [ ] ISO 9001 / ISO/IEC 25000（SQuaRE）/ ISO/IEC 33000（プロセス評価）の関係整理
- [ ] Context-Driven Testing School vs. Factory School など、テスト哲学・学派の整理
- [ ] 日本の品質管理の歴史的系譜（デミング来日〜QCサークル〜TQC/TQM〜ソフトウェアQA）
- [ ] 各著名人の代表的な書籍・論文リスト
- [ ] ドローン事業におけるQAスコープの定義（ハードウェア/ソフトウェア/サービスの境界）
