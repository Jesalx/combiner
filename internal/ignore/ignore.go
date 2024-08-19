package ignore

import (
	"path/filepath"
	"regexp"
	"strings"
)

type IgnoreService struct {
	prefixes []string
	suffixes []string
	regexes  []*regexp.Regexp
}

// New creates a new IgnoreService instance from a slice of string patterns
func New(patterns []string) *IgnoreService {
	ignoreService := &IgnoreService{}
	for _, pattern := range patterns {
		if strings.HasPrefix(pattern, "*") {
			// Suffix match (e.g., "*.go")
			ignoreService.suffixes = append(ignoreService.suffixes, pattern[1:])
		} else if strings.HasSuffix(pattern, "*") {
			// Prefix match (e.g., ".git*")
			ignoreService.prefixes = append(ignoreService.prefixes, pattern[:len(pattern)-1])
		} else if strings.Contains(pattern, "*") {
			// Complex pattern, use regex
			regexPattern := "^" + strings.ReplaceAll(regexp.QuoteMeta(pattern), "\\*", "[^/]*") + "$"
			regex := regexp.MustCompile(regexPattern)
			ignoreService.regexes = append(ignoreService.regexes, regex)
		} else {
			// Exact match, treat as prefix
			ignoreService.prefixes = append(ignoreService.prefixes, pattern)
		}
	}
	return ignoreService
}

// ShouldIgnore checks if a given path should be ignored
func (ignoreService *IgnoreService) ShouldIgnore(path string) bool {
	// Check prefixes
	for _, prefix := range ignoreService.prefixes {
		if strings.HasPrefix(path, prefix) {
			return true
		}
	}

	// Check suffixes
	for _, suffix := range ignoreService.suffixes {
		if strings.HasSuffix(path, suffix) {
			return true
		}
	}

	// Check regexes
	for _, regex := range ignoreService.regexes {
		if regex.MatchString(filepath.ToSlash(path)) {
			return true
		}
	}

	return false
}
