use sea_orm_migration::{async_trait::async_trait, prelude::*};
use crate::domain::{categories::category_defaults, CategoryEntity};
use sea_orm::{EntityTrait, IntoActiveModel};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait]
trait MyMigration<'a> {
    async fn seed_categories(db: &'a SchemaManagerConnection) ;
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        Migration::seed_categories(manager.get_connection()).await;
        Ok(())
    }
    
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        //nothing todo here
        let _ = manager;
        Ok(())
    }
}

#[async_trait::async_trait]
impl<'a> MyMigration<'a> for Migration {
    async fn seed_categories(db: &'a SchemaManagerConnection) {
        for item in category_defaults::get_default_categories()  {
            if let None = CategoryEntity::find_by_id(item.category_id).one(db).await.expect("Error finding category"){
                CategoryEntity::insert(item.into_active_model()).exec(db).await.expect("Error inserting category");
            }
        }
    }
}
