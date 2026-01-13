use std::str::FromStr;

use isahc::http::Request;
use isahc::RequestExt;
use lettre::Address;
use regex::Regex;
use lettre::{transport::smtp::authentication::Credentials, Message, SmtpTransport, Transport};
use lettre::message::{Mailbox, MultiPart, SinglePart};
use serde::Serialize;

use crate::error::IntegrationErrors;

pub fn validate_address(address : &String) -> bool {
    let email_regex = Regex::new(r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})").unwrap();
    email_regex.is_match(&address)
}

pub async fn send_as_html<'a>(server_config: &moma_shared::settings::Email, emailpayload : &EmailData<'a>) ->  Result<(), IntegrationErrors>
{   
    let mut api_authentication = String::from("Bearer ");
    api_authentication.push_str(&server_config.auth_token);

    let response = Request::post(server_config.mailsender_api_url.to_owned())
        .header("Authorization", api_authentication)
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&emailpayload).unwrap()).unwrap()
        .send_async().await;

    println!("{}",serde_json::to_string(&emailpayload).unwrap());
    print!("{}", response.unwrap().body_mut().text_async().await.unwrap());

    Ok(())
}

pub fn send_smtp_as_html<A : Into<SendMailServerConfig>>(server_config: A, emailpayload : SendMailPayload) ->  Result<(), IntegrationErrors>
 {
    let config : SendMailServerConfig = server_config.into();

    let x = emailpayload.from;
    let from_email = Mailbox  { name: None, email: Address::from_str(&x).unwrap() }; // emailpayload.from.parse::<Mailbox>().unwrap();
    let to_email = emailpayload.to.parse::<Mailbox>().unwrap();

    // Define the email with HTML part
    let email = Message::builder()
        .from(from_email)
        .to(to_email)
        .subject(emailpayload.subject)
        .multipart(
            MultiPart::alternative().singlepart(SinglePart::html(emailpayload.message)),
        )
        .unwrap();

    // Set up the SMTP client credentials
    let creds = Credentials::new(config.server_user, config.server_password);

    // Open a remote connection to the SMTP server with STARTTLS
    let mailer = if config.secure_connection { SmtpTransport::starttls_relay(&config.server_address) } 
                                else                         { SmtpTransport::relay(&config.server_address)}
        .unwrap()
        .credentials(creds)
        .build();

    // Send the email
    mailer.send(&email)
        .map_err(|op| op.into())
        .map(|_| ())
    
}

pub struct SendMailServerConfig {
    pub server_address: String,
    pub server_user: String,
    pub server_password: String,
    pub secure_connection: bool,
}
pub struct SendMailPayload {
    pub from: String,
    pub subject: String, 
    pub to: String, 
    pub message : String
}

#[derive(Debug, Serialize)]
pub struct EmailData<'a> {
    pub from: ContactInfo<'a>,
    pub to: Vec<ContactInfo<'a>>,
    pub subject: &'a str,
    pub text: &'a str,
    pub html: &'a str
}

#[derive(Debug, Serialize)]
pub struct ContactInfo<'a> {
    pub email: &'a str,
    pub name: &'a str,
}