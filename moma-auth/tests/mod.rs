

#[cfg(test)]
mod tests {    
    use core::panic;
    use moma_auth::{create_tenant, validate_tenant};
    use moma_auth::data::migration::Migrator;
    use moma_auth::error::AuthErrors;
    use moma_auth::{data::connectionfactory, domain::*};
    use sea_orm::{Database, DatabaseConnection, EntityTrait};
    use sea_orm::Set;
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
    async fn db_migrate() {        
        let db = get_db().await;
        db_init(&db).await;
        db.close().await.unwrap();
    }

    #[tokio::test]
    async fn entities_cruds() {
        let db = get_db().await;
        db_init(&db).await;

        let database = DatabaseAM {
            file_name: Set("Test.db".to_owned()),
            created_on: Set(chrono::Local::now().naive_local()),
            ..Default::default()
        };
        let db_id = DatabaseEntity::insert(database)
        .exec(&db).await.expect("Error inserting database").last_insert_id;
        assert_ne!(
            db_id,
            0
        );
        
        let x = TenantAM {
            name: Set("Test".to_owned()),
            key: Set("554499999999".to_owned()),
            email: Set("testlargeemailstringbutverylarge@testverylargeemail.com".to_owned()),
            database_id: Set(db_id),
            created_on: Set(chrono::Local::now().naive_local()),
            ..Default::default()
        };
        
        let tid = TenantEntity::insert(x).exec(&db).await.expect("Error inserting tenant").last_insert_id;
        let x = TenantEntity::find_by_id(tid).find_also_related(DatabaseEntity).one(&db).await.expect("Error on tenant insert");
        if let Some(qr) = x {
            assert_eq!(qr.0.id,1);
            if let Some(qrl) = qr.1 {
                assert_eq!(qrl.id, 1);
            }
        } else {
            panic!("Tenant not found");
        }

        db.close().await.unwrap();
        remove_data_folder();
    }

    #[tokio::test]
    async fn repositories() {
        let db = get_db().await;
        db_init(&db).await;
    
        let database = DatabaseAM {
            file_name: Set("Test.db".to_owned()),
            created_on: Set(chrono::Local::now().naive_local()),
            ..Default::default()
        };
        let db_id = DatabaseEntity::insert(database).exec(&db).await.expect("Error inserting database record").last_insert_id;

        let mut m = Tenant {
            name: "Test".to_string(),
            email: "test@test.com".to_string(),
            key: "5544999999999".to_owned(),
            enabled: false,
            database_id: db_id,
            created_on: chrono::Local::now().naive_local(),
            ..Default::default()
        };
        
        if let Err(e) = TenantRepo::save(&mut m, &db).await{
            panic!("Error inserting entry as model:{}",e);
        }
        
        println!("{}",m);

        if let Some(model) = TenantRepo::find_id(m.id, &db).await {
            assert_eq!(m.id, model.id);
            assert_eq!(model.database.id, model.database_id);
        } else {
            panic!("Inserted tenant was not found")
        }

        // let x = TenantAM {
        //     name: Set("Test".to_string()),
        //     email: Set("test@test.com".to_string()),
        //     key: Set("5544999999999".to_owned()),
        //     enabled: Set(false),
        //     database_id: Set(db_id),
        //     created_on: Set(chrono::Local::now().naive_local()),
        //     ..Default::default()
        // };   

        // assert_ne!(TenantRepo::save_am(x, &db).await.expect("Error inserting tenant as active model"), 0);
        
        // _ = db.close();

        remove_data_folder();
    }

    #[tokio::test]
    async fn tenant_create() {

        db_init(&connectionfactory::get_connection().await.expect("")).await;

        match create_tenant("Name test".to_owned(), "559999999999".to_owned(), "emailtest@email.com".to_owned()).await
        {
            Err(e) => {
                match e {
                    AuthErrors::DbError(d) => panic!("Database error: {}", d),
                    AuthErrors::UserInputError(m, _) => panic!("User input error: {}", m)
                }
            },
            Ok(tenant) => println!("{}",tenant)
        }

        remove_data_folder();
    }

    #[tokio::test]
    async fn tenant_validate() {
        use moma_auth::error::TenantValidationError;

        db_init(&connectionfactory::get_connection().await.expect("")).await;

        _ = validate_tenant(&"559999999991".to_string()).await
            .map_err(|e| assert!(e == TenantValidationError::Unexists,"Tenant must NOT EXISTS at this time: {}", e))
            .map(|_| panic!("Validate tenant must return error at this time."));

        match create_tenant("Name test".to_owned(), "559999999991".to_owned(), "emailtest@email.com".to_owned()).await
        {
            Err(e) => {
                match e {
                    AuthErrors::DbError(d) => panic!("Database error: {}", d),
                    AuthErrors::UserInputError(m, _) => panic!("User input error: {}", m)
                }
            },
            Ok(tenant) => println!("{}",tenant)
        }
        
        if let Err(e) = validate_tenant(&"559999999991".to_string()).await {
            match e {
                TenantValidationError::Unexists => panic!("Tenant must EXISTS at this time."),
                TenantValidationError::Disabled => panic!("Tenant must be disabled at this time."),
                TenantValidationError::EmailUnconfirmed => println!("Tenant email unconfirmed.") //OK
            }
        } else {
            panic!("Validate tenant must return error at this time.")        
        }

        remove_data_folder();
    }


    fn remove_data_folder() {
        let path= connectionfactory::get_data_path();
        if std::path::Path::new(&path).exists() {
            _ = std::fs::remove_dir_all(path);
        }
    }
}



