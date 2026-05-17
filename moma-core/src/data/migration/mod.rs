pub mod m20240329_initial;
pub mod m20241209;
pub mod m20250209;
pub mod m20260517;
pub mod m20260517_2;
pub mod m20260517_3;

use sea_orm::DatabaseConnection;
use sea_orm_migration::prelude::*;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20240329_initial::Migration),
            Box::new(m20241209::Migration),
            Box::new(m20250209::Migration),
            Box::new(m20260517::Migration),
            Box::new(m20260517_2::Migration),
            Box::new(m20260517_3::Migration)]
    }
}

pub async fn migrate(db: &DatabaseConnection) -> Result<(), DbErr> {
    Migrator::up(db, None).await
}