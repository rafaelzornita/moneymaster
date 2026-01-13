use moma_auth::get_tenant;
use moma_integration::whatsapp;
use moma_routine::domain::routines::repository;
use moma_routine::data::connectionfactory;
use moma_routine::domain::routines::routine_flow_control_log;

pub async fn remove_timed_out_routines() {
    
    let db = &connectionfactory::get_connection().await.unwrap();
    let expired_routines = repository::get_timedout_flows(db).await;
    match expired_routines {
        Ok(rf) => {
            for flow in rf {                            
                let tenant = get_tenant(&flow.tenant_key).await.unwrap();
                if let Some(tenant) = tenant {
                    if tenant.last_interaction_on > flow.due_on {
                        //User is interacting with the system, don't cancel the routine
                        println!("User is interacting with the system, continuing...");
                        continue;                        
                    }
                }
                println!("Removed timed out routine {:?} from user: {:?}", &flow.routine_key, &flow.tenant_key);
                _ = repository::delete(&flow, db).await.unwrap();  
                _ = repository::insert_log(&routine_flow_control_log::from_rfc(&flow,&moma_routine::domain::routines::base::routine_process_result::RoutineStatus::Canceled), db).await;

                match whatsapp::gateway::send_text(&flow.tenant_key, "Processo cancelado por inatividade.").await {
                    Err(e) => println!("Timed out routine: WhatsApp Error: {}",e.to_string()),
                    Ok(()) => println!("Timed out routine: WhatsApp OK;")
                } 
            }             
        },
        Err(e) => {
            println!("Error getting timed out routines: {:?}", e.to_string());
        }
    }
}