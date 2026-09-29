use std::time::Duration;

use super::countdown::Countdown;

pub enum TickOutcome{
    Idle,
    WorkFinished,
    BreakFinished,
}

pub struct PomoSession{
    cnt: usize,
    cycle: usize,
    work_flow: Vec<Countdown>,
    run_session: bool,
}
impl PomoSession {
    pub fn new(cycle: usize) -> PomoSession{
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
            run_session: false,
        }
    }
    pub fn indecator(&self) -> String{
        match self.cnt%2{
            0 => String::from("*Work*"),
            1 => String::from("*Rest*"),
            _ => String::from("")
        }
    }
    pub fn start(&mut self){
        self.run_session = true;
        self.work_flow[self.cnt].start();
    }
    pub fn pause(&mut self){
        self.run_session = false;
        self.work_flow[self.cnt].pause();
    }
    pub fn reset(&mut self){
        self.run_session = false;
        self.cnt = 0;
        self.work_flow[self.cnt].reset();
    }
    pub fn tick(&mut self) -> TickOutcome{
        //if pause the session, end fn
        if !self.run_session{return TickOutcome::Idle;}

        let mut tick_outcome = TickOutcome::Idle;

        self.cnt = self.cnt%(2*self.cycle);
        self.work_flow[self.cnt].tick();

        //notice the index of the working-count is even number
        //so when a count is over, check 
        if self.work_flow[self.cnt].check_is_over(){
            tick_outcome = match self.cnt%2{
                0 => TickOutcome::WorkFinished,
                1 => TickOutcome::BreakFinished,
                _ => TickOutcome::Idle,
            };
            //reset the count down
            self.work_flow[self.cnt].reset();
            //move to next count down
            self.cnt = (self.cnt+1)%(2*self.cycle);
            self.work_flow[self.cnt].start();
            self.work_flow[self.cnt].tick();
        }
        tick_outcome
    }
    pub fn time_format(&self) -> String{
        self.work_flow[self.cnt].time_format()
    }
    pub fn is_runing(&self) -> bool{
        self.run_session
    }
}