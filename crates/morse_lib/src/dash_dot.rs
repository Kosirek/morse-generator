use crate::sound::*;
use crate::sound::GLOBAL_SOUND_PLAYER;

#[derive(Debug)]
pub enum DashDot{
    Dash,
    Dot,
    LetterBreak,
    WordBreak,
}

const DASH_STRING: &str = "-";
const DOT_STRING: &str = ".";
const LETTER_BREAK_STRING: &str = "/";
const WORD_BREAK_STRING: &str = "//";

impl DashDot {
    pub fn to_str(&self) -> &str {
        use DashDot::*;
        match self {
            Dash => DASH_STRING,
            Dot => DOT_STRING,
            LetterBreak => LETTER_BREAK_STRING,
            WordBreak => WORD_BREAK_STRING,
        }
    }

    pub fn from_str(s: &str) -> Option<DashDot> {
        use DashDot::*;
        match s {
            DASH_STRING => Some(Dash),
            DOT_STRING => Some(Dot),
            LETTER_BREAK_STRING => Some(LetterBreak),
            WORD_BREAK_STRING => Some(WordBreak),
            _ => None,
        }
    }
}

impl Sound for DashDot {
    fn play(&self) {
        // Placeholder for actual sound playing logic
        let sound_player = GLOBAL_SOUND_PLAYER.lock().unwrap();
        match self {
            DashDot::Dash => sound_player.generate_sound(sound_player.params.dash_time),
            DashDot::Dot => sound_player.generate_sound(sound_player.params.dot_time),
            DashDot::LetterBreak => SoundPlayer::generate_break(sound_player.params.space_time),
            DashDot::WordBreak => SoundPlayer::generate_break(sound_player.params.long_space_time),
        }
    }
}
