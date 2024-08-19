package ignore

import (
	"testing"
)

func TestNew(t *testing.T) {
	patterns := []string{".git", "*.go", "tmp*", "test/*.txt"}
	is := New(patterns)

	if len(is.prefixes) != 2 {
		t.Errorf("Expected 2 prefixes, got %d", len(is.prefixes))
	}
	if len(is.suffixes) != 1 {
		t.Errorf("Expected 1 suffix, got %d", len(is.suffixes))
	}
	if len(is.regexes) != 1 {
		t.Errorf("Expected 1 regex, got %d", len(is.regexes))
	}
}

func TestShouldIgnore(t *testing.T) {
	patterns := []string{".git", "*.go", "tmp*", "test/*.txt"}
	is := New(patterns)

	testCases := []struct {
		path     string
		expected bool
	}{
		{".git/config", true},
		{".gitignore", true},
		{"main.go", true},
		{"src/helper.go", true},
		{"tmp", true},
		{"tmp/cache", true},
		{"test/file.txt", true},
		{"test/subdir/file.txt", false},
		{"src/main.rs", false},
		{"doc.pdf", false},
	}

	for _, tc := range testCases {
		t.Run(tc.path, func(t *testing.T) {
			result := is.ShouldIgnore(tc.path)
			if result != tc.expected {
				t.Errorf("ShouldIgnore(%q) = %v, expected %v", tc.path, result, tc.expected)
			}
		})
	}
}

func TestShouldIgnoreEdgeCases(t *testing.T) {
	patterns := []string{"", "*", "a*b"}
	is := New(patterns)

	testCases := []struct {
		path     string
		expected bool
	}{
		{"", true},
		{"anything", true},
		{"ab", true},
		{"acb", true},
		{"a/b", true},
		{"file.", true},
		{".", true},
		{"file.txt", true},
	}

	for _, tc := range testCases {
		t.Run(tc.path, func(t *testing.T) {
			result := is.ShouldIgnore(tc.path)
			if result != tc.expected {
				t.Errorf("ShouldIgnore(%q) = %v, expected %v", tc.path, result, tc.expected)
			}
		})
	}
}
