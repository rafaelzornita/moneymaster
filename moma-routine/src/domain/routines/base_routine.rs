use core::fmt;
use std::fmt::Formatter;
use serde::{Serialize, Deserialize};

/*

trait routine
    get_id -> u16
    get_name -> string
    get_key -> string
    get_lifetime -> u16
    to_json -> json (json com os dados da estrutura)
    process_input(payload) -> (status -> done,   answer)
                                        partial
                                        canceled
        -> payload pode ser um comando tipo: desfazer, cancelar, voltar
    
    -> struct with data
    -> struct to template to AI

    to_AI_json - generate the json for AI completition
*/

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct BaseRoutine {
    pub name: String,
    pub description: String,
    pub lifetime: u16,
    pub key: String
}

impl fmt::Display for BaseRoutine{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "key:{}, name:{}, ", self.key, self.name)
    } 
}