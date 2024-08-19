package statistics

import (
	"bufio"
	"fmt"
	"os"
	"sync"
	"time"

	"github.com/olekukonko/tablewriter"
)

type Statistics struct {
	CreationTime       time.Time
	OutputFile         string
	MostTokensFile     string
	DirectoriesVisited int
	FilesProcessed     int
	FilesSkipped       int
	TotalTokens        int
	MostTokens         int
	mu                 sync.Mutex
}

type File struct {
	Path     string
	Contents []byte
}

func New(outputFile string) *Statistics {
	return &Statistics{
		OutputFile:         outputFile,
		DirectoriesVisited: 1, // Start with 1 to account for the starting directory
		CreationTime:       time.Now(),
	}
}

func (s *Statistics) IncrementProcessedFiles() {
	s.mu.Lock()
	defer s.mu.Unlock()
	s.FilesProcessed++
}

func (s *Statistics) UpdateTokenStats(tokens int, filePath string) {
	s.mu.Lock()
	defer s.mu.Unlock()
	s.TotalTokens += tokens
	if tokens > s.MostTokens {
		s.MostTokens = tokens
		s.MostTokensFile = filePath
	}
}

func (s *Statistics) IncrementSkippedFiles() {
	s.mu.Lock()
	defer s.mu.Unlock()
	s.FilesSkipped++
}

func (s *Statistics) IncrementDirectoriesVisited() {
	s.mu.Lock()
	defer s.mu.Unlock()
	s.DirectoriesVisited++
}

func (s *Statistics) TimeSinceCreation() time.Duration {
	return time.Since(s.CreationTime)
}

func formatDurationMS(d time.Duration) string {
	timeInMilliseconds := float64(d.Milliseconds())
	return fmt.Sprintf("%.2f ms", timeInMilliseconds)
}

func (s *Statistics) Print() {
	table := tablewriter.NewWriter(os.Stdout)
	table.SetHeader([]string{"Statistic", "Value"})
	table.SetAlignment(tablewriter.ALIGN_LEFT)

	table.Append([]string{"Output File", s.OutputFile})
	table.Append([]string{"Files Processed", fmt.Sprintf("%d", s.FilesProcessed)})
	table.Append([]string{"Files Skipped", fmt.Sprintf("%d", s.FilesSkipped)})
	table.Append([]string{"Directories Visited", fmt.Sprintf("%d", s.DirectoriesVisited)})
	table.Append([]string{"Total Tokens", fmt.Sprintf("%d", s.TotalTokens)})
	table.Append([]string{"Most Tokens", fmt.Sprintf("%d", s.MostTokens)})
	table.Append([]string{"File with Most Tokens", s.MostTokensFile})
	table.Append([]string{"Processing Time", fmt.Sprintf("%d ms", s.TimeSinceCreation().Milliseconds())})

	table.Render()
}

func (s *Statistics) WriteToFile(files []File) error {
	outputFile, err := os.Create(s.OutputFile)
	if err != nil {
		return fmt.Errorf("failed to create output file: %v", err)
	}
	defer outputFile.Close()

	writer := bufio.NewWriter(outputFile)
	defer writer.Flush()

	for _, file := range files {
		fmt.Fprintf(writer, "--- File: %s ---\n", file.Path)
		fmt.Fprintln(writer, string(file.Contents))
		fmt.Fprintln(writer)
	}

	return nil
}
