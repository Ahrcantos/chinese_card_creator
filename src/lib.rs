pub mod syllable;

use syllable::Syllable;

struct Character {
    symbol: char,
    pronunciation: Option<Syllable>,
    meaning: Option<String>,
    components: Option<String>,
    skip: bool,
}

struct Word {
    characters: Vec<Character>,
    pronunciation: Vec<Syllable>,
    meaning: String,
}

enum SentencePart {
    Text(String),
    Word(Word),
}

struct Sentence(Vec<SentencePart>);
