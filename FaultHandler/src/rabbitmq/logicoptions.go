package rabbitmq

import (
	"encoding/json"

	"message"

	log "github.com/sirupsen/logrus"
	"github.com/streadway/amqp"
)

func CheckState(ch amqp.Channel) {
	for message_id := range SubscribedMessagesMap {
		if SubscribedMessagesMap[message_id].valid == true {
			switch {
			case SubscribedMessagesMap[message_id].routing_key == FAILURENETWORK:
				message.SendEmail("Serious Network failure")
				SubscribedMessagesMap[message_id].valid = false
			case SubscribedMessagesMap[message_id].routing_key == FAILUREDATABASE:
				message.SendEmail("Serious Database failure")
				message.SendSMS("Serious Database failure")
				SubscribedMessagesMap[message_id].valid = false
			case SubscribedMessagesMap[message_id].routing_key == FAILURECOMPONENT:
				message.SendEmail("Serious Component failure")
				SubscribedMessagesMap[message_id].valid = false
			case SubscribedMessagesMap[message_id].routing_key == FAILUREACCESS:
				message.SendEmail("Serious Access Violation")
				message.SendSMS("Serious Access Violation")
				SubscribedMessagesMap[message_id].valid = false
			case SubscribedMessagesMap[message_id].routing_key == FAILURECAMERA:
				var message FailureMessage
				json.Unmarshal([]byte(SubscribedMessagesMap[message_id].message), &message)
				power := RequestPower{"restart", message.Severity, CAMERAMONITOR}
				PublishRequestPower(&ch, power)
				SubscribedMessagesMap[message_id].valid = false
			case SubscribedMessagesMap[message_id].routing_key == ISSUENOTICE:
				var message IssueNotice
				json.Unmarshal([]byte(SubscribedMessagesMap[message_id].message), &message)
				power := RequestPower{message.action, 6, message.component}
				PublishRequestPower(&ch, power)
				SubscribedMessagesMap[message_id].valid = false
			default:
				log.Debug("What?")

			}
		}
	}
}
