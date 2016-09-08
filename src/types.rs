use std::fmt;

//
// New types instead of just strings.
//

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub struct Puzzle(pub String);
#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub struct Word(pub String);
#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub struct Name(pub String);
#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub struct User(pub String);
#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub struct Channel(pub String);

pub type WordHash = String;
pub type TooMany = String;
pub type TooFew = String;

use regex::Regex;

fn normalize_string(s: &String) -> String {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"[^A-Za-zåäöÅÄÖ]").unwrap();
    }

    RE.replace_all(s.as_str(), "").to_uppercase()
}

impl Word {
    // Normalize a word by removing all non-alpha characters.
    pub fn normalize(&self) -> Word {
        let &Word(ref w) = self;
        Word(normalize_string(&w))
    }
}

impl Puzzle {
    pub fn new(s: &String) -> Puzzle {
        Puzzle(normalize_string(&s))
    }
}

impl fmt::Display for Puzzle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Channel {
    pub fn is_private(&self) -> bool {
        self.0.starts_with("D")
    }
}

#[derive(Eq, PartialEq, Debug)]
pub enum Reason {
    NotInDictionary,
    NotNineCharacters,
    NonMatchingWord(TooMany, TooFew),
}

#[derive(Eq, PartialEq, Debug)]
pub enum InvalidReason {
    UnknownCommand,
    WrongNoOfParameters,
}

#[derive(Eq, PartialEq, Debug)]
pub enum Response {
    GetCommand(Channel, Puzzle),
    NoPuzzleSet(Channel),
    SetPuzzle(Channel, Puzzle),
    InvalidPuzzle(Channel, Puzzle, Reason),
    CorrectSolution(Channel, Word),
    Notification(Name, WordHash),
    IncorrectSolution(Channel, Word, Reason),
    Help(Channel),
    DualResponse(Box<Response>, Box<Response>),
    TripleResponse(Box<Response>, Box<Response>, Box<Response>),
}



#[cfg(test)]
mod tests {
    use super::*;

    const NORMALIZATION_TESTS: &'static [(&'static str, &'static str)] = &[
        ("GALLTJUTA", "GALLTJUTA"),
        ("galltjuta", "GALLTJUTA"),
        ("DATORSPEL", "DATORSPEL"),
        ("datorspel", "DATORSPEL"),
        ("dator spel", "DATORSPEL"),
        ("dator-spel", "DATORSPEL"),
        ("  dator-spel\n", "DATORSPEL"),
        ("abcdefåäö", "ABCDEFÅÄÖ")
    ];


    #[test]
    fn normalization_test() {
        for (input, expected) in NORMALIZATION_TESTS.iter().map(|x| (Word(x.0.to_string()), Word(x.1.to_string()))) {
            let actual = input.normalize();
            assert!(actual == expected, "Actual: {:?}, Expected: {:?}", actual, expected);
        }
    }

    #[test]
    fn puzzle_factory_test() {
        for (input, expected) in NORMALIZATION_TESTS.iter().map(|x| (x.0.to_string(), Puzzle(x.1.to_string()))) {
            let actual = Puzzle::new(&input);
            assert!(actual == expected, "Actual: {:?}, Expected: {:?}", actual, expected);
        }
    }

    #[test]
    fn public_private_channels() {
        assert!(!Channel("C0123".into()).is_private());
        assert!(Channel("D0123".into()).is_private());
    }
}
