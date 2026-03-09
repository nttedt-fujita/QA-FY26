-- M3 受入検査DB スキーマ（PostgreSQL版）
-- To-Beモデル（Session 33）に基づく設計
-- 作成日: 2026-03-06

-- ========================================
-- マスタデータ
-- ========================================

-- サプライヤマスタ
CREATE TABLE IF NOT EXISTS suppliers (
    supplier_id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    contact TEXT,
    default_aql TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- 部品マスタ
CREATE TABLE IF NOT EXISTS parts (
    part_id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    category TEXT NOT NULL,
    supplier_id TEXT REFERENCES suppliers(supplier_id),
    spec_doc_url TEXT,
    latest_fw_version TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- 検査項目マスタ
CREATE TABLE IF NOT EXISTS inspection_items (
    item_id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    type TEXT NOT NULL DEFAULT '全数',
    std_time_min REAL,
    description TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- 作業者マスタ
CREATE TABLE IF NOT EXISTS workers (
    worker_id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    role TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- ========================================
-- トランザクションデータ
-- ========================================

-- ロット（入荷単位）
CREATE TABLE IF NOT EXISTS lots (
    lot_id TEXT PRIMARY KEY,
    part_id TEXT NOT NULL REFERENCES parts(part_id),
    po_number TEXT,
    arrival_date DATE,
    quantity INTEGER NOT NULL,
    fw_version TEXT,
    hw_version TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- 検査記録
CREATE TABLE IF NOT EXISTS inspection_records (
    record_id TEXT PRIMARY KEY,
    lot_id TEXT NOT NULL REFERENCES lots(lot_id),
    item_id TEXT NOT NULL REFERENCES inspection_items(item_id),
    worker_id TEXT REFERENCES workers(worker_id),
    inspection_date DATE NOT NULL,
    sample_qty INTEGER,
    result TEXT NOT NULL,
    defect_qty INTEGER DEFAULT 0,
    work_time_min REAL,
    note TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- 不良レポート（8D対応）
CREATE TABLE IF NOT EXISTS defect_reports (
    report_id TEXT PRIMARY KEY,
    record_id TEXT NOT NULL REFERENCES inspection_records(record_id),
    defect_type TEXT,
    detail TEXT,
    root_cause TEXT,
    corrective_action TEXT,
    status TEXT DEFAULT '要協議',
    closed_date DATE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- 不問判定
CREATE TABLE IF NOT EXISTS waivers (
    waiver_id TEXT PRIMARY KEY,
    record_id TEXT NOT NULL REFERENCES inspection_records(record_id),
    reason TEXT NOT NULL,
    approver TEXT,
    approved_at TIMESTAMP,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- ========================================
-- インデックス
-- ========================================

CREATE INDEX IF NOT EXISTS idx_lots_part_id ON lots(part_id);
CREATE INDEX IF NOT EXISTS idx_lots_arrival_date ON lots(arrival_date);
CREATE INDEX IF NOT EXISTS idx_records_lot_id ON inspection_records(lot_id);
CREATE INDEX IF NOT EXISTS idx_records_inspection_date ON inspection_records(inspection_date);
CREATE INDEX IF NOT EXISTS idx_records_result ON inspection_records(result);
CREATE INDEX IF NOT EXISTS idx_defects_status ON defect_reports(status);

-- ========================================
-- 初期データ（プロトタイプ用）
-- ========================================

-- サプライヤの例
INSERT INTO suppliers (supplier_id, name, contact, default_aql) VALUES
    ('SUP-001', 'サンプルサプライヤA', 'supplier-a@example.com', '2.5')
ON CONFLICT (supplier_id) DO NOTHING;

-- 部品の例
INSERT INTO parts (part_id, name, category, supplier_id) VALUES
    ('PART-001', 'サンプル部品', 'エレキ', 'SUP-001')
ON CONFLICT (part_id) DO NOTHING;

-- 検査項目の例
INSERT INTO inspection_items (item_id, name, type) VALUES
    ('ITEM-001', '外観検査', '全数'),
    ('ITEM-002', '通電検査', '全数'),
    ('ITEM-003', '寸法検査', '抜取')
ON CONFLICT (item_id) DO NOTHING;
