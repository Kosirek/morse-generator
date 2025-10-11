use rodio::{OutputStreamBuilder, Sink, source::{SineWave, Source}};
use std::time::Duration;

// pub struct SountPlayer {
//     dot_time: u64,
//     dash_time_multiplier: f32,
//     space_time_multiplier: f32,
//     long_space_time_multiplier: f32,
// }

// obiekt odgrywania dzwiekow mozna zaimplementowac jako lazy singleton z wykorzystaniem np. once_cell
// nalezy uwzglednic mozliwosc zmiany parametrow odtwarzania dzwiekow - czestotliwosc, czas trwania kropki, kreski, przerwy miedzy literami i slowami
// singleton powinien przechowywac OutputStream i Sink (wlasciwie to do daleszego dzialania potrzebny jest tylko Sink, ale OutputStream musi byc zywy tak dlugo jak Sink)

pub fn generate_sound(frequency: f32, duration_ms: u64) {
    let stream_handle = OutputStreamBuilder::open_default_stream().unwrap();
    let sink = Sink::connect_new(&stream_handle.mixer());

    let duration = Duration::from_millis(duration_ms);
    let source = SineWave::new(frequency).take_duration(duration);

    sink.append(source);
    sink.sleep_until_end();
}

pub fn generate_break(duration_ms: u64) {
    std::thread::sleep(Duration::from_millis(duration_ms));
}

pub trait Sound {
    fn play(&self);
}