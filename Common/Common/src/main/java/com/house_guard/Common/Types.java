package com.house_guard.Common;

public interface Types
{
  public enum Actions
  {
    ADD_D1,
    SUB_D1,
    ADD_D2,
    SUB_D2,
    ADD_D3,
    SUB_D3,
    ADD_D4,
    SUB_D4,
    ENTER
  }
  
  public enum State
  {
      ON,
      OFF
  }
  
  final int D1 = 0;
  final int D2 = 1;
  final int D3 = 2;
  final int D4 = 3;
  
  final int RESET = 0;
  final String OFF = "OFF";
  final String ON = "ON";
  final Integer MAX_ATTEMPTS = 3;
  
  final int ATTEMPTS_MAXED = 0;
  final int INCORRECT = 1;
  final int CORRECT = 2;
  final int TIMEOUT = 3;
  
  final String REQUEST_ACCESS_TOPIC = "request.access";
  final String MONITOR_STATE_TOPIC = "monitor.state";
  
  final String PUB_EVENT_TOPIC = "event.UP";
  final String COMPONENT_NAME = "UP";
  final Integer ACCESS_NOT_RECEIVED = 4;
  final Object PASS = "PASS";
  final String RequestFailure = "We have access failured";
  final String ACCESS_RESPONSE = "access.response";
  final Integer MAXPIN = 9999;
  
}
