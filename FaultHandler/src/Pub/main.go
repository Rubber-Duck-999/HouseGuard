package main

import (
	//"config"
	log "github.com/sirupsen/logrus"
	//"os"

	"rabbitmq"
)

func main() {
	log.SetLevel(log.TraceLevel)
	log.Trace("FH - Beginning to run Fault Handler Program")
	//file := "/home/ubuntu/environment/HouseGuard/FaultHandler/config.yml"
	/*var data config.ConfigTypes
	if config.Exists(file) {
		data := config.GetData(file)
		log.Debug(data)
	} else {
		os.Exit(1)
	}
	valid := rabbitmq.SetEmailSettings(data.EmailSettings.Email,
	data.EmailSettings.Password,
	data.EmailSettings.Name,
	data.EmailSettings.To_email)*/
	rabbitmq.Subscribe()
}
