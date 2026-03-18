graph TB
    subgraph Phase1["Phase 1: 現状把握"]
        A1["現場観察・ヒアリング"]
        A2["工数内訳の分解"]
        A3["プロセスフローの把握"]
    end

    subgraph FW1["適用フレームワーク"]
        F1["CoQ\n品質コスト4分類"]
        F2["VSM\nプロセス可視化"]
        F3["特性要因図\n原因の構造化"]
    end

    subgraph Phase2["Phase 2: 記録基盤の構築"]
        B1["記録すべきデータ項目の定義"]
        B2["記録の仕組みの設計\nDB / ツール"]
        B3["記録運用の開始"]
    end

    subgraph Phase3["Phase 3: データ分析・検査最適化"]
        C1["重点問題の特定"]
        C2["検査の重点再配分"]
        C3["サンプルサイズ適正化"]
    end

    subgraph FW3["適用フレームワーク"]
        F4["パレート図\n重点の可視化"]
        F5["FMEA\nリスク優先順位"]
        F6["ISO 2859 / JIS Z 9015\n抜取計画見直し"]
    end

    subgraph Phase4["Phase 4: 検査プロセス高度化"]
        D1["検査信頼性の定量化"]
        D2["AI活用の検討\n判定基準が明確化された段階で"]
    end

    subgraph FW4["適用フレームワーク"]
        F7["MSA / Gauge R&R\n測定システム解析"]
    end

    subgraph Phase5["Phase 5: 中長期的な検査負荷低減"]
        E1["サプライヤー品質の傾向管理"]
        E2["サプライヤー層別管理\nLevel 1→4"]
        E3["検査頻度の段階的軽減"]
    end

    subgraph FW5["適用フレームワーク"]
        F8["SPC\n統計的工程管理"]
        F9["SQM\nサプライヤー品質管理"]
    end

    %% フロー
    Phase1 --> Phase2
    Phase2 --> Phase3
    Phase3 --> Phase4
    Phase4 --> Phase5

    %% フレームワーク紐付け
    A1 -.-> F1
    A1 -.-> F3
    A2 -.-> F1
    A3 -.-> F2

    C1 -.-> F4
    C2 -.-> F5
    C3 -.-> F6

    D1 -.-> F7

    E1 -.-> F8
    E2 -.-> F9
    E3 -.-> F9

    %% 記録基盤が全体を支える
    B3 -.->|"データが蓄積されて初めて可能"| Phase3
    B3 -.->|"データが蓄積されて初めて可能"| Phase4
    B3 -.->|"3〜6ヶ月分のデータ蓄積後"| Phase5

    %% 現在地
    style Phase1 fill:#4a9eff,color:#fff,stroke:#2a7edf
    style FW1 fill:#4a9eff,color:#fff,stroke:#2a7edf

    style Phase2 fill:#6cb86c,color:#fff,stroke:#4c984c
    style Phase3 fill:#999,color:#fff,stroke:#777
    style FW3 fill:#999,color:#fff,stroke:#777
    style Phase4 fill:#999,color:#fff,stroke:#777
    style FW4 fill:#999,color:#fff,stroke:#777
    style Phase5 fill:#999,color:#fff,stroke:#777
    style FW5 fill:#999,color:#fff,stroke:#777
