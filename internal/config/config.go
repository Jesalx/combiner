package config

type Config struct {
	Verbose bool
}

func New(verbose bool) *Config {
	return &Config{
		Verbose: verbose,
	}
}
