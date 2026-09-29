use crossterm::event::{KeyCode, KeyEvent};

use crate::{app::{App,Inputmod}, events::AppEvent};

pub fn handle_events(app: &mut App, event: AppEvent){
    match event{
        AppEvent::Tick => handle_tick_event(app),
        AppEvent::Key(e) => handle_key_events(app, e),
        //currently just send/recive this event to un block the main loop
        AppEvent::Resize(w,h ) => ()
    }
}

fn handle_tick_event(app: &mut App){
    app.tick_event();
}

fn handle_key_events(app: &mut App, key_event: KeyEvent){
    match &app.input_mod {
        Inputmod::Normal =>{
            match key_event.code{
                //unselect
                KeyCode::Left => app.select_none(),
                //toggle todo state
                KeyCode::Right => app.toggle_state(),
                //select previous
                KeyCode::Up => app.select_previous(),
                //select next
                KeyCode::Down => app.select_next(),
                //delete a todo item
                KeyCode::Delete => app.delete_item(),
                //add a new toodo item
                KeyCode::Char('+') => app.start_editing(Inputmod::AddTodo),
                //start pomodoro
                KeyCode::Char(' ') => {
                    if app.is_pomo_session_running(){
                        app.pause_pomodoro();
                    }else{
                        app.start_pomodoro();
                    }
                }
                //rename a todo
                KeyCode::Char('r') => {
                    if let Some(_) = app.selecting_item(){
                        app.start_editing(Inputmod::RenameTodo);
                    }
                }
                //reset pomodoro
                KeyCode::Char('R') => {
                    app.reset_pomodoro();
                }
                //exit the app
                KeyCode::Esc => app.exit_app(),
                KeyCode::Char(c) => {
                    if let Some(_) = app.selecting_item(){
                        app.start_editing(Inputmod::SetPomodoro);
                        app.input_buf.push(c);
                    }
                },
                _ => (),
            };
        },
        Inputmod::SetPomodoro => {
            match key_event.code{
                KeyCode::Enter => app.set_pomodoro(),
                KeyCode::Esc => app.exit_editing(),
                KeyCode::Backspace => { app.input_buf.pop(); },
                KeyCode::Char(c) => app.input_buf.push(c),
                _ => (),
            };
        },
        Inputmod::AddTodo => {
            match key_event.code{
                KeyCode::Enter => app.add_item(),
                KeyCode::Esc => app.exit_editing(),
                KeyCode::Backspace => { app.input_buf.pop(); },
                KeyCode::Char(c) => app.input_buf.push(c),
                _ => (),
            };
        },
        Inputmod::RenameTodo => {
            match key_event.code{
                KeyCode::Enter => app.rename_todo(),
                KeyCode::Esc => app.exit_editing(),
                KeyCode::Backspace => { app.input_buf.pop(); },
                KeyCode::Char(c) => app.input_buf.push(c),
                _ => (),
            };
        }
    };
}