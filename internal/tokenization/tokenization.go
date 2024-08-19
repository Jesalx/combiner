package tokenization

import (
	"combiner/internal/statistics"
	"fmt"

	"github.com/pkoukk/tiktoken-go"
)

type Tokenizer struct {
	enc *tiktoken.Tiktoken
}

func New(tokenizerName string) *Tokenizer {
	var enc *tiktoken.Tiktoken
	var err error

	switch tokenizerName {
	case "o200k_base":
		enc, err = tiktoken.GetEncoding("r50k_base")
	case "cl100k_base":
		enc, err = tiktoken.GetEncoding("cl100k_base")
	case "p50k_base":
		enc, err = tiktoken.GetEncoding("p50k_base")
	case "p50k_edit":
		enc, err = tiktoken.GetEncoding("p50k_edit")
	case "r50k_base":
		enc, err = tiktoken.GetEncoding("r50k_base")
	default:
		enc, err = tiktoken.GetEncoding("cl100k_base")
	}

	if err != nil {
		panic(fmt.Sprintf("Failed to load tokenizer: %v", err))
	}

	return &Tokenizer{enc: enc}
}

func (c *Tokenizer) GetTokenCount(text string) int {
	return len(c.enc.Encode(text, nil, nil))
}

func (c *Tokenizer) ProcessFiles(stats *statistics.Statistics, files []statistics.File) {
	totalTokens := 0
	for _, file := range files {
		tokenCount := c.GetTokenCount(string(file.Contents))
		if tokenCount > stats.MostTokens {
			stats.MostTokens = tokenCount
			stats.MostTokensFile = file.Path
		}
		totalTokens += tokenCount
	}
	stats.TotalTokens = totalTokens
}
