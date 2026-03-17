# Session 224 計画

**目的**: CFG-VALGET実機テスト + Flash搭載確認

**前提**: Session 223でCFG-VALGET API作成完了

---

## やること

| # | 作業 | 読むべきファイル | 参照コマンド |
|---|------|-----------------|--------------|
| 1 | 装置接続 | - | make connect |
| 2 | Flashレイヤーから値取得テスト | - | make cfg-valget-flash |
| 3 | 結果に基づく対応策決定 | - | - |

---

## 詳細

### 1. 実機テスト手順

1. `make dev-backend` でバックエンド起動
2. `make connect` で装置接続
3. `make set-periodic-output-flash` でFlashに値を書き込み
4. `make cfg-valget-flash` でFlashから値を読み取り

### 2. 期待する結果

| 結果 | 意味 | 対応策 |
|------|------|--------|
| 値が返る | Flash搭載、書き込み成功 | set-periodic-outputでFlash保存を使用 |
| エラー | Flash非搭載 | ソフトウェア対応（接続時に自動設定） |

---

## 参照

- [Session 223 summary](../session223/session-summary.md)
- [cfg-valget-spec.md](../session214/cfg-valget-spec.md)
