use std::collections::HashSet;
use std::iter::FromIterator;
use multimap::MultiMap;

use super::types::*;

pub trait CheckWord {
    fn is_solution(&self, w: &Word) -> bool;
    fn no_of_solutions(&self, p: &Puzzle) -> usize;
    fn find_solutions(&self, p: &Puzzle) -> Option<Vec<Word>>;
    fn has_solution(&self, p: &Puzzle) -> bool;
}

pub struct Dictionary {
    words:  HashSet<Word>,
    solutions: MultiMap<Puzzle, Word>,
}

fn sort_word(x: &String) -> String {
    let mut cs: Vec<char> = x.chars().collect();
    cs.sort();
    cs.into_iter().collect()
}

fn sort_puzzle(&Puzzle(ref p): &Puzzle) -> Puzzle {
    Puzzle(sort_word(&p))
}

impl Dictionary {
    pub fn new<I>(it: I) -> Dictionary
        where I: Iterator<Item=String> {

        let word_it = it.map(|x| Word(x).normalize()).filter(|&Word(ref x)| x.chars().count() == 9);
        let words = HashSet::from_iter(word_it);

        let mut solutions = MultiMap::new();
        for &Word(ref w) in &words {
            let p = sort_word(w);
            println!("Puzzle: {}, Word {}", p, w);
            solutions.insert(Puzzle(p), Word(w.clone()));
        }

        Dictionary { words: words, solutions: solutions }
    }
}

impl CheckWord for Dictionary {
    /// Check if a word is in the dictionary.
    fn is_solution(&self, w: &Word) -> bool {
        self.words.contains(&w.normalize())
    }

    /// Check how many solutions a given puzzle has in the dictionary.
    fn no_of_solutions(&self, p: &Puzzle) -> usize {
        let sols = self.solutions.get_vec(&sort_puzzle(p));
        if let Some(v) = sols {
            return v.len();
        }

        0
    }

    /// Find all solutions given a word
    fn find_solutions(&self, p: &Puzzle) -> Option<Vec<Word>> {
        self.solutions.get_vec(&sort_puzzle(p)).cloned()
    }

    fn has_solution(&self, p: &Puzzle) -> bool {
        self.solutions.contains_key(&sort_puzzle(p))
    }
}

#[cfg(test)]
mod tests {
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

    const NO_OF_SOLUTIONS_TESTS: &'static [(&'static str, usize)] = &[
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

        for word in NON_SOLUTION_TESTS.iter().map(|x| Word(x.to_string())) {
            assert!(!d.is_solution(&word), "{:?} should NOT be a solution!", word);
        }
    }

    #[test]
    fn no_of_solutions_test() {
        let d = Dictionary::new(WORDS.iter().map(|x| x.to_string()));

        for (puzzle, expected) in NO_OF_SOLUTIONS_TESTS.iter().map(|x| (Puzzle(x.0.to_string()), x.1)) {
            let actual = d.no_of_solutions(&puzzle);
            assert!(expected == actual, "Puzzle: {:?}, Expected: {}, Actual: {}", puzzle, expected, actual);
        }
    }

    #[test]
    fn find_solutions_test() {
        let d = Dictionary::new(WORDS.iter().map(|x| x.to_string()));

        let puzzle = Puzzle("SPDATOREL".to_string());
        let mut expected: Vec<String> = vec![
            "DATORSPEL".to_string(),
            "SPELDATOR".to_string()];
        expected.sort();

        let actual_words = d.find_solutions(&puzzle).unwrap();
        let mut actual: Vec<String> = actual_words.iter().map(|&Word(ref w)| w.clone()).collect();
        actual.sort();

        assert!(expected == actual, "Expected {:?}, Actual {:?}", expected, actual);
    }

    #[test]
    fn has_solution_test() {
        let d = Dictionary::new(WORDS.iter().map(|x| x.to_string()));

        assert!(d.has_solution(&Puzzle("SPELDATOR".to_string())));
        assert!(!d.has_solution(&Puzzle("NOTAWORDX".to_string())));
    }
}