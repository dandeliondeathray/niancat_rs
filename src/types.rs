pub struct Puzzle(String);
pub struct Word(String);

impl Word {
    // Normalize a word by removing all non-alpha characters.
    pub fn normalize(w: Word) -> Word {
        Word("".to_string())
    }
}