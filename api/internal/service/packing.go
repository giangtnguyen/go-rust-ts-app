package service

import "go-rust-ts-app/api/internal/engine"

type EngineCalculator interface {
	Calculate(engine.PackingRequest) (engine.PackingResult, error)
}

type PackingService struct {
	engineClient EngineCalculator
}

func NewPackingService(engineClient EngineCalculator) *PackingService {
	return &PackingService{
		engineClient: engineClient,
	}
}

func (s *PackingService) Calculate(
	request engine.PackingRequest,
) (engine.PackingResult, error) {
	return s.engineClient.Calculate(request)
}
