# BBR優先順位問題 テスト手順

**作成**: Session 226 (2026-03-17)

---

## テスト方針

各テストは以下をセットで行う:

1. USB抜き差し
2. BE再起動
3. 状態確認（cfg-valget）
4. メッセージスキャン

**理由**: USB抜き差しでRAMがクリアされ、BBR/Flashから値が読み込まれる。この状態を正確に確認するため、毎回クリーンな状態から始める。

---

## 確認すべき事実

| # | 確認事項 | 方法 |
|---|----------|------|
| 1 | 現在のBBRの状態 | cfg-valget-bbr |
| 2 | 現在のFlashの状態 | cfg-valget-flash |
| 3 | 修正後、connectでBBRに書き込まないこと | 接続前後でcfg-valget-bbr比較 |
| 4 | Flashに設定した値がUSB抜き差し後に有効になること | set-periodic-output-flash → 抜き差し → message-scan |

---

## テスト手順

### Step 1: 修正前の状態確認（現状把握）

```bash
# 1. USB抜き差し
# 2. BE起動
make dev-backend

# 3. connect-raw（disable_periodic_outputスキップ）
make connect-raw

# 4. 状態確認
make cfg-valget-bbr
make cfg-valget-flash

# 5. メッセージスキャン
make message-scan

# 6. 切断
make disconnect

# 7. BE停止
```

### Step 2: 修正後の動作確認

```bash
# 1. USB抜き差し
# 2. BE起動（修正済みコード）
make dev-backend

# 3. connect（通常接続 = disable_periodic_output実行）
make connect

# 4. 状態確認 → BBRの値が変わっていないことを確認
make cfg-valget-bbr
make cfg-valget-flash

# 5. set-periodic-output-flash → Flash に設定
make set-periodic-output-flash

# 6. 状態確認
make cfg-valget-flash

# 7. メッセージスキャン
make message-scan

# 8. 切断
make disconnect

# 9. BE停止
```

### Step 3: USB抜き差し後の確認

```bash
# 1. USB抜き差し
# 2. BE起動
make dev-backend

# 3. connect-raw（disable_periodic_outputスキップ）
make connect-raw

# 4. 状態確認
make cfg-valget-bbr
make cfg-valget-flash

# 5. メッセージスキャン → 定期出力されているか確認
make message-scan
```

---

## 期待結果

### Step 1（現状）

- BBR: 0（値あり）
- Flash: 1
- message-scan: 0件（BBRが優先されて出力されない）

### Step 2（修正後）

- connect後もBBR: 0（変化なし = RAMのみに書き込み）
- set-periodic-output-flash後、Flash: 1
- message-scan: 出力あり（RAMに1がコピーされているため）

### Step 3（USB抜き差し後）

- BBR: 0（残っている）
- Flash: 1
- message-scan: **出力あり**（修正が成功していれば）

---

## Session 226 テスト結果

### Step 1 実施結果（2026-03-17）

| レイヤー | NAV_PVT_USB |
|----------|-------------|
| BBR | 0 |
| Flash | 1 |

→ message-scanは未実施（時間切れ）

### Step 2-3

→ 次セッションで実施
