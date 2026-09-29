use std::{error::Error, sync::mpsc::{self}, thread, time::{Duration, Instant}};
use crossterm::event::{self, Event, KeyEvent, KeyEventKind};

pub enum AppEvent{
    Tick,
    Key(KeyEvent),
    Resize(u16,u16)
}
pub struct EventHandler{
    #[allow(dead_code)]
    sender: mpsc::Sender<AppEvent>,

    #[allow(dead_code)]
    reciver: mpsc::Receiver<AppEvent>,

    #[allow(dead_code)]
    handler: thread::JoinHandle<()>
}

impl EventHandler{
    pub fn new(tick_rate: Duration) -> Self{

        let (sender, reciver) = mpsc::channel();
        let handler = {
            let sender = sender.clone();
            thread::spawn(move ||{
                let mut last_tick = Instant::now();
                loop{
                    //timeout indecates how long it will goes to achieve next tick
                    let timeout = tick_rate
                        .checked_sub(last_tick.elapsed())
                        .unwrap_or(tick_rate);
                    
                    if event::poll(timeout).expect("unable to poll the event") {
                        match event::read().expect("unable to read event") {
                            Event::Key(e) => {
                                if e.kind == KeyEventKind::Press{
                                    sender.send(AppEvent::Key(e))
                                } else{
                                    Ok(())
                                }
                            }
                            Event::Resize(w,h) => sender.send(AppEvent::Resize(w, h)),
                            _ => Ok(())
                        }.expect("fail to send the event");
                    }

                    if last_tick.elapsed() >= tick_rate{
                        sender.send(AppEvent::Tick);
                        last_tick = Instant::now();
                    }
                }
            })
        };

        Self { 
            sender,
            reciver,
            handler
        }
    }

    pub fn next(&self) -> Result<AppEvent, Box<dyn Error>>{
        Ok(self.reciver.recv()?)
    }
}