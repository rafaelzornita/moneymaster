use moma_integration::{amqp::consumer_trait::AmqpConsume, whatsapp};
use moma_routine::{domain::routines::base::routine_process_result::RoutineProcessResult, routineflowcontrol};
use moma_shared::messages::whatsapp_message::WppWhatsAppMessage;

pub struct WhatsAppQueueConsumer {}

#[async_trait::async_trait]
impl AmqpConsume for WhatsAppQueueConsumer {
    async fn consume(&self, content : String) -> Result<(), ()>
    {
        let message_obg: WppWhatsAppMessage = serde_json::from_str(&content).unwrap();     
        
        let phone_number = String::from(message_obg.from.unwrap().split("@").next().unwrap());

        if let Some(message) = message_obg.content {
            let res = routineflowcontrol::submit(&phone_number, &message).await;
            match res {
                Ok(o) => send_whatsapp_answare(&phone_number, o).await,
                Err(e) => println!("{}", e)
            }
        }
        else {
            println!("No content in message")
        }       

        Ok(())
    }
}

async fn send_whatsapp_answare(phone_number : &str, process_result : RoutineProcessResult) {

    //todo: AMQP Broker to send answare through whatsapp
    println!("Answare message: {}", process_result.get_user_messages());
    match whatsapp::gateway::send_text(phone_number, &process_result.get_user_messages()).await {
        Err(e) => println!("WhatsApp Error: {}",e.to_string()),
        Ok(()) => println!("WhatsApp OK;")
    } 

}

