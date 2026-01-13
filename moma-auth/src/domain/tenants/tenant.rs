use sea_orm::{entity::prelude::*, Set};
use super::super::Database;
use std::fmt::{self, Formatter};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Default)]
#[sea_orm(table_name = "Tenant")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u16,
    pub key: String,
    pub name: String,
    pub email: String,
    #[sea_orm(default_value = false)]
    pub enabled: bool,
    pub database_id: u16,
    pub created_on: DateTime,
    #[sea_orm(default_value = true)]
    pub emailconfirmed: bool,
    pub interactions_count: u32,
    pub last_interaction_on: DateTime,
    #[sea_orm(ignore)]
    pub database: Database
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        has_one = "super::super::databases::database::Entity",
        belongs_to = "super::super::databases::database::Entity",
        to = "super::super::databases::database::Column::Id",
        from = "super::tenant::Column::DatabaseId"
    )]
    Database
}

impl Related<super::super::DatabaseEntity> for Entity {
    fn to() -> RelationDef {
        Relation::Database.def()
    }
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
        if !self.key.is_empty() {
            am.key = Set(self.key.to_owned()); 
        }
        if !self.name.is_empty() {
          am.name = Set(self.name.to_owned()); 
        }
        if !self.email.is_empty() {
            am.email = Set(self.email.to_owned()); 
        }
        if !self.database_id > 0 {
            am.database_id = Set(self.database_id.to_owned()); 
        }

        am.enabled = Set(self.enabled.to_owned()); 
        am.emailconfirmed = Set(self.emailconfirmed.to_owned()); 
        am.created_on = Set(self.created_on);
        am.interactions_count = Set(self.interactions_count);
        am.last_interaction_on = Set(self.last_interaction_on);
        am
    }
}

impl fmt::Display for Model{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "id:{}, key:{}, name:{}, email:{}, enabled: {}, database_id: {}, created_on:{}, database: {{filename: {}, created_on: {} }}", self.id, self.key, self.name, self.email, self.enabled, self.database_id, self.created_on, self.database.file_name, self.database.created_on)
    }
}