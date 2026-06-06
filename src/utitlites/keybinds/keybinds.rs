use std::{io::stdout, time::Duration};

use crossterm::{
    cursor::Show,
    event::{self, Event},
    execute,
    terminal::{LeaveAlternateScreen, disable_raw_mode},
};

pub fn keybinds() {
    if event::poll(Duration::from_millis(1000)).unwrap() {
        if let Event::Key(key_event) = event::read().unwrap() {
            match key_event.code {
                event::KeyCode::Char('q') => {
                    disable_raw_mode().unwrap();
                    execute!(stdout(), LeaveAlternateScreen).unwrap();
                    execute!(stdout(), Show).unwrap();
                    std::process::exit(0)
                }

                _ => (),
            }
        }
    }
}
