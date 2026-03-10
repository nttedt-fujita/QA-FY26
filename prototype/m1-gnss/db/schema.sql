-- GNSS評価ツール スキーマ定義
-- 作成日: 2026-03-10
-- Session: 63
-- 参照: docs/domain-model.md

PRAGMA foreign_keys = ON;

-- ============================================================
-- デバイス（評価対象のGNSSモジュール）
-- ============================================================
CREATE TABLE IF NOT EXISTS devices (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    model_number TEXT NOT NULL,           -- 型番
    serial_number TEXT NOT NULL UNIQUE,   -- シリアル番号
    memo TEXT,                            -- メモ
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- ============================================================
-- 計測セッション（1回の評価作業）
-- ============================================================
CREATE TABLE IF NOT EXISTS measurement_sessions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    device_id INTEGER NOT NULL,

    -- 基本情報
    started_at TEXT NOT NULL,             -- 開始時刻 (ISO8601)
    ended_at TEXT,                        -- 終了時刻 (ISO8601)
    evaluator TEXT,                       -- 評価者

    -- 評価条件: 場所
    location_lat REAL,                    -- 緯度
    location_lon REAL,                    -- 経度
    location_description TEXT,            -- 場所の説明

    -- 評価条件: 環境
    environment TEXT,                     -- 屋外/屋内/その他
    weather TEXT,                         -- 天候

    -- 評価条件: アンテナ
    antenna_config TEXT,                  -- アンテナ構成

    -- 評価条件: 基地局（RTK評価時）
    base_station_info TEXT,               -- 基地局情報

    -- 評価結果
    judgment TEXT,                        -- 合格/不合格/保留
    comment TEXT,                         -- 所見

    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),

    FOREIGN KEY (device_id) REFERENCES devices(id)
);

-- ============================================================
-- NAV-PVT（位置・状態）
-- ============================================================
CREATE TABLE IF NOT EXISTS nav_pvt_records (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    session_id INTEGER NOT NULL,

    -- タイムスタンプ
    itow INTEGER NOT NULL,                -- GPS Time of Week (ms)
    recorded_at TEXT NOT NULL,            -- ホスト側記録時刻 (ISO8601)

    -- 時刻
    year INTEGER,
    month INTEGER,
    day INTEGER,
    hour INTEGER,
    min INTEGER,
    sec INTEGER,
    nano INTEGER,                         -- ナノ秒

    -- 位置
    lat INTEGER,                          -- 緯度 (1e-7 degrees)
    lon INTEGER,                          -- 経度 (1e-7 degrees)
    height INTEGER,                       -- 楕円体高 (mm)
    h_msl INTEGER,                        -- 海抜高 (mm)

    -- 精度
    h_acc INTEGER,                        -- 水平精度 (mm)
    v_acc INTEGER,                        -- 垂直精度 (mm)

    -- 速度
    vel_n INTEGER,                        -- 北方向速度 (mm/s)
    vel_e INTEGER,                        -- 東方向速度 (mm/s)
    vel_d INTEGER,                        -- 下方向速度 (mm/s)
    g_speed INTEGER,                      -- 地表速度 (mm/s)

    -- 状態
    fix_type INTEGER,                     -- Fix type
    carr_soln INTEGER,                    -- RTK状態: 0=No, 1=Float, 2=Fixed
    num_sv INTEGER,                       -- 使用衛星数

    -- フラグ
    valid_date INTEGER,                   -- 日付有効フラグ
    valid_time INTEGER,                   -- 時刻有効フラグ
    fully_resolved INTEGER,               -- 完全解決フラグ

    FOREIGN KEY (session_id) REFERENCES measurement_sessions(id)
);

CREATE INDEX IF NOT EXISTS idx_nav_pvt_session_itow
    ON nav_pvt_records(session_id, itow);

-- ============================================================
-- NAV-STATUS（FIX状態）
-- ============================================================
CREATE TABLE IF NOT EXISTS nav_status_records (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    session_id INTEGER NOT NULL,

    itow INTEGER NOT NULL,
    recorded_at TEXT NOT NULL,

    fix_type INTEGER,                     -- Fix type
    fix_ok INTEGER,                       -- Fix OK フラグ
    ttff INTEGER,                         -- Time to First Fix (ms)
    msss INTEGER,                         -- Time since startup (ms)

    FOREIGN KEY (session_id) REFERENCES measurement_sessions(id)
);

CREATE INDEX IF NOT EXISTS idx_nav_status_session_itow
    ON nav_status_records(session_id, itow);

-- ============================================================
-- NAV-DOP（精度劣化係数）
-- ============================================================
CREATE TABLE IF NOT EXISTS nav_dop_records (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    session_id INTEGER NOT NULL,

    itow INTEGER NOT NULL,
    recorded_at TEXT NOT NULL,

    g_dop INTEGER,                        -- Geometric DOP (0.01)
    p_dop INTEGER,                        -- Position DOP (0.01)
    t_dop INTEGER,                        -- Time DOP (0.01)
    v_dop INTEGER,                        -- Vertical DOP (0.01)
    h_dop INTEGER,                        -- Horizontal DOP (0.01)
    n_dop INTEGER,                        -- Northing DOP (0.01)
    e_dop INTEGER,                        -- Easting DOP (0.01)

    FOREIGN KEY (session_id) REFERENCES measurement_sessions(id)
);

CREATE INDEX IF NOT EXISTS idx_nav_dop_session_itow
    ON nav_dop_records(session_id, itow);

-- ============================================================
-- NAV-SAT（衛星情報）
-- ============================================================
CREATE TABLE IF NOT EXISTS satellites (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    session_id INTEGER NOT NULL,

    itow INTEGER NOT NULL,
    recorded_at TEXT NOT NULL,

    gnss_id INTEGER NOT NULL,             -- GNSS ID (0=GPS, 2=Galileo, 3=BeiDou, 6=GLONASS)
    sv_id INTEGER NOT NULL,               -- 衛星番号
    cno INTEGER,                          -- C/N0 (dBHz)
    elev INTEGER,                         -- 仰角 (degrees)
    azim INTEGER,                         -- 方位角 (degrees)
    pr_res INTEGER,                       -- 擬似距離残差 (0.1m)

    -- フラグ
    quality_ind INTEGER,                  -- 品質インジケータ
    sv_used INTEGER,                      -- 使用フラグ
    health INTEGER,                       -- ヘルス状態

    FOREIGN KEY (session_id) REFERENCES measurement_sessions(id)
);

CREATE INDEX IF NOT EXISTS idx_satellites_session_itow
    ON satellites(session_id, itow);

-- ============================================================
-- NAV-SIG（信号情報）
-- ============================================================
CREATE TABLE IF NOT EXISTS signals (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    session_id INTEGER NOT NULL,

    itow INTEGER NOT NULL,
    recorded_at TEXT NOT NULL,

    gnss_id INTEGER NOT NULL,             -- GNSS ID
    sv_id INTEGER NOT NULL,               -- 衛星番号
    sig_id INTEGER NOT NULL,              -- 信号ID
    freq_id INTEGER,                      -- 周波数ID (GLONASS)
    cno INTEGER,                          -- C/N0 (dBHz)

    -- 品質
    quality_ind INTEGER,                  -- 品質インジケータ
    corr_source INTEGER,                  -- 補正ソース
    iono_model INTEGER,                   -- 電離層モデル

    -- フラグ
    health INTEGER,
    pr_smoothed INTEGER,
    pr_used INTEGER,
    cr_used INTEGER,
    do_used INTEGER,

    FOREIGN KEY (session_id) REFERENCES measurement_sessions(id)
);

CREATE INDEX IF NOT EXISTS idx_signals_session_itow
    ON signals(session_id, itow);

-- ============================================================
-- MON-SPAN（スペクトラム）
-- ============================================================
CREATE TABLE IF NOT EXISTS mon_span_records (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    session_id INTEGER NOT NULL,

    recorded_at TEXT NOT NULL,

    rf_block_id INTEGER,                  -- RFブロックID
    center INTEGER,                       -- 中心周波数 (Hz)
    span INTEGER,                         -- スパン (Hz)
    res INTEGER,                          -- 分解能 (Hz)
    pga INTEGER,                          -- PGAゲイン

    -- スペクトラムデータ (256バイト)
    spectrum BLOB,

    FOREIGN KEY (session_id) REFERENCES measurement_sessions(id)
);

CREATE INDEX IF NOT EXISTS idx_mon_span_session
    ON mon_span_records(session_id, recorded_at);

-- ============================================================
-- MON-RF（RF状態）
-- ============================================================
CREATE TABLE IF NOT EXISTS mon_rf_records (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    session_id INTEGER NOT NULL,

    recorded_at TEXT NOT NULL,

    block_id INTEGER,                     -- ブロックID

    -- ジャミング状態
    jamming_state INTEGER,                -- 0=Unknown, 1=OK, 2=Warning, 3=Critical

    -- アンテナ状態
    ant_status INTEGER,                   -- 0=Init, 1=Unknown, 2=OK, 3=Short, 4=Open
    ant_power INTEGER,                    -- 0=Off, 1=On, 2=Unknown

    -- ノイズ
    noise_per_ms INTEGER,                 -- ノイズレベル
    agc_cnt INTEGER,                      -- AGCカウント
    jam_ind INTEGER,                      -- ジャミングインジケータ
    ofs_i INTEGER,                        -- オフセットI
    mag_i INTEGER,                        -- マグニチュードI
    ofs_q INTEGER,                        -- オフセットQ
    mag_q INTEGER,                        -- マグニチュードQ

    FOREIGN KEY (session_id) REFERENCES measurement_sessions(id)
);

CREATE INDEX IF NOT EXISTS idx_mon_rf_session
    ON mon_rf_records(session_id, recorded_at);
