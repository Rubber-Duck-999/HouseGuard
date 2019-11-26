package rabbitmq

import (
	"encoding/json"
	"time"

	log "github.com/sirupsen/logrus"
	"github.com/streadway/amqp"
)

func init() {
	log.SetLevel(log.TraceLevel)
	log.Debug("Initialised rabbitmq package")
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

var ch amqp.Channel

func messages(routing_key string, value string) {
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
	x := "First Test"
	messages("Key", x)

	log.Trace("Beginning rabbitmq initialisation")

	var topics = [4]string{
		FAILURE,
		MOTIONDETECTED,
		ISSUENOTICE,
		MONITORSTATE,
	}

	conn, err := amqp.Dial("amqp://guest:guest@localhost:5672/")
	failOnError(err, "Failed to connect to RabbitMQ")
	defer conn.Close()

	ch, err := conn.Channel()
	failOnError(err, "Failed to open a channel")
	defer ch.Close()

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
		}
		checkState()
		//This function is checked after to see if multiple errors occur then to
		//through an event message
	}()

	log.Printf(" [*] Waiting for logs. To exit press CTRL+C")
	<-forever
}

func PublishRequestPower(message RequestPower) {
	requestPower, err := json.Marshal(message)
	failOnError(err, "Failed to convert RequestPower")
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

func PublishEventFH(message EventFH) {
	eventFH, err := json.Marshal(message)
	failOnError(err, "Failed to convert eventFH")
	err = ch.Publish(
		EXCHANGENAME, // exchange
		REQUESTPOWER, // routing key
		false,        // mandatory
		false,        // immediate
		amqp.Publishing{
			ContentType: "application/json",
			Body:        []byte(eventFH),
		})
	failOnError(err, "Failed to publish EventFH topic")
}