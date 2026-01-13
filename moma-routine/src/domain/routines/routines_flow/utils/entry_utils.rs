use moma_core::domain::Entry as EntryM;
use rusty_money::{iso, Money};

pub fn get_user_formated_entry(entry : &EntryM, use_category_name : bool, separator : & str) -> String{
    let mut res = "".to_string(); 
    res.push_str(&entry.id.to_string()); 
    res.push_str(separator);
    res.push_str(&entry.name);
    res.push_str(separator);
    res.push_str(&entry.date.to_string());
    res.push_str(separator);

    if use_category_name {
        res.push_str(&entry.category.name.to_string());
        res.push_str(separator);
    }

    res.push_str(&entry.operation);
    res.push_str(separator);
    res.push_str(&Money::from_str(&entry.value.to_string(),iso::BRL).unwrap().to_string());
    res.push_str(separator);
    res.push_str(&entry.observation);
    res.push('\n');
    res
}
