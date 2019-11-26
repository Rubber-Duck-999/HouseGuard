extern crate amqp;


use std::default::Default;
use amqp::{Options, Session, Channel};

pub struct SessionRabbitmq
{
    _durable: bool,
    _session: Session,
    _channel: Channel,
    _prefetch_count: u16,
    _init: bool,
}

fn getSession() -> Session
{
    let mut session = match Session::new(Options { .. Default::default() }){
         Ok(session) => session,
         Err(error) => panic!("Failed openning an amqp session: {:?}", error)
    };
    return session;
}

fn getChannel(mut session:Session) -> Channel
{
    let mut channel = session.open_channel(1).ok().expect("Can not open a channel");
    return channel;
}


impl Default for SessionRabbitmq
{
    fn default() -> SessionRabbitmq
    {
        let mut session:Session = getSession();
        let mut channel:Channel = getChannel(session);
        let mut session_new:Session = getSession();
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