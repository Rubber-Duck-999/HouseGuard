'''
Created on 9 Oct 2019

@author: simon
'''

#!/usr/bin/env python
import pika
import sys, time

connection = pika.BlockingConnection(
    pika.ConnectionParameters(host='localhost'))
channel = connection.channel()

channel.exchange_declare(exchange='direct_pub', exchange_type='direct')

severity = 'publisher'
message = 'I am the publisher!'
x = 0
while x < 10:
    channel.basic_publish(
        exchange='direct_pub', routing_key=severity, body=message)
    x = x + 1
    time.sleep(1)
    print(" [x] Sent %r:%r" % (severity, message))
    
connection.close()
