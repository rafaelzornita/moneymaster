use async_trait::async_trait;
use chrono::NaiveDate;
use handlebars::Handlebars;
use moma_core::domain::Entry as EntryM;
use moma_core::domain::EntryRepo;
use moma_integration::ai::types::AiRoledPayload;
use moma_shared::resources::get_ai_resource;
use moma_shared::resources::AIResources;
use sea_orm::Condition;
use sea_orm::ConnectionTrait;
use sea_orm::EntityTrait;
use sea_orm::QueryFilter;
use sea_orm::QueryTrait;
use serde::de::DeserializeOwned;
use serde::{Serialize, Deserialize};
use serde_json::json;
use crate::domain::routines::base::routine_process_result;
use crate::domain::routines::base::routine_process_result::RoutineProcessResult;
use crate::domain::routines::base_routine::*;
use crate::domain::routines::routine_trait;
use crate::domain::routines::routine_trait::Routine;
use crate::error::RoutineErrors;
use sea_orm::sea_query::Expr;
use super::utils::entry_utils::get_user_formated_entry;

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct EntryDelete {
    base: BaseRoutine,
    data: EntryRoutineData,
    current_step: u8
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
struct EntryRoutineData {
    tenant_key: String,
    ai_history: Vec<EntryAiHistory>,
    selected_entries : Vec<EntryM>,
    found_entries : Vec<EntryM>,
    deleted_entries : Vec<EntryM>
}                        

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct EntryAiHistory {
    role: String,
    message: String
}   

#[derive(Serialize, Deserialize)]
struct AiResponse{
    #[serde(default)]
    pub filter : Vec<AiResponseFilter>,
    #[serde(default)]
    pub canceled: bool,
    #[serde(default)]
    pub user_answare: String
}

#[derive(Serialize, Deserialize)]
struct AiResponseFilter {
    pub description: Option<String>,
    pub value: Option<f32>,
    pub initial_date: Option<NaiveDate>,
    pub final_date: Option<NaiveDate>,
    pub operation: Option<String>,
    pub categories: Option<Vec<u8>>,
    
}

#[derive(Serialize, Deserialize)]
struct AiResponseConfirmation{
    #[serde(default)]
    pub entries : Vec<u32>,
    #[serde(default)]
    pub user_answare: String,
    #[serde(default)]
    pub canceled: bool,
    #[serde(default)]
    pub confirmed_by_user: bool,
}

#[async_trait]
impl routine_trait::Routine for EntryDelete {
    fn get_name(&self) -> &String {
        &self.base.name
    }
    fn get_key(&self) -> &String {
        &self.base.key
    }
    fn get_lifetime(&self) -> &u16 {
        &self.base.lifetime
    }
    fn into_json(&self) -> String {
        serde_json::to_string(&self).map_or("".to_string(), |v| v.to_string())
    }
    fn from_json(&mut self, json: &String) {
        if json.is_empty() { return; }

        let x: Self = serde_json::from_str(json).unwrap();
        x.clone_into(self);
    }
    fn set_tenant_key(&mut self, tenant_key: String){
        self.data.tenant_key = tenant_key;
    }
    /// Cannot be selected by the user
    fn is_internal_porpouse(&self) -> &bool {
        &false
    }
    async fn process_input(&mut self, payload: &String) -> Result<RoutineProcessResult, RoutineErrors> {
        match &self.current_step {
            1 => self.process_step1(payload).await,
            2 => self.process_data_analysis_step(payload).await,
            3 => self.process_confirmation_step(payload).await,
            _ => Ok(routine_process_result::get_process_result_error())
        }        
    }

    async fn process(&mut self) -> Result<RoutineProcessResult, RoutineErrors> {

        for item in &self.data.selected_entries {
            self.data.deleted_entries.push(self.delete_entry(&item.id).await?);
        }
        
        if self.data.deleted_entries.len() == 0 {
            return Ok(RoutineProcessResult::new()
                .set_done()
                .add_message("Nenhum lançamento foi excluido.")
                .to_owned());
        }

        let mut message = String::from("Removidos o(s) iten(s):");

        for item in &self.data.deleted_entries {
            message.push_str("\n");
            message.push_str(&get_user_formated_entry(&item, true, " - "));
        };

        Ok(RoutineProcessResult::new()
        .set_done()
        .add_message(&message)
        .to_owned())

    }

    fn can_undo(&self) -> &bool {
        &true
    }

    async fn undo(&mut self) -> Result<RoutineProcessResult, RoutineErrors>{        
        if self.data.deleted_entries.len() == 0 {
            return Ok(RoutineProcessResult::new()
                .set_done()
                .add_message("Não encontramos nada para desfazer na última operação.")
                .to_owned());
        }

        let mut message = String::from("Reinseridos o(s) iten(s):");

        for item in &self.data.deleted_entries {

            self.save_entry(item).await?;
            message.push_str("\n");
            message.push_str(&get_user_formated_entry(&item, true, " - "));

        };

        Ok(RoutineProcessResult::new()
        .set_done()
        .add_message(&message)
        .to_owned())
    }
}

impl EntryDelete {
    pub fn new() -> Self {
        EntryDelete {
           base : BaseRoutine {
               name : "Entry Delete".to_owned(),
               description: "Delete entries".to_owned(),
               key : "entry_delete".to_owned(),
               lifetime : 1 * 60 //to seconds
           },
           current_step : 1,
           ..Default::default()
        }
    }

    fn next_step(&mut self) {
        self.current_step += 1;
        println!("Set step: {}", &self.current_step);
    }

    fn add_ai_chat_history(&mut self, system_message : &str, user_message : &str, assistant_message: &str) {
        if !system_message.is_empty() {self.data.ai_history.push(EntryAiHistory {role: "system".to_owned(), message: system_message.to_owned()});}
        if !user_message.is_empty() {self.data.ai_history.push(EntryAiHistory {role: "user".to_owned(), message: user_message.to_owned()});}
        if !assistant_message.is_empty() {self.data.ai_history.push(EntryAiHistory {role: "assistant".to_owned(), message: assistant_message.to_owned()});}
    }

    async fn process_step1(&mut self, payload : &str) -> Result<RoutineProcessResult, RoutineErrors> {

        //Request to AI params do filter data
        let template = &get_ai_delete_template(&self.data.tenant_key).await?;
        let response : Option<AiResponse> = ai_completion( 
                            template,
                            payload, None).await?;

        if response.is_none() {
            return Ok(routine_process_result::get_process_result_error());
        }                  
        
        let resp = response.unwrap();
        self.add_ai_chat_history(&template,&payload, &serde_json::to_string(&resp).map_or("".to_string(), |v| v.to_string()));

        if resp.canceled {
            return Ok(RoutineProcessResult::new()
                .set_canceled()
                .add_message(if resp.user_answare.is_empty() {"Processo cancelado."} else {&resp.user_answare}).to_owned());
        }
 
        let entries = get_tenant_entries(&self.data.tenant_key, &resp.filter).await?;
        self.data.found_entries = entries.clone();
        if entries.len() ==0 {
             //If there are no entries, finish the process
             let message = "Nenhum registro encontrado a patir das informações fornecidas.";
             self.add_ai_chat_history(&message,"", "");
             return Ok(RoutineProcessResult::new() 
                 .set_done()
                 .add_message(message)
                 .to_owned());

        }else if entries.len() > 1 {
            //If there are more than 1 entry, send to AI for data analysis
            println!("*** Set data analysis step: {}", &self.current_step);
            self.next_step();
            return self.process_data_analysis_step(payload).await;
        }

        //If there is only one entry, select it        
        println!("*** Set confirmation step: {}", &self.current_step);
        self.data.selected_entries = entries.clone();
        
        //Skip data analysis step, goes to confirmation step
        self.current_step = 3;

        let formated_entries= &self.data.selected_entries.iter().map(|item|{
            get_user_formated_entry(&item,true, " - ")
        }).collect::<String>();

        //ask for confirmation
        let message = "Confirma a exclusão do registro a seguir?\n".to_string() + &formated_entries;            
        self.add_ai_chat_history(&message,"", "");

        return Ok(RoutineProcessResult::new() 
            .set_partial()
            .add_message(&message)
            .to_owned());
          
    }

    async fn process_data_analysis_step(&mut self, payload: &str) -> Result<RoutineProcessResult, RoutineErrors> {
        println!("*** Set data analysis step: {}", &self.current_step);
        //Completion to AI with data analysis
        let template = &get_ai_delete_data_analysis_template(&self.data.tenant_key, &self.data.found_entries).await?;
        let response: Option<AiResponseConfirmation> = ai_completion(
            &template,
            payload, None).await?;
        let response = response.unwrap();
        
        if !response.entries.is_empty()  {            
            let db = moma_auth::get_tenant_connection(&self.data.tenant_key).await?;
            self.data.selected_entries = EntryRepo::find_ids(&response.entries, &db).await;
            
            self.next_step();
            //return self.process_confirmation_step(payload).await;
        }

        self.add_ai_chat_history(&template,"", &serde_json::to_string(&response).map_or("".to_string(), |v| v.to_string()));
        //send AI response
        return Ok(RoutineProcessResult::new() 
            .set_partial()
            .add_message(&response.user_answare).to_owned());

    }

    //Confirmation step
    async fn process_confirmation_step(&mut self, payload: &str) -> Result<RoutineProcessResult, RoutineErrors> {

        //Request to AI
        let template = get_ai_delete_confirmation_template(&self.data.tenant_key,  &self.data.selected_entries).await?;
        let response : Option<AiResponseConfirmation> = ai_completion( 
            &template,
            payload, None).await?;

        if let Some(response) = response {
            self.add_ai_chat_history(&template,payload, &serde_json::to_string(&response).map_or("".to_string(), |v| v.to_string()));
            if response.canceled {
                println!("canceled");
                return Ok(RoutineProcessResult::new() 
                    .add_message(if response.user_answare.is_empty() {"Processo cancelado."} else {&response.user_answare} )
                    .set_canceled()
                    .to_owned());
            }
            
            if response.entries.len() == 0 {
                println!("No items selected");
                return Ok(RoutineProcessResult::new() 
                    .add_message(if response.user_answare.is_empty() {"Nenhuma entrada foi selecionada para exclusão."} else {&response.user_answare})
                    .set_canceled()
                    .to_owned());
            }

            let db = moma_auth::get_tenant_connection(&self.data.tenant_key).await?;
            self.data.selected_entries = EntryRepo::find_ids(&response.entries, &db).await;

            return self.process().await;            
        }

        Ok(routine_process_result::get_process_result_error())
    }

    async fn delete_entry(&self, entry_id: &u32) -> Result<EntryM, RoutineErrors> {
        
        let db = moma_auth::get_tenant_connection(&self.data.tenant_key).await?;
        let entry = EntryRepo::find_id(entry_id.to_owned(), &db).await.expect("Required entry id was not found.");
        
        EntryRepo::delete_by_id(&entry_id, &db).await?;
        
        Ok(entry)
    }   

    async fn save_entry(&self, entry: &EntryM) -> Result<(), RoutineErrors> {
        
        let entry = entry.clone().get_active_model();
        let db =moma_auth::get_tenant_connection(&self.data.tenant_key).await?;
        EntryRepo::insert(entry, &db).await?;

        Ok(())
    }
    
}

async fn get_ai_delete_confirmation_template(tenant_key : &str, entries : &Vec<EntryM>) -> Result<String, RoutineErrors> {
    let mut reg = Handlebars::new();
    
    let categories = get_tenant_categories_as_string(&tenant_key).await?;
    let entries = format_tenant_entries_as_template_string(&entries, true).await?;
    let data = &json!({"today": chrono::Local::now().naive_local().date().to_string(), "categories": &categories, "data": &entries}); //, "previous_json": &previous_json
    
    reg.register_template_string("entry_delete", get_ai_resource(AIResources::EntryDeleteConfirmation)).unwrap();

    return Ok(reg.render("entry_delete",data).unwrap());
}


async fn get_ai_delete_template(tenant_key : &str) -> Result<String, RoutineErrors> {
    let mut reg = Handlebars::new();
    
    let categories = get_tenant_categories_as_string(&tenant_key).await?;
    let data = &json!({"today": chrono::Local::now().naive_local().date().to_string(), "categories": &categories}); //, "previous_json": &previous_json
    println!("data:{}", data);

    reg.register_template_string("entry_delete", get_ai_resource(AIResources::EntryDelete)).unwrap();

    return Ok(reg.render("entry_delete",data).unwrap());
}

async fn get_ai_delete_data_analysis_template(tenant_key : &str, entries : &Vec<EntryM>) -> Result<String, RoutineErrors> {
    let mut reg = Handlebars::new();
    
    let categories = get_tenant_categories_as_string(&tenant_key).await?;
    let entries = format_tenant_entries_as_template_string(entries, false).await?;
    let data = &json!({"today": chrono::Local::now().naive_local().date().to_string(), "categories": &categories, "data": &entries}); //, "previous_json": &previous_json
    println!("data:{}", data);

    reg.register_template_string("entry_delete", get_ai_resource(AIResources::EntryDeleteDataAnalysis)).unwrap();

    return Ok(reg.render("entry_delete",data).unwrap());
}

async fn ai_completion<TResponse>(system_data: &str, user_input : &str, chat_history: Option<&Vec<EntryAiHistory>>) -> Result<Option<TResponse>, RoutineErrors> 
where TResponse : DeserializeOwned {
    
    println!("pv: {}", system_data);
    println!("ui: {}", user_input.to_string());

    let mut payload : Vec<AiRoledPayload> = vec![];
    if let Some(chat_history) = chat_history {
        for item in chat_history{
            payload.push(AiRoledPayload {role : &item.role, message: &item.message});
        }
    }

    payload.push(AiRoledPayload {
        role: "system",
        message : &system_data
    });
    payload.push(AiRoledPayload {
        role: "user",
        message : &user_input
    });
   
    for t in 1..=3 {        
        println!("AI completion try number: {}", t);
        let response = moma_integration::ai::gateway::send(&moma_shared::settings::get_aiconfig(), &payload).await;
        match response {
            Ok(Some(res)) => return Ok(res),
            Ok(None) => {
                if t == 3 {
                    return Err(RoutineErrors::Generic("Exceded number of tries to completion with AI".to_owned(), "AI".to_owned()));
                }
            }
            Err(err) => return Err(RoutineErrors::Generic(err.to_string(), "AI".to_owned())),
        }
    }

    Ok(None)
}

async fn get_tenant_entries(tenant_key : &str, filters : &Vec<AiResponseFilter>) -> Result<Vec<EntryM>, RoutineErrors>{
    use moma_core::domain::EntryEntity as ee;

    let db = moma_auth::get_tenant_connection(&tenant_key).await?;
    let mut finder = ee::find();

    let mut or_condition = sea_orm::Condition::any();//OR Statement
    for filter in filters {

        or_condition = or_condition.add(get_query_parameter(&filter).await);
    }

    finder = finder.filter(or_condition);
    println!("{}", finder.build(db.get_database_backend()).sql);

    let res =  finder.all(&db).await?;
    
    Ok(res)
}

async fn format_tenant_entries_as_template_string(entries : &Vec<EntryM>, with_row_number: bool) -> Result<String, RoutineErrors> {
    
    let mut entries_as_string : String = 
        if entries.len() > 0 {
            //add header
            ((if with_row_number {"row;"} else {""}).to_owned() + "id;name;date;category_id;operation;value;observation;\n").to_string()
        } else {
            "No data was found based on the provided data".to_string()
        };

    // let formated_entries= entries.iter().map(|item|{
    //     get_ai_formated_entry(&item,false, ";")
    // }).collect::<String>();

    let mut formated_entries = String::new();
    let mut i : u16 = 0;
    for entry in entries {
        i += 1;
        let mut entry = get_ai_formated_entry(&entry, false, ";");
        if with_row_number {
            entry.insert_str(0, &(i.to_string() + ";"));
        }
        formated_entries.push_str(&entry);
    }

    entries_as_string.push_str(&formated_entries);

    Ok(entries_as_string)
}

fn get_ai_formated_entry(entry : &EntryM, use_category_name : bool, separator : & str) -> String{
    let mut res = "".to_string(); 
    res.push_str(&entry.id.to_string()); 
    res.push_str(separator);
    res.push_str(&entry.name);
    res.push_str(separator);
    res.push_str(&entry.date.to_string());
    res.push_str(separator);

    if use_category_name {
        res.push_str(&entry.category.name.to_string());
    }else{
        res.push_str(&entry.category_id.to_string());
    }

    res.push_str(separator);
    res.push_str(&entry.operation);
    res.push_str(separator);
    res.push_str(&entry.value.to_string());
    res.push_str(separator);
    res.push_str(&entry.observation);
    res.push('\n');
    res
}


async fn get_query_parameter(filter: &AiResponseFilter) -> Condition {
    use moma_core::domain::entries::entry as ee;

    let mut and_condition = sea_orm::Condition::all();

    if let Some(description) = &filter.description {
        and_condition = and_condition.add(Expr::col(ee::Column::Name).like(format!("%{}%", description)).or(Expr::col(ee::Column::Observation).like(format!("%{}%", description))));     
    }    
    if let Some(value) = filter.value {
        and_condition = and_condition.add(Expr::col(ee::Column::Value).eq(value));
    }
    if let Some(initial_date) = filter.initial_date {
        and_condition = and_condition.add(Expr::col(ee::Column::Date).gte(initial_date));
    }
    if let Some(final_date) = filter.final_date {
        and_condition = and_condition.add(Expr::col(ee::Column::Date).lte(final_date));
    }
    if let Some(operation) = &filter.operation {
        and_condition = and_condition.add(Expr::col(ee::Column::Operation).eq(operation));
    }
    if let Some(categories) = &filter.categories {
        if categories.len() > 0 {
            let mut temp_filter  = Expr::col(ee::Column::CategoryId).eq(categories[0]);
            for category_id in categories {            
                temp_filter = temp_filter.or(Expr::col(ee::Column::CategoryId).eq(*category_id));
            };
            and_condition = and_condition.add(temp_filter);
        }
    }

    and_condition
}

// async fn get_query_parameter(filter: &AiResponseFilter, finder : sea_orm::query::Select<moma_core::domain::EntryEntity>) -> sea_orm::query::Select<moma_core::domain::EntryEntity> {
//     use moma_core::domain::entries::entry as ee;
//     let mut finder = finder;
//     if let Some(title) = &filter.title {
//         finder = finder.filter(Expr::col(ee::Column::Name).like(format!("%{}%", title)));
//     }
//     if let Some(value) = filter.value {
//         finder = finder.filter(Expr::col(ee::Column::Value).eq(value));
//     }
//     if let Some(initial_date) = filter.initial_date {
//         finder = finder.filter(Expr::col(ee::Column::Date).gte(initial_date));
//     }
//     if let Some(final_date) = filter.final_date {
//         finder = finder.filter(Expr::col(ee::Column::Date).lte(final_date));
//     }
//     if let Some(operation) = &filter.operation {
//         finder = finder.filter(Expr::col(ee::Column::Operation).eq(operation));
//     }
//     if let Some(categories) = &filter.categories {
//         finder = finder.filter(Expr::col(ee::Column::CategoryId).is_in(categories.to_owned()));
//     }
//     if let Some(observation) = &filter.observation {
//         finder = finder.filter(Expr::col(ee::Column::Observation).like(format!("%{}%", observation)));
//     }
//     finder
// }

async fn get_tenant_categories_as_string(tenant_key: &str) -> Result<String, RoutineErrors> {
    use moma_core::domain::CategoryEntity as ce;
    use sea_orm::sea_query::Expr;
    
    let db = moma_auth::get_tenant_connection(&tenant_key.to_owned()).await?;

    let categories = ce::find().filter(Expr::col(moma_core::domain::categories::category::Column::Enabled).eq(true))
        .all(&db).await?;
    let formated_categories= categories.iter().map(|item|{
        let mut res = "".to_string(); 
        res.push_str(&item.category_id.to_string()); 
        res.push(':');
        res.push_str(&item.name);
        res.push(' ');
        res
    }).collect::<String>();
    Ok(formated_categories)
}

