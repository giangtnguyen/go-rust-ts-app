package handler

import (
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"strings"
	"testing"

	"go-rust-ts-app/api/internal/engine"
	"go-rust-ts-app/api/internal/service"
)

func TestPackingHandlerInvalidJSON(t *testing.T) {
	engineClient := engine.NewClient("http://localhost:9000")
	packingService := service.NewPackingService(engineClient)
	handler := NewPackingHandler(packingService)

	request := httptest.NewRequest(
		http.MethodPost,
		"/calculate",
		strings.NewReader(`{"invalid"`),
	)

	recorder := httptest.NewRecorder()

	handler.Calculate(recorder, request)

	if recorder.Code != http.StatusBadRequest {
		t.Fatalf(
			"expected status %d, got %d",
			http.StatusBadRequest,
			recorder.Code,
		)
	}
}
func TestPackingHandlerCalculate(t *testing.T) {
	engineClient := engine.NewClient("http://localhost:9000")
	packingService := service.NewPackingService(engineClient)
	handler := NewPackingHandler(packingService)

	request := httptest.NewRequest(
		http.MethodPost,
		"/calculate",
		strings.NewReader(`{
			"container": {
				"width": 100,
				"height": 60
			},
			"item": {
				"width": 25,
				"height": 20
			},
			"allow_rotation": true
		}`),
	)

	recorder := httptest.NewRecorder()

	handler.Calculate(recorder, request)

	if recorder.Code != http.StatusOK {
		t.Fatalf(
			"expected status %d, got %d",
			http.StatusOK,
			recorder.Code,
		)
	}

	var result engine.PackingResult

	if err := json.NewDecoder(recorder.Body).Decode(&result); err != nil {
		t.Fatalf("failed to decode response: %v", err)
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
func TestPackingHandlerEngineError(t *testing.T) {
	engineClient := engine.NewClient("http://localhost:9001")
	packingService := service.NewPackingService(engineClient)
	handler := NewPackingHandler(packingService)

	request := httptest.NewRequest(
		http.MethodPost,
		"/calculate",
		strings.NewReader(`{
			"container": {
				"width": 100,
				"height": 60
			},
			"item": {
				"width": 25,
				"height": 20
			},
			"allow_rotation": true
		}`),
	)

	recorder := httptest.NewRecorder()

	handler.Calculate(recorder, request)

	if recorder.Code != http.StatusBadGateway {
		t.Fatalf(
			"expected status %d, got %d",
			http.StatusBadGateway,
			recorder.Code,
		)
	}
}
