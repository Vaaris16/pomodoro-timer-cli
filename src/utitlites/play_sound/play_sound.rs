use std::{io::Cursor, time::Duration};

use rodio::{Decoder, DeviceSinkBuilder, Player, Source};

pub fn play_sound(audio: &'static [u8]) {
    let sink = DeviceSinkBuilder::open_default_sink().unwrap();

    let cursor = Cursor::new(audio);
    let source = Decoder::new(cursor)
        .unwrap()
        .take_duration(Duration::from_secs_f32(3.0));

    let player = Player::connect_new(sink.mixer());

    player.append(source);
    player.sleep_until_end();

    drop(sink);
}
