
pub enum AIResources {
    About,
    EntryFind,
    EntryFindDataAnalysis,
    EntryInsert,
    EntryDelete,
    EntryDeleteDataAnalysis,
    EntryDeleteConfirmation,
    Greet,
    RoutineSelect,
}

pub enum MailResources {
    ConfirmationMail
}

pub fn get_mail_resource(res : MailResources) -> &'static str {
    match res {
        MailResources::ConfirmationMail => include_str!("../resources/confirmation_mail_template.html"),
    }
}

pub fn get_ai_resource(res : AIResources) -> &'static str {
    match res {
        AIResources::About => include_str!("../resources/ai_instructions/about.txt"),
        AIResources::EntryFind => include_str!("../resources/ai_instructions/entry_find_ai_input.txt"),
        AIResources::EntryFindDataAnalysis => include_str!("../resources/ai_instructions/entry_find_ai_data_analisys.txt"),
        AIResources::EntryInsert => include_str!("../resources/ai_instructions/entry_insert_ai_input.txt"),
        AIResources::EntryDelete => include_str!("../resources/ai_instructions/entry_delete_ai_input.txt"),
        AIResources::EntryDeleteDataAnalysis => include_str!("../resources/ai_instructions/entry_delete_ai_data_analisys.txt"),
        AIResources::EntryDeleteConfirmation => include_str!("../resources/ai_instructions/entry_delete_ai_confirmation.txt"),
        AIResources::Greet => include_str!("../resources/ai_instructions/greet.txt"),
        AIResources::RoutineSelect => include_str!("../resources/ai_instructions/routine_select_ai_input.txt"),
    }
}
