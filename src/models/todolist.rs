use std::{error::Error, fs};
use ratatui::widgets::TableState;

use super::todo::{Todo, Status};

pub struct Todolist{
    pub items: Vec<Todo>,
    //state: ListState
    pub state: TableState
}

impl Todolist {
    pub fn new() -> Todolist {
        Todolist{
            items: Todolist::init_todo_list(),
            //state: ListState::default()
            state: TableState::new()
        }
    }
    //return next todo which is pending and has been set a pomodoro
    fn next_pomo_todo(&self) -> Option<usize>{
        self.items.iter().position(|item|{
            item.pomo_total > Some(0) && item.state == Status::Pending
        })
    }
    pub fn update_pomo(&mut self){
        if let Some(i) = self.next_pomo_todo(){
            self.items[i].pomo_done += 1;
            if self.items[i].pomo_done >= self.items[i].pomo_total.unwrap(){
                self.items[i].toggle_state();
            }
        }
    }
    //modify todolist
    pub fn add_todo(&mut self, name: &str){
        self.items.push(Todo::new(name));
    }
    //save todolist to local file
    pub fn init_todo_list() -> Vec<Todo>{
        match fs::read_to_string(".TodoList") {
            Ok(txt) => {
                let todo_list:Vec<Todo> = serde_json::from_str(&txt).unwrap_or_default();
                todo_list
            }
            _ => Vec::new()
        }
    }
    pub fn save_to_file(&self)->Result<(), Box<dyn Error>>{
        let txt = serde_json::to_string_pretty(&self.items)?;
        fs::write(".TodoList", txt)?;
        Ok(println!("Todo list has been saved"))
    }
}