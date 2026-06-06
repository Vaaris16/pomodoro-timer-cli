use std::{thread, time::Duration};

use crossterm::terminal;
use figlet_rs::FIGlet;

const WORK_TIME: usize = 25 * 60;
const REST_TIME: usize = 10 * 60;

struct Pomodoro {
    total_time: usize,
    state: State,
}

enum State {
    Work,
    Rest,
}

impl Pomodoro {
    fn new() -> Self {
        Self {
            total_time: WORK_TIME,
            state: State::Work,
        }
    }

    fn tick(&mut self) {
        self.total_time -= 1
    }

    fn set_state_work(&mut self) {
        self.total_time = WORK_TIME;
        self.state = State::Work
    }

    fn set_state_rest(&mut self) {
        self.total_time = REST_TIME;
        self.state = State::Rest
    }
}

fn get_time(total_time: usize) -> String {
    let secs = total_time % 60;
    let min = total_time / 60;

    format!("{:02}:{:02}", min, secs)
}

fn center(time: &str, font: FIGlet) {
    let rendered = font.convert(time).unwrap().to_string();
    let lines: Vec<&str> = rendered.lines().collect();

    let tw = lines.iter().map(|l| l.len()).max().unwrap();

    let (cols, rows) = terminal::size().unwrap();

    let pad_x = (cols as usize).saturating_sub(tw) / 2;
    let pad_y = (rows as usize).saturating_sub(time.len()) / 2;

    print!("\x1B[2J\x1B[1;1H");

    print!("{}", "\n".repeat(pad_y));

    for line in lines {
        println!("{}{}", " ".repeat(pad_x), line);
    }
}

fn change_state(pomodoro: &mut Pomodoro) {
    if pomodoro.total_time == 0 {
        match pomodoro.state {
            State::Work => pomodoro.set_state_rest(),
            State::Rest => pomodoro.set_state_work(),
        }
    }
}

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
