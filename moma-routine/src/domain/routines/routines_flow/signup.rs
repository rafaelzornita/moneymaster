use async_trait::async_trait;
use moma_integration::email;
use moma_integration::email::ContactInfo;
use moma_integration::error::IntegrationErrors;
use moma_shared::settings;
use rand::Rng;
use regex::Regex;
use serde::{Serialize, Deserialize};
use crate::domain::routines::base::routine_process_result;
use crate::domain::routines::base::routine_process_result::RoutineProcessResult;
use crate::domain::routines::base_routine::*;
use crate::domain::routines::routine_trait;
use crate::domain::routines::routine_trait::Routine;
use crate::error::RoutineErrors;

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Signup {
    base: BaseRoutine,
    data: SignupData,
    current_step: u8
}
// #[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
// pub struct SignupStep{
//     id: u8,
//     mensagem: String
// }

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
struct SignupData {
    name: String,
    email: String,
    tenant_key: String,
    confirmation_code: String
}

#[async_trait]
impl routine_trait::Routine for Signup {
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

        let x: Signup = serde_json::from_str(json).unwrap();
        x.clone_into(self);
    }
    fn set_tenant_key(&mut self, tenant_key: String){
        self.data.tenant_key = tenant_key;
    }
    /// Cannot be selected by the user or AI
    fn is_internal_porpouse(&self) -> &bool {
        &true
    }
    fn can_undo(&self) -> &bool {
        &false
    }
    async fn process_input(&mut self, payload: &String) -> Result<RoutineProcessResult, RoutineErrors> {

        if payload.to_lowercase().eq("cancelar"){
            return Ok(RoutineProcessResult::new().set_canceled().add_message("Processo de criação de conta cancelado.").to_owned());
        }
        
        match &self.current_step {
            1 => Ok(self.process_step1()),
            2 => Ok(self.process_step2(payload).await),
            // 3 => Ok(self.process_step3(payload).await),//Disabled confirmation email for now
            // 4 => self.process_step4(payload).await, 
            _ => Ok(routine_process_result::get_process_result_error())
        }
    }
    async fn process(&mut self) -> Result<RoutineProcessResult, RoutineErrors> {

        if let Err(e) = moma_auth::create_tenant(self.data.name.clone(), self.data.tenant_key.clone(), "nomail".to_owned()).await {
            //todo: log
            println!("{}", e.to_string());
            return Ok(RoutineProcessResult::new()
            .set_canceled()
            .add_message("Infelizmente tive um problema técnico e não consegui criar a sua conta. Tente novamente mais tarde!")
            .to_owned());
        }

        Ok(RoutineProcessResult::new()
            .set_done()
            .add_message(&format!("Tudo pronto, {}! Você já pode começar a efetuar lançamentos.", self.data.name))
            .add_message("Para isso, basta informar o valor e a descrição do lançamento, por exemplo: 'R$ 100,00 - Compra de pão'")
            .add_message("Opcionalmente você também pode informar a data e uma observação como por exemplo: 'Compra de pão R$ 100,00 no supermercado da esquina, ontem'")
            .add_message("Caso tenha alguma dúvida, basta perguntar!")
            .to_owned())
    }
    async fn undo(&mut self) -> Result<RoutineProcessResult, RoutineErrors>{
        Ok(RoutineProcessResult::new().set_done().to_owned())
    }
}

impl Signup {
    pub fn new() -> Self {
        Signup {
           base : BaseRoutine {
               name : "Signup".to_owned(),
               description: "Signup routine".to_owned(),
               key : "signup".to_owned(),
               lifetime : 15 * 60 //segundos
           },
           current_step : 1,
           ..Default::default()
        }
       }
    pub fn get_confirmation_code(&self) -> &String {
        &self.data.confirmation_code
    }
    fn process_step1(&mut self) -> RoutineProcessResult {   
        self.next_step();

        RoutineProcessResult::new()
            .set_partial()
            .add_message("Olá, bem vindo(a) ao assistente financeiro Money Master.\nPara começar preciso que me diga como gostaria de ser chamado(a)")
            .to_owned()
    }
    async fn process_step2(&mut self, payload: &String) -> RoutineProcessResult {
        self.data.name = payload.clone();
        self.process().await.unwrap()

        //Account Recovery mode was discontinued
        // self.next_step();

        // RoutineProcessResult::new()
        //     .add_message("Agora, informe seu email para que possamos assegurar sua identidade em caso de recuperação de conta")
        //     .to_owned()
    }
    async fn process_step3(&mut self, payload: &String) -> RoutineProcessResult {
        if moma_integration::email::validate_address(payload) == false {
            return RoutineProcessResult::new().add_message("Aparentemente o endereço não foi informado corretamente. Digite novamente seu endereço de email").to_owned();
        }
        if let Err(e) = self.send_confirmation_email(&payload).await {
            //Todo: Log
            return RoutineProcessResult::new().set_canceled().add_message("Tive um problema aqui e não consegui enviar um email de confirmação para o seu endereço").add_message("Processo encerrado.").add_message(&format!("{}",e)).to_owned();
        }

        self.data.email = payload.clone();
        self.next_step();

        RoutineProcessResult::new()
            .add_message("Informe o código de confirmação que você recebeu no endereço informado")
            .to_owned()
    }
    async fn process_step4(&mut self, payload: &String) -> Result<RoutineProcessResult, RoutineErrors> {
        
        if payload.to_lowercase().eq("reenviar"){
            if let Err(_) = self.send_confirmation_email(&self.data.email.clone()).await {
                //Todo: Log
                return Ok(RoutineProcessResult::new().set_canceled().add_message("Tive um problema aqui e não consegui enviar um email de confirmação para o seu endereço").add_message("Processo encerrado.").to_owned());
            }
        }

        if !Regex::new(r"\d+").unwrap().is_match(&payload) {
            return Ok(RoutineProcessResult::new()
            .add_message("O que nós precisamos agora é o código de verificação que voce recebeu. Se precisar reenviar o código digita 'Reenviar' ou se quiser cancelar, digita 'Cancelar'.")
            .to_owned());
        }
        if &self.data.confirmation_code != payload {
            let email = &self.data.email;
            return Ok(RoutineProcessResult::new()
            .add_message(&format!("O código de confirmação que voce informou não confere com o código enviado para o endereço {email}"))
            .to_owned());
        }

        return self.process().await;
    }

    fn next_step(&mut self) {
        self.current_step += 1;
    }

   async fn send_confirmation_email(&mut self, to_address: &String) -> Result<(), IntegrationErrors> {
        let confirmation_number = rand::thread_rng().gen_range(1001..9999).to_string();
        
        let mail_config = settings::get_email_config();
        let email_payload = email::EmailData { 
            from: ContactInfo {
                email: &mail_config.from_address,
                name: "Money Master"
             },
             to: vec![ContactInfo{
                email: to_address,
                name: "Test"
             }],
            text: "",
            subject: "Código de confirmação",
            html: &get_confirmation_email(&self.data.name,&confirmation_number, &self.data.tenant_key)?
        };

        self.data.confirmation_code = confirmation_number;
        email::send_as_html(&mail_config, &email_payload).await?;

        Ok(())
    }
    
}

fn get_confirmation_email(tenant_name: &String, confirmation_code : &String, phone_number: &String) -> Result<String, IntegrationErrors>{
    let template = moma_shared::resources::get_mail_resource(moma_shared::resources::MailResources::ConfirmationMail);
    
    Ok(template.replace("{USER}", &tenant_name)
                .replace("{CODE}", &confirmation_code)
                .replace("{PHONE}", &phone_number)
                .replace("{MESSAGE}", &confirmation_code))
}
