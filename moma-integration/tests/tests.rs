#[cfg(test)]
mod tests {
    use core::panic;
    use amqprs::{channel::{BasicAckArguments, Channel}, consumer::AsyncConsumer, BasicProperties, Deliver};
    use handlebars::Handlebars;
    use moma_integration::{ai::{self, types::AiPayload}, email::{self, ContactInfo}};
    use moma_shared::{resources, settings::AiConfig};
    use serde_json::json;
    

    #[tokio::test]
    async fn ai() {

        //use openai token
        let config = AiConfig {
            ai_auth_token : "".to_string(),
            ai_model_id: "gpt-4o-mini".to_string(),
        };

        let mut reg = Handlebars::new();
        reg.register_template_string("select_routine", moma_shared::resources::get_ai_resource(moma_shared::resources::AIResources::RoutineSelect)).unwrap();
        reg.register_template_string("entry_insert", moma_shared::resources::get_ai_resource(moma_shared::resources::AIResources::EntryInsert)).unwrap();

        let pv_data = reg.render("select_routine",&json!({})).unwrap();        
        
        let payload = AiPayload {
            input_message: "cem reais para o meu amor",
            system_message : &pv_data
        };
        let res : Option<Resp> = ai::gateway::send_simple(&config, &payload).await.expect("Error communicating AI. ");
        if let Some(res) = res {
            assert!(res.input_is_valid);
            assert_eq!(res.routine_key, "entry_insert");
        }        

        let pv_data = reg.render("entry_insert",&json!({"today": "26/07/2024", "categories": "1:Necessário; 2:Diversão; 3:Other; 4:Receitas; 5:Transporte"})).unwrap();
        let payload = AiPayload {
            input_message: "cem reais para o meu amor",
            system_message : &pv_data
        };
        let res : Option<Resp> = ai::gateway::send_simple(&config, &payload).await.expect("Error communicating AI. ").unwrap();
        if let Some(res) = res {
            println!("{} : {}", res.input_is_valid, res.routine_key)
        }
    }

    #[derive(serde::Deserialize)]
    pub struct Resp{
        pub input_is_valid: bool,
        pub routine_key: String
    }

    #[tokio::test]
    async fn mailing() {


        //Use valid address to test
        let to_address = "...@....com";
        let from_address = "...@....com";

        if email::validate_address(&to_address.to_owned()) == false{
            panic!("Email adress must be VALID");
        }

        if email::validate_address(&to_address.to_owned()) == true{
            panic!("Email adress must be Invalid");
        }

        let emailpayload = email::EmailData { 
            from: ContactInfo {
                email: from_address,
                name: "Money Master"
             },
             to: vec![ContactInfo{
                email: to_address,
                name: "Test"
             }],
            text: "",
            subject: "Money Master Test",
            html: resources::get_mail_resource(resources::MailResources::ConfirmationMail)
        };
        
        email::send_as_html(&moma_shared::settings::get_email_config(), &emailpayload).await.expect("Error sending email. ");

    }

    #[tokio::test]
    async fn amqp() {

        //Use Rabbit MQ Connection string
        let amqp_uri = "amqps://";
        let res = moma_integration::amqp::gateway_factory::get_new_gateway(&amqp_uri).await;
        if let Err(e) = res{
            panic!("{:?}",e);
        }
        let mut amqp = res.unwrap();

        amqp.register_consumer("amq.direct", "routing_key_test", "queue_test", ConsumerTest {}).await;
        amqp.publish("amq.direct", "routing_key_test", "Amqp test".to_owned()).await;

        //Wait receive the message
        tokio::time::sleep(tokio::time::Duration::from_secs(4)).await;    
    }

    struct ConsumerTest {}

    #[async_trait::async_trait]
impl AsyncConsumer for ConsumerTest {
    async fn consume(
        &mut self,
        channel: &Channel,
        deliver: Deliver,
        _basic_properties: BasicProperties,
        content: Vec<u8>,
    ) {
        println!("Delivered: {:?}", String::from_utf8(content).unwrap());
        _ = channel.basic_ack(BasicAckArguments::new(deliver.delivery_tag(), false)).await;
        
    }
}
}
