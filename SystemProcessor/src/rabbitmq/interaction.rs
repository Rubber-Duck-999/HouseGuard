extern crate amqp;

use amqp::{
    Session,
    Channel,
    Table,
    Basic,
    Options,
};

extern crate log;
extern crate simple_logger;

use log::Level;

use amqp::protocol::basic::{
    BasicProperties,
    Deliver,
};

use std::{
    str,
    time,
    thread,
};

use std::default::Default;

use crate::rabbitmq::types;
use crate::system::constants;

pub struct SessionRabbitmq
{
    pub _durable: bool,
    pub _session: Session,
    pub _channel: Channel,
    pub _prefetch_count: u16,
    pub _init: bool,
}

fn GetSession() -> Session
{
    let session = match Session::new(Options { .. Default::default() }){
         Ok(session) => session,
         Err(error) => panic!("Failed openning an amqp session: {:?}", error)
    };
    return session;
}

fn GetChannel(mut session:Session) -> Channel
{
    let channel = session.open_channel(1).ok().expect("Can not open a channel");
    return channel;
}


impl Default for SessionRabbitmq
{
    fn default() -> SessionRabbitmq
    {
        let session:Session = GetSession();
        let channel:Channel = GetChannel(session);
        let session_new:Session = GetSession();
        SessionRabbitmq
        {
            _durable: false,
            _session: session_new,
            _channel: channel,
            _prefetch_count: 1,
            _init: false,
        }
    }
}

pub fn consumer_function(channel: &mut Channel, deliver: Deliver, headers: BasicProperties, body: Vec<u8>)
{
    println!("Got a delivery:");
    println!("[function] Content body(as string): {:?}", String::from_utf8(body));
}

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
        warn!("Declaring queue for consumption");
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

    pub fn GetMessage(&mut self)
    {

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
    pub fn Create_session_and_channel(&mut self)
    {
        if self._init
        {
            println!("Initialised Rabbitmq Connection = {}", constants::COMPONENT_NAME);
        }
        else
        {
            warn!("Creating session and channel");
            self._session = Session::open_url(types::QUEUE_URL).unwrap();
            self._channel = self._session.open_channel(1).unwrap();

            if self._prefetch_count != 0
            {
                self._channel.basic_prefetch(self._prefetch_count).unwrap();
            }
            self._init = true;
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

    pub fn publish(&mut self,topic: &str, message: &str)
    {

        self._channel.basic_publish(
            types::EXCHANGE_NAME,
            topic,
            false,
            false,
            BasicProperties {
                content_type: Some("text".to_string()),
                ..Default::default()
            },
            message.to_string().into_bytes(),
        ).unwrap();


        //self.terminate_session_and_channel();
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
    pub fn Consume(&mut self)
    {
        let queue_name: &str = "";
        warn!("Beginning consumption");
        self.declare_queue(&queue_name);

        self._channel.exchange_declare(
            "topics",
            "topic",
            true,
            false,
            false,
            false,
            false,
            Table::new(),
        ).unwrap();

        self._channel.queue_bind(
            queue_name,
            types::EXCHANGE_NAME,
            types::POWER_NOTICE,
            false,
            Table::new(),
        ).unwrap();

        /*self._channel.basic_consume(
            consumer_function,
            queue_name,
            "",
            false,
            false,
            false,
            false,
            Table::new(),
        ).unwrap();*/
        
        while expected
        {
            for get_result in self._channel.basic_get(queue_name, false)
            {
                println!("Received: {:?}", String::from_utf8_lossy(&get_result.body));
                get_result.ack();
            }
        }

        println!(
            "[{} Consumer ] Started.",
            queue_name,
        );
        //self._channel.start_consuming();

        //self.terminate_session_and_channel();
    }


}
