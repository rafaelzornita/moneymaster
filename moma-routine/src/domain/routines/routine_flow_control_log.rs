use sea_orm::{entity::prelude::*, Set};
use crate::domain::routines::base::routine_process_result::RoutineStatus;
use crate::domain::RFC;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Default)]
#[sea_orm(table_name = "RoutineFlowControlLog")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: u32,
    pub tenant_key: String,
    pub routine_key: String,
    pub routine_data: String,
    pub created_on: DateTime,
    pub last_iteration_on: DateTime,
    pub last_result: String,
}


#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
}

impl ActiveModelBehavior for ActiveModel {}

impl Model {
    //Create an active model from a Model. Indicates for new models created instances otherwise
    //recommended into_active_model fn.
    pub fn get_active_model(&self) -> ActiveModel {
         ActiveModel {
             tenant_key : Set(self.tenant_key.clone()),
             routine_key : Set(self.routine_key.clone()),
             routine_data : Set(self.routine_data.clone()),
             created_on : Set(self.created_on),
             last_iteration_on : Set(self.last_iteration_on),
             last_result : Set(self.last_result.clone()),
            ..Default::default()
        }
    }
}

pub fn from_rfc(rfc: &RFC, status: &RoutineStatus) -> Model {
    Model {
        id: 0,
        tenant_key : rfc.tenant_key.clone(),
        routine_key : rfc.routine_key.clone(),
        routine_data: rfc.routine_data.clone(),
        created_on: rfc.created_on.clone(),
        last_iteration_on : rfc.last_iteration_on.clone(),
        last_result : status.to_string()
    }
}

