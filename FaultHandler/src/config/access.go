package config

import (
	"gopkg.in/yaml.v2"
	"os"

	log "github.com/sirupsen/logrus"
)

// Exists reports whether the named file or directory exists.
func Exists(name string) bool {
	if _, err := os.Stat(name); err != nil {
		if os.IsNotExist(err) {
			log.Fatal("File doesn't exist")
			return false
		}
	}
	return true
}

func GetData(file string) ConfigTypes {
	f, err := os.Open(file)
	if err != nil {
		log.Fatal("Failed to open file")
	}

	var cfg ConfigTypes
	decoder := yaml.NewDecoder(f)
	err = decoder.Decode(&cfg)
	if err != nil {
		log.Fatal("Couldn't edit file")
	}
	return cfg
}
