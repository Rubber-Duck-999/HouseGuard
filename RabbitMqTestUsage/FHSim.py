'''
Created on 9 Oct 2019

@author: simon
'''

#!/usr/bin/env python
import pika
import sys


print("## FH Start up")
connection = pika.BlockingConnection(pika.ConnectionParameters(host='localhost'))
channel = connection.channel()

channel.exchange_declare(exchange='topics', exchange_type='topic')

result = channel.queue_declare(queue='', exclusive=True)
queue_name = result.method.queue

key = 'motion.detected'

channel.queue_bind(exchange='topics', queue=queue_name, routing_key=key)

#print(' [*] Waiting for logs. To exit press CTRL+C')
print("Waiting for notifications")

def callback(ch, method, properties, body):
    #print(" [x] %r:%r" % (method.routing_key, body))
    str = body.decode()
    print("FH: I think we received a message: " + str)
    print("Warning operator of error, shut down of all secondary software")
    sys.exit()


channel.basic_consume(
    queue=queue_name, on_message_callback=callback, auto_ack=True)

channel.start_consuming()