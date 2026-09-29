use std::{error::Error, time::Duration};
use ratatui::{
    DefaultTerminal, Frame, buffer::Buffer, layout::{Constraint::{self, Length}, Direction::{self, Horizontal, Vertical}, Layout, Rect}, style::{Modifier, Style, Stylize, palette::tailwind::SLATE}, text::{Line, ToSpan}, widgets::{Block, Cell, Paragraph, Row, StatefulWidget, Table, Widget}
};

use crate::{
    events::EventHandler, 
    handle_events::handle_events,
    models::{Status, Todolist},
    pomodoro::{PomoSession, TickOutcome},
};

const SELECTED_STYLE: Style = Style::new().bg(SLATE.c800).add_modifier(Modifier::BOLD);

pub enum Inputmod{
    Normal,
    AddTodo,
    SetPomodoro,
    RenameTodo,
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
    pub fn is_pomo_session_running(&self) -> bool{
        self.pommo_session.is_runing()
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
    //pause pomodoro
    pub fn pause_pomodoro(&mut self){
        self.pommo_session.pause();
    }
    //reset pomodoro
    pub fn reset_pomodoro(&mut self){
        self.pommo_session.reset();
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
    pub fn rename_todo(&mut self){
        if let Some(i) = self.todolist.state.selected(){
            self.todolist.items[i].rename(self.input_buf.clone());
            self.input_buf.clear();
            self.input_mod = Inputmod::Normal;
        }
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
        match self.pommo_session.tick(){
            TickOutcome::WorkFinished => {
                self.todolist.update_pomo();
            },
            _ => {}
        };
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
                let right_text = if let Some(pomos) = item.pomo_total{
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
            .direction(Vertical)
            .constraints(vec![
                Constraint::Fill(1),
                Constraint::Length(3),
                Constraint::Fill(1),
            ]).areas(r_timer);

        let text = vec![
            Line::from(self.pommo_session.time_format()),
            Line::from(""),
            Line::from(self.pommo_session.indecator())
        ];
        Paragraph::new(text)
            .centered()
            .render(middle, buf);
    }

    fn render_footer(&self, area: Rect, buf: &mut Buffer){
        let text = match self.input_mod{
            Inputmod::Normal => "↓↑ to move, ← to unselect, → to toggle, + to add, r to rename, R to reset .",
            _ => self.input_buf.as_str(),
        };
        Paragraph::new(text)
            .centered()
            .render(area, buf);
    }
}