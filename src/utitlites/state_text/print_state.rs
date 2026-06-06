use colored::Colorize;
use crossterm::terminal;
use figlet_rs::FIGlet;

use crate::{
    pomodoro::{Pomodoro, State},
    utitlites::state_text::get_state_text::get_state_text,
};

pub fn print_state(font: FIGlet, pomodoro: &Pomodoro) {
    let state_text = get_state_text(pomodoro);
    let rendered = font.convert(&state_text).unwrap().to_string();
    let lines: Vec<&str> = rendered.lines().collect();

    let tw = lines.iter().map(|l| l.len()).max().unwrap();

    let (cols, rows) = terminal::size().unwrap();

    let pad_x = (cols as usize).saturating_sub(tw) / 2;
    let pad_y = (rows as usize).saturating_sub(state_text.len()) / 2;

    println!("{}", "\n".repeat(pad_y.saturating_sub(20)));
    for line in lines {
        match pomodoro.state {
            State::Work => println!("{}{}\r", " ".repeat(pad_x), line.red()),
            State::Rest => println!("{}{}\r", " ".repeat(pad_x), line.green()),
        }
    }
}
