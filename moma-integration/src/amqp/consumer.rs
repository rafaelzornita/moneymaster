use amqprs::{channel::{BasicAckArguments, BasicNackArguments, Channel}, consumer::AsyncConsumer, BasicProperties, Deliver};
use super::consumer_trait::AmqpConsume;

pub struct AmqpConsumer {
    consumer : Box<dyn AmqpConsume + Send + 'static>,
}

pub fn new(y : Box<dyn AmqpConsume + Send + 'static>) -> AmqpConsumer {
    AmqpConsumer {consumer : y}
}

#[async_trait::async_trait]
impl AsyncConsumer for AmqpConsumer {
    async fn consume(
        &mut self,
        channel: &Channel,
        deliver: Deliver,
        _basic_properties: BasicProperties,
        content: Vec<u8>,
    ) {

        match self.consumer.consume(String::from_utf8(content).unwrap()).await {
            Ok(_) => _ = channel.basic_ack(BasicAckArguments::new(deliver.delivery_tag(), false)).await,
            Err(_) => _ = channel.basic_nack(BasicNackArguments::new(deliver.delivery_tag(), false, false)).await
        }

    }
}