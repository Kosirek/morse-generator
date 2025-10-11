use crate::sound::*;

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
            DashDot::Dash => generate_sound(440.0, 200),
            DashDot::Dot => generate_sound(440.0, 100),
            DashDot::LetterBreak => generate_break(100),
            DashDot::WordBreak => generate_break(200),
        }
    }
}
