package service

import (
	"errors"
	"testing"

	"go-rust-ts-app/api/internal/engine"
)

type fakeEngine struct {
	result engine.PackingResult
	err    error
}

func (f *fakeEngine) Calculate(
	request engine.PackingRequest,
) (engine.PackingResult, error) {
	return f.result, f.err
}

func TestPackingServiceCalculate(t *testing.T) {
	fake := &fakeEngine{
		result: engine.PackingResult{
			MaxItems:    12,
			ItemWidth:   25,
			ItemHeight:  20,
			Utilization: 1,
		},
	}

	service := NewPackingService(fake)

	result, err := service.Calculate(engine.PackingRequest{})

	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if result.MaxItems != 12 {
		t.Fatalf("expected max_items 12, got %d", result.MaxItems)
	}

	if result.ItemWidth != 25 {
		t.Fatalf("expected item_width 25, got %v", result.ItemWidth)
	}

	if result.ItemHeight != 20 {
		t.Fatalf("expected item_height 20, got %v", result.ItemHeight)
	}

	if result.Utilization != 1 {
		t.Fatalf("expected utilization 1, got %v", result.Utilization)
	}
}

func TestPackingServiceCalculateError(t *testing.T) {
	expectedErr := errors.New("engine failure")

	fake := &fakeEngine{
		err: expectedErr,
	}

	service := NewPackingService(fake)

	_, err := service.Calculate(engine.PackingRequest{})

	if err != expectedErr {
		t.Fatalf("expected error %v, got %v", expectedErr, err)
	}
}
