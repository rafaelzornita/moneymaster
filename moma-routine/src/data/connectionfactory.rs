use std::env;
use sea_orm::{self, error::DbErr, Database, DatabaseConnection};
use crate::error::RoutineErrors;
use std::path::PathBuf;

pub fn get_data_path() -> PathBuf {
    let base_dir = env::current_exe().unwrap().parent().unwrap().to_path_buf();
    let folder_dir = base_dir.join("db");

    if std::path::Path::new(&folder_dir).exists() == false {
        println!("Creating \\db");
        std::fs::create_dir(folder_dir.clone()).expect("Error creating db dir");
    }

    folder_dir
}

pub async fn get_connection() -> Result<DatabaseConnection, RoutineErrors> {
    get_connection_ex(&"routine.db".to_string()).await.map_err(|e| e.into())
}

async fn get_connection_ex(file_name: &String) -> Result<DatabaseConnection, DbErr> {
    let data_path = get_data_path();
    let full_dir = data_path.join(file_name);
    let full_dir_string = full_dir.to_str().unwrap();
    Database::connect(format!("sqlite://{full_dir_string}?mode=rwc")).await
}  