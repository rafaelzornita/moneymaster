use std::vec;
use async_trait::async_trait;
use chrono::NaiveDate;
use handlebars::Context;
use handlebars::Handlebars;
use handlebars::Helper;
use handlebars::HelperResult;
use handlebars::Output;
use handlebars::RenderContext;
use moma_core::domain::CategoryEntity as ce;
use moma_core::domain::Entry;
use moma_core::domain::EntryRepo;
use moma_integration::ai::types::AiPayload;
use moma_shared::resources::get_ai_resource;
use moma_shared::resources::AIResources;
use sea_orm::EntityTrait;
use sea_orm::QueryFilter;
use serde::{Serialize, Deserialize};
use serde_json::json;
use crate::domain::routines::base::routine_process_result::RoutineProcessResult;
use crate::domain::routines::base_routine::*;
use crate::domain::routines::routine_trait;
use crate::error::RoutineErrors;
use moma_core::domain::Entry as EntryM;

use super::utils::entry_utils::get_user_formated_entry;

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct EntryInsert {
    base: BaseRoutine,
    data: EntryRoutineData,
    current_step: u8
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
struct EntryRoutineData {
    tenant_key: String,
    entries : Vec<EntryData>,
    saved_entries : Vec<EntryData>
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
struct EntryData {
    id: u32,
    title: String,
    value: f32,
    date: NaiveDate,
    observation: String,
    operation: String,
    category_id: u8
}

#[derive(Deserialize)]
struct AiResponse{
    pub title: String,
    pub value: f32,
    pub date:	NaiveDate,
    pub operation: String,
    pub category_id: u8,
    pub observation: String,
    pub canceled: bool
}

impl EntryData {
    pub fn validate(&self) -> (bool, String) {
        let mut invalid_fields : String = "".to_owned();
        if self.title.is_empty() {
            invalid_fields = format!(" o titulo para o lançamento de R$ {}", &self.value);
        }
        if self.value == 0.0 {
            invalid_fields = format!(" o valor do lançamento {}", &self.title);
        }

        (invalid_fields.len() == 0, invalid_fields)
    }

}

#[async_trait]
impl routine_trait::Routine for EntryInsert {
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
        
        //When current step > 1 we have some stored data 
        let prev_json = if self.current_step > 1 {"".to_owned()} else {serde_json::to_string(&self.data.entries)?};
        let mut valid_entries : Vec<EntryData> = vec![];
        //Request to IA
        let response = ai_completion(&prev_json, &self.data.tenant_key,&payload).await?;
        if let Some(entries) = response {
            for item in entries {
                if !item.canceled {
                    valid_entries.push(EntryData {
                        id: 0,
                        category_id:item.category_id,
                        date : item.date,
                        observation : item.observation,
                        title : item.title,
                        value : item.value,
                        operation : item.operation
                    });                    
                }
            }
        }
        let mut invalid_entry_count : u8 = 0;
        let mut user_message = "Informe ".to_owned();
        for entry in &valid_entries {
            //verificar se todos os campos estão preenchidos
            let (success, message) = entry.validate();
            if !success {
                invalid_entry_count += 1;
                user_message.push_str(&message);
            }
        };
        self.data.entries = valid_entries;
        if invalid_entry_count > 1 {
            //if there is more than one invalid entry, cancel the operation
            return Ok(RoutineProcessResult::new()
                    .set_canceled()
                    .add_message("Verificamos que foi informado mais de um lançamento, porém tivemos que cancelar a operação pois as informações fornecidas não foram suficientes para preenche-los corretamente. Sugerimos que informe ao menos descrição e valor para cada lançamento.")
                    .to_owned());
        }else if invalid_entry_count == 1 {
            //if there is just one invalid entry, suggest user to correct
            return Ok(RoutineProcessResult::new()
                    .set_partial()
                    .add_message(&user_message).to_owned());
        }
        
        self.process().await
    }

    async fn process(&mut self) -> Result<RoutineProcessResult, RoutineErrors> {

        let mut str_entries = vec!(String::new());
        for item in &self.data.entries {
            let res = self.save_entry(&item).await?;
            self.data.saved_entries.push(res.0);
            str_entries.push(get_user_formated_entry(&res.1, true, "; "));
        }
        
        Ok(RoutineProcessResult::new()
        .add_message("Registrado:")
        .add_message(&str_entries.join("\n"))
        .set_done()
        .to_owned())
    }

    fn can_undo(&self) -> &bool {
        &true
    }

    async fn undo(&mut self) -> Result<RoutineProcessResult, RoutineErrors>{
        use rusty_money::{Money, iso};
        if self.data.saved_entries.len() == 0 {
            return Ok(RoutineProcessResult::new()
                .set_done()
                .add_message("Não encontramos nada para desfazer na última operação.")
                .to_owned());
        }

        let mut message = String::from("Removidos o(s) iten(s):");

        for item in &self.data.saved_entries {

            self.delete_entry(item).await?;

            message.push_str("\n");
            message.push_str(&item.title);
            message.push_str(" - ");
            message.push_str(&Money::from_str(&item.value.to_string(),iso::BRL).unwrap().to_string());
        };

        Ok(RoutineProcessResult::new()
        .set_done()
        .add_message(&message)
        .to_owned())
    }
}

impl EntryInsert {
    pub fn new() -> Self {
        EntryInsert {
           base : BaseRoutine {
               name : "Entry Insert".to_owned(),
               description: "Register entries".to_owned(),
               key : "entry_insert".to_owned(),
               lifetime : 1 * 60 //to seconds
           },
           current_step : 1,
           ..Default::default()
        }
    }

    async fn save_entry(&self, entry: &EntryData) -> Result<(EntryData, EntryM), RoutineErrors> {
        
        let mut entry = entry.to_owned();

        let db =moma_auth::get_tenant_connection(&self.data.tenant_key).await?;
        let mut model = Entry {
            name: entry.title.to_owned(),
            operation: entry.operation.to_owned(),
            category_id: entry.category_id.to_owned(),
            category : ce::find_by_id(entry.category_id).one(&db).await?.unwrap(),
            date: entry.date.to_owned(),
            value: entry.value.to_owned(),
            observation: entry.observation.to_owned(),
            ..Default::default()
        };

        EntryRepo::save(&mut model, &db).await?;
        
        entry.id = model.id;

        Ok((entry, model))
    }

    async fn delete_entry(&self, entry: &EntryData) -> Result<(), RoutineErrors> {
        
        let db = moma_auth::get_tenant_connection(&self.data.tenant_key).await?;
        EntryRepo::delete_by_id(&entry.id, &db).await?;
        
        Ok(())
    }   
}

async fn ai_completion(previous_json : &String, tenant_key : &String, user_input : &String) -> Result<Option<Vec<AiResponse>>, RoutineErrors> {

    let mut reg = Handlebars::new();
    
    let categories = get_tenant_categories_as_string(&tenant_key).await?;
    reg.register_helper("previous_json", Box::new(|_: &Helper, _: &Handlebars, _: &Context, _: &mut RenderContext, out: &mut dyn Output| -> HelperResult {
        let previous_json = if previous_json.is_empty() {"".to_owned()} else {"This is the previous user informed data. Use value of this fields as default value to the response json, including 'date' field \n".to_owned() + &previous_json};
        out.write(&previous_json)?;
        Ok(())
    }));
    
    let data = &json!({"today": chrono::Local::now().naive_local().date().to_string(), "categories": &categories}); //, "previous_json": &previous_json
    println!("data:{}", data);

    reg.register_template_string("entry_insert", get_ai_resource(AIResources::EntryInsert)).unwrap();
    let system_prompt = reg.render("entry_insert",data).unwrap();
    println!("system prompt: {}", system_prompt);
    println!("User prompt: {}", user_input.to_string());
    let payload = AiPayload {
        input_message: user_input,
        system_message : &system_prompt
    };
    
    let mut response : Option<Vec<AiResponse>> = None;
    for tries in 1..4{        
        response = moma_integration::ai::gateway::send_simple(&moma_shared::settings::get_aiconfig(), &payload).await?;
        if response.is_some() {
            break;
        } else {
            if tries > 1 {
                return Ok(None);
            }
        }        
    }
    
    Ok(response)
}

async fn get_tenant_categories_as_string(tenant_key: &String) -> Result<String, RoutineErrors> {
    use sea_orm::sea_query::Expr;
    
    let db = moma_auth::get_tenant_connection(&tenant_key).await?;
    println!("Busca categorias");
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

