use std::env;

use message_broker_service::MessageBrokerService;
use tokio::sync::Notify;

pub mod message_broker_service;
pub mod consumers;
pub mod schedule_service;
pub mod services;

#[tokio::main]
async fn main() {
    println!("{}", env::current_dir().unwrap().to_str().unwrap().to_owned());

    //Initialize the config file component
    _ = dotenv::dotenv().unwrap();
    
    //This var keeps the message broker service active
    let mb_service = match MessageBrokerService::initialize().await {
       Ok(obj) => obj,
       Err(e) => panic!("moma-worker can't be loaded. Messagebroker initializer error: {:?}", e.to_string())
    }; 
    
    let sched_service = schedule_service::initialize().await;
    if let Err(e) = &sched_service {
        println!("moma-worker can't be loaded. Scheduler initializer error: {:?}", e.to_string())
    };
    let mut sched_service = sched_service.unwrap();

    println!("Consume forever... Ctrl+C to exit");
    let guard = Notify::new();
    guard.notified().await;
    
    mb_service.stop().await;
    sched_service.stop().await;
}

