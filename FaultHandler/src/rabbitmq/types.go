package rabbitmq

type FailureMessage struct {
	Time         string `json:"time"`
	Failure_type string `json:"type"`
	Severity     int    `json:"severity"`
}

type MotionDetected struct {
	time string
}

type IssueNotice struct {
	time      string
	component string
	action    string
}

type MonitorState struct {
	state bool
}

type RequestPower struct {
	power     string
	severity  int
	component string
}

type EventFH struct {
	component    string
	error_string string
	time         string
	severity     int
}

type MapMessage struct {
	message     string
	routing_key string
	time        string
	valid       bool
}

const FAILURE string = "Failure.*"
const FAILURENETWORK string = "Failure.Network"
const FAILUREDATABASE string = "Failure.Database"
const FAILURECOMPONENT string = "Failure.Component"
const FAILUREACCESS string = "Failure.Access"
const FAILURECAMERA string = "Failure.Camera"
const MOTIONDETECTED string = "Motion.Detected"
const ISSUENOTICE string = "Issue.Notice"
const MONITORSTATE string = "Monitor.State"
const REQUESTPOWER string = "Request.Power"
const EXCHANGENAME string = "topics"
const EXCHANGETYPE string = "topic"
const TIMEFORMAT string = "20060102150405"
const CAMERAMONITOR string = "CM"

var SubscribedMessagesMap map[uint32]*MapMessage
var key_id uint32 = 0
