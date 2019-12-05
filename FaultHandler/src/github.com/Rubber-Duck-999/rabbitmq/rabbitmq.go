package rabbitmq

import (
	"time"

	log "github.com/sirupsen/logrus"
	"github.com/streadway/amqp"
	"github.com/clarketm/json"
)

var conn *amqp.Connection
var ch *amqp.Channel

func init() {
	var err error
	log.Trace("Initialised rabbitmq package")
	conn, err = amqp.Dial("amqp://guest:guest@localhost:5672/")
	failOnError(err, "Failed to connect to RabbitMQ")

	ch, err = conn.Channel()
	failOnError(err, "Failed to open a channel")
}

func failOnError(err error, msg string) {
	if err != nil {
		log.WithFields(log.Fields{
			"Message": msg, "Error": err,
		}).Fatal("Rabbitmq error")
	}
}

func getTime() string {
	t := time.Now()
	log.Trace(t.Format(TIMEFORMAT))
	return t.Format(TIMEFORMAT)
}

func messages(routing_key string, value string) {
	log.Debug("Adding messages to map")
	if SubscribedMessagesMap == nil {
		SubscribedMessagesMap = make(map[uint32]*MapMessage)
		messages(routing_key, value)
	} else {
		if key_id >= 0 {
			_, valid := SubscribedMessagesMap[key_id]
			if valid {
				log.Debug("Key already exists, checking next field")
				key_id++
				messages(routing_key, value)
			} else {
				log.WithFields(log.Fields{
					"Value": value,
				}).Debug("Received this string")
				log.Debug("Key does not exists, adding new field")
				entry := MapMessage{value, routing_key, getTime(), true}
				SubscribedMessagesMap[key_id] = &entry
				key_id++
			}
		}
	}
}

func Subscribe() {
	log.Trace("Beginning rabbitmq initialisation")
	var err error

	var topics = [4]string{
		FAILURE,
		MOTIONDETECTED,
		ISSUENOTICE,
		MONITORSTATE,
	}

	err = ch.ExchangeDeclare(
		EXCHANGENAME, // name
		EXCHANGETYPE, // type
		true,         // durable
		false,        // auto-deleted
		false,        // internal
		false,        // no-wait
		nil,          // arguments
	)
	failOnError(err, "FH - Failed to declare an exchange")

	q, err := ch.QueueDeclare(
		"",    // name
		false, // durable
		false, // delete when usused
		true,  // exclusive
		false, // no-wait
		nil,   // arguments
	)
	failOnError(err, "Failed to declare a queue")

	for _, s := range topics {
		log.Printf("Binding queue %s to exchange %s with routing key %s",
			q.Name, EXCHANGENAME, s)
		err = ch.QueueBind(
			q.Name,       // queue name
			s,            // routing key
			EXCHANGENAME, // exchange
			false,
			nil)
		failOnError(err, "Failed to bind a queue")
	}

	msgs, err := ch.Consume(
		q.Name, // queue
		"",     // consumer
		true,   // auto ack
		false,  // exclusive
		false,  // no local
		false,  // no wait
		nil,    // args
	)
	failOnError(err, "Failed to register a consumer")

	forever := make(chan bool)

	go func() {
		for d := range msgs {
			log.Debug("Sending message to callback")
			log.Debug(d.RoutingKey)
			s := string(d.Body[:])
			messages(d.RoutingKey, s)
			log.Debug("Checking states of received messages")
			checkState()
		}
		//This function is checked after to see if multiple errors occur then to
		//through an event message
	}()

	log.Printf(" [*] Waiting for logs. To exit press CTRL+C")
	<-forever
}


func PublishRequestPower(this_power string, this_severity int, this_component string) {
	requestPower, err := json.Marshal(&RequestPower{
			Power: this_power,
			Severity: this_severity,
			Component: this_component, })
	failOnError(err, "Failed to convert RequestPower")
	log.Debug(string(requestPower))

	err = ch.Publish(
		EXCHANGENAME, // exchange
		REQUESTPOWER, // routing key
		false,        // mandatory
		false,        // immediate
		amqp.Publishing{
			ContentType: "application/json",
			Body:        []byte(requestPower),
		})
	failOnError(err, "Failed to publish RequestPower topic")
}

func PublishEventFH(component string, error_string string, time string, severity int) {

	eventFH, err := json.Marshal(&EventFH{
			Component: component,
			Error_string: error_string,
			Time: time,
		 	Severity: severity, })
	failOnError(err, "Failed to convert EventFH")
	log.Debug(string(eventFH))

	err = ch.Publish(
		EXCHANGENAME, // exchange
		EVENTFH, // routing key
		false,        // mandatory
		false,        // immediate
		amqp.Publishing{
			ContentType: "application/json",
			Body:        []byte(eventFH),
		})
	failOnError(err, "Failed to publish EventFH topic")
}