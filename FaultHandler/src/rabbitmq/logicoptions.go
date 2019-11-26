package rabbitmq

import (
	"encoding/json"

	"message"

	log "github.com/sirupsen/logrus"
)

func messageFailure(issue bool) {
	if issue {
		event := EventFH{COMPONENT, SERVERERROR, getTime(), SERVERSEVERITY}
		PublishEventFH(event)
	}
}

func SetEmailSettings(email string, password string, from_name string, to_email string) bool {
	shutdown_valid := false
	message.SetSettings(email, password, email, from_name, to_email)
	setup_valid := message.TestEmail()
	if setup_valid == false {
		log.Fatal("We have major flaw - shutting down node and diagonose")
		shutdown_valid = true
		//messageFailure(shutdown_valid)
	}
	return shutdown_valid
}

func checkState() {
	for message_id := range SubscribedMessagesMap {
		if SubscribedMessagesMap[message_id].valid == true {
			switch {
			case SubscribedMessagesMap[message_id].routing_key == FAILURENETWORK:
				messageFailure(message.SendEmailRoutine("Serious Network failure"))
				SubscribedMessagesMap[message_id].valid = false

			case SubscribedMessagesMap[message_id].routing_key == FAILUREDATABASE:
				messageFailure(message.SendEmailRoutine("Serious Database failure"))
				messageFailure(message.SendSMS("Serious Database failure"))
				SubscribedMessagesMap[message_id].valid = false

			case SubscribedMessagesMap[message_id].routing_key == FAILURECOMPONENT:
				messageFailure(message.SendEmailRoutine("Serious Component failure"))
				SubscribedMessagesMap[message_id].valid = false

			case SubscribedMessagesMap[message_id].routing_key == FAILUREACCESS:
				messageFailure(message.SendEmailRoutine("Serious Access Violation"))
				messageFailure(message.SendSMS("Serious Access Violation"))
				SubscribedMessagesMap[message_id].valid = false

			case SubscribedMessagesMap[message_id].routing_key == FAILURECAMERA:
				var message FailureMessage
				json.Unmarshal([]byte(SubscribedMessagesMap[message_id].message), &message)
				power := RequestPower{"restart", message.Severity, CAMERAMONITOR}
				PublishRequestPower(power)
				SubscribedMessagesMap[message_id].valid = false

			case SubscribedMessagesMap[message_id].routing_key == ISSUENOTICE:
				var message IssueNotice
				json.Unmarshal([]byte(SubscribedMessagesMap[message_id].message), &message)
				power := RequestPower{message.action, message.severity, message.component}
				PublishRequestPower(power)
				SubscribedMessagesMap[message_id].valid = false

			case SubscribedMessagesMap[message_id].routing_key == MONITORSTATE:
				var monitor MonitorState
				json.Unmarshal([]byte(SubscribedMessagesMap[message_id].message), &monitor)
				message.SetState(monitor.state)
				event := EventFH{COMPONENT, UPDATESTATEERROR, getTime(), STATEUPDATESEVERITY}
				PublishEventFH(event)
				SubscribedMessagesMap[message_id].valid = false

			default:
				log.Debug("What?")
			}
		}
	}

}
