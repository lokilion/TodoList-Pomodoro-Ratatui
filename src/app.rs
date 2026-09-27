use std::{error::Error, fmt::{self, Display}, fs, thread::current, time::{Duration, Instant}};
use clap::ArgAction::Count;
use ratatui::{DefaultTerminal, Frame, buffer::Buffer, layout::{Constraint::{self, Length, Percentage}, Direction, Layout, Rect}, style::{Modifier, Style, Stylize, palette::tailwind::SLATE}, text::{Line, ToSpan}, widgets::{Block, Cell, List, ListItem, ListState, Paragraph, Row, StatefulWidget, Table, TableState, Widget}};
use serde::{Deserialize, Serialize};

use crate::{events::EventHandler, handle_events::handle_events};

const SELECTED_STYLE: Style = Style::new().bg(SLATE.c800).add_modifier(Modifier::BOLD);

pub enum Inputmod{
    Normal,
    AddTodo,
    SetPomodoro,
}

pub struct App{
    //header time
    header_time: String,

    //material
    todolist: Todolist,
    should_exit: bool,
    pommo_session: PomoSession,

    //event handler
    event_handler: EventHandler,

    //about input
    pub input_mod: Inputmod,
    pub input_buf: String,
}

impl App {
    pub fn new() -> App{
        App{
            header_time: chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            todolist: Todolist::new(),
            should_exit: false,
            pommo_session: PomoSession::new(4),

            event_handler: EventHandler::new(Duration::from_secs(1)),

            input_mod: Inputmod::Normal,
            input_buf: String::new(),
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<(), Box<dyn Error>> {
        //main loop
        while !self.should_exit{
            terminal.draw(|frame|self.draw(frame))?;
            let event_input = self.event_handler.next()?;
            handle_events(self, event_input);
        }
        //save current todolist to local file ".TodoList"
        self.todolist.save_to_file()?;
        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame){
        frame.render_widget(self, frame.area());
    }
    //tool fn
    pub fn selecting_item(&self)->Option<usize>{
        self.todolist.state.selected()
    }
    //select item
    pub fn select_none(&mut self){
        self.todolist.state.select(None);
    }
    pub fn select_next(&mut self){
        self.todolist.state.select_next();
    }
    pub fn select_previous(&mut self){
        self.todolist.state.select_previous();
    }
    //toggle state
    pub fn toggle_state(&mut self){
        if let Some(i) = self.todolist.state.selected(){
            self.todolist.items[i].toggle_state();
        }
    }
    //start pomodoro
    pub fn start_pomodoro(&mut self){
        self.pommo_session.start();
    }
    //Input mod
    pub fn start_editing(&mut self, input_mod: Inputmod) {
        self.input_mod = input_mod;
        self.input_buf.clear();
    }
    pub fn exit_editing(&mut self) {
        self.input_mod = Inputmod::Normal;
        self.input_buf.clear();
    }
    pub fn add_item(&mut self){
        let name = self.input_buf.trim();
        self.todolist.add_todo(name);

        self.input_buf.clear();
        self.input_mod = Inputmod::Normal;
    }
    pub fn set_pomodoro(&mut self){
        let new_pomo = match self.input_buf.trim().parse::<u8>(){
            Ok(num) => Some(num),
            Err(_) => None,
        };
        let i =  self.todolist.state.selected().unwrap();
        self.todolist.items[i].set_pomo(new_pomo);

        self.input_buf.clear();
        self.input_mod = Inputmod::Normal;
    }
    
    //delete a todo item
    pub fn delete_item(&mut self){
        if let Some(i) = self.todolist.state.selected() {
            self.todolist.items.remove(i);
        }
    }
    //exit app
    pub fn exit_app(&mut self){
        self.should_exit = true;
    }

    //when tick-event happens
    pub fn tick_event(&mut self){
        self.pommo_session.tick();
        self.header_time = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    }
}

impl Widget for &mut App{
    fn render(self, area: Rect, buffer: &mut Buffer) {
        let main_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Length(2),
                Constraint::Fill(1),
                Constraint::Length(1),
            ]);
        let [header_area, content_area, footer_area] = area.layout(&main_layout);
        
        App::render_header(self, header_area, buffer);
        App::render_content(self, content_area, buffer);
        App::render_footer(self, footer_area, buffer);
    }
}
//render logic for app
impl App {
    fn render_header(&self, area: Rect, buf: &mut Buffer){
        Paragraph::new(self.header_time.as_str())
            .bold()
            .centered()
            .render(area, buf);
    }

    fn render_content(&mut self, area: Rect, buf: &mut Buffer){
        let main_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Fill(1),
                Constraint::Length(16),
            ]);
        let [l_todolist, r_timer] = area.layout(&main_layout);

        //left is the todolist
        let todolist_block = Block::bordered()
            .title(" Todo List ".to_span().into_centered_line());
        
        let rows: Vec<Row> = self.todolist.items
            .iter()
            .map(|item|{
                let marker = match item.state{
                    Status::Done => "[√]",
                    Status::Pending => "[ ]"
                };
                let left_cell = Cell::from(format!("{} {}",marker, item.name));
                let right_text = if let Some(pomos) = item.pomo_remain{
                    format!("{}/{}", item.pomo_done, pomos)
                }else{
                    String::from("")
                };
                let right_cell = Cell::from(right_text)
                    .style(Style::default().fg(ratatui::style::Color::Yellow));
                
                Row::new(vec![left_cell, right_cell])

            })
            .collect();
        let todolist_table = Table::new(rows, [
                Constraint::Fill(1),
                Length(8),
            ])
            .block(todolist_block)
            .row_highlight_style(SELECTED_STYLE)
            .highlight_symbol(">");
        StatefulWidget::render(todolist_table, l_todolist, buf, &mut self.todolist.state);

        /* List version
        let items:Vec<ListItem> = self.todolist.items
            .iter().enumerate().map(|(i, item)|{
                ListItem::from(item)
            })
            .collect();

        let list = List::new(items)
            .block(todolist_block)
            .highlight_style(SELECTED_STYLE)
            .highlight_symbol(">");

        StatefulWidget::render(list, l_todolist, buf, &mut self.todolist.state);
        */

        //right is the timer
        let timer_block = Block::bordered()
            .title(" Timer ".to_span().into_centered_line());
        timer_block.render(r_timer, buf);
        let [_, middle,_] = Layout::default()
            .constraints(vec![
                Constraint::Fill(1),
                Constraint::Length(3),
                Constraint::Fill(1),
            ]).areas(r_timer);

        let text = vec![
            Line::from(self.pommo_session.time_format()),
            Line::from(""),
            Line::from(
                if self.pommo_session.is_runing(){
                    "▶RUNNING"
                }else{
                    "#PAUSE"
                }
            )
        ];
        Paragraph::new(text)
            .centered()
            .render(middle, buf);
    }

    fn render_footer(&self, area: Rect, buf: &mut Buffer){
        let text = match self.input_mod{
            Inputmod::Normal => "Use ↓↑ to move, ← to unselect, → to change status | Space to add a todo.",
            _ => self.input_buf.as_str(),
        };
        Paragraph::new(text)
            .centered()
            .render(area, buf);
    }
}

struct Todolist{
    items: Vec<Todo>,
    //state: ListState
    state: TableState
}

impl Todolist {
    fn new() -> Todolist {
        Todolist{
            items: Todolist::init_todo_list(),
            //state: ListState::default()
            state: TableState::new()
        }
    }
    //return next todo which is pending and has been set a pomodoro
    fn next_pomo_todo(&self) -> Option<usize>{
        self.items.iter().position(|item|{
            item.pomo_remain > Some(0) && item.state == Status::Pending
        })
    }
    //modify todolist
    fn add_todo(&mut self, name: &str){
        self.items.push(Todo::new(name));
    }
    //save todolist to local file
    fn init_todo_list() -> Vec<Todo>{
        match fs::read_to_string(".TodoList") {
            Ok(txt) => {
                let todo_list:Vec<Todo> = serde_json::from_str(&txt).unwrap_or_default();
                todo_list
            }
            _ => Vec::new()
        }
    }
    fn save_to_file(&self)->Result<(), Box<dyn Error>>{
        let txt = serde_json::to_string_pretty(&self.items)?;
        fs::write(".TodoList", txt)?;
        Ok(println!("Todo list has been saved"))
    }
}

struct PomoSession{
    cnt: usize,
    cycle: usize,
    work_flow: Vec<Countdown>,
    run_session: bool,
}
impl PomoSession {
    fn new(cycle: usize) -> PomoSession{
        PomoSession {
            cnt: 0,
            cycle: cycle,
            work_flow: {
                let mut flow = Vec::new();
                for _ in 1..cycle{
                    //work count
                    flow.push(Countdown::new(Duration::from_mins(25)));
                    //short break count
                    flow.push(Countdown::new(Duration::from_mins(5)));
                }
                //the last pomo
                flow.push(Countdown::new(Duration::from_mins(25)));
                //long break count
                flow.push(Countdown::new(Duration::from_mins(30)));
                flow
            },
            run_session: false
        }
    }

    fn start(&mut self){
        self.run_session = true;
        self.work_flow[self.cnt].start();
    }

    fn tick(&mut self){
        //if pause the session, end fn
        if !self.run_session{return;}

        self.cnt = self.cnt%self.cycle;
        if self.work_flow[self.cnt].remaining.is_zero(){
            //reset the count down
            self.work_flow[self.cnt].reset();
            //move to next count down
            self.cnt += (self.cnt+1)%self.cycle;
            self.work_flow[self.cnt].start();
            
        }
        self.work_flow[self.cnt].tick()
    }
    fn time_format(&self) -> String{
        self.work_flow[self.cnt].time_format()
    }
    fn is_runing(&self) -> bool{
        self.run_session
    }
}
struct Countdown{
    total_duration: Duration,
    remaining: Duration,
    last_tick: Instant,
    running: bool,
    current_todo: Option<usize>,
}
impl Countdown {
    fn new(total_duration: Duration) -> Countdown{
        Countdown {
            total_duration: total_duration,
            remaining: total_duration,
            last_tick: Instant::now(),
            running: false,
            current_todo: None,
        }
    }
    fn tick(&mut self){
        //if is not running end fn
        if !self.running{ return; }

        //current-time - last-time = passed time
        let now = Instant::now();
        let dt = now-self.last_tick;
        self.last_tick = Instant::now();

        // if no time remain to count, end fn
        if self.remaining.is_zero(){
            self.running = false;
            return;
        }
        self.remaining = self.remaining.saturating_sub(dt);
    }
    fn end(&mut self) {
        self.running = false;
    }
    fn start(&mut self){
        self.running = true;
        //set start time
        self.last_tick = Instant::now();
    }
    fn reset(&mut self){
        self.running = false;
        self.remaining = self.total_duration;
    }
    fn time_format(&self) -> String{
        let sec = self.remaining.as_secs();
        let min = sec / 60;
        let sec = sec % 60;

        format!("{:02}:{:02}",min, sec)
    }
}

#[derive(Serialize, Deserialize, PartialEq)]
enum Status{
    Done,
    Pending
}

#[derive(Serialize, Deserialize)]
struct Todo{
    name: String,
    state: Status,
    pomo_remain: Option<u8>,
    pomo_done: u8,
}

impl Todo{
    fn new(name: &str)->Todo{
        Todo {
            name: name.to_string(), 
            state: Status::Pending,
            pomo_remain: None,
            pomo_done: 0
        }
    }
    fn rename(&mut self, new_name: String){
        self.name = new_name;
    }
    fn set_pomo(&mut self, num: Option<u8>){
        if let Some(n) = num{
            self.pomo_remain = if n==0{None}else{num};
        }
    }
    fn toggle_state(&mut self){
        self.state = match self.state{
            Status::Done => Status::Pending,
            Status::Pending => Status::Done,
        }
    }
}

impl Display for Todo{
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result{
        let marker = match self.state{
            Status::Done => "[√]",
            Status::Pending => "[ ]"
        };
        write!(f, "{} {}", marker, self.name)
    }
}
impl From<&Todo> for ListItem<'_>{
    fn from(value: &Todo) -> Self {
        let marker = match value.state{
            Status::Done => "[√]",
            Status::Pending => "[ ]"
        };
        let display_pomo = if let Some(i) = value.pomo_remain{
            format!("| {}", i)
        }else{
            String::from("")
        };
        let line = Line::raw(format!("{} {} {}", marker, value.name, display_pomo));
        ListItem::new(line)
    }
}