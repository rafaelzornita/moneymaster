use async_trait::async_trait;

#[async_trait]
pub trait AmqpConsume{
    async fn consume(&self, content : String) -> Result<(), ()>;
}