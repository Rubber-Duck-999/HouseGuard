'''
Created on 10 Oct 2019

@author: simon
'''

#!/usr/bin/env python
import pika
import sys, time

###
# Network Access Controller Simulator Interface
# This is to show how the NAC could manager
# its necessary pub & sub topics with rabbitmq

### Setup of EVM connection
print("## Beginning EVM")
connection = pika.BlockingConnection(pika.ConnectionParameters(host='localhost'))
channel = connection.channel()
channel.exchange_declare(exchange='topics', exchange_type='topic', durable=False)
binding_key = 'access.request'
#

# Publishing
result = channel.queue_declare('', exclusive=False, durable=True)
#

#
queue_name = result.method.queue
channel.queue_bind(exchange='topics', queue=queue_name, routing_key=binding_key)
print("Waiting for request from UP")
#

def callback(ch, method, properties, body):
    #print(" [x] %r:%r" % (method.routing_key, body))
    str = body.decode()
    print("UP sent us a request, we received " + str)
    key = 'access.response'
    print("The pin is incorrect!")
    time.sleep(0.5)
    print("Sending warning to UP")
    channel.queue_bind(exchange='topics', queue=queue_name, routing_key=key)
    x = True
    while x:
        channel.basic_publish(exchange='topics', routing_key=key, body='FAIL')
    sys.exit()

channel.basic_consume(queue=queue_name, on_message_callback=callback, auto_ack=False)
channel.start_consuming()
