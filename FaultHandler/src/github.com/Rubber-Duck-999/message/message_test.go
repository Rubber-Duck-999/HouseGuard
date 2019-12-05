// access_test.go (access-check)

package message

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

// Check that a test email will
// to send as no details have been inputted
func TestSendFailureEmail(t *testing.T) {
	if TestEmail() == false {
		t.Error("Failure")
	}
}
