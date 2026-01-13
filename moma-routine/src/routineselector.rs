use moma_integration::ai::types::AiPayload;
use moma_shared::resources::{get_ai_resource, AIResources};
use serde::Deserialize;

use crate::domain::routines::routine_trait::*;
use crate::domain::routines::routines_flow::{entry_insert::EntryInsert, entry_find::EntryFind, entry_delete::EntryDelete, undo::Undo, about::About, greet::Greet, signup::Signup};

pub async fn select(key: &String) -> Option<Box<dyn Routine + Send>> {

    let routine = raw_select(key).await;
    if routine.is_some() {
        return routine;
    };

    //AI Completion
    let payload = AiPayload {
        input_message: key,
        system_message : get_ai_resource(AIResources::RoutineSelect)
    };

    
    //TODO: log
    for t in 1..3{        
        println!("Routine select try number: {}", t);
        let result  =  moma_integration::ai::gateway::send_simple(&moma_shared::settings::get_aiconfig(), &payload).await
        .map_or_else(|e| {println!("routine_select_ai_error:{}", e.to_string()); None}, |ok: Option<AiResponse>| ok);
        if let Some(res) = result {
            return raw_select(&res.routine_key).await;
        }
    }
            
    return None;
}

pub async fn raw_select(key: &String) -> Option<Box<dyn Routine + Send>> {
    let mut routines = get_routines().into_iter();
    let x = routines.find(|x| x.0.eq(key));
    if x.is_some() {
        return Some(x.unwrap().1);
    }
    return None;
}

pub fn get_routines() -> Vec<(String, Box<dyn Routine + Send>)> {
    let signup = Box::new(Signup::new());
    let mut vect: Vec<(String,Box<dyn Routine + Send>)> = vec![(signup.get_key().to_owned(), signup)];

    let entry_insert = Box::new(EntryInsert::new());
    vect.push((entry_insert.get_key().to_owned(), entry_insert));

    let entry_find = Box::new(EntryFind::new());
    vect.push((entry_find.get_key().to_owned(), entry_find));

    let entry_delete = Box::new(EntryDelete::new());
    vect.push((entry_delete.get_key().to_owned(), entry_delete));

    let undo = Box::new(Undo::new());
    vect.push((undo.get_key().to_owned(), undo));

    let about = Box::new(About::new());
    vect.push((about.get_key().to_owned(), about));

    let greet = Box::new(Greet::new());
    vect.push((greet.get_key().to_owned(), greet));

    return vect
}

#[derive(Deserialize)]
pub struct AiResponse{		
    pub input_is_valid: bool,				
    pub routine_key: String
}
