


use candid::{CandidType};
use std::{cell::RefCell};
use serde::Deserialize;
#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}
#[ic_cdk::update]
fn create_todo(title: String, description : String) -> String{
    let e = Todo::new(title, description);
    TODO.with(|el| el.borrow_mut().push(e));
    "todo added successfully".to_string()
}
#[ic_cdk::update]
fn mark_complete(){
    
}

thread_local! {
  
    static TODO: RefCell<Vec<Todo>> = RefCell::new(Vec::new());
}
#[derive(CandidType, Deserialize)]
pub struct Todo {
   pub is_completed : bool,
   pub title : String,
   pub description :String,
}
impl Todo {
    pub fn new(title: String, description : String) -> Self {
        Self{
            is_completed :false,
            title,
            description
        }
    }

    pub fn mark_as_complete(&mut self){
        self.is_completed=true
    }
}
