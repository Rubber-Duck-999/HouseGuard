'''
Created on 8 Oct 2019

@author: simon
'''

#!/usr/bin/python

import threading
import time
from PublisherDirect import *
from SubscribeDirect import *

threadLock = threading.Lock()
threads = []

# Create new threads
thread1 = Publisher(1, "Thread-1", 1)
thread2 = Subscriber(2, "Thread-2", 5)

# Start new Threads
thread1.start()
thread2.start()

# Add threads to thread list
threads.append(thread1)
threads.append(thread2)

# Wait for all threads to complete
for t in threads:
    t.join()
print("Exiting Main Thread")