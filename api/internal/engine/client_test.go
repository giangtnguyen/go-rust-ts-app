package engine

import (
	"net/http"
	"net/http/httptest"
	"testing"
)

func TestClientCalculate(t *testing.T) {
	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.Method != http.MethodPost {
			t.Fatalf("expected POST, got %s", r.Method)
		}

		if r.URL.Path != "/packing" {
			t.Fatalf("expected /packing, got %s", r.URL.Path)
		}

		w.Header().Set("Content-Type", "application/json")

		w.WriteHeader(http.StatusOK)

		w.Write([]byte(`{
			"max_items": 12,
			"item_width": 25,
			"item_height": 20,
			"utilization": 1
		}`))
	}))
	defer server.Close()

	client := NewClient(server.URL)

	result, err := client.Calculate(PackingRequest{
		Container: Container{
			Width:  100,
			Height: 60,
		},
		Item: Item{
			Width:  25,
			Height: 20,
		},
		AllowRotation: true,
	})

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

func TestClientCalculateHTTPError(t *testing.T) {
	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		http.Error(w, "engine failure", http.StatusInternalServerError)
	}))
	defer server.Close()

	client := NewClient(server.URL)

	_, err := client.Calculate(PackingRequest{
		Container: Container{
			Width:  100,
			Height: 60,
		},
		Item: Item{
			Width:  25,
			Height: 20,
		},
		AllowRotation: true,
	})

	if err == nil {
		t.Fatal("expected error, got nil")
	}
}
func TestClientCalculateEngineUnavailable(t *testing.T) {
	client := NewClient("http://127.0.0.1:1")

	_, err := client.Calculate(PackingRequest{
		Container: Container{
			Width:  100,
			Height: 60,
		},
		Item: Item{
			Width:  25,
			Height: 20,
		},
		AllowRotation: true,
	})

	if err == nil {
		t.Fatal("expected error, got nil")
	}
}
