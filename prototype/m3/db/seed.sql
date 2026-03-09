-- M3 受入検査DB シードデータ（プロトタイプ用）
-- Session 42で作成
-- 目的: カウンター画面のプロトタイプに必要な最小限のテストデータ

-- ========================================
-- サプライヤー（3社）
-- ========================================
INSERT INTO suppliers (supplier_id, name, contact, default_aql) VALUES
    ('SUP-001', '小峰無線株式会社', 'komine@example.com', '1.0'),
    ('SUP-002', '株式会社ABC電子', 'abc@example.com', '2.5'),
    ('SUP-003', 'XYZ機械工業', 'xyz@example.com', '1.0')
ON CONFLICT (supplier_id) DO NOTHING;

-- ========================================
-- 部品（10種類）
-- ========================================
-- カテゴリ: メカ / エレキ / Api
INSERT INTO parts (part_id, name, category, supplier_id, spec_doc_url, latest_fw_version) VALUES
    -- メカ部品
    ('PART-001', 'フレーム本体', 'メカ', 'SUP-003', NULL, NULL),
    ('PART-002', 'アーム（左）', 'メカ', 'SUP-003', NULL, NULL),
    ('PART-003', 'アーム（右）', 'メカ', 'SUP-003', NULL, NULL),
    ('PART-004', 'プロペラ', 'メカ', 'SUP-003', NULL, NULL),
    -- エレキ部品
    ('PART-005', 'FCU（フライトコントローラー）', 'エレキ', 'SUP-002', '/docs/fcu-spec.pdf', 'v2.1.0'),
    ('PART-006', 'ESC', 'エレキ', 'SUP-002', '/docs/esc-spec.pdf', 'v1.5.0'),
    ('PART-007', 'バッテリー', 'エレキ', 'SUP-002', NULL, NULL),
    ('PART-008', 'モーター', 'エレキ', 'SUP-002', NULL, NULL),
    -- Api部品（GPSモジュール等）
    ('PART-009', 'GPSモジュール（UBX）', 'Api', 'SUP-001', '/docs/ubx-spec.pdf', 'v3.0.1'),
    ('PART-010', 'LiDARセンサー', 'Api', 'SUP-001', '/docs/lidar-spec.pdf', 'v1.2.0')
ON CONFLICT (part_id) DO NOTHING;

-- ========================================
-- 検査項目（5種類）
-- ========================================
INSERT INTO inspection_items (item_id, name, type, std_time_min, description) VALUES
    ('ITEM-001', '外観検査', '全数', 2.0, '傷、汚れ、変形等の目視確認'),
    ('ITEM-002', '通電検査', '全数', 5.0, '電源投入、LED点灯、初期動作確認'),
    ('ITEM-003', '寸法検査', '抜取', 10.0, 'ノギス等を用いた寸法測定'),
    ('ITEM-004', 'FWバージョン確認', '全数', 3.0, 'ファームウェアバージョンの確認'),
    ('ITEM-005', '機能テスト', '抜取', 15.0, '実動作による機能確認')
ON CONFLICT (item_id) DO NOTHING;

-- ========================================
-- 作業者（3名）
-- ========================================
INSERT INTO workers (worker_id, name, role) VALUES
    ('WKR-001', '田中太郎', 'エレキ'),
    ('WKR-002', '鈴木花子', 'メカ'),
    ('WKR-003', '佐藤一郎', 'Api')
ON CONFLICT (worker_id) DO NOTHING;

-- ========================================
-- 動作確認用サンプルロット（2件）
-- ========================================
INSERT INTO lots (lot_id, part_id, po_number, arrival_date, quantity, fw_version, hw_version) VALUES
    ('LOT-SAMPLE1', 'PART-009', 'PO-2026-001', '2026-03-09', 100, 'v3.0.1', 'rev.A'),
    ('LOT-SAMPLE2', 'PART-005', 'PO-2026-002', '2026-03-09', 50, 'v2.1.0', 'rev.B')
ON CONFLICT (lot_id) DO NOTHING;
