use ratatui::{text::Line, widgets::ListItem};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq)]
pub enum Status{
    Done,
    Pending
}

#[derive(Serialize, Deserialize)]
pub struct Todo{
    pub name: String,
    pub state: Status,
    pub pomo_total: Option<u8>,
    pub pomo_done: u8,
}

impl Todo{
    pub fn new(name: &str)->Todo{
        Todo {
            name: name.to_string(), 
            state: Status::Pending,
            pomo_total: None,
            pomo_done: 0
        }
    }
    pub fn rename(&mut self, new_name: String){
        self.name = new_name;
    }
    pub fn set_pomo(&mut self, num: Option<u8>){
        if let Some(n) = num{
            self.pomo_total = if n==0{None}else{num};
        }
    }
    pub fn toggle_state(&mut self){
        self.state = match self.state{
            Status::Done => Status::Pending,
            Status::Pending => Status::Done,
        }
    }
}

impl From<&Todo> for ListItem<'_>{
    fn from(value: &Todo) -> Self {
        let marker = match value.state{
            Status::Done => "[√]",
            Status::Pending => "[ ]"
        };
        let display_pomo = if let Some(i) = value.pomo_total{
            format!("| {}", i)
        }else{
            String::from("")
        };
        let line = Line::raw(format!("{} {} {}", marker, value.name, display_pomo));
        ListItem::new(line)
    }
}