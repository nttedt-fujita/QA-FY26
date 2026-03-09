package session_test

import (
	"testing"
	"time"

	"github.com/nttedt/qa-inspection-api/internal/session"
)

// テストシナリオ（Session 43で承認済み）
// | # | シナリオ | 操作 | 期待結果 |
// |---|---------|------|---------|
// | 1 | セッション作成 | 新規作成 | セッションIDが発行される |
// | 2 | 合格カウント追加 | pass追加 | 合格=1, 不合格=0, 不問=0 |
// | 3 | 不合格カウント追加 | fail追加 | 合格=0, 不合格=1, 不問=0 |
// | 4 | 不問カウント追加 | waiver追加 | 合格=0, 不合格=0, 不問=1 |
// | 5 | 複数カウント追加 | pass×2, fail×1 | 合格=2, 不合格=1, 不問=0 |
// | 6 | 1件取り消し | pass→pass→取消 | 合格=1 |
// | 7 | 複数取り消し | pass→fail→取消→取消 | 全て0 |
// | 8 | 空で取り消し | 取消 | エラー |

// TestInspectionSession_New はセッション作成の振る舞いをテストする
func TestInspectionSession_New(t *testing.T) {
	// Arrange
	lotID := "LOT-001"
	itemID := "ITEM-001"
	workerID := "WORKER-001"

	// Act
	s := session.NewInspectionSession(lotID, itemID, workerID)

	// Assert
	if s.ID == "" {
		t.Error("セッションIDが空")
	}
	if s.LotID != lotID {
		t.Errorf("LotID = %s, want %s", s.LotID, lotID)
	}
	if s.ItemID != itemID {
		t.Errorf("ItemID = %s, want %s", s.ItemID, itemID)
	}
	if s.WorkerID != workerID {
		t.Errorf("WorkerID = %s, want %s", s.WorkerID, workerID)
	}
	if s.StartedAt.IsZero() {
		t.Error("StartedAtが設定されていない")
	}
}

// TestInspectionSession_AddCount はカウント追加の振る舞いをテストする
func TestInspectionSession_AddCount(t *testing.T) {
	tests := []struct {
		name          string
		actions       []string // 追加するresult
		wantPass      int
		wantFail      int
		wantWaiver    int
		shouldSucceed bool
	}{
		// 正常系
		{
			name:          "合格カウント追加",
			actions:       []string{"pass"},
			wantPass:      1,
			wantFail:      0,
			wantWaiver:    0,
			shouldSucceed: true,
		},
		{
			name:          "不合格カウント追加",
			actions:       []string{"fail"},
			wantPass:      0,
			wantFail:      1,
			wantWaiver:    0,
			shouldSucceed: true,
		},
		{
			name:          "不問カウント追加",
			actions:       []string{"waiver"},
			wantPass:      0,
			wantFail:      0,
			wantWaiver:    1,
			shouldSucceed: true,
		},
		{
			name:          "複数カウント追加",
			actions:       []string{"pass", "pass", "fail"},
			wantPass:      2,
			wantFail:      1,
			wantWaiver:    0,
			shouldSucceed: true,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			// Arrange
			s := session.NewInspectionSession("LOT-001", "ITEM-001", "WORKER-001")

			// Act
			var lastErr error
			for _, action := range tt.actions {
				lastErr = s.AddCount(action)
			}

			// Assert
			if tt.shouldSucceed {
				if lastErr != nil {
					t.Errorf("予期しないエラー: %v", lastErr)
				}
				counts := s.GetCounts()
				if counts.Pass != tt.wantPass {
					t.Errorf("Pass = %d, want %d", counts.Pass, tt.wantPass)
				}
				if counts.Fail != tt.wantFail {
					t.Errorf("Fail = %d, want %d", counts.Fail, tt.wantFail)
				}
				if counts.Waiver != tt.wantWaiver {
					t.Errorf("Waiver = %d, want %d", counts.Waiver, tt.wantWaiver)
				}
			} else {
				if lastErr == nil {
					t.Error("エラーが発生するはずだった")
				}
			}
		})
	}
}

// TestInspectionSession_Undo は取り消しの振る舞いをテストする
func TestInspectionSession_Undo(t *testing.T) {
	tests := []struct {
		name          string
		setupActions  []string // 事前にカウント追加
		undoCount     int      // 取り消し回数
		wantPass      int
		wantFail      int
		wantWaiver    int
		shouldSucceed bool
	}{
		// 正常系
		{
			name:          "1件取り消し",
			setupActions:  []string{"pass", "pass"},
			undoCount:     1,
			wantPass:      1,
			wantFail:      0,
			wantWaiver:    0,
			shouldSucceed: true,
		},
		{
			name:          "複数取り消し",
			setupActions:  []string{"pass", "fail"},
			undoCount:     2,
			wantPass:      0,
			wantFail:      0,
			wantWaiver:    0,
			shouldSucceed: true,
		},
		// 異常系
		{
			name:          "空で取り消し",
			setupActions:  []string{},
			undoCount:     1,
			wantPass:      0,
			wantFail:      0,
			wantWaiver:    0,
			shouldSucceed: false,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			// Arrange
			s := session.NewInspectionSession("LOT-001", "ITEM-001", "WORKER-001")
			for _, action := range tt.setupActions {
				s.AddCount(action)
			}

			// Act
			var lastErr error
			for i := 0; i < tt.undoCount; i++ {
				lastErr = s.Undo()
			}

			// Assert
			if tt.shouldSucceed {
				if lastErr != nil {
					t.Errorf("予期しないエラー: %v", lastErr)
				}
				counts := s.GetCounts()
				if counts.Pass != tt.wantPass {
					t.Errorf("Pass = %d, want %d", counts.Pass, tt.wantPass)
				}
				if counts.Fail != tt.wantFail {
					t.Errorf("Fail = %d, want %d", counts.Fail, tt.wantFail)
				}
				if counts.Waiver != tt.wantWaiver {
					t.Errorf("Waiver = %d, want %d", counts.Waiver, tt.wantWaiver)
				}
			} else {
				if lastErr == nil {
					t.Error("エラーが発生するはずだった")
				}
			}
		})
	}
}

// TestInspectionSession_Finish は検査終了の振る舞いをテストする
func TestInspectionSession_Finish(t *testing.T) {
	// Arrange
	s := session.NewInspectionSession("LOT-001", "ITEM-001", "WORKER-001")
	s.AddCount("pass")
	s.AddCount("pass")
	s.AddCount("fail")

	// 少し待って時間経過をシミュレート（テスト用に最小限）
	time.Sleep(10 * time.Millisecond)

	// Act
	result, err := s.Finish("テストメモ")

	// Assert
	if err != nil {
		t.Errorf("予期しないエラー: %v", err)
	}
	if result.LotID != "LOT-001" {
		t.Errorf("LotID = %s, want LOT-001", result.LotID)
	}
	if result.PassCount != 2 {
		t.Errorf("PassCount = %d, want 2", result.PassCount)
	}
	if result.FailCount != 1 {
		t.Errorf("FailCount = %d, want 1", result.FailCount)
	}
	// 丸め処理により瞬時のテストでは0になる（小数点1桁に丸めているため）
	if result.WorkTimeMin < 0 {
		t.Error("WorkTimeMinが負の値")
	}
	if result.Note != "テストメモ" {
		t.Errorf("Note = %s, want テストメモ", result.Note)
	}
}
