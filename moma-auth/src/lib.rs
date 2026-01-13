pub mod domain;
pub mod data;
pub mod error;

use data::connectionfactory;
use domain::{Tenant, TenantRepo};
use sea_orm::DatabaseConnection;
use sea_orm_migration::MigratorTrait;
use error::{AuthErrors, TenantValidationError};

/// Returns true if tenant exists
pub async fn tenant_exists(key: String) -> bool {
    let db = connectionfactory::get_connection().await.unwrap();
    let exists = TenantRepo::find_key(&key, &db).await.is_some();
    _ = db.close().await;
    exists
}

/// Returns an enumerated error if any problem is caught
pub async fn validate_tenant(key: &String) -> Result<(), TenantValidationError> {
    let db = connectionfactory::get_connection().await.unwrap();
    let result = TenantRepo::find_key(&key, &db).await;
    _ = db.close().await;

    match result {
        None => return Err(TenantValidationError::Unexists),
        Some(tenant) => {
            if !tenant.enabled {
                return Err(TenantValidationError::Disabled);
            }else if !tenant.emailconfirmed {
                return Err(TenantValidationError::EmailUnconfirmed);
            }
            return Ok(());
        }
    }
}

/// Creates a new tenant and a default database to it. Default values:
/// 
/// Enabled false
/// 
/// Database.filename: random
pub async fn create_tenant(name: String, key: String, email: String) -> Result<Tenant, AuthErrors> {
    let auth_db = connectionfactory::get_connection().await?;

    let mut tenant = domain::Tenant {
        key: key.clone(), 
        name: name, 
        email: email, 
        enabled: true, 
        created_on: chrono::Local::now().naive_local(), 
        emailconfirmed: true,
        ..Default::default() };
    
    if let Some(mut result) = TenantRepo::find_key(&key, &auth_db).await {
        result.emailconfirmed = false;
        TenantRepo::save(&mut result, &auth_db).await?;
        return Ok(result);
    }

    TenantRepo::save(&mut tenant, &auth_db).await?;
        
    //migrate database
    let tenant_db = moma_core::data::connectionfactory::get_connection(&tenant.database.file_name).await?;
    moma_core::data::migration::Migrator::up(&tenant_db, None).await?;
            
    _ = tenant_db.close().await;
    _ = auth_db.close().await;  

    Ok(tenant)
}

pub async fn get_tenant(key: &String) -> Result<Option<Tenant>, AuthErrors> {
    
    let auth_db = connectionfactory::get_connection().await?;
    
    Ok(TenantRepo::find_key(key, &auth_db).await)

}

pub async fn get_tenant_connection(key: &str) -> Result<DatabaseConnection, AuthErrors> {
    
    let auth_db = connectionfactory::get_connection().await?;
    let tenant = TenantRepo::find_key(key, &auth_db).await;
    if let Some(tenant) = tenant {
        let tenant_database = moma_core::data::connectionfactory::get_connection(&tenant.database.file_name).await?;
        return Ok(tenant_database);
    }
    
    Err(AuthErrors::DbError(format!("Tenant database was not found. key: {}", key)))
}
pub async fn get_tenants() -> Result<Vec<Tenant>, AuthErrors> {
    
    let auth_db = connectionfactory::get_connection().await?;

    Ok(TenantRepo::list(&auth_db).await?)
}

pub async fn get_tenants_database() -> Result<Vec<String>, AuthErrors> {
    
    let auth_db = connectionfactory::get_connection().await?;

    Ok(TenantRepo::list(&auth_db).await?
        .into_iter()
        .map(|f| f.database.file_name)
        .collect())
}

pub async fn update_tenant_interaction(tenant_key: &String) -> Result<(), AuthErrors> {
    let tenant = get_tenant(&tenant_key).await?;
    if let None = tenant{
        return Ok(());
    }

    let mut tenant = tenant.unwrap();
    tenant.interactions_count += 1;
    tenant.last_interaction_on = chrono::Local::now().naive_local();

    let db = connectionfactory::get_connection().await?;
    TenantRepo::save(&mut tenant, &db).await?;

    Ok(())
}