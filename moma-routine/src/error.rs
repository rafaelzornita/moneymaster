use std::error::{self, Error};
use moma_auth::error::AuthErrors;
use moma_integration::error::IntegrationErrors;

#[derive(Debug, Clone)]
pub enum RoutineErrors {
    DbError(String),
    ///Params: Msg, field
    UserInputError(String, String),
    ///Params: Message, IntegrationName
    IntegrationErrors(String, String),
    ///Params: Message, Type
    AuthErros(String, String),
    ///Params: Message, Type
    Generic(String, String)
  }

  impl error::Error for RoutineErrors {
    fn description(&self) -> &str {
      match self {
        Self::DbError(msg) => &msg,
        Self::UserInputError(msg,_) => &msg,
        Self::IntegrationErrors(msg,integration_name) => {integration_name.to_owned().push_str("->"); integration_name.to_owned().push_str(&msg); &integration_name},
        Self::AuthErros(msg,t) => {t.to_owned().push_str("->"); t.to_owned().push_str(&msg); &t},
        Self::Generic(msg,t) => {t.to_owned().push_str("->"); t.to_owned().push_str(&msg); &t}
      }
    }

}
impl From<sea_orm::DbErr> for RoutineErrors {
  fn from(_t: sea_orm::DbErr) -> Self {
    Self::DbError(_t.to_string())
  }
}
impl From<Box<dyn Error + 'static>> for RoutineErrors {
  fn from(_t: Box<dyn Error + 'static>) -> Self {
    Self::DbError(_t.to_string())
  }
}
impl From<serde_json::Error> for RoutineErrors {
  fn from(_t: serde_json::Error) -> Self {
    Self::Generic(_t.to_string(), "SERDE_JSON".to_owned())
  }
}

impl From<IntegrationErrors> for RoutineErrors {
  fn from(_t: IntegrationErrors) -> Self {
    match _t {
      IntegrationErrors::Amqp(msg) => Self::IntegrationErrors(msg, "Amqp".to_string()),
      IntegrationErrors::MailError(msg) => Self::IntegrationErrors(msg, "Mail".to_string()),
      IntegrationErrors::Http(msg) => Self::IntegrationErrors(msg, "Http".to_string()),
      IntegrationErrors::Generic(msg, title) => Self::Generic(msg,title),
      IntegrationErrors::WhatsApp(msg) => Self::IntegrationErrors(msg, "WhatsApp".to_string()),
    }
    
  }
}

impl From<AuthErrors> for RoutineErrors {
  fn from(_t: AuthErrors) -> Self {
    match _t {
      AuthErrors::DbError(msg) => RoutineErrors::AuthErros(msg, "Amqp".to_string()),
      AuthErrors::UserInputError(_, _) => RoutineErrors::AuthErros("UserInput".to_string(), "UserInput".to_string()),
    }    
  }
}

impl std::fmt::Display for RoutineErrors {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::DbError(e) => write!(f, "DBError: {e}"),
      Self::UserInputError(m,field ) =>  write!(f, "UserInputError: {m} Field: {field}"),
      Self::IntegrationErrors(msg,integration_name) => write!(f, "{integration_name}: {msg}"),
      Self::AuthErros(msg,integration_name) => write!(f, "{integration_name}: {msg}"),
      Self::Generic(msg,t) => write!(f, "{t}: {msg}"),
    }
  }
}