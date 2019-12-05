// rabbitmq_test.go

package rabbitmq

import (
	"testing"
)

// Check that State is set
// then run this test will prove it is set
func TestStateSetTrue(t *testing.T) {
	SetState(true)
	if getState() == false {
		t.Error("Failure")
	}
}
