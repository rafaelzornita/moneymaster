use std::error::{self, Error};

#[derive(Debug, Clone)]
pub enum AuthErrors {
    DbError(String),
    ///Params: Msg, field
    UserInputError(String, String),
  }

#[derive(Debug, Clone, PartialEq)]
pub enum TenantValidationError {
    ///Params: Msg, field
    Unexists,
    Disabled,
    EmailUnconfirmed
  }

  impl error::Error for AuthErrors {
    fn description(&self) -> &str {
      match self {
        AuthErrors::DbError(msg) => &msg,
        AuthErrors::UserInputError(msg,_) => &msg
      }
    }

}
impl From<sea_orm::DbErr> for AuthErrors {
  fn from(_t: sea_orm::DbErr) -> Self {
    AuthErrors::DbError(_t.to_string())
  }
}
impl From<Box<dyn Error + 'static>> for AuthErrors {
  fn from(_t: Box<dyn Error + 'static>) -> Self {
    AuthErrors::DbError(_t.to_string())
  }
}


impl std::fmt::Display for AuthErrors {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::DbError(e) => write!(f, "DBError: {e}"),
      Self::UserInputError(m,field ) =>  write!(f, "UserInputError: {m} Field: {field}"),
    }
  }
}
  
impl std::fmt::Display for TenantValidationError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Unexists => write!(f, "Unexists"),
      Self::Disabled => write!(f, "Disabled"),
      Self::EmailUnconfirmed => write!(f, "EmailUnconfirmed")
    }
  }
}