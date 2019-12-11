'''
Created on 10 Oct 2019

@author: simon
'''

#!/usr/bin/env python
import pika
import sys, time, json

###
# Network Access Controller Simulator Interface
# This is to show how the NAC could manager
# its necessary pub & sub topics with rabbitmq

### Setup of EVM connection
print("## Beginning EVM")
connection = pika.BlockingConnection(pika.ConnectionParameters(host='localhost'))
channel = connection.channel()
channel.exchange_declare(exchange='topics', exchange_type='topic', durable=True)
key = 'request.access'
key_publish = 'access.response'
key_event = 'event.UP'
#

# Publishing
result = channel.queue_declare('', exclusive=False, durable=True)
queue_name = result.method.queue
channel.queue_bind(exchange='topics', queue=queue_name, routing_key=key)
channel.queue_bind(exchange='topics', queue=queue_name, routing_key=key_event)
print("Waiting for request from UP")
while True:
    channel.basic_publish(exchange='topics', routing_key=key_publish, body="{ 'id': 1, 'result': 'FAIL' }")
#

def callback(ch, method, properties, body):
    print(" [x] %r:%r" % (method.routing_key, body))
    if method.routing_key == "key_event":
        print("Event Up receieved : " + str)
        my_json = body.decode('utf8').replace("'", '"')
        #print(my_json)
        #print('- ' * 20)
        data = json.loads(my_json)
        #print(data)
        #print(data["component"])
    else:
        print("UP sent us a request, we received = " + body.decode())
        my_json = body.decode('utf8').replace("'", '"')
        #print(my_json)
        #print('- ' * 20)
        # Load the JSON to a Python list & dump it back out as formatted JSON
        data = json.loads(my_json)
        #print(data)
        #print(data["id"])
        #print(data["pin"])
        pin_correct = 1234
        if data["pin"] == pin_correct:
            time.sleep(1)
            #channel.queue_bind(exchange='topics', queue=queue_name, routing_key=key_publish)
            channel.basic_publish(exchange='topics', routing_key=key_publish, body="{ 'id': 1, 'result': 'PASS' }")
        else:
            time.sleep(1)
            #channel.queue_bind(exchange='topics', queue=queue_name, routing_key=key_publish)
            channel.basic_publish(exchange='topics', routing_key=key_publish, body="{ 'id': 1, 'result': 'FAIL' }")
        #sys.exit(0)

channel.basic_consume(queue=queue_name, on_message_callback=callback, auto_ack=False)
channel.start_consuming()
