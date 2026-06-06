use crate::{REST_TIME, WORK_TIME};

pub struct Pomodoro {
    pub total_time: usize,
    pub state: State,
}

pub enum State {
    Work,
    Rest,
}

impl Pomodoro {
    pub fn new() -> Self {
        Self {
            total_time: WORK_TIME,
            state: State::Work,
        }
    }

    pub fn tick(&mut self) {
        self.total_time -= 1
    }

    pub fn set_state_work(&mut self) {
        self.total_time = WORK_TIME;
        self.state = State::Work
    }

    pub fn set_state_rest(&mut self) {
        self.total_time = REST_TIME;
        self.state = State::Rest
    }
}
