package message

import (
	"log"
	"net/mail"
	"net/smtp"
	"email"
	"fmt"
)

func Email() {
	// compose the message
	m := email.NewMessage("Subject", "this is the body")
	m.From = mail.Address{Name: "From", Address: "email"}
	m.To = []string{"to"}

	// add headers
	m.AddHeader("X-CUSTOMER-id", "xxxxx")

	// send it
	auth := smtp.PlainAuth("", "email", "pwd", "smtp.zoho.eu")
	if err := email.Send("smtp.zoho.eu:587", auth, m); err != nil {
		log.Fatal(err)
		fmt.Println("Issue occured")
	}
}
