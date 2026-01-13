use chrono::NaiveDate;
use sea_orm::{prelude::Expr, DatabaseConnection, DbErr, EntityTrait, QueryFilter};
use super::{super::{Entry, EntryAM, EntryEntity}, entry};

///Save an Entry Model to database and return id value if it was inserted. If id Is NotSet, Insert, Else, Update"
pub async fn save(entry: &mut Entry, db: &DatabaseConnection) -> Result<(), DbErr> {
    let m = entry.clone().get_active_model();
    match save_am(m, &db).await {
        Err(e) => Err(e),
        Ok(id)=> {
                        entry.id = id; 
                        Ok(())
                    }
    }
}

///Save an Entry 'Active' Model to database and return id value if it was inserted.\n If id Is NotSet, Insert, Else, Update
pub async fn save_am(entry: EntryAM, db: &DatabaseConnection) -> Result<u32, DbErr> {
    if entry.id.is_not_set(){
        insert(entry, db).await
    } else {
        if let Err(e) = update(entry, db).await {
            Err(e)
        }else {
            Ok(0)
        }        
    }
}

///Find an entry by id
pub async fn find_id(id: u32, db: &DatabaseConnection) -> Option<Entry> {
    if let Some(e) = EntryEntity::find_by_id(id).find_also_related(super::super::CategoryEntity).one(db).await.expect("Error finding entry by id") {
        let mut ret: Entry = e.0;        
        if let Some(c) = e.1 {
            ret.category = c;
        }
        return Some(ret);
    }
    //Default returning
    None
}

///Find many entries by id
pub async fn find_ids(ids: &Vec<u32>, db: &DatabaseConnection) -> Vec<Entry> {
    let filter  = Expr::col(entry::Column::Id).is_in(ids.to_owned());

    EntryEntity::find()
                .filter(filter)
                .left_join(super::super::CategoryEntity)
                .all(db)
                .await.expect("Error finding entry by ids")
}

///Delete an entry from database
pub async fn delete_by_id(id: &u32, db: &DatabaseConnection) -> Result<(), DbErr> {
    EntryEntity::delete_by_id(id.clone()).exec(db).await?;
    Ok(())
}

pub async fn insert(entry: EntryAM, db: &DatabaseConnection) -> Result<u32, DbErr> {
    match EntryEntity::insert(entry.clone()).exec(db).await {
        Ok(x) => Ok(x.last_insert_id),
        Err(e) => Err(e)
    }        
}

async fn update(entry: EntryAM, db: &DatabaseConnection) -> Result<(), DbErr> {
    match EntryEntity::update(entry.clone()).exec(db).await {
        Ok(_) => Ok(()),
        Err(e) => Err(e)
    }        
}

///Find many entries by date
pub async fn find_by_date_and_operation(start_date: NaiveDate, end_date: NaiveDate, signal_operation: &str, db: &DatabaseConnection) -> Vec<Entry> {
    let filter  = Expr::col(entry::Column::Date).between(start_date, end_date)
                                .and(Expr::col(entry::Column::Operation).eq(signal_operation));

    EntryEntity::find()
                .filter(filter)
                .find_with_related(super::super::CategoryEntity)
                .all(db)
                .await.expect("Error finding entry by ids")
                .into_iter()
                .map(|(entry, categories)| Entry {
                    category : categories.first().unwrap().clone(), 
                    ..entry
                })
                .collect()
}