use crate::pomodoro::{Pomodoro, State};

pub fn get_state_text(pomodoro: &Pomodoro) -> String {
    match pomodoro.state {
        State::Work => "Work".to_string(),
        State::Rest => "Rest".to_string(),
    }
}
