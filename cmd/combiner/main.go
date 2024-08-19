package main

import (
	"combiner/internal/ignore"
	"combiner/internal/statistics"
	"combiner/internal/tokenization"
	"combiner/internal/traversal"
	"fmt"
	"os"

	"github.com/spf13/cobra"
)

func main() {
	var directory string
	var outputFile string
	var ignorePatterns []string
	var tokenizer string
	var includeHidden bool

	rootCmd := &cobra.Command{
		Use:   "combiner",
		Short: "Description of combiner",
		Run: func(cmd *cobra.Command, args []string) {
			stats := statistics.New(outputFile)
			ignorePatterns = append(ignorePatterns, outputFile) // Ignore the output file itself
			if !includeHidden {
				ignorePatterns = append(ignorePatterns, ".*") // Ignore hidden files
			}
			ignoreService := ignore.New(ignorePatterns)
			files := traversal.CollectFiles(directory, ignoreService, stats)
			tokenizer := tokenization.New(tokenizer)
			tokenizer.ProcessFiles(stats, files)
			stats.Print()
			stats.WriteToFile(files)
		},
	}
	rootCmd.Flags().StringVarP(&directory, "directory", "d", ".", "directory to traverse")
	rootCmd.Flags().StringVarP(&outputFile, "output", "o", "combined_output.txt", "output file path/name")
	rootCmd.Flags().StringVarP(&tokenizer, "tokenizer", "t", "p50k_base", "tokenizer to use")
	rootCmd.Flags().StringSliceVarP(&ignorePatterns, "ignore", "i", nil, "files/directories to ignore")
	rootCmd.Flags().BoolVar(&includeHidden, "include-hidden", false, "include hidden files and directories")

	if err := rootCmd.Execute(); err != nil {
		fmt.Println(err)
		os.Exit(1)
	}
}
