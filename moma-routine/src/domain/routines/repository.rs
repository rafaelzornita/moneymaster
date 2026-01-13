use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder};
use crate::domain;
use crate::error::RoutineErrors;

use super::super::{RFC, RFCEntity};
use super::super::{RFCLog, RFCLogEntity};

///Find an RFC by key 
pub async fn find_tenant_key(tenant_key: &String, db: &DatabaseConnection) -> Result<Option<RFC>, RoutineErrors> {
    //find by pk
    RFCEntity::find_by_id(tenant_key).one(db).await.map_err(|e| e.into())
}

pub async fn insert(rc: &RFC, db: &DatabaseConnection) -> Result<(), RoutineErrors> {    
    RFCEntity::insert(rc.get_active_model()).exec(db).await?;
    Ok(())
}

pub async fn update(rc: &RFC, db: &DatabaseConnection) -> Result<(), RoutineErrors> {
    RFCEntity::update(rc.get_active_model()).exec(db).await?;
    Ok(())
}

pub async fn delete(rc: &RFC, db: &DatabaseConnection) -> Result<(), RoutineErrors> {
    RFCEntity::delete_by_id(rc.tenant_key.clone()).exec(db).await?;
    Ok(())
}

pub async fn insert_log(rfcl: &RFCLog, db: &DatabaseConnection) -> Result<(), RoutineErrors> {
    RFCLogEntity::insert(rfcl.get_active_model()).exec(db).await?;
    Ok(())
}

pub async fn get_last_tenant_flow(tenant_key: &String, db: &DatabaseConnection) -> Result<Option<RFCLog>, RoutineErrors> {
    use domain::routines::routine_flow_control_log::Column as RCFLogColumn;
    Ok(RFCLogEntity::find().filter(RCFLogColumn::TenantKey.eq(tenant_key.clone())).order_by_desc(RCFLogColumn::Id)
    .one(db).await?)
}

pub async fn get_timedout_flows(db: &DatabaseConnection) -> Result<Vec<RFC>, RoutineErrors> {
    use domain::routines::routine_flow_control::Column as RCFColumn;
    
    Ok(RFCEntity::find().filter(RCFColumn::DueOn.lt(chrono::Local::now().naive_local())).all(db).await?)
}