#[derive(Eq, PartialEq, Hash, Debug)]
pub struct Puzzle(pub String);
#[derive(Hash, Eq, PartialEq, Debug)]
pub struct Word(pub String);

use regex::Regex;

impl Word {
    // Normalize a word by removing all non-alpha characters.
    pub fn normalize(&self) -> Word {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"[^A-Za-zåäöÅÄÖ]").unwrap();
        }

        let &Word(ref w) = self;
        Word(RE.replace_all(w.as_str(), "").to_uppercase())
    }
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
}
