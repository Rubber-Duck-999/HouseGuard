'''
Created on 10 Oct 2019

@author: simon
'''

#!/usr/bin/env python
import pika
import sys, time

###
# Database Manager Simulator Interface 
# This is to show how the DBM could manager
# its necessary pub & sub topics with rabbitmq

## Setup connection and exchange
connection = pika.BlockingConnection(pika.ConnectionParameters(host='localhost'))
channel = connection.channel()
channel.exchange_declare(exchange='topics', exchange_type='topic')
#

# Specific settings for subscribe key
print("## Beginning DBM Component")
result = channel.queue_declare('', exclusive=True)
queue_name = result.method.queue
binding_key = 'event.*'
channel.queue_bind(exchange='topics', queue=queue_name, routing_key=binding_key)
print(' DBM [*] Waiting for events. To exit press CTRL+C')


def callback(ch, method, properties, body):
    print(" DBM received an event [x] %r:%r" % (method.routing_key, body))
    time.sleep(2)
    routing_key = 'database.failure'
    print("We have an error, send help!")
    channel.basic_publish(exchange='topic_logs', routing_key=routing_key, body='failure:DBM')
    print(" [x] Sent %r:%r" % (routing_key, 'failure:DBM'))

channel.basic_consume(queue=queue_name, on_message_callback=callback, auto_ack=True)

channel.start_consuming()
connection.close()