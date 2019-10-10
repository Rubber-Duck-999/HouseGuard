'''
Created on 9 Oct 2019

@author: simon
'''

#!/usr/bin/env python
import pika
import sys

def sub(): 
    x = False
    while x == False:  
        connection = pika.BlockingConnection(
            pika.ConnectionParameters(host='localhost'))
        channel = connection.channel()
        
        channel.exchange_declare(exchange='direct_pub', exchange_type='direct')
        
        result = channel.queue_declare(queue='', exclusive=True)
        queue_name = result.method.queue
        
        severity = 'publisher'
        
        channel.queue_bind(exchange='direct_pub', queue=queue_name, routing_key=severity)
        
        print(' [*] Waiting for logs. To exit press CTRL+C')
        
        
        def callback(ch, method, properties, body):
            print(" [x] %r:%r" % (method.routing_key, body))
            
            
        
        
        channel.basic_consume(
            queue=queue_name, on_message_callback=callback, auto_ack=True)
        
        try:
            channel.start_consuming()
        except KeyboardInterrupt:
            channel.stop_consuming()
            connection.close()
            break
    
def pub():
    connection = pika.BlockingConnection(
    pika.ConnectionParameters(host='localhost'))
    channel = connection.channel()
    
    channel.exchange_declare(exchange='direct_logs', exchange_type='direct')
    
    severity ='subscriber'
    message = 'I am the subscriber'
    channel.basic_publish(
        exchange='direct_logs', routing_key=severity, body=message)
    print(" [x] Sent %r:%r" % (severity, message))
    connection.close()
    
    
sub()
pub()