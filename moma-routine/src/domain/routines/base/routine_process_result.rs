use strum_macros::Display;

#[derive(Clone, Debug, PartialEq, Default)]
pub struct RoutineProcessResult{
    pub status: RoutineStatus,
    pub user_messages: Vec<String>
}


#[derive(Clone, Debug, PartialEq, Default, Display)]
pub enum RoutineStatus{
    Canceled,
    Done,
    #[default]
    Partial,
}

impl RoutineProcessResult {
    pub fn new() -> Self {
        RoutineProcessResult {
            ..Default::default()
        }
    }
    pub fn set_partial(&mut self) -> &mut Self {
        self.status = RoutineStatus::Partial;
        self
    }
    pub fn set_done(&mut self) -> &mut Self {
        self.status = RoutineStatus::Done;
        self
    }
    pub fn set_canceled(&mut self) -> &mut Self {
        self.status = RoutineStatus::Canceled;
        self
    }
    pub fn add_message(&mut self, message: &str) -> &mut Self {
        self.user_messages.push(message.to_owned());
        self
    }
    pub fn get_user_messages(&self) -> String {
        self.user_messages.join("\n")
    }
}

pub fn get_process_result_error() -> RoutineProcessResult {
    RoutineProcessResult {
        status : RoutineStatus::Canceled,
        ..Default::default()
    }
    .add_message("Houve um problema e tivemos que cancelar o processo.")
    .to_owned()
}