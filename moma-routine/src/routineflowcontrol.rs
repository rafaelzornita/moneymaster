use chrono::Duration;
use moma_auth::error::TenantValidationError;
use crate::{data::connectionfactory, domain::{self, routines::{base::routine_process_result::{self, RoutineProcessResult, RoutineStatus}, repository, routine_flow_control_log, routine_trait::Routine}, RFC}, error::RoutineErrors, routineselector};
use domain::routines::routines_flow::signup::Signup;

pub async fn submit(tenant_key:  &String, message: &String) -> Result<RoutineProcessResult, RoutineErrors> {

    let flow = get_tenant_flow(&tenant_key).await?;
    let tenant_validation = moma_auth::validate_tenant(&tenant_key).await;

    let routine: Option<Box<dyn Routine + Send>>;
    let mut flow = if let Some(flow) = flow {
                            routine = routineselector::select(&flow.routine_key).await;
                            flow
                        } else {
                            routine = routineselector::select(&if let Err(TenantValidationError::Unexists) = tenant_validation {
                                                                        Signup::new().get_key().to_owned()} 
                                                                    else {
                                                                        message.to_owned()
                                                                    }).await;
                            RFC::new(tenant_key)
                        };

    if routine.is_none() {
        return Ok(RoutineProcessResult::new().set_canceled().add_message("Desculpe, não foi possível contextualizar a sua mensagem.").to_owned());
    }
    match tenant_validation{
        Err(TenantValidationError::Disabled) => return Ok(RoutineProcessResult::new().set_done().add_message("Operação cancelada. Sua conta esta desativada.").to_owned()),
        Err(TenantValidationError::EmailUnconfirmed) => return Ok(RoutineProcessResult::new().set_done().add_message("Operação cancelada. Seu email encontra-se pendente de confirmação.").to_owned()),
        _ => ()
    }

    moma_auth::update_tenant_interaction(&tenant_key).await?;
    
    let mut routine = routine.unwrap();
    routine.set_tenant_key(tenant_key.to_owned());
    routine.from_json(&flow.routine_data);

     let result = routine.process_input(message).await.map_or_else(
        |res| {
            //Todo: log error
            println!("Routine process error: {}", res.to_string());
            routine_process_result::get_process_result_error()
        },
        |res| {
           res
        });

    println!("Flow control result:{}", result.status.to_string());
    
    flow.last_iteration_on = chrono::Local::now().naive_local();
    flow.routine_data = routine.into_json();
    flow.routine_key = routine.get_key().to_owned();
    let db = &connectionfactory::get_connection().await?;
    if [RoutineStatus::Canceled, RoutineStatus::Done].contains(&result.status) {
        _ = repository::insert_log(&routine_flow_control_log::from_rfc(&flow, &result.status), db).await?;
        _ = repository::delete(&flow, db).await?;
        return Ok(result);
    }

    flow.due_on = flow.last_iteration_on + Duration::seconds((*routine.get_lifetime()).into());
    if flow.not_saved {       
        let _ = repository::insert(&flow, &db).await?;
    } else {
        let _ = repository::update(&flow, &db).await?;
    }
    
    Ok(result)
}


async fn get_tenant_flow(tenant_key:  &String) -> Result<Option<RFC>, RoutineErrors> {
    let db = &connectionfactory::get_connection().await?;

    repository::find_tenant_key(tenant_key,
                                db
                                ).await
}