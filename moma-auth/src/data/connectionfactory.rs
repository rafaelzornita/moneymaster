use std::{borrow::Borrow, env, path::PathBuf};

use sea_orm::{self, error::DbErr, Database, DatabaseConnection};

pub fn get_data_path() -> PathBuf {
    let base_dir = env::current_exe().unwrap().parent().unwrap().to_path_buf();
    let folder_dir = base_dir.join("db");

    if std::path::Path::new(&folder_dir).exists() == false {
        println!("Creating \\db");
        std::fs::create_dir(folder_dir.clone()).expect("Error creating db dir");
    }

    folder_dir
}

pub async fn get_connection() -> Result<DatabaseConnection, DbErr> {
    get_connection_ex("auth.db".to_string().borrow()).await
}

async fn get_connection_ex(file_name: &String) -> Result<DatabaseConnection, DbErr> {
    let data_path = get_data_path().join(file_name);
    let str_data_path = data_path.to_str().unwrap();
    Database::connect(format!("sqlite://{str_data_path}?mode=rwc")).await
}  