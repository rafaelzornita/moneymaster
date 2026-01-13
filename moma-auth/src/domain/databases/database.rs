use core::fmt;
use std::fmt::Formatter;
use sea_orm::{entity::prelude::*, Set};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Default)]
#[sea_orm(table_name = "Database")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u16,
    pub file_name: String,
    pub created_on: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {

}

impl ActiveModelBehavior for ActiveModel {}


impl Model {
    //Create an active model from a Model. Indicates for new models created instances otherwise recomended into_active_model fn.
    pub fn get_active_model(self) -> ActiveModel {
        let mut am = ActiveModel {
          ..Default::default()
        };

        if self.id > 0 {
            am.id = Set(self.id);
        }  
        if !self.file_name.is_empty() {
          am.file_name = Set(self.file_name.to_owned()); 
        }
        
        am.created_on = Set(self.created_on);
        am
    }
}

impl fmt::Display for Model{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "id:{}, file_name:{}, created_on:{}", self.id, self.file_name, self.created_on)
    } 
}