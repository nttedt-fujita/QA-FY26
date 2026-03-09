// Package session は検査セッションの状態管理を行う
// プロトタイプ段階ではインメモリで管理
package session

import (
	"errors"
	"time"

	"github.com/google/uuid"
)

// InspectionSession は検査セッションの状態を管理する
type InspectionSession struct {
	ID        string
	LotID     string
	ItemID    string
	WorkerID  string
	StartedAt time.Time
	history   []string // カウント履歴（pass/fail/waiver）
}

// Counts はカウント集計結果
type Counts struct {
	Pass   int
	Fail   int
	Waiver int
}

// FinishResult は検査終了時の結果
type FinishResult struct {
	LotID       string
	ItemID      string
	WorkerID    string
	PassCount   int
	FailCount   int
	WaiverCount int
	WorkTimeMin float64
	Note        string
}

// NewInspectionSession は新しい検査セッションを作成する
func NewInspectionSession(lotID, itemID, workerID string) *InspectionSession {
	return &InspectionSession{
		ID:        uuid.New().String(),
		LotID:     lotID,
		ItemID:    itemID,
		WorkerID:  workerID,
		StartedAt: time.Now(),
		history:   make([]string, 0),
	}
}

// AddCount はカウントを追加する
func (s *InspectionSession) AddCount(result string) error {
	// resultの検証
	if result != "pass" && result != "fail" && result != "waiver" {
		return errors.New("無効な結果種別: " + result)
	}
	s.history = append(s.history, result)
	return nil
}

// Undo は直前のカウントを取り消す
func (s *InspectionSession) Undo() error {
	if len(s.history) == 0 {
		return errors.New("取り消すカウントがありません")
	}
	s.history = s.history[:len(s.history)-1]
	return nil
}

// GetCounts は現在のカウント集計を返す
func (s *InspectionSession) GetCounts() Counts {
	counts := Counts{}
	for _, r := range s.history {
		switch r {
		case "pass":
			counts.Pass++
		case "fail":
			counts.Fail++
		case "waiver":
			counts.Waiver++
		}
	}
	return counts
}

// Finish は検査を終了し、結果を返す
func (s *InspectionSession) Finish(note string) (*FinishResult, error) {
	counts := s.GetCounts()
	duration := time.Since(s.StartedAt)
	workTimeMin := duration.Minutes()

	return &FinishResult{
		LotID:       s.LotID,
		ItemID:      s.ItemID,
		WorkerID:    s.WorkerID,
		PassCount:   counts.Pass,
		FailCount:   counts.Fail,
		WaiverCount: counts.Waiver,
		WorkTimeMin: workTimeMin,
		Note:        note,
	}, nil
}
