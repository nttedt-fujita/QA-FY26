# セッション履歴: Session 51〜60

## Session 51 (2026-03-09)

**概要**: DB図とドメインモデルの乖離確認（短時間セッション）。

**実施内容**:
1. **乖離確認** — Session 50のER図（schema.sql）とSession 33のTo-Beドメインモデルを比較
2. **結論** — 乖離なし。8テーブルすべて一致、属性レベルでも一致

**比較ファイル**:
- [prototype/db/schema.sql](../../prototype/db/schema.sql)
- [docs/missions/m3-incoming-inspection-db/to-be/to-be-model.drawio](../../docs/missions/m3-incoming-inspection-db/to-be/to-be-model.drawio)

**作成ファイル**:
| ファイル | 内容 |
|----------|------|
| [session51/session-summary.md](../session51/session-summary.md) | セッションサマリー |
| [session52/session-plan.md](../session52/session-plan.md) | 次セッション計画 |

**未実施タスク（Session 52へ持ち越し）**:
- M1-M4の進捗整理
- M4ヒアリング項目の整理
- 過去セッション資料のメンテナンス

**次セッション（Session 52）でやること**:
- Session 51計画をそのまま引き継ぎ

---

## Session 52 (2026-03-09)

**概要**: ミッション進捗棚卸し + M2方針転換 + 小笠原さんフィードバック反映

**実施内容**:
1. **M1-M4進捗整理** — 各ミッションの現状を一覧表にまとめ
2. **小笠原さんフィードバック反映**:
   - M3プロトタイプは一旦ストップ
   - M4（工程不良DB）の優先度を上げる
   - 300時間削減はバッテリー・チャージャー検査が対象
3. **M2方針転換** — 「点群データ検証」→「障害物検知評価」として再定義
4. **M2障害物検知評価の調査** — 業界規格・指標を調査（EUROCAE ED-267等）
5. **M4ヒアリング項目整理**

**重要な決定**:
- M3プロトタイプは一旦ストップ
- M4の優先度上昇（既に流出している問題への対策が先）
- M2は「障害物検知システムの評価」として再定義
- M1-B GNSS評価を再開

**作成ファイル**:
| ファイル | 内容 |
|----------|------|
| [session52/mission-progress.md](../session52/mission-progress.md) | M1-M4進捗一覧表 |
| [session52/m2-obstacle-detection-report.md](../session52/m2-obstacle-detection-report.md) | M2障害物検知評価の調査レポート（ソース併記） |
| [session52/m2-confirmation-checklist.md](../session52/m2-confirmation-checklist.md) | FA率評価方法の確認リスト |
| [docs/missions/m4-defect-db/hearing-items-m4.md](../../docs/missions/m4-defect-db/hearing-items-m4.md) | M4固有ヒアリング項目 |
| [session53/session-plan.md](../session53/session-plan.md) | 次セッション計画 |

**次セッション（Session 53）でやること**:
- M1-B GNSS評価再開（末永さんヒアリング準備）
- M4工程不良Excel入手依頼
- M2確認リストの回答収集

---

## Session 53 (2026-03-09)

**概要**: M1-B GNSS設計検証基準の業界標準調査

**実施内容**:
1. **GNSS設計検証基準の業界標準調査** — C/N0、RTK FIX時間、TTFF等の閾値を調査
2. **文脈の整理** — 「受入検査」ではなく「原理試作に対する設計検証」であることを明確化
3. **ドキュメント整理** — READMEに文脈追記、調査資料を正式配置

**重要な発見**:
- L1受信感度: 業界標準は≥30 dBHz（叩き台の25 dBHzは甘すぎる）
- RTK FIX時間: 業界標準は≤30秒（叩き台の3分は緩すぎる）
- 叩き台に含まれていない項目: TTFF、PDOP、再捕捉時間、ジャミング耐性

**重要な学び**:
- 評価の文脈（何のための評価か）はREADMEに最初に書いておくべき

**作成・更新ファイル**:
| ファイル | 内容 |
|----------|------|
| [docs/missions/m1-sensor-evaluation/gnss/README.md](../../docs/missions/m1-sensor-evaluation/gnss/README.md) | 評価の文脈を追記 |
| [docs/missions/m1-sensor-evaluation/gnss/07-cross-sheet-findings.md](../../docs/missions/m1-sensor-evaluation/gnss/07-cross-sheet-findings.md) | 業界標準調査結果で更新 |
| [docs/missions/m1-sensor-evaluation/gnss/09-design-verification-criteria.md](../../docs/missions/m1-sensor-evaluation/gnss/09-design-verification-criteria.md) | **新規** 設計検証基準調査レポート |
| [session53/session-summary.md](../session53/session-summary.md) | セッションサマリー |
| [session54/session-plan.md](../session54/session-plan.md) | 次セッション計画 |

**次セッション（Session 54）でやること**:
- 小板橋さんへの認識確認
- 末永さんへの相談内容精査
- M4工程不良Excel入手（継続）

---

## Session 54 (2026-03-09)

**概要**: 小板橋さん・末永さんへの確認資料作成

**実施内容**:
1. **製品名・メーカーの裏取り** — 「HORIBAウルトラライト」→「Holybro H-RTK F9P Ultralight」に修正
2. **小板橋さんへの認識確認チェックリスト作成** — 業界標準を適用してよいか確認するための資料
3. **末永さんへの相談チェックリスト作成** — AirGrowのGNSS要件を確認するための資料

**重要な確認事項**:
- 「叩き台」はSession 18でClaudeが仮作成したもの（Excelには合格基準なし）
- 30 dBHz基準を適用するとNo.5は明確に不合格、他はギリギリ合格
- ツール作成はPhase 2以降（まず検証項目を固める必要あり）

**作成ファイル**:
| ファイル | 内容 |
|----------|------|
| [session54/koitabashi-confirmation-checklist.md](../session54/koitabashi-confirmation-checklist.md) | 小板橋さんへの認識確認チェックリスト |
| [session54/suenaga-consultation-checklist.md](../session54/suenaga-consultation-checklist.md) | 末永さんへの相談チェックリスト |
| [session54/session-summary.md](../session54/session-summary.md) | セッションサマリー |
| [session55/session-plan.md](../session55/session-plan.md) | 次セッション計画 |

**次セッション（Session 55）でやること**:
- 小板橋さんへ確認（チェックリスト使用）
- 末永さんへ相談（チェックリスト使用）
- ツール作成タイミングの議論
- M4工程不良Excel入手（継続）

---

## Session 55 (2026-03-09)

**概要**: M3ネットワークアクセス対応 + GNSS調査資料の再発見

**実施内容**:
1. **M3プロトタイプ：別PCからのアクセス対応** — api.ts, package.json変更
2. **GNSSツール調査** — 新規調査は進まず、過去資料（Session 17）を再発見

**問題点**:
- GNSS関連の調査資料が散在しており、見つけにくい
- 適切な場所に整理されていない → 次セッションで整理作業を実施

**変更ファイル**:
| ファイル | 内容 |
|----------|------|
| [prototype/frontend/src/lib/api.ts](../../prototype/frontend/src/lib/api.ts) | API URLを動的に決定（別PCアクセス対応） |
| [prototype/frontend/package.json](../../prototype/frontend/package.json) | `--hostname 0.0.0.0` 追加 |

**作成ファイル**:
| ファイル | 内容 |
|----------|------|
| [session55/session-summary.md](../session55/session-summary.md) | セッションサマリー |
| [session56/session-plan.md](../session56/session-plan.md) | 次セッション計画（GNSS資料整理方針） |

**次セッション（Session 56）でやること**:
- GNSS調査資料の整理・統合（`docs/missions/m1-sensor-evaluation/gnss/`へ）
- 小板橋さんへ確認（継続）
- 末永さんへ相談（継続）
- M4工程不良Excel入手（継続）

---

## Session 56 (2026-03-09)

**概要**: GNSS関連調査資料の整理・統合

**実施内容**:
1. **ツール設計メモ作成** — Session 16-17の内容を統合
2. **PX4 uORBマッピング作成** — ULogから取得可能なデータを明確化
3. **既存資料のステータス更新** — session16の調査タスクステータス

**重要な発見**:
- L1/L2別C/N0、スペアナデータはフライトログ経由では取得できない可能性
- 直接UBX通信が必要な項目と、フライトログで十分な項目を明確化

**作成ファイル**:
| ファイル | 内容 |
|----------|------|
| [docs/missions/m1-sensor-evaluation/gnss/10-tool-design-notes.md](../../docs/missions/m1-sensor-evaluation/gnss/10-tool-design-notes.md) | ツール設計メモ |
| [docs/missions/m1-sensor-evaluation/gnss/11-px4-uorb-mapping.md](../../docs/missions/m1-sensor-evaluation/gnss/11-px4-uorb-mapping.md) | PX4 uORBとUBXの対応 |
| [session56/session-summary.md](../session56/session-summary.md) | セッションサマリー |
| [session57/session-plan.md](../session57/session-plan.md) | 次セッション計画 |

**更新ファイル**:
| ファイル | 内容 |
|----------|------|
| [docs/missions/m1-sensor-evaluation/gnss/README.md](../../docs/missions/m1-sensor-evaluation/gnss/README.md) | 新規ファイル追加 |
| [sessions/session16/gnss-hearing-koitabashi-01.md](../session16/gnss-hearing-koitabashi-01.md) | 調査タスクステータス更新 |

**方針転換**:
- 確認待ちではなく、**先にプロトタイプを作る**方向に転換
- PX4ソースコード調査 → リポジトリ整理 → 技術選定 → プロトタイプ作成

**次セッション（Session 57）でやること**:
- PX4ソースコード調査（事前調査）
- リポジトリ整理（prototype/の命名問題）
- 技術選定・環境構築（C++? Linux/Windows両対応）
- プロトタイプ設計・作成開始

---

## Session 57 (2026-03-09)

**概要**: PX4ソースコード調査 + ADR-004作成 + リポジトリ整理

**実施内容**:
1. **PX4ソースコード調査** — GitHub経由でPX4-GPSDriversを確認
   - UBX-NAV-SIG: 未実装（L1/L2別C/N0取得不可）
   - TTFF: 未実装（構造体にはあるが抽出していない）
2. **ADR-004作成** — 直接UBX通信ツールを採用（PX4改造は将来の選択肢）
3. **エビデンスドキュメント作成** — URL+原文抜粋
4. **リポジトリ整理** — `tools/gnss-eval/` 作成

**重要な決定**:
- GNSS評価ツールは `tools/gnss-eval/` に配置（M3プロトタイプと分離）
- PX4改造は将来の選択肢として保留
- 直接UBX通信ツールを先に作る

**作成ファイル**:
| ファイル | 内容 |
|----------|------|
| [prototype/docs/adr/ADR-004-gnss-tool-approach.md](../../prototype/docs/adr/ADR-004-gnss-tool-approach.md) | 方針決定ADR |
| [docs/missions/m1-sensor-evaluation/gnss/12-px4-source-evidence.md](../../docs/missions/m1-sensor-evaluation/gnss/12-px4-source-evidence.md) | PX4調査エビデンス |
| [tools/gnss-eval/README.md](../../tools/gnss-eval/README.md) | GNSS評価ツール設計メモ |
| [session57/session-summary.md](../session57/session-summary.md) | セッションサマリー |
| [session58/session-plan.md](../session58/session-plan.md) | 次セッション計画 |

**次セッション（Session 58）でやること**:
- 技術選定（Python + pyserial）
- 環境構築
- プロトタイプ設計・実装開始

---

## Session 58 (2026-03-09)

**概要**: リポジトリ整理（prototype/の命名問題解決）

**実施内容**:
1. **ADRをプロジェクトルートに移動** — `prototype/docs/adr/` → `docs/adr/`
2. **M3コードをprototype/m3/に移動** — prototype/直下 → prototype/m3/
3. **参照パスの修正** — CLAUDE.md、docs/index.md、各README等
4. **動作確認** — Goビルド、docker-compose config

**重要な変更**:
- ADRはM3固有ではなく、プロジェクト全体の設計判断記録として `docs/adr/` に配置
- M3プロトタイプは `prototype/m3/` に移動（今後他のプロトタイプが追加される可能性を考慮）

**新しいディレクトリ構成**:
```
prototype/
└── m3/               ← M3受入検査DBプロトタイプ

tools/
└── gnss-eval/        ← GNSS評価ツール（これから実装）

docs/
└── adr/              ← 設計判断記録（全ミッション共通）
```

**更新ファイル**:
| ファイル | 内容 |
|----------|------|
| [CLAUDE.md](../../CLAUDE.md) | ADRパス、implementation-plan.mdパス |
| [docs/index.md](../../docs/index.md) | prototype/m3/への参照、ADRへの参照、ディレクトリ構成 |
| [docs/adr/ADR-003-lot-list-view.md](../../docs/adr/ADR-003-lot-list-view.md) | 相対パス修正 |
| [docs/adr/ADR-004-gnss-tool-approach.md](../../docs/adr/ADR-004-gnss-tool-approach.md) | 相対パス修正 |
| [docs/missions/m3-incoming-inspection-db/README.md](../../docs/missions/m3-incoming-inspection-db/README.md) | prototype/m3/への参照 |
| [tools/gnss-eval/README.md](../../tools/gnss-eval/README.md) | ADRへの参照 |

**作成ファイル**:
| ファイル | 内容 |
|----------|------|
| [session58/session-summary.md](../session58/session-summary.md) | セッションサマリー |
| [session59/session-plan.md](../session59/session-plan.md) | 次セッション計画 |

**未実施（Session 59へ持ち越し）**:
- GNSS評価ツールのプロトタイプ作成（技術選定、環境構築、実装開始）

**次セッション（Session 59）でやること**:
- 技術選定（Python + pyserial）
- 環境構築
- プロトタイプ設計・実装開始

---

## Session 59 (2026-03-09)

**概要**: GNSS評価ツールの技術選定比較

**実施内容**:
1. **GNSSView API調査結果の確認** — 以前の調査レポートを確認
2. **技術選定の比較検討** — 言語、Webフレームワーク、UBXパース方式を網羅的に比較
3. **ADR判断根拠ドキュメント作成** — 全選択肢のメリット・デメリットを整理

**重要な決定**:
- **アプローチ**: Webサーバー型（ブラウザでアクセス）
- **パフォーマンス最重視**: Rust + Actix-web（19-20k req/s）
- **UBXパース**: 自前実装（仕様書ベース、5-6メッセージのみ）or ubxlib（公式）
- **衛星位置計算**: F9PのNAV-SATから直接取得（計算ライブラリ信頼性問題を回避）
- **推奨案**: 案A（Rust + Actix-web + 自前UBXパース）

**作成ファイル**:
| ファイル | 内容 |
|----------|------|
| [session59/gnss-tool-tech-comparison.md](../session59/gnss-tool-tech-comparison.md) | 技術選定比較ドキュメント（ADR判断根拠） |
| [session59/GNSS_View_WebAPI_調査レポート.md](../session59/GNSS_View_WebAPI_調査レポート.md) | GNSSView API調査レポート |
| [session59/session-summary.md](../session59/session-summary.md) | セッションサマリー |
| [session60/session-plan.md](../session60/session-plan.md) | 次セッション計画 |

**次セッション（Session 60）でやること**:
- 技術選定の最終決定（ADR-005作成）
- Rust + Actix-web環境構築
- UBXパース設計
- モック実装開始

---
