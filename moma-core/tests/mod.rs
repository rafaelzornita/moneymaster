
#[cfg(test)]
mod tests {    
    use core::panic;
    use std::path::Path;
    use moma_core::data::migration::Migrator;
    use moma_core::domain::categories::category_defaults::get_default_categories;
    use moma_core::{data::connectionfactory, domain::*};
    use sea_orm::{Database, DatabaseConnection};
    use sea_orm::{EntityTrait, Set};
    use sea_orm_migration::MigratorTrait;
    
    async fn get_db() -> DatabaseConnection {   
        Database::connect(format!("sqlite::memory:")).await.expect("Error connecting to database")
    }   

    async fn db_init(db : &DatabaseConnection){
        Migrator::up(db, None)
        .await
        .expect("Error migrating database");
    }

    #[tokio::test]
    async fn db_create() {
        let pathb= connectionfactory::get_data_path();
        let path = pathb.to_str().unwrap();
        let db_name: String = String::from("dbtest.sqlite");
        let db_path= pathb.join(db_name.to_owned()); //format!("{}{}", path, db_name);
        
        println!("db dir: {}",db_path.to_str().unwrap());
        println!("parent folder:{}", Path::new(&db_path).parent().and_then(Path::parent).unwrap().to_str().unwrap());

        let _ = connectionfactory::get_connection(&db_name).await;
        if std::path::Path::new(&db_path).exists() {
            std::fs::remove_dir_all(std::path::Path::new(&db_path).parent().and_then(Path::parent).unwrap_or_else(|| panic!("db folder could not be evaluated"))).expect("Error deleting data dir");
        }else {
            panic!("Database file not found{}",path);
        }
    }

    #[tokio::test]
    async fn db_migrate() {        
        let db = get_db().await;
        db_init(&db).await;

        //Inserted categories
        assert_eq!(CategoryEntity::find().all(&db).await.unwrap().len(), 5);
        
        db.close().await.unwrap();
    }

    #[tokio::test]
    async fn entities_cruds() {
        let db = get_db().await;
        db_init(&db).await;

        let category = CategoryAM {
            name: Set("Test".to_owned()),
            ..Default::default()
        };
        assert_ne!(
            CategoryEntity::insert(category)
                .exec(&db)
                .await
                .expect("Error inserting category")
                .last_insert_id,
            0
        );           

        let category = CategoryAM {
            name: Set("Test2".to_owned()),
            ..Default::default()
        };
        
        if let Ok(cat) = category.save(&db).await{
            assert!(!CategoryEntity::find_by_id(cat).one(&db).await.expect("Error finding category").is_none());
        }
        
        let x = EntryAM {
            name: Set("Test".to_owned()),
            category_id: Set(get_default_categories()[0].category_id),
            date: Set(chrono::Local::now().date_naive()),
            ..Default::default()
        };
        
        let _ = EntryEntity::insert(x).exec(&db).await;
        let x = EntryEntity::find_by_id(1 as u32).find_also_related(CategoryEntity).one(&db).await.expect("Error on entry insert");
        if let Some(qr) = x {
            assert_eq!(qr.0.id,1);
            if let Some(qrl) = qr.1 {
                assert_eq!(qrl.category_id, 1);
            }
        } else {
            panic!("Entry not found");
        }

        db.close().await.unwrap();
    }

    #[tokio::test]
    async fn repositories() {
        let db = get_db().await;
        db_init(&db).await;
    
        let mut m = Entry {
            name: "Test".to_string(),
            category_id: 1,
            date: chrono::Local::now().naive_local().date(),
            ..Default::default()
        };
        
        if let Err(e) = EntryRepo::save(&mut m, &db).await{
            panic!("Error inserting entry as model:{}",e);
        }
        
        println!("{}",m);

        if let Some(model) = EntryRepo::find_id(m.id, &db).await {
            assert_eq!(m.id, model.id);
            assert_eq!(model.category.category_id, model.category_id);
        } else {
            panic!("Inserted entry was not found")
        }

        let x = EntryAM {
            name: Set("Test5".to_owned()),
            category_id: Set(get_default_categories()[0].category_id),
            date: Set(chrono::Local::now().date_naive()),
            ..Default::default()
        };   

        assert_ne!(EntryRepo::save_am(x, &db).await.expect("Error inserting entry as active model"), 0);
        
        _ = db.close();
    }
}



