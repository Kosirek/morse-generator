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
        match self {
            DashDot::Dash => GLOBAL_SOUND_PLAYER.generate_sound(200),
            DashDot::Dot => GLOBAL_SOUND_PLAYER.generate_sound(100),
            DashDot::LetterBreak => SoundPlayer::generate_break(100),
            DashDot::WordBreak => SoundPlayer::generate_break(200),
        }
    }
}
