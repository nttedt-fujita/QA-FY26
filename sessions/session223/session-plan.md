# Session 223 計画

**目的**: CFG-VALGET API作成 + Flash搭載確認

**前提**: Session 222でFlashレイヤーへの保存がACKを返すが維持されない問題を確認

---

## やること

| # | 作業 | 読むべきファイル | 参照コマンド |
|---|------|-----------------|--------------|
| 1 | CFG-VALGET仕様確認 | sessions/session214/cfg-valget-spec.md | - |
| 2 | CFG-VALGET API作成 | - | - |
| 3 | Flashレイヤーから値読み取りテスト | - | make cfg-valget-flash |

---

## 詳細

### 1. CFG-VALGET仕様確認

Session 214で抽出済みの仕様を確認:
- リクエスト: layer指定 + key指定
- レスポンス: 値 or エラー

### 2. CFG-VALGET API作成

```
GET /api/devices/{path}/cfg-valget?layer=flash&key=CFG-MSGOUT-UBX_NAV_PVT_USB
```

レスポンス例:
```json
{
  "key": "CFG-MSGOUT-UBX_NAV_PVT_USB",
  "layer": "Flash",
  "value": 1
}
```

### 3. Flashレイヤー確認

1. set-periodic-output-flash実行（Flashに保存）
2. cfg-valget-flash実行（Flashから読み取り）
3. 結果:
   - 値が返る → Flash搭載、書き込み成功
   - エラー → Flash非搭載

---

## 期待する結論

| 結果 | 意味 | 対応策 |
|------|------|--------|
| Flash搭載 | 設定は維持されるはず | 別の問題を調査 |
| Flash非搭載 | 設定は維持できない | ソフトウェア対応（接続時に自動設定） |

---

## 参照

- [Session 222 summary](../session222/session-summary.md)
- [cfg-valget-spec.md](../session214/cfg-valget-spec.md)
