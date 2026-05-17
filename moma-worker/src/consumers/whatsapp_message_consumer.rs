use moma_integration::{amqp::consumer_trait::AmqpConsume, whatsapp};
use moma_routine::{domain::routines::base::routine_process_result::RoutineProcessResult, routineflowcontrol};
use moma_shared::messages::whatsapp_message::WppWhatsAppMessage;

pub struct WhatsAppQueueConsumer {}

#[async_trait::async_trait]
impl AmqpConsume for WhatsAppQueueConsumer {
    async fn consume(&self, content : String) -> Result<(), ()>
    {
        let message_obj: WppWhatsAppMessage = serde_json::from_str(&content).unwrap();
        
        let whatsapp_from = String::from(message_obj.from.unwrap());
        println!("{}",&whatsapp_from);

        if let Some(message) = message_obj.content {
            let res = routineflowcontrol::submit(&whatsapp_from, &message).await;
            match res {
                Ok(o) => send_whatsapp_answare(&whatsapp_from, o).await,
                Err(e) => println!("{}", e)
            }
        }
        else {
            println!("No content in message")
        }       

        Ok(())
    }
}

async fn send_whatsapp_answare(whatsapp_to : &str, process_result : RoutineProcessResult) {

    //todo: AMQP Broker to send answare through whatsapp
    println!("Answare message: {}", process_result.get_user_messages());
    match whatsapp::gateway::send_text(whatsapp_to, &process_result.get_user_messages()).await {
        Err(e) => println!("WhatsApp Error: {}",e.to_string()),
        Ok(()) => println!("WhatsApp OK;")
    } 

}

