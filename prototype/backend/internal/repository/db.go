package repository

import (
	"context"
	"fmt"

	"github.com/jackc/pgx/v5/pgxpool"
)

// DB はPostgreSQL接続プールをラップする
type DB struct {
	pool *pgxpool.Pool
}

// NewDB は新しいDB接続を作成する
func NewDB(ctx context.Context, dsn string) (*DB, error) {
	pool, err := pgxpool.New(ctx, dsn)
	if err != nil {
		return nil, fmt.Errorf("DB接続に失敗: %w", err)
	}

	// 接続テスト
	if err := pool.Ping(ctx); err != nil {
		pool.Close()
		return nil, fmt.Errorf("DB接続テストに失敗: %w", err)
	}

	return &DB{pool: pool}, nil
}

// Close はDB接続を閉じる
func (db *DB) Close() {
	if db.pool != nil {
		db.pool.Close()
	}
}

// Exec はSQLを実行する
func (db *DB) Exec(ctx context.Context, sql string, args ...interface{}) (int64, error) {
	result, err := db.pool.Exec(ctx, sql, args...)
	if err != nil {
		return 0, err
	}
	return result.RowsAffected(), nil
}

// Pool は内部のpgxpoolを返す（リポジトリ層で使用）
func (db *DB) Pool() *pgxpool.Pool {
	return db.pool
}
