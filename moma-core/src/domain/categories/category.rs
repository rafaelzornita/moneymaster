use sea_orm::entity::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Default, Serialize, Deserialize)]
#[sea_orm(table_name = "Category")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub category_id: u8,
    pub name: String,
    #[sea_orm(default_value = true)]
    pub enabled: bool
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {

}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    ///If id Is NotSet, Insert, Else, Update
    pub async fn save(self, db: &DatabaseConnection) -> Result<u8, DbErr> {
        if self.category_id.is_not_set(){
            self.insert(db).await
        } else {
            self.update(db).await
        }
    }

    async fn insert(self, db: &DatabaseConnection) -> Result<u8, DbErr> {
        match super::category::Entity::insert(self).exec(db).await {
            Ok(x) => Ok(x.last_insert_id),
            Err(e) => Err(e)
        }        
    }

    async fn update(self, db: &DatabaseConnection) -> Result<u8, DbErr> {
        match super::category::Entity::insert(self).exec(db).await {
            Ok(x) => Ok(x.last_insert_id),
            Err(e) => Err(e)
        }        
    }
}