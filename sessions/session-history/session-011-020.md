# セッション履歴: Session 11〜20

## Session 11 (2026-03-05)

**概要**: M1/M2の現状把握と1次調査方針の整理。スケジュール・進め方の集約ドキュメント作成。

**背景**: PowerPoint資料作成中に、M1/M2の具体的な進め方が全く見えていないことが判明。ヒアリングを通じて現状を把握し、次セッション以降の調査方針を整理した。

**作成ファイル**:
| ファイル | 内容 |
|----------|------|
| [session11/schedule-and-m1m2-summary.md](../session11/schedule-and-m1m2-summary.md) | スケジュール・M1/M2進め方の集約（PPTX転記用） |
| [session11/m1m2-current-status-and-next-steps.md](../session11/m1m2-current-status-and-next-steps.md) | M1/M2現状整理・次セッション方針 |
| [session11/session-summary.md](../session11/session-summary.md) | セッションサマリー |

**重要な発見**:
- **M1-A（Lidar）**: 評価環境ゼロ、規格・設備を探すところから
- **M1-B（GNSS）**: 小板橋さんが知見あり、ヒアリング必須
- **M2**: False Alarm低減作業を実施中、開発中・前例なし
- **M1が重すぎる問題**: Lidar + GNSSの2アイテムを1ミッションで抱えている
- **1次調査ができていない**: 規格名は知っているが、自社でどう適用するか見えていなかった

**次セッション（Session 12）でやること**:
- 小板橋さん向けGNSSヒアリング項目の作成
- Lidar評価環境のWeb調査（規格・設備・外注可能性）
- 1次調査の実施準備

## Session 13 (2026-03-05)

**概要**: 石川さんヒアリング（#02）結果の整理、PDFスライド内容のドキュメント化、プロジェクト状況の再整理。

**背景**: Session 12で作成したスライドを使って石川さんとの認識合わせを実施。スケジュール・優先度に関する重要な認識更新があった。

**作成ファイル**:
| ファイル | 内容 |
|----------|------|
| [session13/hearing-notes-ishikawa-02.md](../session13/hearing-notes-ishikawa-02.md) | 石川さんヒアリングメモ #02（スケジュール・M1〜M4方針） |
| [session13/project-status-reorg.md](../session13/project-status-reorg.md) | プロジェクト状況 再整理（全ミッション・待ち事項） |
| [session13/session-summary.md](../session13/session-summary.md) | セッションサマリー |

**重要な発見**:
- **優先度変更**: M3/M4が「優先度1」→ M1/M2の方が重要・緊急
- **20L機体 2027年2月飛行** → M1/M2の実質的な締め切り
- **M1の意味確定**: ドローン本体搭載状態での評価手法の確立
- **M2の意味確定**: LiDAR・点群データの専門知識を身につける（外注発注者として）
- **ロット定義**: こちらで自社定義してよい（石川さん承認）
- **バッテリー問題**: 不合格記録できない・管理者未定
- **M4論点未解決**: モックアップ先行（石川さん案）vs ドメインモデリング先行（藤田案）

**次セッション（Session 14）でやること**:
- 待ち事項（小板橋さんGNSS / HORIBA回答 / 小笠原さんM4）の進捗確認
- M4の進め方の方針決定
- 進んだ案件のヒアリング結果整理

## Session 12 (2026-03-05)

**概要**: M1/M2の1次調査準備。GNSSヒアリング項目作成、Lidar評価環境Web調査、ドキュメント整備。

**背景**: Session 11で判明した「M1/M2は1次調査ができていない」問題に対応。スライド完成に必要な情報を揃えるため、ヒアリング準備と外部調査を実施。

**作成ファイル**:
| ファイル | 内容 |
|----------|------|
| [session12/gnss-hearing-items.md](../session12/gnss-hearing-items.md) | 小板橋さん向けGNSSヒアリング項目（16問） |
| [session12/lidar-evaluation-research.md](../session12/lidar-evaluation-research.md) | Lidar評価環境Web調査結果 |
| [session12/schedule-and-m1m2-summary-v2.md](../session12/schedule-and-m1m2-summary-v2.md) | スケジュール・M1/M2進め方（v2、最新） |
| [session12/session-summary.md](../session12/session-summary.md) | セッションサマリー |

**重要な発見**:
- **Lidar評価環境構築**: 3つの選択肢（外部委託/自社構築/ハイブリッド）
- **国内サービス候補**: HORIBA Techno Service、OEC、北陽電機等
- **スライド作成に渡す資料**: schedule-and-m1m2-summary-v2.md を渡せばOK
- **古いスライド削除**: fujita-mission-slide.md、qa-overview-slide.md を削除

**次セッション（Session 13）でやること**:
- 小板橋さんヒアリング結果の整理（実施後）
- HORIBA・OECへの問い合わせ結果の整理（回答後）
- スライド更新の確認

## Session 14 (2026-03-05)

**概要**: ヒアリング前準備。クローズドクエスチョン整理、モックアップ構成案、分析観点の整理。

**背景**: ヒアリング（小笠原さん・小板橋さん）がまだ実施できていない中、明日のSCM/品質保証DX打ち合わせに向けて準備を進めた。

**作成ファイル**:
| ファイル | 内容 |
|----------|------|
| [session14/closed-questions-m3m4.md](../session14/closed-questions-m3m4.md) | クローズドクエスチョン（A〜K、28問） |
| [session14/mockup-concepts.md](../session14/mockup-concepts.md) | モックアップ構成案（テキスト版） |
| [session14/analysis-what-to-build.md](../session14/analysis-what-to-build.md) | 自作 vs QuickSightの仕分け |
| [session14/ears-prevention-hypotheses.md](../session14/ears-prevention-hypotheses.md) | EARS要求（予防・監視追加版） |
| [session14/analysis-to-input-mapping.md](../session14/analysis-to-input-mapping.md) | 分析要求→入力項目の逆算 |
| [session14/session-summary.md](../session14/session-summary.md) | セッションサマリー |

**重要な発見**:
- **分析フレームワーク**: 時間軸（過去/現在/未来）× 4M（Man/Machine/Material/Method）で整理
- **Session 7のEARS要求の不足**: 「現在監視」「未来予測」の観点がなかった → 追加
- **自作 vs QuickSight**: 入力は自作、分析はQuickSightに任せる
- **逆算の重要性**: 「何を分析したいか」→「何を記録すべきか」→「入力項目」

**次セッション（Session 15）でやること**:
- sessions/配下の資料をdocs/に集約
- 全体レビュー（資料の構成・整合性チェック）

## Session 15 (2026-03-05)

**概要**: 資料集約、GNSSヒアリング対応（割り込み）。

**背景**: 計画通り資料集約を開始したところ、小板橋さんが捕まったため急遽GNSSヒアリングを実施。

**作成ファイル**:
| ファイル | 内容 |
|----------|------|
| [session14/closed-questions-m3m4.csv](../session14/closed-questions-m3m4.csv) | M3/M4クローズドクエスチョンCSV版 |
| [session12/gnss-hearing-items.csv](../session12/gnss-hearing-items.csv) | GNSSヒアリングCSV版 |
| docs/missions/ 配下に多数 | 資料集約（M1, M3/M4） |
| docs/technical-research/ 配下に多数 | 技術調査集約 |

**重要な発見**:
- **GNSSはLidarより要求が明確** → 先に取り組む価値あり
- u-centerというツールが既にある
- 小板橋さんの期待: ツール作成 + 標準化
- Xmindにメモあり（次回読み取り）

**未完了タスク**:
- docs/index.md 更新（途中）
- CLAUDE.md 索引更新（未着手）

**次セッション（Session 16）でやること**:
- Xmindメモの読み取り
- GNSSヒアリング結果整理
- M1-B（GNSS）優先着手の判断
- Session 15残タスク完了

## Session 16 (2026-03-06)

**概要**: 小板橋さんGNSSヒアリング結果整理、u-center調査、M1-B作業フロー整理。

**背景**: Session 15で小板橋さんにヒアリングした内容（Xmindメモ）を読み取り、整理。ツール設計の前に確認すべきことを明確化。

**作成ファイル**:
| ファイル | 内容 |
|----------|------|
| [session16/gnss-hearing-koitabashi-01.md](../session16/gnss-hearing-koitabashi-01.md) | ヒアリング結果（Xmind + CSV統合） |
| [session16/u-center-research.md](../session16/u-center-research.md) | u-center調査結果 |
| [session16/m1b-gnss-roadmap.md](../session16/m1b-gnss-roadmap.md) | 全体の作業フロー・確認事項 |
| [session16/session-summary.md](../session16/session-summary.md) | セッションサマリー |

**重要な発見**:
- **小板橋さん指摘**: ツール作成の前に検証項目を固めるのが重要
- **合格基準**: 現状は曖昧
- **評価手順書**: ない
- **相談できる人**: 末永さん
- **u-center**: UBXプロトコル仕様書が公開されている（逆コンパイル不要）

**作業フロー確定**:
```
Phase 0: 現状把握      ← 完了
Phase 1: 検証項目の妥当性検証  ← 次はここ
Phase 2: ツール設計
Phase 3: ツール実装
Phase 4: 標準化
```

**次セッション（Session 17）でやること**:
- アプリケーションノートPDF確認
- 小板橋さんのExcel確認
- UBXプロトコル仕様書調査（余裕があれば）

## Session 17 (2026-03-06)

**概要**: PDF（UBX Interface Description）確認・Excel（小峰無線GPS確認）全シート詳細分析。

**背景**: Phase 1（検証項目の妥当性検証）の準備として、技術資料と現状データを読み込んだ。

**作成ファイル**:
| ファイル | 内容 |
|----------|------|
| [session17/pdf-excel-analysis.md](../session17/pdf-excel-analysis.md) | PDF・Excel全シート詳細分析（未確認項目リスト付き） |
| [session17/session-summary.md](../session17/session-summary.md) | セッションサマリー |
| [.gitignore](../../.gitignore) | Excel/PDFをgit管理外に設定 |

**重要な発見**:
- **No.5のRTK FIX不可の原因仮説**: Q2（QZSS）L1 = 13dBHz（他機体は30〜48）→ 受信感度の低さが直結
- **全項目で合格基準なし**: 受信感度・RTK FIX時間・水平精度・衛星数すべて未定
- **内部設定の仕様合意が未完了**: 小峰無線との取り交わし必要とのコメントあり
- **QZSS L2は全機体で受信不可**: 設定か仕様か不明
- **Spectrum Analyzeは2社比較**: 自社Ultralight vs HORIBA参考品

**未確認項目（次セッションで補完）**:
- 画像88枚（スペアナ波形・飛行ログ・MAGずれ量等）→ スクリーンショット共有して補完予定

**PDFの性格確認**:
- UBX Interface Description = チップの通信プロトコル仕様書（u-centerのAPI仕様書）
- Phase 1では不要。Phase 2（ツール設計）で使う

**次セッション（Session 18）でやること**:
- 画像88枚の内容補完（スクリーンショット共有）
- 合格基準の検討（Phase 1の核心）
- 末永さんヒアリング準備

## Session 18 (2026-03-06)

**概要**: Excel画像29枚の読み取り・分析。Session 17テキスト分析と統合し、シートごとの詳細ドキュメントを `docs/missions/m1-sensor-evaluation/gnss/` に作成。

**背景**: Session 17でExcelのテキストデータは分析済みだったが、画像88枚が未読だった。スクリーンショットを共有してもらい、受信感度・スペアナ・飛行ログ・MAG確認の4シート分（29枚）を読み取り。電池確認・No.02シートは意味が薄いと判断しスキップ。

**作成ファイル**:

統合ドキュメント（`docs/missions/m1-sensor-evaluation/gnss/`）:
| ファイル | 内容 |
|----------|------|
| [gnss/README.md](../../docs/missions/m1-sensor-evaluation/gnss/README.md) | インデックス（全8ドキュメントの一覧、Phase位置づけ） |
| [gnss/01-internal-settings.md](../../docs/missions/m1-sensor-evaluation/gnss/01-internal-settings.md) | 内部設定（FW/PROTVER/パラメータ、小峰無線との合意未完了） |
| [gnss/02-reception-sensitivity.md](../../docs/missions/m1-sensor-evaluation/gnss/02-reception-sensitivity.md) | 受信感度 全3回測定データ + u-center画像 + 仰角別整理 |
| [gnss/03-spectrum-analyze.md](../../docs/missions/m1-sensor-evaluation/gnss/03-spectrum-analyze.md) | スペアナ波形（PGA値・波形形状、No.5 L2帯異常） |
| [gnss/04-flight-test.md](../../docs/missions/m1-sensor-evaluation/gnss/04-flight-test.md) | 飛行試験テーブル + 飛行中ログ7項目の時系列分析 |
| [gnss/05-mag-check.md](../../docs/missions/m1-sensor-evaluation/gnss/05-mag-check.md) | MAG確認（No.02の45度ずれ→キャリブ解消） |
| [gnss/06-battery-check.md](../../docs/missions/m1-sensor-evaluation/gnss/06-battery-check.md) | バックアップ電池確認（テキストのみ） |
| [gnss/07-cross-sheet-findings.md](../../docs/missions/m1-sensor-evaluation/gnss/07-cross-sheet-findings.md) | 横断発見事項・合格基準叩き台・末永さんヒアリング項目10問 |
| [gnss/08-ubx-protocol-index.md](../../docs/missions/m1-sensor-evaluation/gnss/08-ubx-protocol-index.md) | UBX仕様書索引（Phase 2用、主要メッセージ7件） |

セッション作業ファイル（`sessions/session18/`）:
| ファイル | 内容 |
|----------|------|
| [session18/reception-sensitivity-analysis.md](../session18/reception-sensitivity-analysis.md) | 受信感度画像分析（作業用） |
| [session18/spectrum-analyze-analysis.md](../session18/spectrum-analyze-analysis.md) | スペアナ画像分析（作業用） |
| [session18/flight-log-20260218-analysis.md](../session18/flight-log-20260218-analysis.md) | 飛行ログ画像分析（作業用） |
| [session18/mag-check-analysis.md](../session18/mag-check-analysis.md) | MAG確認画像分析（作業用） |

**重要な発見**:
- **No.5の異常が4つのデータソースで裏付けられた**: 受信感度（Q2 L1=13dBHz）、スペアナ（RF2 PGA=38dB、波形異常）、飛行試験（RTK FIX NG、衛星19）、L2帯に限定された問題
- **根本原因の仮説**: L2帯の受信不良 → RTK FIXにはL1+L2の二周波が必要
- **No.02のMAG 45度ずれ**: 2/8にキャリブで解消済み
- **合格基準の叩き台を作成**: ただしエビデンス（業界標準・メーカー推奨値）がまだない

**次セッション（Session 19）でやること**:
- PDF（UBX仕様書）の詳細分析（未読部分の確認）
- Web調査で合格基準のエビデンス収集（論文・メーカー資料・業界標準）
- 末永さんヒアリング準備（エビデンスを踏まえた質問構成）
