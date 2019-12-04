// access_test.go (access-check)

package config

import (
	"testing"
)

// We can check the file exist
// then run this test will prove it checks correctly exists
func TestExists(t *testing.T) {
	var this_file string = `/home/ubuntu/environment/HouseGuard/FaultHandler/config.yml-sample`
	if Exists(this_file) != true {
		t.Error("Failure")
	}
}

// We can check the file doesn't exist
// then run this test will prove it checks correctly
func TestDoesntExists(t *testing.T) {
	var this_file string = `/home/ubuntu/environment/HouseGuard/cheese.txt`
	if Exists(this_file) == true {
		t.Error("Failure")
	}
}

// We can check the folder exists
// then run this test will prove it checks correctly exists
func TestFolder(t *testing.T) {
	var this_file string = `/home/ubuntu/environment/HouseGuard/`
	if Exists(this_file) == true {
		t.Error("Failure")
	}
}

func TestConfigPullNull(t *testing.T) {
	var file string = `/home/ubuntu/environment/HouseGuard/FaultHandler/config.yml-sample`
	var data ConfigTypes
	GetData(&data, file)
	if data.EmailSettings.Email == "" {
		t.Error("Failure")
	}
	if data.EmailSettings.Password == "" {
		t.Error("Failure")
	}
	if data.EmailSettings.Name == "" {
		t.Error("Failure")
	}
	if data.EmailSettings.To_email == "" {
		t.Error("Failure")
	}
}
