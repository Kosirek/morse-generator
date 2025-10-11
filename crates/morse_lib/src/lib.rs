pub mod dash_dot;
pub mod sound;
pub mod morse_alphabet;

use crate::sound::Sound;
use crate::morse_alphabet::MorseCodes;
use std::fmt::Display;

#[derive(Debug)]
pub struct MorseText {
    morse_text: Vec<MorseCodes>,
}

impl MorseText {
    pub fn new() -> Self {
        MorseText { morse_text: Vec::new() }
    }
}

impl From<String> for MorseText {
    fn from(text: String) -> Self {
        let mut new_text = MorseText::new();
        new_text.morse_text.push(MorseCodes::EndOfChar);
        new_text.morse_text.push(MorseCodes::EndOfChar);

        text.chars().for_each(|c| {
            if let Some(code) = MorseCodes::from_str(&c.to_string()) {
                new_text.morse_text.push(code);
                new_text.morse_text.push(MorseCodes::EndOfChar);
            }
        });

        new_text.morse_text.push(MorseCodes::EndOfChar);

        new_text
    }
}

impl Sound for MorseText {
    fn play(&self) {
        self.morse_text.iter().for_each(|code| code.play());
    }
}

impl Display for MorseText {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let morse_string: String = self.morse_text.iter().map(|code| code.to_morse_str()).collect();
        write!(f, "{}", morse_string)
    }
}
