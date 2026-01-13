use std::sync::Arc;
use coi::{Inject, Provide};

#[derive(Inject)]
pub struct SettingsApiService {
    pub amqp_uri: String,
    pub amqp_exchange_name: String,
    pub amqp_whatsapp_routing_key: String
}

impl SettingsApiService {
    pub fn copy(settings : Arc<SettingsApiService>) -> Self {
       SettingsApiService {
           amqp_uri: settings.amqp_uri.clone(),
           amqp_exchange_name: settings.amqp_exchange_name.clone(),
           amqp_whatsapp_routing_key: settings.amqp_whatsapp_routing_key.clone()
       }
    }
}

pub fn new() -> SettingsApiService{
    let amqp_settings = moma_shared::settings::get_amqp();
    SettingsApiService{
        amqp_uri: amqp_settings.amqp_uri.clone(),
        amqp_exchange_name: amqp_settings.amqp_exchange_name.clone(),
        amqp_whatsapp_routing_key: amqp_settings.amqp_whatsapp_routing_key.clone()
        }
}

#[derive(Provide)]
#[coi(provides SettingsApiService with SettingsApiService::copy(self.0.clone()))]
pub struct SettingsServiceProvider(Arc<SettingsApiService>);

impl SettingsServiceProvider {
    pub fn new(settings: Arc<SettingsApiService>) -> Self {
        Self(settings)
    }
}
