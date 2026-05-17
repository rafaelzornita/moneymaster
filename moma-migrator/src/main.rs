#[tokio::main]
async fn main() {

    {
        use moma_auth::data::connectionfactory;
        use moma_auth::data::migration;

        if let Err(e) = migration::migrate(&connectionfactory::get_connection().await.unwrap()).await {
            print!("Error updating auth database: {}", e.to_string())
        }
    }
    
    {
        use moma_core::data::connectionfactory;
        use moma_core::data::migration;

        let databases = moma_auth::get_tenants_database().await;
        if let Err(e) = databases {
            println!("{}", e);
            return;
        }

        for database in databases.unwrap() {
            println!("Migrating tenant: {}", database);
            let tenant_db = connectionfactory::get_connection(&database).await.expect("Error connecting to tenant database: ");
            if let Err(e) = migration::migrate(&tenant_db).await {
                print!("{}", e.to_string())
            }
        }
    }

    {
        use moma_routine::data::connectionfactory;
        use moma_routine::data::migration;


        if let Err(e) = migration::migrate(&connectionfactory::get_connection().await.unwrap()).await {
            print!("Error updating routine database: {}", e.to_string())
        }
       
    }


    println!("Migration pçrocess finalized.");

}
