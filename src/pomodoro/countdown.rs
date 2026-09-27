use std::time::{Duration, Instant};

pub struct Countdown{
    total_duration: Duration,
    remaining: Duration,
    last_tick: Instant,
    running: bool,
    current_todo: Option<usize>,
}
impl Countdown {
    pub fn new(total_duration: Duration) -> Countdown{
        Countdown {
            total_duration: total_duration,
            remaining: total_duration,
            last_tick: Instant::now(),
            running: false,
            current_todo: None,
        }
    }
    pub fn tick(&mut self){
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
    pub fn pause(&mut self) {
        self.running = false;
    }
    pub fn start(&mut self){
        self.running = true;
        //set start time
        self.last_tick = Instant::now();
    }
    pub fn reset(&mut self){
        self.running = false;
        self.remaining = self.total_duration;
    }
    pub fn time_format(&self) -> String{
        let sec = self.remaining.as_secs();
        let min = sec / 60;
        let sec = sec % 60;

        format!("{:02}:{:02}",min, sec)
    }
    pub fn check_is_over(&self) -> bool{
        self.remaining.is_zero()
    }
}