use async_trait::async_trait;
use chrono::NaiveDate;
use handlebars::Handlebars;
use moma_integration::ai::types::AiPayload;
use moma_shared::resources::get_ai_resource;
use moma_shared::resources::AIResources;
use sea_orm::Condition;
use sea_orm::ConnectionTrait;
use sea_orm::EntityTrait;
use sea_orm::QueryFilter;
use sea_orm::QueryTrait;
use serde::{Serialize, Deserialize};
use serde_json::json;
use crate::domain::routines::base::routine_process_result;
use crate::domain::routines::base::routine_process_result::RoutineProcessResult;
use crate::domain::routines::base_routine::*;
use crate::domain::routines::routine_trait;
use crate::error::RoutineErrors;
use sea_orm::sea_query::Expr;

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct EntryFind {
    base: BaseRoutine,
    data: EntryRoutineData,
    current_step: u8
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
struct EntryRoutineData {
    tenant_key: String
}                        

#[derive(Deserialize)]
struct AiResponse{
    #[serde(default)]
    pub filter : Vec<AiResponseFilter>,
    #[serde(default)]
    pub canceled: bool,
    #[serde(default)]
    pub user_answare: String
}

#[derive(Deserialize)]
struct AiResponseFilter {
    pub title: Option<String>,
    pub value: Option<f32>,
    pub initial_date: Option<NaiveDate>,
    pub final_date: Option<NaiveDate>,
    pub operation: Option<String>,
    pub categories: Option<Vec<u8>>,
    pub observation: Option<String>,
}

#[async_trait]
impl routine_trait::Routine for EntryFind {
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
        
        //Request to IA
        let response = ai_completion(&self.data.tenant_key,&payload).await?;
        if let Some(resp) = response {
            if resp.canceled {
                return Ok(RoutineProcessResult::new()
                    .set_canceled()
                    .add_message(&resp.user_answare).to_owned());
            }

            if let Some(response) = ai_completion_data_analysis(&self.data.tenant_key, &payload, &resp).await? {
                return Ok(RoutineProcessResult::new() 
                    .set_done()
                    .add_message(&response.user_answare).to_owned());
            }
        } else{
           return Ok(routine_process_result::get_process_result_error().to_owned());
        }
        
        self.process().await
    }

    async fn process(&mut self) -> Result<RoutineProcessResult, RoutineErrors> {
        Ok(RoutineProcessResult::new()
        .set_done()
        .to_owned())
    }

    fn can_undo(&self) -> &bool {
        &false
    }

    async fn undo(&mut self) -> Result<RoutineProcessResult, RoutineErrors>{        
        Ok(RoutineProcessResult::new()
        .set_done()
        .to_owned())
    }
}

impl EntryFind {
    pub fn new() -> Self {
        EntryFind {
           base : BaseRoutine {
               name : "Entry Find".to_owned(),
               description: "Find and list entries".to_owned(),
               key : "entry_find".to_owned(),
               lifetime : 1 * 60 //to seconds
           },
           current_step : 1,
           ..Default::default()
        }
    }
}

async fn get_ai_find_template(tenant_key : &String) -> Result<String, RoutineErrors> {
    let mut reg = Handlebars::new();
    
    let categories = get_tenant_categories_as_string(&tenant_key).await?;   
    let data = &json!({"today": chrono::Local::now().naive_local().date().to_string(), "categories": &categories}); //, "previous_json": &previous_json
    println!("data:{}", data);

    reg.register_template_string("entry_find", get_ai_resource(AIResources::EntryFind)).unwrap();

    return Ok(reg.render("entry_find",data).unwrap());
}

async fn get_ai_find_datanalysis_template(tenant_key : &String, filters : &Vec<AiResponseFilter>) -> Result<String, RoutineErrors> {
    let mut reg = Handlebars::new();
    
    let categories = get_tenant_categories_as_string(&tenant_key).await?;   
    let entries = get_tenant_entries_as_string(&tenant_key, &filters).await?;
    let data = &json!({"today": chrono::Local::now().naive_local().date().to_string(), "categories": &categories, "data": &entries}); //, "previous_json": &previous_json
    println!("data:{}", data);

    reg.register_template_string("entry_find", get_ai_resource(AIResources::EntryFindDataAnalysis)).unwrap();

    return Ok(reg.render("entry_find",data).unwrap());
}

async fn ai_completion(tenant_key : &String, user_input : &String) -> Result<Option<AiResponse>, RoutineErrors> {
    
    let pv_data = get_ai_find_template(tenant_key).await?;
    println!("pv: {}", pv_data);
    println!("ui: {}", user_input.to_string());

    let payload = AiPayload {
        input_message: user_input,
        system_message : &pv_data
    };
    
    let mut response : Option<AiResponse> = None;
    for tries in 1..=3{        
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

async fn ai_completion_data_analysis(tenant_key : &String, user_input : &String, ai_response: &AiResponse) -> Result<Option<AiResponse>, RoutineErrors> {
    
    let pv_data = get_ai_find_datanalysis_template(tenant_key, &ai_response.filter).await?;
    println!("pv: {}", pv_data);
    println!("ui: {}", user_input.to_string());

    let payload = AiPayload {
        input_message: user_input,
        system_message : &pv_data
    };
    
    let mut response : Option<AiResponse> = None;
    for tries in 1..3{        
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

async fn get_tenant_entries_as_string(tenant_key: &String, filters : &Vec<AiResponseFilter>) -> Result<String, RoutineErrors> {
    use moma_core::domain::EntryEntity as ee;
    
    let db = moma_auth::get_tenant_connection(&tenant_key).await?;
    let mut finder = ee::find();

    let mut or_condition = sea_orm::Condition::any();//OR Statement
    for filter in filters {

        or_condition = or_condition.add(get_query_parameter(&filter).await);
    }

    finder = finder.filter(or_condition);
    println!("{}", finder.build(db.get_database_backend()).sql);

    let entries = finder.all(&db).await?;
    let mut entries_as_string : String = 
        if entries.len() > 0 {
            //adds header
            "id;name;date;category_id;operation;value;observation;\n".to_string()
        } else {
            "No data was found".to_string()
        };

    let formated_entries= entries.iter().map(|item|{
        let mut res = "".to_string(); 
        res.push_str(&item.id.to_string()); 
        res.push(';');
        res.push_str(&item.name);
        res.push(';');
        res.push_str(&item.date.to_string());
        res.push(';');
        res.push_str(&item.category_id.to_string());
        res.push(';');
        res.push_str(&item.operation);
        res.push(';');
        res.push_str(&item.value.to_string());
        res.push(';');
        res.push_str(&item.observation);
        res.push('\n');
        res
    }).collect::<String>();
    
    entries_as_string.push_str(&formated_entries);

    Ok(entries_as_string)
}

async fn get_query_parameter(filter: &AiResponseFilter) -> Condition {
    use moma_core::domain::entries::entry as ee;

    let mut and_condition = sea_orm::Condition::all();

    if let Some(title) = &filter.title {
        and_condition = and_condition.add(Expr::col(ee::Column::Name).like(format!("%{}%", title)));       
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
    if let Some(observation) = &filter.observation {
        print!("obs:'{}'", observation);
        and_condition = and_condition.add(Expr::col(ee::Column::Observation).like(format!("%{}%", observation)));
    }

    and_condition
}


async fn get_tenant_categories_as_string(tenant_key: &String) -> Result<String, RoutineErrors> {
    use moma_core::domain::CategoryEntity as ce;
    use sea_orm::sea_query::Expr;
    
    let db = moma_auth::get_tenant_connection(&tenant_key).await?;

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

