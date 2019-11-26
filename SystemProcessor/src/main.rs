mod system;
mod rabbitmq;

use clap::{
    App,
    Arg,
};

use std::{process,};

fn main() 
{
    let matches = App::new("rust-rabbitmq-example")
        .version("0.0.1")
        .about("Simulate a RabbitMQ environment with consumer(s) and producer(s).")
        .arg(Arg::with_name("startup-script")
             .short("s")
             .long("startup-script")
             .help("Config file required by Component for startup settings")
             .takes_value(true)
        )
        .get_matches();

    let startup_script: usize = matches.value_of("startup-script")
        .unwrap_or("1")
        .parse()
        .unwrap();
    
    
    const COMPONENT_NAME:&str = "SYP";
    let _value:u32 = 32;
    println!("Initialising System Processor Component = {}", COMPONENT_NAME);
    system::processes::ps();
    
    process::exit(1);
    
}
