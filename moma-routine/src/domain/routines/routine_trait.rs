use async_trait::async_trait;
use crate::error::RoutineErrors;

use super::base::routine_process_result::RoutineProcessResult;

#[async_trait]
pub trait Routine{
    //Getters
    fn get_name(&self) -> &String;
    fn get_key(&self) -> &String;
    fn get_lifetime(&self) -> &u16;
    // Setters
    fn set_tenant_key(&mut self, tenant_key: String);
    /// Cannot be selected by the user or AI
    fn is_internal_porpouse(&self) -> &bool;
    ///It can be undo
    fn can_undo(&self) -> &bool;
    fn into_json(&self) -> String;
    fn from_json(&mut self, json: &String);
    //Process natural language input
    async fn process_input(&mut self, payload: &String) -> Result<RoutineProcessResult, RoutineErrors>;
    //Process loaded data
    async fn process(&mut self) -> Result<RoutineProcessResult, RoutineErrors>;
    //Undo processed data
    async fn undo(&mut self) -> Result<RoutineProcessResult, RoutineErrors>;
}