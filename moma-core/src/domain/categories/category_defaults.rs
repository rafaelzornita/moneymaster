
use crate::domain::categories::category::Model as CategoriaModel;

pub fn get_default_categories() -> Vec<CategoriaModel>{
    vec![CategoriaModel {
        category_id: 1,
        name : "Necessário".to_string(),
        enabled: true
    },
    CategoriaModel {
        category_id: 2,
        name : "Moradia".to_string(),
        enabled: true
    },
    CategoriaModel {
        category_id: 3,
        name : "Transporte".to_string(),
        enabled: true
    },
    CategoriaModel {
        category_id: 4,
        name : "Saúde".to_string(),
        enabled: true
    },
    CategoriaModel {
        category_id: 5,
        name : "Contas".to_string(),
        enabled: true
    },
    CategoriaModel {
        category_id: 6,
        name : "Diversão".to_string(),
        enabled: true
    },
    CategoriaModel {
        category_id: 7,
        name : "Lazer".to_string(),
        enabled: true
    },
    CategoriaModel {
        category_id: 8,
        name : "Eletro Eletrônicos".to_string(),
        enabled: true
    },
    CategoriaModel {
        category_id: 9,
        name : "Educação".to_string(),
        enabled: true
    },
    CategoriaModel {
        category_id: 10,
        name : "Trabalho".to_string(),
        enabled: true
    },
    CategoriaModel {
        category_id: 11,
        name : "Outros".to_string(),
        enabled: true
    },
    CategoriaModel {
        category_id: 12,
        name : "Vestuario".to_string(),
        enabled: true
    },
    CategoriaModel {
        category_id: 13,
        name : "Estética pessoal".to_string(),
        enabled: true
    }]
}