'''
Created on 8 Oct 2019

@author: simon
'''

#!/usr/bin/env python
import pika
import sys, threading, time

class Subscriber (threading.Thread):
   def __init__(self, threadID, name, counter):
      threading.Thread.__init__(self)
      self.threadID = threadID
      self.name = name
      self.counter = 0
      connection = pika.BlockingConnection(
      pika.ConnectionParameters(host='localhost'))
      self.channel = connection.channel()
    
      self.channel.exchange_declare(exchange='direct_logs', exchange_type='direct')
    
      result = self.channel.queue_declare(queue='', exclusive=True)
      self.queue_name = result.method.queue
    
      severity = 'publisher'
    
      self.channel.queue_bind(exchange='direct_logs', queue=self.queue_name, routing_key=severity)
    
      
   def run(self):
      print("Starting " + self.name)
      self.runSub(self.name) 

   def runSub(self, threadName):
      time.sleep(5)
      print("%s: %s" % (threadName, time.ctime(time.time())))

      print(' [*] Waiting for logs. To exit press CTRL+C')

      self.channel.basic_consume(
         queue=self.queue_name, on_message_callback=callback, auto_ack=True)

      self.channel.start_consuming()

def callback(ch, method, properties, body):
    print(" [x] %r:%r" % (method.routing_key, body))
    sys.exit()
    


