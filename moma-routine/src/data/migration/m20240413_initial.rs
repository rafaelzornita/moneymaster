use sea_orm_migration::{async_trait::async_trait, prelude::*};
use crate::domain::{RFCEntity, RFCLogEntity};
use sea_orm::{ConnectionTrait, EntityTrait, Schema};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait]
trait MyMigration<'a> {
    async fn create_table<E>(manager: &'a SchemaManager, entity: E) where E: EntityTrait;
    async fn create_tables(manager: &'a SchemaManager);
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        Migration::create_tables(manager).await;
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
    
    async fn create_tables(manager: &'a SchemaManager) {
        Self::create_table(manager, RFCEntity).await;
        Self::create_table(manager, RFCLogEntity).await;
    }

    async fn create_table<E>(manager: &'a SchemaManager, entity: E)
    where E: EntityTrait,
    {
        let builder = manager.get_database_backend();
        let schema = Schema::new(builder);
        let stmt = builder.build(schema.create_table_from_entity(entity).if_not_exists());

        match manager.get_connection().execute(stmt).await {
            Ok(_) => println!("Migrated {}", entity.table_name()),
            Err(e) => println!("Error: {}", e),
        }
    }
}
