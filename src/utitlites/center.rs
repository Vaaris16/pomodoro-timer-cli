use crossterm::terminal;
use figlet_rs::FIGlet;

pub fn center(time: &str, font: FIGlet) {
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
