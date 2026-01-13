use moma_shared::settings::AiConfig;
use openai_dive::v1::api::Client;
use openai_dive::v1::resources::chat::{ChatCompletionParametersBuilder, ChatCompletionResponseFormat, ChatMessage, ChatMessageContent};
use serde::de::DeserializeOwned;
use super::super::error::IntegrationErrors;
use super::types::{AiPayload,AiRoledPayload};

pub async fn send_simple<'a, TResponseType>(ai_config: &AiConfig, payload: &AiPayload<'a>) -> Result<Option<TResponseType>, IntegrationErrors>
where TResponseType : DeserializeOwned {
    send(ai_config, &vec![AiRoledPayload {role: "system", message : payload.system_message}, 
        AiRoledPayload {role: "user", message : &payload.input_message}]).await
}

pub async fn send<'a, TResponseType>(ai_config: &AiConfig, payload: &Vec<AiRoledPayload<'a>>) -> Result<Option<TResponseType>, IntegrationErrors>
where TResponseType : DeserializeOwned {
    //TODO: Create client with prompt cache header 

    let client = Client::new(ai_config.ai_auth_token.clone());
    let mut messages = vec![];
    for message in payload {
        messages.push(
            match message.role {
                "system" => ChatMessage::System { name : None, content: ChatMessageContent::Text(message.message.to_string()) },
                "user" => ChatMessage::User { name : None, content: ChatMessageContent::Text(message.message.to_string()) },
                "assistant" => ChatMessage::Assistant { name : None, content: Some(ChatMessageContent::Text(message.message.to_string())), refusal: None, tool_calls: None, reasoning_content: None, audio: None },
                _ => return Result::Err(IntegrationErrors::Http("Invalid role".to_string()))
            });
        
    }    
    //let sx= Gpt4Engine::Gpt4O.to_string();
    let parameters = ChatCompletionParametersBuilder::default()
        .model(&ai_config.ai_model_id)
        .messages(messages)
        .response_format(ChatCompletionResponseFormat::Text)
        .build();
    
    let res = client
        .chat()
        .create(parameters.unwrap())
        .await?;

    if let Some(choice) = res.choices.get(0) {
        if let ChatMessage::Assistant { content: user_message, .. } = &choice.message {            
            if let Some(ChatMessageContent::Text(message)) = user_message {
                println!("Tokens {} :{:?}", res.usage.unwrap().total_tokens, message);
                let result = serde_json::from_str(&message.trim_start_matches('`')
                                                                                .trim_start_matches("json")
                                                                                .trim_start_matches("\n")
                                                                                .trim_end_matches('`'))?;
                return Ok(result);
            } else {
                return Ok(None);
            }
        }else {
            return Ok(None);
        }
    }else {
        return Ok(None);
    }
}