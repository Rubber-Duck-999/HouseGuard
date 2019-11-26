extern crate amqp;

use amqp::{
    Session,
    Channel,
    Table,
    Basic,
};

use amqp::protocol::basic::{
    BasicProperties,
    Deliver,
};

use std::thread::spawn;
use std::{
    str,
    time,
    thread,
};

impl SessionRabbitmq
{

    /// Refactor of the queue creation process.
    ///
    /// Args:
    ///
    /// `queue_name` - the name of the queue to declare
    /// `durable` - indicates if the queue is durable
    fn declare_queue(&mut self, queue_name:&str) 
    {
        /* TODO: add parameters documentation */
        self._channel.queue_declare(
            queue_name,
            false,
            self._durable,
            false,
            false,
            false,
            Table::new()
        ).unwrap();
    }
    
    /// Generates a session and a channel for a consumer or producer.
    /// Terminates the program if either the session, channel or queue can be created.
    ///
    /// Args:
    ///
    /// `prefetch_count` - maximum non aknowledged messages a consumer can consume before refusing new messages
    ///
    /// Returns:
    ///
    /// (Session, Channel)
    fn create_session_and_channel(&mut self) 
    {
        if self._init
        {
            
        }
        else
        {
            self._session = Session::open_url(QUEUE_URL).unwrap();
            self._channel = session.open_channel(1).unwrap();
        
            if self._prefetch_count != 0 
            {
                channel.basic_prefetch(prefetch_count).unwrap();
            }
            self._init = false;
        }
    }
    
    /// Correctly terminates the given session and channel, sterminate a successfull reply code with close-ok message.
    /// Terminates the program immediately if the channel cannot be closed.
    ///
    /// Args:
    ///
    /// `session` - the session to close
    /// `channel` - the channel to close
    fn terminate_session_and_channel(&mut self) 
    {
        const CLOSE_REPLY_CODE: u16 = 200;
        const CLOSE_REPLY_TEXT: &str = "closing producer";
        self._channel.close(
            CLOSE_REPLY_CODE,
            CLOSE_REPLY_TEXT,
        ).unwrap();
        self._session.close(
            CLOSE_REPLY_CODE,
            CLOSE_REPLY_TEXT,
        );
    }
    
    /// Simulates a consumer (worker). Continuously checks for messages from the queue.
    ///
    /// Args:
    ///
    /// `queue_name` - the name of the concerned queue
    /// `consumer_index` - the index of the consumer to use for logging
    /// `enable_ack` - enables acknowledgment of consumed messages
    /// `durable` - indicates if the queue messages are durable (if they are written on disk in case of queue failure/stop)
    /// `prefetch_count` - maximum non aknowledged messages a consumer can consume before refusing new messages
    /// `fanout` - indicates if fanout is enabled: creates an exchange
    pub fn consume(&mut self) 
    {
        let mut queue_name: &str = "";
    
        declare_queue(&queue_name);
    
        channel.exchange_declare(
            "topics",
            "topic",
            true,
            false,
            false,
            false,
            false,
            Table::new(),
        ).unwrap();
    
        channel.queue_bind(
            queue_name,
            EXCHANGE_NAME,
            POWER_NOTICE,
            false,
            Table::new(),
        ).unwrap();
    
        channel.basic_consume(
            move |
                _chan: &mut Channel,
                _deliver: Deliver,
                _headers: BasicProperties,
                data: Vec<u8>,
            | {
                let message = str::from_utf8(&data).unwrap();
                println!(
                    "[{} Consumer ] Start handling message: {}",
                    queue_name,
                    message,
                );
    
                /* simulate a working task */
                const TASK_SECONDS_DURATION: u64 = 3;
                thread::sleep(time::Duration::from_secs(TASK_SECONDS_DURATION));
    
                println!(
                    "[{} Consumer ] Terminate handling message: {}",
                    queue_name,
                    message,
                );
            },
            queue_name,
            "",
            false,
            false,
            false,
            false,
            Table::new(),
        ).unwrap();
    
        println!(
            "[{} Consumer ] Started.",
            queue_name,
        );
    
        channel.start_consuming();
    
        terminate_session_and_channel(
            _session,
            channel,
        );
    }
    
    pub fn publish(&mut self,topic: &str, message: &str) 
    {
        let _message: &str = "I am the SYP, Hi!";
        
    
        channel.exchange_declare(
            EXCHANGE_NAME,
            "topic",
            true,
            false,
            false,
            false,
            false,
            Table::new(),
        ).unwrap();
        
    
        channel.basic_publish(
            EXCHANGE_NAME,
            topic,
            false,
            false,
            BasicProperties {
                content_type: Some("text".to_string()),
                ..Default::default()
            },
            _message.to_string().into_bytes(),
        ).unwrap();
    
    
        terminate_session_and_channel(
            session,
            channel,
        );
    }
}