use std::collections::{HashSet, HashMap};
use std::iter::FromIterator;
use std::iter::Map;
use std::iter::Iterator;
use super::types::*;

pub struct Dictionary {
    words:  HashSet<Word>,
    solutions: HashMap<String, Vec<String>>,
}

impl Dictionary {
    pub fn new<I>(it: I) -> Dictionary
        where I: Iterator<Item=String> {

        let word_it = it.map(|x| Word(x).normalize());
        Dictionary { words: HashSet::from_iter(word_it), solutions: HashMap::new(), }
    }

    /// Check if a word is in the dictionary.
    pub fn is_solution(&self, w: &Word) -> bool {
        self.words.contains(&w.normalize())
    }

    /// Check how many solutions a given puzzle has in the dictionary.
    pub fn no_of_solutions(&self, p: &Puzzle) -> u32 {
        0
    }

    /// Find all solutions given a word
    pub fn find_solutions(&self, p: &Puzzle) -> Vec<String> {
        vec![]
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use types::*;

    const WORDS: &'static [&'static str] = &[
        "GALLTJUTA",
        "DATORSPEL",
        "SPELDATOR",
        "abcdefghi",
        "ABCDEFåäö",
        "ABCDEF---åäö",
        "  ABCDEF   åäö  ",
        "abc",
        "abcdefghijkl",
        "ÅÄÖABC",
        "abcåäö",
    ];

    const SOLUTION_TESTS: &'static [&'static str] = &[
        "GALLTJUTA", "DATORSPEL", "SPELDATOR", "ABCDEFGHI", "ABCDEFÅÄÖ",
        "galltjuta", "datorspel", "speldator", "abcdefghi", "abcdefåäö",
        "gall tjuta", "  galltjuta  ", "gall-tjuta", "-galltjuta -----     "
    ];

    const NON_SOLUTION_TESTS: &'static [&'static str] = &[
        "GALLTJUT", "GALLTJUTAA", "åäöabcdef",
        "abc", "abcdefghijkl", "ÅÄÖABC", "abcåäö",
    ];

    const NO_OF_SOLUTIONS_TESTS: &'static [(&'static str, u32)] = &[
        ("GALLTJUTA", 1),
        ("TJUTAGALL", 1),
        ("DATORSPEL", 2),
        ("SPELDATOR", 2),
        ("SPDATOREL", 2),
        ("ÅÄÖABCDEF", 1),
        ("AAAAAAAAA", 0)
    ];

    #[test]
    fn solution_tests() {
        let d = Dictionary::new(WORDS.iter().map(|x| x.to_string()));

        for word in SOLUTION_TESTS.iter().map(|x| Word(x.to_string())) {
            assert!(d.is_solution(&word), "Word should be a solution: {:?}", word);
        }
    }
}