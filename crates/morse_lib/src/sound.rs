use rodio::{OutputStreamBuilder, Sink, source::{SineWave, Source}, OutputStream};
use std::time::Duration;
use once_cell::sync::Lazy;

 pub struct SoundPlayer {
    _stream_handle: OutputStream,
    sink: Sink,
    frequency: f32,
//     dot_time: u64,
//     dash_time_multiplier: f32,
//     space_time_multiplier: f32,
//     long_space_time_multiplier: f32,
}

// nalezy uwzglednic mozliwosc zmiany parametrow odtwarzania dzwiekow - czestotliwosc, czas trwania kropki, kreski, przerwy miedzy literami i slowami
// singleton powinien przechowywac OutputStream i Sink (wlasciwie to do daleszego dzialania potrzebny jest tylko Sink, ale OutputStream musi byc zywy tak dlugo jak Sink)

pub static GLOBAL_SOUND_PLAYER: Lazy<SoundPlayer> = Lazy::new(|| SoundPlayer::new());

impl SoundPlayer {
    pub fn generate_sound(&self, duration_ms: u64) {
        let duration = Duration::from_millis(duration_ms);
        let source = SineWave::new(self.frequency).take_duration(duration);

        GLOBAL_SOUND_PLAYER.sink.append(source);
        GLOBAL_SOUND_PLAYER.sink.sleep_until_end();
    }

    pub fn generate_break(duration_ms: u64) {
        std::thread::sleep(Duration::from_millis(duration_ms));
    }

    pub fn new() -> Self {
        let stream_handle = OutputStreamBuilder::open_default_stream().unwrap();
        let sink = Sink::connect_new(&stream_handle.mixer());
        SoundPlayer {
            _stream_handle: stream_handle,
            sink,
            frequency: 440.0,
        }
    }

    pub fn set_frequency(&mut self, frequency: f32) {
        self.frequency = frequency;
    }
}

pub trait Sound {
    fn play(&self);
}