use std::collections::{HashSet, HashMap};
use super::types::*;

pub struct Dictionary {
    words:  HashSet<String>,
    solutions: HashMap<String, Vec<String>>,
}

impl Dictionary {
    // Check if a word is in the dictionary.
    pub fn is_solution(&self, w: Word) -> bool {
        false
    }

    // Check how many solutions a given puzzle has in the dictionary.
    pub fn no_of_solutions(&self, p: Puzzle) -> u32 {
        0
    }

    // Find all solutions given a word
    pub fn find_solutions(&self, p: Puzzle) -> Vec<String> {
        vec![]
    }
}

#[cfg(test)]
mod test {

    const WORDS: &'static [&'static str] = &[
        "GALLTJUTA",
        "DATORSPEL",
        "SPELDATOR",
        "abcdefghi",
        "ABCDEFåäö",
        "ABCDEF---åäö",
        "  ABCDEF   åäö  "
    ];

    const SOLUTION_TESTS: &'static [&'static str] = &[
        "GALLTJUTA", "DATORSPEL", "SPELDATOR", "ABCDEFGHI", "ABCDEFÅÄÖ",
        "galltjuta", "datorspel", "speldator", "abcdefghi", "abcdefåäö",
        "gall tjuta", "  galltjuta  ", "gall-tjuta", "-galltjuta -----     "
    ];

    const NON_SOLUTION_TESTS: &'static [&'static str] = &[
        "GALLTJUT", "GALLTJUTAA", "åäöabcdef"
    ];

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

    }
}