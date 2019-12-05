#!/bin/sh



if [ -z $BWD ];
then
    echo "Failure in Env Var BWD - " $BWD
    exit 1
else
    cd src
    go get -v github.com/streadway/amqp
    go get -v github.com/sirupsen/logrus
    go get -v github.com/scorredoira/email
    go get -v gopkg.in/yaml.v2
    go get -v github.com/akamensky/argparse
    go get -v github.com/clarketm/json
    go install github.com/Rubber-Duck-999/exeFaultHandler
    go test -cover github.com/Rubber-Duck-999/...
    #go install src/Pub/main.go
fi
