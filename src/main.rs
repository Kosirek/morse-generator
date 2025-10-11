use morse_lib::MorseText;
use morse_lib::sound::Sound;

fn main() {
    let text = "HELLO WORLD 123".to_string();
    let morse_text = MorseText::from(text);
    println!("Playnig: {}", morse_text);
    morse_text.play();

}
