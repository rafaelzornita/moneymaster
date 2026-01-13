use sea_orm::{ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter};
use super::{super::{Tenant, TenantAM, TenantEntity, DatabaseEntity, Database}, tenant};

///Save an Tenant Model to database and return id value if it was inserted. If id Is NotSet, Insert, Else, Update"
pub async fn save(tenant: &mut Tenant, db: &DatabaseConnection) -> Result<(), DbErr> {
    match process_save(tenant, &db).await {
        Err(e) => Err(e),
        Ok(id)=> {
                        tenant.id = id; 
                        tenant.database = DatabaseEntity::find_by_id(tenant.database_id).one(db).await.unwrap().unwrap();
                        Ok(())
                    }
    }
}

///Save an Tenant 'Active' Model to database and return id value if it was inserted.\n If id Is NotSet, Insert, Else, Update
async fn process_save(tenant: &mut Tenant, db: &DatabaseConnection) -> Result<u16, DbErr> {
    if tenant.id == 0 {
        insert(tenant, db).await
    } else {
        if let Err(e) = update(tenant.clone().get_active_model(), db).await {
            Err(e)
        }else {
            Ok(0)
        }        
    }
}

///Find an Tenant by id
pub async fn find_id(id: u16, db: &DatabaseConnection) -> Option<Tenant> {
    if let Some(e) = TenantEntity::find_by_id(id).find_also_related(super::super::DatabaseEntity).one(db).await.expect("Error finding entry by id") {
        let mut ret: Tenant = e.0;        
        if let Some(c) = e.1 {
            ret.database = c;
        }
        return Some(ret);
    }
    //Default returning
    None
}

///Find an Tenant by key
pub async fn find_key(key: &str, db: &DatabaseConnection) -> Option<Tenant> {
    if let Some(e) = TenantEntity::find().filter(tenant::Column::Key.eq(key)).find_also_related(super::super::DatabaseEntity).one(db).await.expect("Error finding tenant by id") {
        let mut ret: Tenant = e.0;
        if let Some(c) = e.1 {
            ret.database = c;
        }
        return Some(ret);
    }
    //Default returning
    None
}

async fn insert(tenant: &mut Tenant, db: &DatabaseConnection) -> Result<u16, DbErr> {    
    if tenant.database_id == 0 {
        match insert_database(&db).await {
            Ok(id) => tenant.database_id = id,
            Err(e) => return Err(e)
        }        
    }
    
    match TenantEntity::insert(tenant.clone().get_active_model()).exec(db).await {
        Ok(x) => Ok(x.last_insert_id),
        Err(e) => Err(e)
    }        
}

async fn insert_database(db: &DatabaseConnection) -> Result<u16, DbErr> {    
    match DatabaseEntity::insert(Database {
        file_name: uuid::Uuid::new_v4().to_string()[..8].to_string() + ".db", 
        created_on: chrono::Local::now().naive_local(),
        ..Default::default()
    }.get_active_model()).exec(db).await
    {
        Ok(x) => Ok(x.last_insert_id),
        Err(e) => Err(e)
    }        
}

async fn update(tenant: TenantAM, db: &DatabaseConnection) -> Result<(), DbErr> {
    match TenantEntity::update(tenant.clone()).exec(db).await {
        Ok(_) => Ok(()),
        Err(e) => Err(e)
    }        
}


pub async fn list(db: &DatabaseConnection) -> Result<Vec<Tenant>, DbErr> {
    Ok(TenantEntity::find()
        .find_also_related(super::super::DatabaseEntity)
        .all(db).await?.into_iter()
        .map(|mut f| {f.0.database = f.1.unwrap(); f.0}).collect())
}