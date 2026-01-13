#[cfg(test)]
mod tests {
    use std::env;
    use moma_auth::data::connectionfactory;
    use moma_routine::{domain::{routines::{base::routine_process_result::RoutineStatus, routine_trait::Routine, routines_flow::{entry_insert, signup}}, RoutineRepo}, routineflowcontrol, routineselector};
    use sea_orm_migration::MigratorTrait;

    #[tokio::test]
    async fn database() {
        use moma_routine::data::{migration::Migrator, connectionfactory};

        let db = &connectionfactory::get_connection().await.unwrap();
        let x = Migrator::up(db, None).await;

        assert_eq!(x.is_ok(), true);

        let binding = "test".to_string();
        let y = RoutineRepo::find_tenant_key(&binding, &db).await;

        assert_eq!(y.is_ok(), true);
    }

    #[tokio::test]
    async fn routine_selector() {

        for r in routineselector::get_routines() {
            let routine = routineselector::select(&r.0).await
                .expect(&format!("{:?} routine was not found in selector", r.0));    

            assert_eq!(routine.get_key(), &r.0);
        }

        //IA Select
        assert_eq!(routineselector::select(&"Comprei um ramalhete de flores".to_string()).await.unwrap().get_key(), &"entry_insert");

        assert!(routineselector::select(&"anyelse".to_string()).await.is_none(), "Expected 'anyelse' routine to be not found");
    }

    #[tokio::test]
    async fn routine_signup() {
        use moma_auth::data::migration as auth_migration;
        
        let dir = env::current_exe().unwrap().parent().unwrap().to_path_buf();
        dotenv::from_path(dir.join(".env")).expect("Error .env");

        _ = auth_migration::Migrator::up(&connectionfactory::get_connection().await.expect(""), None).await;

        let mut routine = signup::Signup::new();
        
        assert_eq!(routine.get_key(), "signup");
        assert_eq!(routine.is_internal_porpouse(), &true);
        //whats app number to test
        routine.set_tenant_key("+55...".to_string());
        
        let res = routine.process_input(&String::from("Hello")).await.expect("Error on processing input");
        assert_eq!(res.status, RoutineStatus::Partial);

        let res = routine.process_input(&String::from("New User")).await.expect("Error on processing input");
        assert_eq!(res.status, RoutineStatus::Partial);

        let res = routine.process_input(&String::from("momatest@tuamaeaquelaursa.com")).await.expect("Error on processing input");
        assert_eq!(res.status, RoutineStatus::Partial);

        //tests json serde
        let x = routine.into_json();
        let mut routine = signup::Signup::new();
        routine.from_json(&x);
        
        println!("Confirmation code:{}", routine.get_confirmation_code());
        let res = routine.process_input(&routine.get_confirmation_code().to_string()).await.expect("Error on processing input");
        assert!(res.status == RoutineStatus::Done,"Status is NOT Done. {}", res.user_messages[0]);


    }

    #[tokio::test]
    async fn routine_entry() {
        use moma_auth::data::migration as auth_migration;
        
        let dir = env::current_exe().unwrap().parent().unwrap().to_path_buf();
        dotenv::from_path(dir.join(".env")).expect("Error .env");
        //Whats app number to test
        let t_key = "+55...".to_string();

        _ = auth_migration::Migrator::up(&connectionfactory::get_connection().await.expect(""), None).await;
        let tenant = moma_auth::get_tenant(&t_key).await.unwrap().unwrap();
        let tenant_db = moma_core::data::connectionfactory::get_connection(&tenant.database.file_name).await.unwrap();
        _ = moma_core::data::migration::Migrator::up(&tenant_db, None).await;

        let mut routine = entry_insert::EntryInsert::new();
        
        assert_eq!(routine.get_key(), "entry_insert");
        assert_eq!(routine.is_internal_porpouse(), &false);

        routine.set_tenant_key(t_key);
        
        let res = routine.process_input(&String::from("supermercado 500,00")).await.expect("Error on processing input");
        assert_eq!(res.status, RoutineStatus::Done);
        assert!(res.status == RoutineStatus::Done,"Status is NOT Done. {}", res.user_messages[0]);

        //tests json serde
        let x = routine.into_json();
        let mut routine = entry_insert::EntryInsert::new();
        routine.from_json(&x);
        
    }

    #[tokio::test]
    async fn routine_flow_control() {
        use moma_routine::data::{migration::Migrator, connectionfactory};

        let _ = Migrator::up(&connectionfactory::get_connection().await.unwrap(), None).await;

        let ret = routineflowcontrol::submit(&"tenanttest".to_string(), &"signup".to_string()).await;
        match ret {
            Err(e) => panic!("Step 1 error: {}",e),
            Ok(res) => {
                assert_eq!(res.status, RoutineStatus::Partial, "Step 1 need to exit with partial status. Current: {:?}, Msg: {}", res.status, res.get_user_messages());
            }
        }

        let ret = routineflowcontrol::submit(&"tenanttest".to_string(), &"New Big User".to_string()).await;
        match ret {
            Err(e) => panic!("Step 2 error: {}",e),
            Ok(res) => {
                assert_eq!(res.status, RoutineStatus::Partial, "Step 1 need to exit with partial status. Current: {:?}, Msg: {}", res.status, res.get_user_messages());
            }
        }

        let ret = routineflowcontrol::submit(&"tenanttest".to_string(), &"momatest@tuamaeaquelaursa.com".to_string()).await;
        match ret {
            Err(e) => panic!("Cancel process error: {}",e),
            Ok(res) => {
                assert_eq!(res.status, RoutineStatus::Partial, "Step 1 need to exit with partial status. Current: {:?}, Msg: {}", res.status, res.get_user_messages());
            }
        }

    }
}