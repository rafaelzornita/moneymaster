use isahc::{http::Request, HttpClient, RequestExt, ResponseExt};
use moma_shared::settings::{get_whatsapp_wpp, WhatsAppWppConfig};
use serde::Deserialize;

use crate::error::IntegrationErrors;

#[derive(Deserialize)]
struct SendMessageResponse {
    status: String,
}

#[derive(Deserialize)]
struct TokenResponse {
    status: String,
    token: String,
}

pub async fn send_text(destination: &str, message: &str) -> Result<(), IntegrationErrors> {

    let config = get_whatsapp_wpp();
    let url = format!(
        "{}/api/{}/send-message",
        config.whatsapp_wpp_api_url, config.whatsapp_wpp_session
    );

    let payload = serde_json::json!({
        "phone": destination,
        "message": &message,
        "isLid": destination.ends_with("@lid"),
    });
    
    let mut response = 
            Request::post(&url)
            .header("Authorization", format!("Bearer {}", generate_bearer_token(&config).await?))
            .header("Content-Type", "application/json")
            .body(serde_json::to_string(&payload).unwrap())
            .map_err(|e| IntegrationErrors::Http(e.to_string()))?
            .send_async().await
            .map_err(|e| IntegrationErrors::Http(e.to_string()))?;

    if !response.status().is_success() {
        return Err(IntegrationErrors::Http(format!(
            "Failed to send message: HTTP {}",
            response.status()
        )));
    }

    let result: SendMessageResponse = response
    .json()
    .map_err(|e| IntegrationErrors::Http(e.to_string()))?;

    if result.status == "success" {
        Ok(())
    } else {
        Err(IntegrationErrors::WhatsApp(format!(
            "API returned error: {}",
            result.status
        )))
    }
}

async fn generate_bearer_token(config: &WhatsAppWppConfig) -> Result<String, IntegrationErrors> {

    let url = format!(
        "{}/api/{}/{}/generate-token",
        config.whatsapp_wpp_api_url, config.whatsapp_wpp_session, config.whatsapp_wpp_token
    );
    println!("{url}");

    let client = HttpClient::new();
    let mut response = client
        .post_async(&url,serde_json::to_string("").unwrap())
        .await
        .map_err(|e| IntegrationErrors::Http(e.to_string()))?;

    if !response.status().is_success() {
        return Err(IntegrationErrors::Http(format!(
            "Failed to generate token: HTTP {}",
            response.status()
        )));
    }

    let result: TokenResponse = response
        .json()
        .map_err(|e| IntegrationErrors::WhatsApp(e.to_string()))?;

    if result.status == "success" {
        Ok(result.token)
    } else {
        Err(IntegrationErrors::WhatsApp(format!(
            "Failed to generate token: {}",
            result.status
        )))
    }
}