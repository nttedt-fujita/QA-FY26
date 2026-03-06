-- M3 受入検査DB スキーマ（プロトタイプ版）
-- To-Beモデル（Session 33）に基づく設計
-- 作成日: 2026-03-06

-- ========================================
-- マスタデータ
-- ========================================

-- サプライヤマスタ
CREATE TABLE suppliers (
    supplier_id TEXT PRIMARY KEY,
    name TEXT NOT NULL,                -- 会社名
    contact TEXT,                      -- 連絡先
    default_aql TEXT,                  -- 標準AQL（例: "1.0", "2.5"）
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- 部品マスタ
CREATE TABLE parts (
    part_id TEXT PRIMARY KEY,
    name TEXT NOT NULL,                -- 品名
    category TEXT NOT NULL,            -- カテゴリ（メカ/エレキ/Api）
    supplier_id TEXT,                  -- サプライヤ（FK）
    spec_doc_url TEXT,                 -- 検査基準書URL
    latest_fw_version TEXT,            -- 最新FWバージョン
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (supplier_id) REFERENCES suppliers(supplier_id)
);

-- 検査項目マスタ
CREATE TABLE inspection_items (
    item_id TEXT PRIMARY KEY,
    name TEXT NOT NULL,                -- 検査名（外観/通電/寸法等）
    type TEXT NOT NULL DEFAULT '全数', -- 検査タイプ（全数/抜取）
    std_time_min REAL,                 -- 標準工数（分）
    description TEXT,                  -- 手順説明
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- 作業者マスタ
CREATE TABLE workers (
    worker_id TEXT PRIMARY KEY,
    name TEXT NOT NULL,                -- 氏名
    role TEXT,                         -- 担当カテゴリ
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- ========================================
-- トランザクションデータ
-- ========================================

-- ロット（入荷単位）
CREATE TABLE lots (
    lot_id TEXT PRIMARY KEY,
    part_id TEXT NOT NULL,             -- 部品（FK）
    po_number TEXT,                    -- 発注番号（トレーサビリティ用）
    arrival_date DATE,                 -- 入荷日
    quantity INTEGER NOT NULL,         -- 入荷数量
    fw_version TEXT,                   -- FWバージョン
    hw_version TEXT,                   -- HWバージョン
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (part_id) REFERENCES parts(part_id)
);

-- 検査記録
CREATE TABLE inspection_records (
    record_id TEXT PRIMARY KEY,
    lot_id TEXT NOT NULL,              -- ロット（FK）
    item_id TEXT NOT NULL,             -- 検査項目（FK）
    worker_id TEXT,                    -- 作業者（FK）
    inspection_date DATE NOT NULL,     -- 検査日
    sample_qty INTEGER,                -- サンプル数（抜取検査用）
    result TEXT NOT NULL,              -- 結果（合格/不合格/不問）
    defect_qty INTEGER DEFAULT 0,      -- 不良数量
    work_time_min REAL,                -- 工数（分）
    note TEXT,                         -- 備考
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (lot_id) REFERENCES lots(lot_id),
    FOREIGN KEY (item_id) REFERENCES inspection_items(item_id),
    FOREIGN KEY (worker_id) REFERENCES workers(worker_id)
);

-- 不良レポート（8D対応）
CREATE TABLE defect_reports (
    report_id TEXT PRIMARY KEY,
    record_id TEXT NOT NULL,           -- 検査記録（FK）
    defect_type TEXT,                  -- 不良種別
    detail TEXT,                       -- 詳細
    root_cause TEXT,                   -- 原因分析（8D: D4-D5）
    corrective_action TEXT,            -- 対策（8D: D6-D7）
    status TEXT DEFAULT '要協議',      -- ステータス（要協議/対処中/完了）
    closed_date DATE,                  -- 完了日
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (record_id) REFERENCES inspection_records(record_id)
);

-- 不問判定
CREATE TABLE waivers (
    waiver_id TEXT PRIMARY KEY,
    record_id TEXT NOT NULL,           -- 検査記録（FK）
    reason TEXT NOT NULL,              -- 不問理由
    approver TEXT,                     -- 承認者
    approved_at TIMESTAMP,             -- 承認日時
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (record_id) REFERENCES inspection_records(record_id)
);

-- ========================================
-- インデックス
-- ========================================

-- よく検索される項目にインデックス
CREATE INDEX idx_lots_part_id ON lots(part_id);
CREATE INDEX idx_lots_arrival_date ON lots(arrival_date);
CREATE INDEX idx_records_lot_id ON inspection_records(lot_id);
CREATE INDEX idx_records_inspection_date ON inspection_records(inspection_date);
CREATE INDEX idx_records_result ON inspection_records(result);
CREATE INDEX idx_defects_status ON defect_reports(status);

-- ========================================
-- 初期データ（プロトタイプ用）
-- ========================================

-- カテゴリの例
-- INSERT INTO inspection_items (item_id, name, type) VALUES
--     ('ITEM-001', '外観検査', '全数'),
--     ('ITEM-002', '通電検査', '全数'),
--     ('ITEM-003', '寸法検査', '抜取');
