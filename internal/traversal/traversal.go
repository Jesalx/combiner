package traversal

import (
	"combiner/internal/ignore"
	"combiner/internal/statistics"
	"fmt"
	"io/fs"
	"os"
	"path/filepath"
	"unicode/utf8"
)

func CollectFiles(directory string, ignoreService *ignore.IgnoreService, stats *statistics.Statistics) []statistics.File {
	output := []statistics.File{}

	filepath.WalkDir(directory, func(path string, d fs.DirEntry, err error) error {
		if err != nil {
			fmt.Printf("Error accessing path: %v\n", err)
		}
		if ignoreService.ShouldIgnore(path) {
			return nil
		}
		if d.IsDir() {
			stats.IncrementDirectoriesVisited()
			return nil
		}

		contents, err := os.ReadFile(path)
		if err != nil {
			fmt.Printf("Error reading file: %v\n", err)
			return nil
		}

		if !utf8.Valid(contents) {
			fmt.Printf("Skipping File: %s is not valid UTF-8\n", path)
			stats.IncrementSkippedFiles()
			return nil // Return nil so that traversal continues
		}

		relPath, _ := filepath.Rel(directory, path)
		output = append(output, statistics.File{
			Path:     relPath,
			Contents: contents,
		})
		// fmt.Printf("File contents: %s\n", string(contents))
		fmt.Printf("File %s\n", path)
		stats.IncrementProcessedFiles()
		return nil
	})

	return output
}
