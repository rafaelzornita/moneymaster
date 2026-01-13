use std::time::Duration;

use amqprs::{callbacks::{DefaultChannelCallback, DefaultConnectionCallback}, channel::{BasicConsumeArguments, BasicPublishArguments, Channel, QueueBindArguments, QueueDeclareArguments}, connection::{Connection, OpenConnectionArguments}, consumer::AsyncConsumer, BasicProperties};
use serde::Serialize;
use tokio::time::sleep;

#[derive(Clone)]
pub struct  AmqpGateway {
    base_connection: Option<Connection>,
    base_channels: Vec<Channel>, //one per thread, so, one per consumer
    amqp_uri: String,
}

impl AmqpGateway {

    pub fn new(amqp_uri: &str) -> Self {
        AmqpGateway {
            base_connection: None, 
            base_channels : Vec::new(),
            amqp_uri: amqp_uri.to_string(),
        }
    }
    pub fn get_connection(&self) -> &Connection {
        self.base_connection.as_ref().unwrap()
    }
    pub async fn ensure_connection(&mut self) -> bool {
        let is_connected = self.base_connection.is_some() && self.get_connection().is_open();

        if !is_connected {
            println!("Trying connect AMQP.");
            self.reconnect().await;
        }
        //Return if was needed to reconnect
        !is_connected
    }

    async fn connect(amqp_uri: &str) -> Result<(Connection, Channel), String> {
        let mut args: OpenConnectionArguments = amqp_uri.try_into().unwrap();
        args.heartbeat(30);
        match Connection::open(&args).await {
            Ok(connection) => {
                connection.register_callback(DefaultConnectionCallback).await.unwrap();
                let channel = connection.open_channel(None).await.unwrap();
                channel.register_callback(DefaultChannelCallback).await.unwrap();
                Ok((connection, channel))
            }
            Err(e) => {
                println!("Erro ao conectar: {}", e);
                Err(e.to_string())
            }
        }
    }
    
    async fn reconnect(&mut self) {
        let mut attempts = 0;
        while attempts < 20_u32 {
            
            attempts += 1;
            println!("Connection attempt #{}", attempts);

            match Self::connect(&self.amqp_uri).await {
                Ok((new_connection, new_channel)) => {
                    self.base_connection = Some(new_connection);
                    self.base_channels.clear();
                    self.base_channels.push(new_channel);

                    println!("Connection sucessfull!");
                    return;
                }
                Err(e) => {
                    println!("Connection failed: {}", e);
                    let wait_time = Duration::from_secs(2_u64.pow(attempts.min(5))); // Exponential backoff
                    sleep(wait_time).await;
                }
            }
        }
    }

    pub async fn publish(&mut self, exchange_name : &str, routing_key: &str, content: String) {
        self.ensure_connection().await;

        let args = BasicPublishArguments::new(exchange_name, routing_key);
        _ = self.base_channels.first().unwrap().basic_publish(BasicProperties::default(), content.into_bytes(), args).await
                .map_err(|e| println!("{}", e.to_string()));
        //todo log err
    }
    pub async fn publish_obj<T>(&mut self, exchange_name : &str, routing_key: &str, obj: T) where T: Sized + Serialize, {
        let content = serde_json::to_string(&obj).unwrap_or_else(|_| "".to_string());
        let _ = &self.publish(exchange_name, routing_key, content).await;
    }
    
    pub async fn register_consumer<F>(&mut self, exchange_name : &str, routing_key: &str, queue_name: &str, consumer : F)
    where F: AsyncConsumer + Send + 'static {

        self.ensure_connection().await;
        self.register_queue(exchange_name, routing_key, queue_name).await;

        let args = BasicConsumeArguments::new(&queue_name, "").manual_ack(true).finish();
        
        let channel = self.get_connection().open_channel(None).await.unwrap();
        channel.register_callback(DefaultChannelCallback).await.unwrap();

        //set the consumer
        channel.basic_consume(consumer, args).await
                .unwrap();
        
        //channel keeps the consumer alive, so we need to store it
        self.base_channels.push(channel);
    }

    async fn register_queue(&self, exchange_name : &str, routing_key: &str, queue_name: &str) {
        // declare a queue
        _ = self.base_channels.first().unwrap().
                queue_declare(QueueDeclareArguments::durable_client_named(&queue_name))
                .await;

        // bind the queue to exchange
        self.base_channels.first().unwrap()
        .queue_bind(QueueBindArguments::new(&queue_name,&exchange_name,&routing_key,)).await
        .unwrap();
    }

    pub async fn dispose(self) {
        if let Some(connection) = self.base_connection {
            _ = connection.close().await;
        }
    }
}