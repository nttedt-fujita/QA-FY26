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
