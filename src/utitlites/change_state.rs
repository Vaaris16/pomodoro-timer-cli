use crate::{
    pomodoro::{Pomodoro, State},
    utitlites::play_sound::play_sound::play_sound,
};

pub fn change_state(pomodoro: &mut Pomodoro, alarm: bool) {
    if pomodoro.total_time == 0 {
        match pomodoro.state {
            State::Work => {
                if alarm {
                    let path_rest_transition = include_bytes!("../../audio/main_audio.mp3");
                    play_sound(path_rest_transition);
                }
                pomodoro.set_state_rest()
            }

            State::Rest => {
                if alarm {
                    let path_rest_transition = include_bytes!("../../audio/main_audio.mp3");
                    play_sound(path_rest_transition);
                }
                pomodoro.set_state_work();
            }
        }
    }
}
