package handler

import (
	"encoding/json"
	"log"
	"net/http"

	"go-rust-ts-app/api/internal/engine"
	"go-rust-ts-app/api/internal/service"
)

type PackingHandler struct {
	service *service.PackingService
}

func NewPackingHandler(service *service.PackingService) *PackingHandler {
	return &PackingHandler{
		service: service,
	}
}

func (h *PackingHandler) Calculate(
	w http.ResponseWriter,
	r *http.Request,
) {
	var request engine.PackingRequest

	if err := json.NewDecoder(r.Body).Decode(&request); err != nil {
		http.Error(w, "invalid JSON", http.StatusBadRequest)
		return
	}

	result, err := h.service.Calculate(request)
	if err != nil {
		log.Printf("packing calculation error: %v", err)
		http.Error(w, "calculation failed", http.StatusBadGateway)
		return
	}

	w.Header().Set("Content-Type", "application/json")

	if err := json.NewEncoder(w).Encode(result); err != nil {
		log.Printf("encode response error: %v", err)
	}
}
