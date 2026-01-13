pub mod m20240413_initial;

use sea_orm::DatabaseConnection;
use sea_orm_migration::prelude::*;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20240413_initial::Migration)]
    }
}

pub async fn migrate(db: &DatabaseConnection) -> Result<(), DbErr> {
    Migrator::up(db, None).await
}