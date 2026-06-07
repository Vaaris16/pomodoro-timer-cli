use std::{path::PathBuf, str::FromStr};

use crate::{
    pomodoro::{Pomodoro, State},
    utitlites::play_sound::play_sound::play_sound,
};

pub fn change_state(pomodoro: &mut Pomodoro) {
    if pomodoro.total_time == 0 {
        match pomodoro.state {
            State::Work => {
                let path_rest_transition = include_bytes!("../../audio/main_audio.mp3");
                play_sound(path_rest_transition);
                pomodoro.set_state_rest()
            }

            State::Rest => {
                let path_rest_transition = include_bytes!("../../audio/main_audio.mp3");
                play_sound(path_rest_transition);
                pomodoro.set_state_work();
            }
        }
    }
}
