'''
Created on 8 Oct 2019

@author: simon
'''

#!/usr/bin/env python
import pika, sys, threading, time

class Publisher (threading.Thread):
   def __init__(self, threadID, name, counter):
      threading.Thread.__init__(self)
      self.threadID = threadID
      self.name = name
      self.counter = counter
      
   def run(self):
      print("Starting " + self.name)
      runPub(self.name, self.counter, 3)
      # Free lock to release next thread
      
def runPub(threadName, delay, counter):
   while counter:
      time.sleep(delay)
      print("%s: %s" % (threadName, time.ctime(time.time())))
      counter -= 1
      connection = pika.BlockingConnection(
      pika.ConnectionParameters(host='localhost'))
      channel = connection.channel()

      channel.exchange_declare(exchange='direct_logs', exchange_type='direct')

      severity = 'subscriber'
      message = 'I am the publisher'
      channel.basic_publish(
      exchange='direct_logs', routing_key=severity, body=message)
      print(" [x] Sent %r:%r" % (severity, message))
      connection.close()

