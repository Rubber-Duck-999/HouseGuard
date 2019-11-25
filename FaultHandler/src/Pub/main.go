package main

import (
	"fmt"

	log "github.com/sirupsen/logrus"
	"rabbitmq"
)

func main() {
	log.SetLevel(log.DebugLevel)
	log.Trace("FH - Beginning to run Fault Handler Program")
	ch := rabbitmq.Subscribe(rabbitmq.Messages)
	log.Trace("Channel is : ")
	fmt.Println(ch)
}
