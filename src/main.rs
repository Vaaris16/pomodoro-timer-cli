use std::{thread, time::Duration};

use figlet_rs::FIGlet;

mod pomodoro;
mod utitlites;

use pomodoro::*;
use utitlites::{center::center, change_state::change_state, get_text::get_time};

const WORK_TIME: usize = 25 * 60;
const REST_TIME: usize = 10 * 60;

fn main() {
    let mut pomodoro = Pomodoro::new();

    loop {
        let big_font = FIGlet::big().unwrap();
        let time = get_time(pomodoro.total_time);

        center(&time, big_font);

        change_state(&mut pomodoro);

        thread::sleep(Duration::from_secs(1));
        pomodoro.tick();
    }
}
