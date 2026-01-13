use std::{env, path::PathBuf};

use sea_orm::{self, error::DbErr, Database, DatabaseConnection};

pub fn get_data_path() -> PathBuf {
    let base_dir = env::current_exe().unwrap().parent().unwrap().to_path_buf();
    let folder_dir = base_dir.join("db");
    let subfolder_dir= folder_dir.join("core");

    if std::path::Path::new(&folder_dir).exists() == false {
        println!("Creating \\db");
        std::fs::create_dir(folder_dir.clone()).expect("Error creating db dir");
    }

    if std::path::Path::new(&subfolder_dir).exists() == false {
        println!("Creating \\db\\core ");
        std::fs::create_dir(subfolder_dir.clone()).expect("Error creating core dir");
    }

    subfolder_dir
}

pub async fn get_connection(file_name: &String) -> Result<DatabaseConnection, DbErr> {
    let data_path = get_data_path();
    let full_dir = data_path.join(file_name);
    let full_dir_string = full_dir.to_str().unwrap();
    println!("Connecting to core database:{} - {}", file_name, full_dir_string);
    Database::connect(format!("sqlite://{full_dir_string}?mode=rwc")).await
}  