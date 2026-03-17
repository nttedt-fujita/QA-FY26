# Session 226 計画

**目的**: BBR優先順位問題の解決

**前提**: Session 225でBBRの「値の存在」がFlash設定を無視する原因と特定

---

## やること

| # | 作業 | 読むべきファイル | 参照コマンド |
|---|------|-----------------|--------------|
| 1 | 対策案の検討・決定 | session225/session-summary.md | - |
| 2 | 対策の実装 | cfg_valset.rs, device_api.rs | - |
| 3 | 実機テスト | - | make connect, make cfg-valget-bbr |
| 4 | ドキュメント更新 | config-layers-spec.md, 32-cfg-msgout-periodic-output.md | - |

---

## 詳細

### 1. 対策案の検討・決定

Session 225で挙げた3つの案:

| 案 | 内容 | メリット | デメリット |
|----|------|----------|-----------|
| A | disable_periodic_outputをRAMのみに変更 | BBRに書き込まない | 接続中のみ有効（意図通り） |
| B | CFG-VALDELでBBRから値を削除 | 既存のBBR値をクリア | 効果要検証、APIが必要 |
| C | 接続時にset-periodic-output実行 | 毎回設定で確実 | 冗長な設定 |

**推奨**: 案A（disable_periodic_outputをRAMのみに変更）
- シンプルで副作用が少ない
- 接続時の一時的な無効化という意図に合致

### 2. 対策の実装

`disable_periodic_output`のレイヤー指定を変更:

```rust
// 変更前
cfg_valset::set_periodic_output(ZERO_CONFIG, Layer::RamAndBbr)

// 変更後
cfg_valset::set_periodic_output(ZERO_CONFIG, Layer::Ram)
```

### 3. 実機テスト

1. connect-rawで接続（disable_periodic_outputスキップ）
2. cfg-valget-bbrで現在のBBR状態を確認
3. connect（通常接続）でdisable_periodic_output実行
4. cfg-valget-bbrで再確認 → 値が変わっていないことを確認
5. set-periodic-output-flashで設定
6. USB抜き差し
7. message-scanで出力確認

### 4. ドキュメント更新

Session 225の発見事項を反映:

| ドキュメント | 追記内容 |
|-------------|----------|
| config-layers-spec.md | BBRの「値の存在」について |
| 32-cfg-msgout-periodic-output.md | USB vs UART1の説明 |

---

## 参照

- [session225/session-summary.md](../session225/session-summary.md) - 発見事項の詳細
- [config-layers-spec.md](../session220/config-layers-spec.md) - 現在の仕様書
- [32-cfg-msgout-periodic-output.md](../../docs/missions/m1-sensor-evaluation/gnss/32-cfg-msgout-periodic-output.md) - 定期出力設定ドキュメント

---

*作成: Session 225 (2026-03-17)*
