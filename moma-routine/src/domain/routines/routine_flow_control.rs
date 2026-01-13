use sea_orm::{entity::prelude::*, Set};
use std::fmt::{self, Formatter};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Default)]
#[sea_orm(table_name = "RoutineFlowControl")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub tenant_key: String,
    pub routine_key: String,
    pub current_step_id: u8,
    pub routine_data: String,
    pub created_on: DateTime,
    pub last_iteration_on: DateTime,
    pub due_on: DateTime,
    #[sea_orm(ignore)]
    pub not_saved: bool
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
            tenant_key : Set(self.tenant_key.to_owned()), 
            current_step_id : Set(self.current_step_id.to_owned()),
            routine_key : Set(self.routine_key.to_owned()),
            routine_data : Set(self.routine_data.clone()),
            due_on : Set(self.due_on),
            created_on : Set(self.created_on),
            last_iteration_on : Set(self.last_iteration_on),
            ..Default::default()
        }
    }
    pub fn new(tenant_key : &String) -> Model {
        Model {
            tenant_key : tenant_key.to_owned(),
            not_saved : true,
            created_on : chrono::Local::now().naive_local(),
            ..Default::default()
        }
    }
}

impl fmt::Display for Model{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "key:{}, routine_data:{}, current_step_id:{}, routine_id: {}, created_on:{}, due_on: {}, routine_data: {}", self.tenant_key, self.routine_data, self.current_step_id, self.routine_key, self.created_on, self.due_on, self.routine_data)
    }
}
