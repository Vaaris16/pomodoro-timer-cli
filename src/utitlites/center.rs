use crossterm::style::Stylize;
use crossterm::terminal;
use figlet_rs::FIGlet;

use crate::pomodoro::{Pomodoro, State};

pub fn center(pomodoro: &Pomodoro, time: &str, font: FIGlet) {
    let rendered = font.convert(time).unwrap().to_string();
    let lines: Vec<&str> = rendered.lines().collect();

    let tw = lines.iter().map(|l| l.len()).max().unwrap();

    let (cols, rows) = terminal::size().unwrap();

    let pad_x = (cols as usize).saturating_sub(tw) / 2;
    let pad_y = (rows as usize).saturating_sub(time.len()) / 2;

    print!("{}", "\r\n".repeat(pad_y));

    for line in lines {
        match pomodoro.state {
            State::Work => println!("{}{}\r", " ".repeat(pad_x), line.red()),
            State::Rest => println!("{}{}\r", " ".repeat(pad_x), line.green()),
        }
    }
}
