use std::sync::Arc;

use moma_integration::{amqp::{consumer::{self}, gateway::AmqpGateway}, error::IntegrationErrors};
use moma_shared::settings::AmqpSettings;
use tokio::sync::{watch, Mutex};

use crate::consumers::whatsapp_message_consumer::WhatsAppQueueConsumer;

pub struct MessageBrokerService {
    stop_signal: Arc<Mutex<watch::Sender<()>>>
}

impl MessageBrokerService {

    pub async fn initialize() -> Result<Self, IntegrationErrors> {

        let amqp_settings = moma_shared::settings::get_amqp();
        let amqp_gateway = moma_integration::amqp::gateway_factory::get_new_gateway(&amqp_settings.amqp_uri).await?;
        //self.amqp = amqp_gateway;

        async fn create_whatsapp_consumer(amqp_gateway: &mut AmqpGateway, amqp_settings: &AmqpSettings) -> Result<(), IntegrationErrors> {
            amqp_gateway.register_consumer(&amqp_settings.amqp_exchange_name, &amqp_settings.amqp_whatsapp_routing_key, &amqp_settings.amqp_whatsapp_queue_name, 
                consumer::new(Box::new(WhatsAppQueueConsumer {}))
            ).await;

            Ok(())
        }
        
        let (tx, mut rx) = watch::channel(());
        let stop_signal = Arc::new(Mutex::new(tx));
        tokio::spawn(async move {
            let amqp_settings = amqp_settings;
            let mut amqp_gateway = amqp_gateway;
            loop {
                tokio::select! {
                    _ = rx.changed() => {//Future sequence
                        println!("Stop signal received. Exiting...");
                        break;
                    }
                    _ = async {
                        if amqp_gateway.ensure_connection().await {
                            create_whatsapp_consumer(&mut amqp_gateway, &amqp_settings).await.unwrap();
                            create_whatsapp_consumer(&mut amqp_gateway, &amqp_settings).await.unwrap();
                            create_whatsapp_consumer(&mut amqp_gateway, &amqp_settings).await.unwrap();
                        }                        
                    } => {//Future sequence
                        tokio::time::sleep(tokio::time::Duration::from_secs(30)).await;
                    }      
                }       
            }
        });

        Ok(MessageBrokerService {
            stop_signal
        })
    }

    pub async fn stop(self) {
        _ = self.stop_signal.lock().await.send(());
    }
}