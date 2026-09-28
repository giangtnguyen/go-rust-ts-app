package main

import (
	"fmt"
	"log"
	"net/http"

	"go-rust-ts-app/api/internal/engine"
	"go-rust-ts-app/api/internal/handler"
	"go-rust-ts-app/api/internal/service"
)

func healthHandler(w http.ResponseWriter, r *http.Request) {
	w.Header().Set("Content-Type", "application/json")

	fmt.Fprint(w, `{"status":"ok","service":"go-api"}`)
}

func main() {
	engineClient := engine.NewClient("http://localhost:9000")

	packingService := service.NewPackingService(engineClient)

	packingHandler := handler.NewPackingHandler(packingService)

	http.HandleFunc("/health", healthHandler)
	http.HandleFunc("/calculate", packingHandler.Calculate)

	log.Println("Go API listening on :8009")

	if err := http.ListenAndServe(":8009", nil); err != nil {
		log.Fatal(err)
	}
}
