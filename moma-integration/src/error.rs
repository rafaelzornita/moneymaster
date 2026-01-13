use std::error::{self, Error};
use lettre::transport::smtp;
use openai_dive::v1::error::APIError;

#[derive(Debug, Clone)]
pub enum IntegrationErrors {
    ///Params: Message
    MailError(String),
    Amqp(String),
    Http(String),
    ///Params: Message, title
    Generic(String, String),
    WhatsApp(String)
  }


impl error::Error for IntegrationErrors {
  fn description(&self) -> &str {
    match self {
      Self::MailError(msg) => &msg,
      Self::Amqp(msg) => &msg,
      Self::Http(msg) => &msg,
      Self::Generic(msg,t) => {t.to_owned().push_str("->"); t.to_owned().push_str(&msg); &t}
      Self::WhatsApp(msg) => &msg,
    }
  }
}

impl From<serde_json::Error> for IntegrationErrors {
  fn from(_t: serde_json::Error) -> Self {
    Self::Generic(_t.to_string(), "SERDE_JSON".to_owned())
  }
}
impl From<std::io::Error> for IntegrationErrors {
  fn from(e: std::io::Error) -> Self {
    Self::MailError(e.to_string())
    }
}

impl From<smtp::Error> for IntegrationErrors {
  fn from(_t: smtp::Error) -> Self {
    Self::MailError(_t.to_string())
  }
}
impl From<Box<dyn Error + 'static>> for IntegrationErrors {
  fn from(_t: Box<dyn Error + 'static>) -> Self {
    Self::MailError(_t.to_string())
  }
}
impl From<amqprs::error::Error> for IntegrationErrors {
  fn from(_t: amqprs::error::Error) -> Self {
    Self::MailError(_t.to_string())
  }
}

impl From<isahc::Error> for IntegrationErrors {
  fn from(_t: isahc::Error) -> Self {
    Self::Http(_t.to_string())
  }
}

impl From<APIError> for IntegrationErrors {
  fn from(_t: APIError) -> Self {
    Self::Http(_t.to_string())
  }
}

impl std::fmt::Display for IntegrationErrors {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::MailError(e) => write!(f, "MailError: {e}"),
      Self::Amqp(e) => write!(f, "Amqp: {e}"),
      Self::Http(e) => write!(f, "Http: {e}"),
      Self::Generic(msg,t) => write!(f, "{t}: {msg}"),
      Self::WhatsApp(e) => write!(f, "WhatsApp: {e}"),
    }
  }
}
  