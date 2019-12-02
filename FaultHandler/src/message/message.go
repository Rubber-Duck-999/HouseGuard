package message

import (
	"github.com/scorredoira/email"
	log "github.com/sirupsen/logrus"
	"net/mail"
	"net/smtp"
)

var _state bool
var _email string
var _password string
var _subject string
var _body string
var _from_email string
var _from_name string
var _to_email string

func init() {
	log.Trace("Initialised message package")
	_state = true
	_subject = ""
	_body = ""
	_email = ""
	_password = ""
	_from_email = ""
	_from_name = ""
	_to_email = ""
}

func SetState(state bool) {
	log.Debug("Requested to change our monitoring state")
	log.WithFields(log.Fields{
		"State": _state, "New State": state,
	}).Debug("State change")
	_state = state
}

func SetSettings(email string, password string, from_email string, from_name string, to_email string) {
	log.Trace("Setting settings")
	_subject = "Test Email"
	_body = ""
	_email = email
	_password = password
	_from_email = from_email
	_from_name = from_name
	_to_email = to_email
}

func TestEmail() bool {
	_subject = "Test Email"
	_body = ""
	fatal := sendEmail("Test")
	return fatal
}

func SendSMS(issue string) bool {
	if _state == true {
		log.Debug("Sending important SMS")
	}
	return false
}

func SendEmailRoutine(issue string) bool {
	event := sendEmail(issue)
	return event
}

func sendEmail(issue string) bool {
	// compose the message
	fatal := false
	if _state {
		_body = issue
		m := email.NewMessage(_subject, _body)
		m.From = mail.Address{Name: _from_name, Address: _from_email}
		m.To = []string{_to_email}

		// send it
		auth := smtp.PlainAuth("", _email, _password, "smtp.zoho.eu")
		if err := email.Send("smtp.zoho.eu:587", auth, m); err != nil {
			log.Warn("Found a issue")
			log.Warn(err)
			fatal = true
		}
	}
	return fatal
}
