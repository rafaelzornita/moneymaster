use core::fmt;
use std::fmt::Formatter;
use sea_orm::{entity::prelude::*, Set};
use crate::domain::Category;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Default, Serialize, Deserialize)]
#[sea_orm(table_name = "Entry")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u32,
    pub name: String,
    pub operation: String,
    pub category_id: u8,
    pub date: Date,
    pub value: f32,
    pub observation: String,
    #[sea_orm(ignore)]
    pub category: Category
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        has_one = "super::super::categories::category::Entity",
        belongs_to = "super::super::categories::category::Entity",
        to = "super::super::categories::category::Column::CategoryId",
        from = "super::entry::Column::CategoryId"
    )]
    Category
}

impl Related<super::super::CategoryEntity> for Entity {
    fn to() -> RelationDef {
        Relation::Category.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}


impl Model {
    ///Create an active model from a Model. Indicated for new models created instances otherwise recomended into_active_model fn.
    pub fn get_active_model(self) -> ActiveModel {
        let mut am = ActiveModel {
            ..Default::default()
        };

        if self.id > 0 {
            am.id = Set(self.id);
        }  
        if self.category_id > 0 {
            am.category_id = Set(self.category_id);
        }  
        if !self.name.is_empty() {
            am.name = Set(self.name.to_owned()); 
        }
        if self.value > 0.0 {
            am.value = Set(self.value.to_owned()); 
        }
        am.date = Set(self.date);
        am.observation = Set(self.observation.to_owned());
        am.operation = Set(self.operation.to_owned());
        am
    }
}

impl fmt::Display for Model{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "id:{}, name:{}, category:{}, date:{}", self.id, self.name, self.category_id, self.date)
    }
}