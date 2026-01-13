use crate::error::IntegrationErrors;

use super::gateway::AmqpGateway;

pub async fn get_new_gateway(amqp_uri: &str) -> Result<AmqpGateway, IntegrationErrors> {

    // let args: OpenConnectionArguments = amqp_uri.try_into().unwrap();
    
    // let connection = Connection::open(&args).await?;
    // connection.register_callback(DefaultConnectionCallback).await?;

    // //base channel exclusive to set configurations
    // let channel = connection.open_channel(None).await?;
    // channel.register_callback(DefaultChannelCallback).await?;
    
    return Ok(AmqpGateway::new(amqp_uri));

}