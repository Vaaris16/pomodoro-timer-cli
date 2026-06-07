use crossterm::cursor::Hide;
use crossterm::execute;
use crossterm::terminal::{EnterAlternateScreen, enable_raw_mode};
use figlet_rs::FIGlet;
use std::io::stdout;

mod pomodoro;
mod utitlites;

use pomodoro::*;
use utitlites::{center::center, change_state::change_state, get_text::get_time};

use crate::utitlites::{keybinds::keybinds::keybinds, state_text::print_state::print_state};

const WORK_TIME: usize = 1 * 60;
const REST_TIME: usize = 1 * 60;

fn main() {
    let mut pomodoro = Pomodoro::new();

    enable_raw_mode().unwrap();
    execute!(stdout(), EnterAlternateScreen).unwrap();
    execute!(stdout(), Hide).unwrap();

    loop {
        let path_font_banner = include_str!("../fonts/Big Font.flf");
        let path_font_small_slant = include_str!("../fonts/SMSlant Font.flf");
        let big_font = FIGlet::from_content(path_font_banner).unwrap();
        let small_font = FIGlet::from_content(path_font_small_slant).unwrap();
        let time = get_time(pomodoro.total_time);
        print!("\x1B[2J\x1B[1;1H");

        center(&pomodoro, &time, big_font);
        print_state(small_font, &pomodoro);

        change_state(&mut pomodoro);

        keybinds();
        pomodoro.tick();
    }
}
