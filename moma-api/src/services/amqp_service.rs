use std::sync::{Arc, Mutex};
use coi::{Inject, Provide};
use moma_integration::amqp::gateway::AmqpGateway;
use serde::Serialize;

//pub trait IAmqpService : Inject {}

#[derive(Inject)]
pub struct AmqpService {
    pub gateway: Arc<Mutex<AmqpGateway>>
}

impl AmqpService {
    pub fn new(_gateway : Arc<Mutex<AmqpGateway>>) -> Self { //
        AmqpService { 
            gateway : _gateway 
        }
    }
    pub async fn publish_obj<T>(&self, exchange_name : &String, routing_key : &String,  obj : &T) where T: Sized + Serialize {
        let mut gateway = self.gateway.lock().unwrap();
        gateway.publish_obj(exchange_name, routing_key, &obj).await;
    }
}

#[derive(Provide)]
#[coi(provides AmqpService with AmqpService::new(self.0.clone()))]
pub struct AmqpServiceProvider(Arc<Mutex<AmqpGateway>>);

impl  AmqpServiceProvider {
    pub fn new(gateway: Arc<Mutex<AmqpGateway>>) -> Self {
        Self(gateway)
    }
}
