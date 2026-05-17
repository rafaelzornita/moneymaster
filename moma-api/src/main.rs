use std::sync::{Arc, Mutex};
use actix_web::{web, App, HttpServer};
use coi::container;
use coi_actix_web::inject;
use moma_shared::messages::whatsapp_message::WppWhatsAppMessage;
use services::{amqp_service::{AmqpService, AmqpServiceProvider}, settings_service::{self, SettingsApiService, SettingsServiceProvider}};

mod services;
mod routes;

#[tokio::main]
async fn main() {

    _ = dotenv::dotenv().unwrap();

    //Initialize the services
    let settings_service = settings_service::new();
    let amqp_gat = moma_integration::amqp::gateway_factory::get_new_gateway(&settings_service.amqp_uri).await
        .map_or_else(|e| {println!("{}", e); panic!("Could not initialize the Amqpgateway for Moma API.")}, |o| o);

    let amqp_provider = AmqpServiceProvider::new(Arc::new(Mutex::new(amqp_gat)));
    let settings_service = SettingsServiceProvider::new(Arc::new(settings_service));
    
    let container = container! {
        settings => settings_service; singleton,
        amqp => amqp_provider; singleton
    };
    
    //Initialize http server
    let serve_build = HttpServer::new(move || {
        App::new()
            .app_data(container.clone()) // Register the first dependency
            .route("/WppWhatsApp", web::post().to(wppwhatsapp_handler))
            .route("/status", web::get().to(status))
    }).keep_alive(std::time::Duration::from_secs(60)).bind("0.0.0.0:8081");

    if let Err(e) = serve_build {
        println!("API initializing binding: {}", e)
    } else if let Ok(serve) = serve_build {
        if let Err(e) = serve.run().await{
            println!("API Running error: {}", e);
        }
    }

    //inject the amqp gateway 
    //https://github.com/Nashenas88/coi-actix-sample/blob/main/src/routes/data.rs
}
async fn status(_req: actix_web::HttpRequest) -> &'static str {
    println!("GET: Status");
    "I'm online!"
}

#[inject()]
async fn wppwhatsapp_handler(message: web::Json<WppWhatsAppMessage>, #[inject] amqp: Arc<AmqpService>,
                          #[inject] settings: Arc<SettingsApiService>) -> Result<String, actix_web::Error> {
    
    let default_reponse = Ok(String::new());
    let mut is_valid : bool = true;

    let raw_message = message.0.clone();

    is_valid = is_valid && raw_message.msg_type.map_or(false, |message| message.eq("chat"));
    is_valid = is_valid && raw_message.event.map_or(false, |event| event.eq("onmessage"));
    is_valid = is_valid && raw_message.from.clone().map_or(false, |user| user.ends_with("@c.us") || user.ends_with("@lid"));
    //Dont validate in field because sometimes can be "out"

    println!("Msg valid? {is_valid} from: {:?}", &raw_message.from);
    if !is_valid {
        return default_reponse;
    }
    
    amqp.publish_obj(&settings.amqp_exchange_name, &settings.amqp_whatsapp_routing_key, &message).await;

    Ok("".to_string())
}