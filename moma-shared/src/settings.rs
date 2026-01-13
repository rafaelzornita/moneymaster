#[derive(Debug, Clone)]
pub struct Email {
    pub auth_token: String,
    pub mailsender_api_url: String,
    pub from_address: String
}

pub fn get_email_config() -> Email {
    _ = dotenv::dotenv();

    Email {
        auth_token : dotenv::var("EMAIL_AUTH_TOKEN").unwrap().parse().unwrap(),
        mailsender_api_url : dotenv::var("EMAIL_MAILSENDER_API_URL").unwrap().parse().unwrap(),
        from_address : dotenv::var("EMAIL_FROM_ADDRESS").unwrap().parse().unwrap(),
    }
}


pub struct AiConfig {
    pub ai_auth_token: String,
    pub ai_model_id: String,
}

pub fn get_aiconfig() -> AiConfig {
    AiConfig {
        ai_auth_token : dotenv::var("AI_AUTH_TOKEN").unwrap().parse().unwrap(),
        ai_model_id : dotenv::var("AI_MODEL_ID").unwrap().parse().unwrap(),
    }
}

pub struct WhatsAppWppConfig {
    pub whatsapp_wpp_token: String,
    pub whatsapp_wpp_api_url: String,
    pub whatsapp_wpp_session: String
}

pub fn get_whatsapp_wpp() -> WhatsAppWppConfig {
    WhatsAppWppConfig {
        whatsapp_wpp_token : dotenv::var("WHATSAPP_WPP_TOKEN").unwrap().parse().unwrap(),
        whatsapp_wpp_api_url : dotenv::var("WHATSAPP_WPP_API_URL").unwrap().parse().unwrap(),
        whatsapp_wpp_session : dotenv::var("WHATSAPP_WPP_SESSION").unwrap().parse().unwrap(),
    }
}

pub struct AmqpSettings {
    pub amqp_uri: String,
    pub amqp_exchange_name: String,
    pub amqp_whatsapp_routing_key: String,
    pub amqp_whatsapp_queue_name: String
}

pub fn get_amqp() -> AmqpSettings {
    AmqpSettings {
        amqp_uri: dotenv::var("AMQP_URI").unwrap().parse().unwrap(),
        amqp_exchange_name: dotenv::var("AMQP_EXCHANGE_NAME").unwrap().parse().unwrap(),
        amqp_whatsapp_routing_key: dotenv::var("AMQP_WHATSAPP_QUEUE_ROUTING_KEY").unwrap().parse().unwrap(),
        amqp_whatsapp_queue_name: dotenv::var("AMQP_WHATSAPP_QUEUE_NAME").unwrap().parse().unwrap()
    }
}