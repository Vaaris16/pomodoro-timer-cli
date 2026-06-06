use crate::pomodoro::{Pomodoro, State};

pub fn change_state(pomodoro: &mut Pomodoro) {
    if pomodoro.total_time == 0 {
        match pomodoro.state {
            State::Work => pomodoro.set_state_rest(),
            State::Rest => pomodoro.set_state_work(),
        }
    }
}
