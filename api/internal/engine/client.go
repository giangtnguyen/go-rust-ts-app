package engine

import (
	"bytes"
	"encoding/json"
	"fmt"
	"io"
	"net/http"
)

type Container struct {
	Width  float64 `json:"width"`
	Height float64 `json:"height"`
}

type Item struct {
	Width  float64 `json:"width"`
	Height float64 `json:"height"`
}

type PackingRequest struct {
	Container     Container `json:"container"`
	Item          Item      `json:"item"`
	AllowRotation bool      `json:"allow_rotation"`
}

type PackingResult struct {
	MaxItems    uint32  `json:"max_items"`
	ItemWidth   float64 `json:"item_width"`
	ItemHeight  float64 `json:"item_height"`
	Utilization float64 `json:"utilization"`
}

type Client struct {
	BaseURL string
}

func NewClient(baseURL string) *Client {
	return &Client{
		BaseURL: baseURL,
	}
}

func (c *Client) Calculate(request PackingRequest) (PackingResult, error) {
	payload, err := json.Marshal(request)
	if err != nil {
		return PackingResult{}, fmt.Errorf("encode request: %w", err)
	}

	resp, err := http.Post(
		c.BaseURL+"/packing",
		"application/json",
		bytes.NewBuffer(payload),
	)
	if err != nil {
		return PackingResult{}, fmt.Errorf("engine unavailable: %w", err)
	}
	defer resp.Body.Close()

	body, err := io.ReadAll(resp.Body)
	if err != nil {
		return PackingResult{}, fmt.Errorf("read engine response: %w", err)
	}

	if resp.StatusCode != http.StatusOK {
		return PackingResult{}, fmt.Errorf(
			"engine returned HTTP %d: %s",
			resp.StatusCode,
			body,
		)
	}

	var result PackingResult

	if err := json.Unmarshal(body, &result); err != nil {
		return PackingResult{}, fmt.Errorf(
			"decode engine response: %w",
			err,
		)
	}

	return result, nil
}
