use morse_lib::MorseText;
use morse_lib::sound::{Sound, GLOBAL_SOUND_PLAYER};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short)]
    text: String,

    #[arg(short, required_unless_present = "verbose")]
    print_morse: bool,

    #[arg(short, required_unless_present = "print_morse")]
    verbose: bool,

    #[arg(short, default_value_t = 440.0)]
    frequency: f32,

    // #[arg(short, default_value_t = 100)]
    // dot_duration: u64,

    // #[arg(short, default_value_t = 3.0)]
    // dash_duration_multiplier: f32,

    // #[arg(short, default_value_t = 1.0)]
    // letter_space_duration_multiplier: f32,

    // #[arg(short, default_value_t = 3.0)]
    // word_space_duration_multiplier: f32,
}

fn main() {
    let args = Args::parse();

    let morse_text = MorseText::from(args.text);
    if args.print_morse {
        println!("{}", morse_text);
    }

    if args.verbose {
//        GLOBAL_SOUND_PLAYER.get_or_init(|e| e.set_frequency(args.frequency));
        morse_text.play();
    }
}
