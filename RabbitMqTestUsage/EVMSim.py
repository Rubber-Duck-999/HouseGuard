'''
Created on 10 Oct 2019

@author: simon
'''

#!/usr/bin/env python
import pika
import sys, time

###
# Environment Manager Simulator Interface 
# This is to show how the EVM could manager
# its necessary pub & sub topics with rabbitmq

### Setup of EVM connection
print("## Beginning EVM")
connection = pika.BlockingConnection(pika.ConnectionParameters(host='localhost'))
channel = connection.channel()
channel.exchange_declare(exchange='topics', exchange_type='topic', durable=False)
routing_key = 'motion.request'
#

# Publishing
message = 'Was there motion?'
print("My sensors have detected a motion, will confirm with CM")
print("Sending motion request to CM")
time.sleep(1)
channel.basic_publish(
    exchange='topics', 
    routing_key=routing_key, 
    body=message,
    properties=pika.BasicProperties(
        delivery_mode=2,  # make message persistent
    ))
#print(" [x] Sent %r:%r" % (routing_key, message))
result = channel.queue_declare('', exclusive=False, durable=True)
#

#
queue_name = result.method.queue
binding_key = 'motion.response'
channel.queue_bind(exchange='topics', queue=queue_name, routing_key=binding_key)
#print(' [*] Waiting for logs. To exit press CTRL+C')
print("Waiting for response from CM")
#

def callback(ch, method, properties, body):
    #print(" [x] %r:%r" % (method.routing_key, body))
    str = body.decode()
    print("CM sent us a response, we received " + str)
    key = 'motion.detected'
    print("CM detected motion, EVM to warn FH, send help!")
    print("Sending warning to FH")
    channel.queue_bind(exchange='topics', queue=queue_name, routing_key=key)
    channel.basic_publish(exchange='topics', routing_key=key, body='motion_detected')
    #print(" [x] Sent %r:%r" % (key, ':motion_detected'))
    channel.basic_ack(delivery_tag=method.delivery_tag)
    sys.exit()

channel.basic_consume(queue=queue_name, on_message_callback=callback, auto_ack=False)
channel.start_consuming()
