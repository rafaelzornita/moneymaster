use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AiPayload<'a>{    
    pub input_message: &'a str,
    pub system_message: &'a str,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AiRoledPayload<'a>{    
    pub message: &'a str,
    pub role: &'a str,
}