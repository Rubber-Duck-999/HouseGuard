[![Build Status](https://dev.azure.com/simoncrowther95/simoncrowther95/_apis/build/status/Rubber-Duck-999.HouseGuard?branchName=master)](https://dev.azure.com/simoncrowther95/simoncrowther95/_build/latest?definitionId=6&branchName=master)


HouseGuard

![Architecture](/architecture.jpg)


Overview
The house guard system is a system specifically designed to monitor and report motion in the property the device is installed in. Any other features are secondary. This document will lay out the systems requirements, how it is designed to be interacted with and usage.
System Goals
Alert and contact user upon unauthorised access
Measure property environment
Monitor system performance and events
Warn user of system faults
Document Goals
To explain the systems high level design and usage
Creation of functional system requirements
Software component design and internal communication requirements
System Behaviour
The system has to be standalone and require no input from the user to diagnose basic faults, also capable of alerting the user to severe issues. Severity is based on a scaling of problems that could occur and will be listed later on. The system should start up from powering up the hardware and report daily status at the minimum to the user’s choice upon installation of the software.
An operator may be able to access information stored on the system but only when they are authorised by the admin user of the system. The admin account must be setup upon installation of the software. The system will contain a camera that will calculate whether authorised motion has occurred. The operator can turn on or off the monitoring of unauthorised access by the User Interface panel or the UI browser hosted by the system.
The system must be able to detect whether unauthorised access has occurred on the network. The system has standalone permissions so it can shut down parts of the network if it so needs.
Functional Requirements Broken Down
The system must not allow unauthorised access to any of the systems data held.
The system must be capable of restarting services if errors occur.
The system must have a user interface login for the operator to access data held onboard.
The system must not be accessible outside of the current network.
The camera shall work in low light and daylight.
The system must be able to operate up to 7 days without human diagnostic checks.
The system must be able to hold up to 180 days of system information.
The system must be able to accept weather data requests for the current time at request.
The system must be able to contact the relevant operators by email and sms if a level 2 or above fault occurs.
The system must record all events and errors internally following requirement 7.
The system must be able to record the systems performance.
The system must be able to have at least one user account and one admin account for access.
The admin may only be able to run and stop monitoring sensors on the system.
The system must be able to transfer information stored internally to external software if the admin is operating.

