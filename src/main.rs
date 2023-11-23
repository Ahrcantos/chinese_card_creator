use card_generator::syllable::{Final, Initial, Syllable, Tone};
fn main() {
    println!("{}", Syllable(Initial::Z, Final::I, Tone::FallingRising));
}
