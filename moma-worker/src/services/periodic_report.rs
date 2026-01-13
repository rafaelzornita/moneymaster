use std::collections::HashMap;

use chrono::{Local, NaiveDate};
use moma_core::domain::EntryRepo::find_by_date_and_operation;
use moma_integration::whatsapp;

pub async fn send_tenants_periodic_report(start_date: NaiveDate) {
    
    let end_date = Local::now().naive_local().date();
    
    println!("Generating periodic report from {} to {}", start_date, end_date);

    let tenants = 
    match moma_auth::get_tenants().await {
        Err(e) => {
            println!("Error retrieving tenants: {}", e.to_string());
            return;
        }
        Ok(tenants) => tenants
    };        
    let days = end_date.signed_duration_since(start_date).num_days();
    for tenant in tenants.iter().filter(|t| t.enabled) {
        
        let mut has_credits = true;
        let mut has_debits = true;

        //Revenues
        let mut report = format!("Olá! O seu resumo de movimentação dos últimos {days} dias chegou.\n");
        match generate_periodic_report(start_date, end_date, "C",&tenant.key).await
        {
            None => has_credits = false,
            Some(r) => {
                    report.push_str("*Entradas*\n"); 
                    report.push_str(&r);
                }
        }
        
        //Debits
        match generate_periodic_report(start_date, end_date, "D",&tenant.key).await
        {
            None => has_debits = false,
            Some(r) => {
                if has_credits {
                    report.push_str("\n");
                }
                report.push_str("*Saídas*\n"); 
                report.push_str(&r);
            }
        }

        if !has_credits && !has_debits {
            report.push_str("\n_Não encontramos nenhum registro para o período. Aproveite para registrar sua movimentação financeira e tenha um ótimo dia!_")
        }

        match whatsapp::gateway::send_text(&tenant.key, &report).await {
            Err(e) => println!("tenants_periodic_report: WhatsApp Error: {}",e.to_string()),
            Ok(()) => println!("tenants_periodic_report: WhatsApp OK;")
        } 
    }

}

pub async fn generate_periodic_report(start_date: NaiveDate, end_date: NaiveDate, signal_operation: &str, tenant_key: &str) -> Option<String> {

    let tenant_db = moma_auth::get_tenant_connection(&tenant_key).await.unwrap();    
    let data = find_by_date_and_operation(start_date, end_date, signal_operation, &tenant_db).await;

    if data.is_empty() {
        return None;
    }

    let mut map: HashMap<String, f32> = HashMap::new();
    
    //group by name and sum values
    for item in data {
        let entry = map.entry(item.category.name).or_insert(0.0);
        *entry += item.value;
        
    }
    let total:f32 = map.values().sum();

    let mut res = map.into_iter()
                            .map(|(name, value)| format!("{}: {:.2}", name, value))
                            .collect::<Vec<String>>()
                            .join("\n");

    
    
    res.push_str(&format!("\n*Total {:.2}*\n", total));
    
    Some(res)
}
