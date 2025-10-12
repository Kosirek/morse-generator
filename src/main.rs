use morse_lib::MorseText;
use morse_lib::sound::{GLOBAL_SOUND_PLAYER, Sound, SoundParams};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short)]
    text: String,

    #[arg(
        short,
        required_unless_present = "verbose",
        help = "Print the Morse code representation of the input text"
    )]
    print_morse: bool,

    #[arg(
        short,
        required_unless_present = "print_morse",
        help = "Play the Morse code as sound"
    )]
    verbose: bool,

    #[arg(
        short,
        default_value = "440.0",
        help = "Frequency of the Morse code sound in Hz"
    )]
    frequency: Option<f32>,

    #[arg(
        short,
        default_value = "100",
        help = "Duration of a dot in milliseconds"
    )]
    dot_duration: Option<u64>,

    #[arg(
        short = 'D',
        default_value = "3.0",
        help = "Multiplier for dash duration"
    )]
    dash_duration_multiplier: Option<f32>,

    #[arg(
        short,
        default_value = "1.0",
        help = "Multiplier for space between dots and dashes"
    )]
    letter_space_duration_multiplier: Option<f32>,

    #[arg(
        short,
        default_value = "3.0",
        help = "Multiplier for space between letters"
    )]
    word_space_duration_multiplier: Option<f32>,
}

fn main() {
    let args = Args::parse();

    let morse_text = MorseText::from(args.text);
    if args.print_morse {
        println!("{}", morse_text);
    }

    if args.verbose {
        let mut sound_player = GLOBAL_SOUND_PLAYER.lock().unwrap();
        sound_player.update_params(SoundParams {
            frequency: args.frequency.unwrap_or(440.0),
            dot_time: args.dot_duration.unwrap_or(100),
            dash_time: ((args.dot_duration.unwrap_or(100) as f32)
                * args.dash_duration_multiplier.unwrap_or(2.0)) as u64,
            space_time: ((args.dot_duration.unwrap_or(100) as f32)
                * args.letter_space_duration_multiplier.unwrap_or(1.0))
                as u64,
            long_space_time: ((args.dot_duration.unwrap_or(100) as f32)
                * args.word_space_duration_multiplier.unwrap_or(2.0))
                as u64,
        });
        std::mem::drop(sound_player);
        morse_text.play();
    }
}
