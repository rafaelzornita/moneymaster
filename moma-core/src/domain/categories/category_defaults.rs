
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
    },
    CategoriaModel {
        category_id: 14,
        name : "Alimentação".to_string(),
        enabled: true
    },
    CategoriaModel {
        category_id: 15,
        name : "Limpeza e Casa".to_string(),
        enabled: true
    },
    CategoriaModel {
        category_id: 16,
        name : "Pets".to_string(),
        enabled: true
    },
    CategoriaModel {
        category_id: 17,
        name : "Crianças e Família".to_string(),
        enabled: true
    },
    CategoriaModel {
        category_id: 18,
        name : "Seguros".to_string(),
        enabled: true
    },
    CategoriaModel {
        category_id: 19,
        name : "Impostos e Taxas".to_string(),
        enabled: true
    },
    CategoriaModel {
        category_id: 20,
        name : "Finanças".to_string(),
        enabled: true
    },
    CategoriaModel {
        category_id: 21,
        name : "Investimentos".to_string(),
        enabled: true
    },
    CategoriaModel {
        category_id: 22,
        name : "Presentes e Doações".to_string(),
        enabled: true
    },
    CategoriaModel {
        category_id: 23,
        name : "Viagens".to_string(),
        enabled: true
    },
    CategoriaModel {
        category_id: 24,
        name : "Emergências".to_string(),
        enabled: true
    }]    
}