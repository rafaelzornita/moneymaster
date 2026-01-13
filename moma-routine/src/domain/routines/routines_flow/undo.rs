use async_trait::async_trait;
use serde::{Serialize, Deserialize};
use crate::data;
use crate::domain::routines::base::routine_process_result::RoutineProcessResult;
use crate::domain::routines::base_routine::*;
use crate::domain::routines::routine_trait;
use crate::domain::RoutineRepo;
use crate::error::RoutineErrors;
use crate::routineselector;

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Undo {
    base: BaseRoutine,
    tenant_key: String,
    payload : String
}

#[async_trait]
impl routine_trait::Routine for Undo {
    fn get_name(&self) -> &String {
        &self.base.name
    }
    fn get_key(&self) -> &String {
        &self.base.key
    }
    fn get_lifetime(&self) -> &u16 {
        &self.base.lifetime
    }
    fn set_tenant_key(&mut self, tenant_key: String){
        self.tenant_key = tenant_key;
    }
    /// Cannot be selected by the user or AI
    fn is_internal_porpouse(&self) -> &bool {
        &false
    }
    fn can_undo(&self) -> &bool {
        &false
    }
    fn into_json(&self) -> String {
        serde_json::to_string(&self).map_or("".to_string(), |v| v.to_string())
    }
    fn from_json(&mut self, json: &String) {
        if json.is_empty() { return; }

        let restored: Undo = serde_json::from_str(json).unwrap();
        restored.clone_into(self);
    }

    async fn process_input(&mut self, payload : &String) -> Result<RoutineProcessResult, RoutineErrors> {
        //Save the user payload to be logged
        self.payload = payload.clone();
        self.process().await
    }

    async fn process(&mut self) -> Result<RoutineProcessResult, RoutineErrors> {
                
        //load last tenant operation
        let flow = RoutineRepo::get_last_tenant_flow(&self.tenant_key, &data::connectionfactory::get_connection().await?).await?;
        if let None = flow {
            return Ok(RoutineProcessResult::new().set_done()
            .add_message("Não há uma última operação concluida disponível para ser desfeita.").clone());
        }

        let flow = flow.unwrap();
        let mut routine = routineselector::raw_select(&flow.routine_key).await.unwrap();
        if !routine.can_undo(){
            return Ok(RoutineProcessResult::new().set_done()
            .add_message("A última operação não pode ser desfeita.").clone());
        }

        routine.from_json(&flow.routine_data);
        routine.undo().await
    }

    async fn undo(&mut self) -> Result<RoutineProcessResult, RoutineErrors>{
        Ok(RoutineProcessResult::new().set_done()
            .add_message("A operação não pode ser desfeita.").clone())
    }
}

impl Undo {
    pub fn new() -> Self {
        Undo {
           base : BaseRoutine {
               name : "Undo".to_owned(),
               description: "Desfaz a última operação realizada".to_owned(),
               key : "undo".to_owned(),
               lifetime : 0 //segundos
           },
           ..Default::default()
        }
    }    
}
