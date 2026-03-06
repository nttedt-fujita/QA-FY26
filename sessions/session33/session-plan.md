# Session 33 計画

**目的**: To-Beドメインモデルの作成

---

## やること

### 1. To-Beドメインモデル作成

Excelから読み取れた範囲で、To-Be（あるべき姿）のドメインモデルを作成:

- 品質管理フレームワーク（IQC/PQC/OQC）を反映
- ロット概念、サプライヤID等のTo-Be要素を含める
- Draw.ioで作成

### 2. as-is-model.drawioの修正

配置がおかしい問題を修正（右下に寄っている）

### 3. ダッシュボードの動作確認

```bash
streamlit run tools/incoming_inspection/dashboard.py
```

実際に起動して、グラフの表示を確認

---

## 参照資料

- [domain-modeling-approach.md](../../docs/missions/m3-incoming-inspection-db/as-is/domain-modeling-approach.md)
- [ears-requirements-hypotheses.md](../../docs/missions/m3-incoming-inspection-db/to-be/ears-requirements-hypotheses.md)
- [qa-gap-analysis.drawio](../../docs/missions/m3-incoming-inspection-db/as-is/qa-gap-analysis.drawio)
