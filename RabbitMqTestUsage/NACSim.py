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
channel.exchange_declare(exchange='topics', exchange_type='topic', durable=False)
key = 'request.access'
key_event = 'Event.UP'
#

# Publishing
result = channel.queue_declare('', exclusive=False, durable=True)
#

#
queue_name = result.method.queue
channel.queue_bind(exchange='topics', queue=queue_name, routing_key=key)
channel.queue_bind(exchange='topics', queue=queue_name, routing_key=key_event)
print("Waiting for request from UP")
#

def callback(ch, method, properties, body):
    #print(" [x] %r:%r" % (method.routing_key, body))
    str = body.decode()
    print("UP sent us a request, we received = " + str)
    #body = '{ "name":"John", "age":30, "city":"New York"}'
    str_dict = json.loads(str)
    print(str_dict["id"])
    key = 'access.response'
    if '1234' in str:
        time.sleep(1)
        channel.queue_bind(exchange='topics', queue=queue_name, routing_key=key)
        channel.basic_publish(exchange='topics', routing_key=key, body="{ 'id': 1, 'result': 'PASS' }")
    else:
        time.sleep(1)
        channel.queue_bind(exchange='topics', queue=queue_name, routing_key=key)
        channel.basic_publish(exchange='topics', routing_key=key, body='FAIL')
    #sys.exit()

channel.basic_consume(queue=queue_name, on_message_callback=callback, auto_ack=False)
channel.basic_consume(queue=queue_name, on_message_callback=callback, auto_ack=False)
channel.start_consuming()
