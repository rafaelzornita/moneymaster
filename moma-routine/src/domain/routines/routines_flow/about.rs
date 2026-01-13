use async_trait::async_trait;
use moma_integration::ai::types::AiPayload;
use moma_shared::resources::get_ai_resource;
use moma_shared::resources::AIResources;
use serde::{Serialize, Deserialize};
use crate::domain::routines::base::routine_process_result::RoutineProcessResult;
use crate::domain::routines::base_routine::*;
use crate::domain::routines::routine_trait;
use crate::error::RoutineErrors;

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct About {
    base: BaseRoutine,
    data: AboutData,
    current_step: u8
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
struct AboutData {
    tenant_key: String,
    user_input : String,
    ai_answare : String
}


#[derive(Deserialize)]
struct AiResponse{
    pub answare: String,
}

#[async_trait]
impl routine_trait::Routine for About {
    fn get_name(&self) -> &String {
        &self.base.name
    }
    fn get_key(&self) -> &String {
        &self.base.key
    }
    fn get_lifetime(&self) -> &u16 {
        &self.base.lifetime
    }
    fn into_json(&self) -> String {
        serde_json::to_string(&self).map_or("".to_string(), |v| v.to_string())
    }
    fn from_json(&mut self, json: &String) {
        if json.is_empty() { return; }

        let x: Self = serde_json::from_str(json).unwrap();
        x.clone_into(self);
    }
    fn set_tenant_key(&mut self, tenant_key: String){
        self.data.tenant_key = tenant_key;
    }
    /// Cannot be selected by the user
    fn is_internal_porpouse(&self) -> &bool {
        &false
    }
    async fn process_input(&mut self, payload: &String) -> Result<RoutineProcessResult, RoutineErrors> {
        
        self.data.user_input = payload.clone();
        
        self.process().await
    }

    async fn process(&mut self) -> Result<RoutineProcessResult, RoutineErrors> {

        let response = ai_completion(&self.data.user_input).await?;
        if let Some(response) = response {
            return Ok(RoutineProcessResult::new()
                .add_message(&response.answare)
                .set_done()
                .to_owned())
        }
        
        Ok(RoutineProcessResult::new()
            .add_message("Não foi possível processar sua mensagem, aguarde instantes e tente novamente.")
            .set_done()
            .to_owned())
        
    }

    fn can_undo(&self) -> &bool {
        &false
    }

    async fn undo(&mut self) -> Result<RoutineProcessResult, RoutineErrors>{
        Ok(RoutineProcessResult::new().set_done().to_owned())
    }
}

async fn ai_completion(user_input : &String) -> Result<Option<AiResponse>, RoutineErrors> {

    let payload = AiPayload {
        input_message: user_input,
        system_message : get_ai_resource(AIResources::About)
    };
    
    let mut response : Option<AiResponse> = None;
    for tries in 1..3{        
        response = moma_integration::ai::gateway::send_simple(&moma_shared::settings::get_aiconfig(), &payload).await?;
        if response.is_some() {
            break;
        } else {
            if tries > 1 {
                return Ok(None);
            }
        }        
    }
    
    Ok(response)
}

impl About {
    pub fn new() -> Self {
        About {
           base : BaseRoutine {
               name : "About".to_owned(),
               description: "Help guide".to_owned(),
               key : "about".to_owned(),
               lifetime : 1 * 60 //to seconds
           },
           current_step : 1,
           ..Default::default()
        }
    }
}