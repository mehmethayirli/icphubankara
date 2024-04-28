mod types;

use crate::types::*;

#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}


#[ic_cdk::update]
fn save_list(todo: Todo) -> String {
    return "icphub".to_string();
}