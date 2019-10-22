'''
Created on 10 Oct 2019

@author: simon
'''

#!/usr/bin/env python
import pika
import sys, time

###
# Camera Monitor Simulator Interface 
# This is to show how the CM could manager
# its necessary pub & sub topics with rabbitmq

## Setup connection and exchange
connection = pika.BlockingConnection(pika.ConnectionParameters(host='localhost'))
channel = connection.channel()
channel.exchange_declare(exchange='topics', exchange_type='topic', durable=False)
#

# Specific settings for subscribe key
print("## Beginning CM Component")
result = channel.queue_declare('', exclusive=False, durable=False)
queue_name = result.method.queue
binding_key = 'motion.request'
channel.queue_bind(exchange='topics', queue=queue_name, routing_key=binding_key)
#print(' CM [*] Waiting for events. To exit press CTRL+C')


def callback(ch, method, properties, body):
    #print("CM received an event [x] %r:%r" % (method.routing_key, body))
    time.sleep(2)
    key = 'motion.response'
    print("EVM has asked us to confirm whether we had motion at this timeframe")
    print(".... checking")
    time.sleep(1)
    print(".... checking")
    print("Camera detected motion")
    print("Returning response to EVM for management of situation")
    channel.queue_bind(exchange='topics', queue=queue_name, routing_key=key)
    channel.basic_publish(exchange='topics', routing_key=key, body='-CM Confirmed Unauthorised Motion-End!')
    #print(" [x] Sent %r:%r" % (key, ':motion_detected'))
    sys.exit()

channel.basic_consume(queue=queue_name, on_message_callback=callback, auto_ack=True)

channel.start_consuming()
connection.close()
