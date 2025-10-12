use rodio::{OutputStreamBuilder, Sink, source::{SineWave, Source}, OutputStream};
use std::time::Duration;
use once_cell::sync::Lazy;
use std::sync::Mutex;

 pub struct SoundPlayer {
    _stream_handle: OutputStream,
    sink: Sink,
    pub(crate) params: SoundParams,
}

pub struct SoundParams {
    pub frequency: f32,
    pub dot_time: u64,
    pub dash_time: u64,
    pub space_time: u64,
    pub long_space_time: u64,
}

impl SoundParams {
    pub fn new() -> Self {
        SoundParams {
            frequency: 440.0,
            dot_time: 100,
            dash_time: 200,
            space_time: 100,
            long_space_time: 200,
        }
    }
}

pub static GLOBAL_SOUND_PLAYER: Lazy<Mutex<SoundPlayer>> = Lazy::new(|| Mutex::new(SoundPlayer::new()));

impl SoundPlayer {
    pub fn generate_sound(&self, duration_ms: u64) {
        let duration = Duration::from_millis(duration_ms);
        let source = SineWave::new(self.params.frequency).take_duration(duration);

        self.sink.append(source);
        self.sink.sleep_until_end();
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
            params: SoundParams::new(),
        }
    }

    pub fn update_params(&mut self, new_params: SoundParams) {
        self.params = new_params;
    }

}

pub trait Sound {
    fn play(&self);
}