use crypto::digest::Digest;
use crypto::sha2::Sha256;
use std::collections::{HashMap, HashSet};
use std::iter::{FromIterator, repeat};
use multimap::MultiMap;

use types::*;
use dictionary::*;

pub struct Niancat<'a> {
    puzzle: Option<Puzzle>,
    solutions: MultiMap<Word, String>,
    dictionary: &'a CheckWord,
}

impl<'a> Niancat<'a> {
    pub fn new<T: CheckWord>(dictionary: &'a T) -> Niancat<'a> {
        Niancat { puzzle: None,
                  solutions: MultiMap::new(),
                  dictionary: dictionary,
                }
    }
}

#[derive(PartialEq, Eq, Debug)]
pub enum Command {
    GetPuzzle(Channel),
    SetPuzzle(Channel, Puzzle),
    CheckSolution(Channel, Name, Word),
}

pub fn apply(command: &Command, state: &mut Niancat) -> Response {
    match command {
        &Command::GetPuzzle(ref c) => get_puzzle(state, &c),
        &Command::SetPuzzle(ref channel, ref puzzle) => set_puzzle(state, &channel, &puzzle),
        &Command::CheckSolution(ref chan, ref name, ref word) => check_solution(state, &chan, &name, &word),
    }
}


fn get_puzzle(state: &mut Niancat, channel: &Channel) -> Response {
    match state.puzzle {
        Some(ref puzzle) => Response::GetCommand(channel.clone(), puzzle.clone()),
        None => Response::NoPuzzleSet(channel.clone())
    }
}

fn set_puzzle(state: &mut Niancat, channel: &Channel, puzzle: &Puzzle) -> Response {
    if !is_right_length(&puzzle.0) {
        return Response::InvalidPuzzle(channel.clone(), puzzle.clone(), Reason::NotNineCharacters);
    }
    if state.dictionary.has_solution(&puzzle) {
        state.puzzle = Some(puzzle.clone());
        Response::SetPuzzle(channel.clone(), puzzle.clone())
    } else {
        Response::InvalidPuzzle(channel.clone(), puzzle.clone(), Reason::NotInDictionary)
    }
}

fn check_solution(state: &mut Niancat, channel: &Channel, name: &Name, word: &Word) -> Response {
    if let Some(ref puzzle) = state.puzzle {
        if !is_right_length(&word.0) {
            return Response::IncorrectSolution(channel.clone(), word.clone(),
                Reason::NotNineCharacters);
        }

        if let Some((too_few, too_many)) = non_match(&puzzle, &word) {
            let reason = Reason::NonMatchingWord(too_many, too_few);
            return Response::IncorrectSolution(channel.clone(), word.clone(),
                reason)
        }

        if state.dictionary.is_solution(&word) {
            let hash = solution_hash(&word.normalize(), &name);
            let correct_solution = Response::CorrectSolution(channel.clone(),
                word.clone());
            let notification = Response::Notification(name.clone(), hash);
            return Response::DualResponse(Box::new(correct_solution), Box::new(notification));
        } else {
            return Response::IncorrectSolution(channel.clone(), word.clone(),
                Reason::NotInDictionary);
        }
    } else {
        Response::NoPuzzleSet(channel.clone())
    }
}

pub fn solution_hash(&Word(ref s): &Word, &Name(ref nick): &Name) -> String {
    let mut hasher = Sha256::new();
    hasher.input_str(s.as_str());
    hasher.input_str(nick.as_str());
    hasher.result_str()
}

pub fn string_to_dict(s: &String) -> HashMap<char, u32> {
    let mut h = HashMap::new();
    for c in s.chars() {
        let counter = h.entry(c).or_insert(0);
        *counter += 1;
    }
    h
}

fn is_right_length(w: &String) -> bool {
    return w.chars().collect::<Vec<char>>().len() == 9
}

pub fn non_match(&Puzzle(ref puzzle): &Puzzle, &Word(ref word): &Word) -> Option<(String, String)> {
    let mut too_many = String::new();
    let mut too_few = String::new();

    let puzzle_chars = string_to_dict(&puzzle);
    let word_chars = string_to_dict(&word);

    let all_chars: HashSet<char> = HashSet::from_iter(puzzle.chars().chain(word.chars()));
    for c in &all_chars {
        let p_count: u32 = puzzle_chars.get(c).map(|&x| x).unwrap_or(0);
        let w_count: u32 = word_chars.get(c).map(|&x| x).unwrap_or(0);

        if w_count > p_count {
            let count = w_count - p_count;
            let char_string: String = repeat(c).cloned().take(count as usize).collect();
            too_many.push_str(&char_string);
        } else if w_count < p_count {
            let count = p_count - w_count;
            let char_string: String = repeat(c).cloned().take(count as usize).collect();
            too_few.push_str(&char_string);
        }
    }

    if too_many.is_empty() && too_few.is_empty() {
        None
    } else {
        let mut tm: Vec<char> = too_many.chars().collect();
        let mut tf: Vec<char> = too_few.chars().collect();
        tm.sort();
        tf.sort();

        Some((tf.into_iter().collect(), tm.into_iter().collect()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use types::*;
    use dictionary::*;

    const HASH_TESTS: &'static [(&'static str, &'static str, &'static str)] = &[
        ("GALLTJUTA", "f00ale",   "f72e9a9523bbc72bf7366a58a04046408d2d88ea811afdc9a459d24e077fa71d"),
        ("GALLTJUTA", "erike",    "d8e7363cdad6303dd4c41cb2ad3e2c35759257ca8ac509107e4e9e9ff5741933"),
        ("GALLTJUTA", "cadaker",  "203ecbdeba638d0c6c4a3a3ab17c2704bdf9c79016a392ccf303615534392e9c"),
        ("GALLTJUTA", "johaper",  "80b3ac9c8150684994df7302a3897fbfe551c52dcd2c8cb2e1cf948129ce9483"),
        ("GALLTJUTA", "andrnils", "2da8b95f6d58652bf87547ed0106e3d2a8e2915cc9b09710ef52d57aa43df5c8"),

        ("ÅÄÖABCDEF", "f00ale",   "71edbbe7b1905edc4daf94208ce22eb570fc478de0b346743abd7449d1e7d822"),
        ("ÅÄÖABCDEF", "erike",    "adbc40e1e9d2c5da069c410a9d6e6d485fd2f7e14856b97560e759ad028b9d2d"),
        ("ÅÄÖABCDEF", "cadaker",  "0d7d353ab20469b1c4bf8446d7297860022bbc19c6ae771f351ae597bf56e0dd"),
        ("ÅÄÖABCDEF", "johaper",  "9280130c1ee9109b63810d5cfdcb456fba8fd5d742b578f60e947c24ba5a6c4f"),
        ("ÅÄÖABCDEF", "andrnils", "8027afb1b362daa27be64edf1806d50a344082d3a534cfc38c827a7e71bc8779")
    ];

    const NON_MATCHING_TESTS: &'static [(&'static str, &'static str, &'static str, &'static str)] = &[
        ("GALLTJUTA", "GALLTJUTR", "R", "A"),
        ("GALLTJUTA", "GALRTJUTA", "R", "L"),
        ("GALLTJUTA", "GBLLTJUTC", "BC", "AA"),
        ("ABCDEFÅÄÖ", "ABCDEFÅÄÄ", "Ä", "Ö")
    ];

    const MATCHING_TESTS: &'static [(&'static str, &'static str)] = &[
        ("GALLTJUTA", "GALLTJUTA"),
        ("GALLTJUTA", "AGALLTJUT"),
        ("GALLTJUTA", "GLLUTATJA"),
        ("ABCDEFÅÄÖ", "ÅÄÖABCDEF")
    ];

    #[derive(Clone)]
    struct FakeCheckWord {
        is_solution_v: bool,
        no_of_solutions_v: usize,
        find_solutions_v: Option<Vec<Word>>,
        has_solution_v: bool,
    }

    impl CheckWord for FakeCheckWord {
        fn is_solution(&self, _: &Word) -> bool { self.is_solution_v }
        fn no_of_solutions(&self, _: &Puzzle) -> usize { self.no_of_solutions_v }
        fn find_solutions(&self, _: &Puzzle) -> Option<Vec<Word>> { self.find_solutions_v.clone() }
        fn has_solution(&self, _: &Puzzle) -> bool { self.has_solution_v }
    }

    static DEFAULT_CHECKWORD: FakeCheckWord = FakeCheckWord {
        is_solution_v: true,
        no_of_solutions_v: 1,
        find_solutions_v: None,
        has_solution_v: true };

    static NOT_SOLUTION_CHECKWORD: FakeCheckWord = FakeCheckWord {
        is_solution_v: false,
        no_of_solutions_v: 0,
        find_solutions_v: None,
        has_solution_v: false };

    #[test]
    fn solution_hash_test() {
        for &(word, nick, expected) in HASH_TESTS {
            let actual = solution_hash(&Word(word.to_string()), &Name(nick.to_string()));
            assert!(actual == expected, "Actual hash: {}, expected {}", actual, expected);
        }
    }

    #[test]
    fn non_match_test() {
        for &(puzzle, word, too_many, too_few) in NON_MATCHING_TESTS {
            let actual =
                non_match(&Puzzle(puzzle.to_string()), &Word(word.to_string()));
            if let Some((actual_too_few, actual_too_many)) = actual {
                assert!(too_few.to_string() == actual_too_few, "Too few, expected: {:?}, actual {:?}, puzzle {:?}, word {:?}", too_few, actual_too_few, puzzle, word);
                assert!(too_many.to_string() == actual_too_many, "Too many: expected {:?}, actual {:?}, puzzle {:?}, word {:?}", too_many, actual_too_many, puzzle, word);
            } else {
                assert!(false, "Expected non-matching {:?}, but got match, puzzle {:?}, word {:?}", (too_few, too_many), puzzle, word);
            }
        }
    }

    #[test]
    fn match_test() {
        for &(puzzle, word) in MATCHING_TESTS {
            let actual = non_match(&Puzzle(puzzle.to_string()), &Word(word.to_string()));
            assert!(None == actual, "Expected match, expected: None, actual {:?}, puzzle {:?}, word {:?}", actual, puzzle, word);
        }
    }

    #[test]
    fn get_puzzle_test() {
        let chan = Channel("channel".into());
        let mut state = Niancat::new(&DEFAULT_CHECKWORD);
        let puzzle = Puzzle("ABCDEFGHI".to_string());
        state.puzzle = Some(puzzle.clone());
        let command = Command::GetPuzzle(chan.clone());
        let expected = Response::GetCommand(chan.clone(), puzzle.clone());

        let actual = apply(&command, &mut state);
        assert!(actual == expected);
    }

//    #[test]
//    fn no_puzzle_set_test() {
//        let chan = Channel("channel".into());
//        let mut state = Niancat::new(&DEFAULT_CHECKWORD);
//        let command = GetCommand { channel: &chan };
//        let expected = Response::NoPuzzleSet(chan.clone());
//        let actual = command.apply(&mut state);
//
//        assert!(actual == expected);
//    }
//
//
//    #[test]
//    fn set_puzzle_test() {
//        let channel = Channel("channel".into());
//        let p = Puzzle("ABCDEFGHI".to_string());
//        let mut state = Niancat::new(&DEFAULT_CHECKWORD);
//        let set_command = SetPuzzleCommand { channel: &channel, puzzle: p.clone() };
//        let response = set_command.apply(&mut state);
//
//        assert!(response == Response::SetPuzzle(channel.clone(), p.clone()));
//        assert!(state.puzzle == Some(p));
//    }
//
//    #[test]
//    fn set_invalid_puzzle_test() {
//        let channel = Channel("channel".into());
//        let p = Puzzle("ABCDEF".to_string());
//        let mut state = Niancat::new(&NOT_SOLUTION_CHECKWORD);
//        let set_command = SetPuzzleCommand { channel: &channel, puzzle: p.clone() };
//        let response = set_command.apply(&mut state);
//
//        assert!(response == Response::InvalidPuzzle(channel.clone(), p.clone(), Reason::NotNineCharacters), "Actual response: {:?}", response);
//        assert!(state.puzzle == None);
//
//        let p = Puzzle("IHGFEDCBA".into());
//        let set_command = SetPuzzleCommand { channel: &channel, puzzle: p.clone() };
//        let response = set_command.apply(&mut state);
//
//        assert!(response == Response::InvalidPuzzle(channel.clone(), p.clone(), Reason::NotInDictionary));
//        assert!(state.puzzle == None);
//    }
//
//    #[test]
//    fn solve_puzzle_test() {
//        let channel = Channel("channel".into());
//        let name = Name("erike".to_string());
//        let word = Word("GALLTJUTA".to_string());
//
//        let mut state = Niancat::new(&DEFAULT_CHECKWORD);
//        state.puzzle = Some(Puzzle("AGALLTJUT".into()));
//
//        let command = CheckSolutionCommand {
//            channel: &channel,
//            name: name.clone(),
//            word: word.clone(),
//        };
//        let expected_hash = "d8e7363cdad6303dd4c41cb2ad3e2c35759257ca8ac509107e4e9e9ff5741933".to_string();
//        let expected_solution_response = Response::CorrectSolution(channel.clone(), word.clone());
//        let expected_notification_response = Response::Notification(name.clone(), expected_hash);
//
//        let response = command.apply(&mut state);
//        match response {
//            Response::DualResponse(solution_response, notification_response) => {
//                assert!(*solution_response == expected_solution_response);
//                assert!(*notification_response == expected_notification_response);
//            },
//            _ => assert!(false, "Actual: {:?}", response)
//        }
//    }
//
//    #[test]
//    fn incorrect_word_not_in_dict() {
//        let channel = Channel("channel".into());
//        let name = Name("erike".to_string());
//        let word = Word("IHGFEDCBA".to_string());
//
//        let mut state = Niancat::new(&NOT_SOLUTION_CHECKWORD);
//        state.puzzle = Some(Puzzle("ABCDEFGHI".to_string()));
//
//        let command = CheckSolutionCommand {
//            channel: &channel,
//            name: name.clone(),
//            word: word.clone(),
//        };
//        let response = command.apply(&mut state);
//        assert!(response == Response::IncorrectSolution(channel.clone(), word.clone(), Reason::NotInDictionary));
//    }
//
//    #[test]
//    fn incorrect_word_not_nine() {
//        let channel = Channel("channel".into());
//        let name = Name("erike".to_string());
//        let word = Word("NOTNINE".to_string());
//
//        let mut state = Niancat::new(&NOT_SOLUTION_CHECKWORD);
//        state.puzzle = Some(Puzzle("ABCDEFGHI".to_string()));
//
//        let command = CheckSolutionCommand {
//            channel: &channel,
//            name: name.clone(),
//            word: word.clone(),
//        };
//        let response = command.apply(&mut state);
//        assert!(response == Response::IncorrectSolution(channel.clone(),
//                                                        word.clone(),
//                                                        Reason::NotNineCharacters));
//    }
//
//    #[test]
//    fn incorrect_word_no_puzzle_set() {
//        let channel = Channel("channel".into());
//        let name = Name("erike".to_string());
//        let word = Word("ABCDEFGHI".to_string());
//
//        let mut state = Niancat::new(&NOT_SOLUTION_CHECKWORD);
//
//        let command = CheckSolutionCommand {
//            channel: &channel,
//            name: name.clone(),
//            word: word.clone(),
//        };
//        let response = command.apply(&mut state);
//        assert!(response == Response::NoPuzzleSet(channel.clone()));
//    }
//
//    #[test]
//    fn incorrect_word_non_matching() {
//        let channel = Channel("channel".into());
//        let name = Name("erike".to_string());
//        let word = Word("GALLTJUTA".to_string());
//
//        let mut state = Niancat::new(&NOT_SOLUTION_CHECKWORD);
//        state.puzzle = Some(Puzzle("ABCDEFGHI".to_string()));
//
//        let command = CheckSolutionCommand {
//            channel: &channel,
//            name: name.clone(),
//            word: word.clone(),
//        };
//        let response = command.apply(&mut state);
//        let reason = Reason::NonMatchingWord("AJLLTTU".to_string(), "BCDEFHI".to_string());
//        assert!(response == Response::IncorrectSolution(channel.clone(), word.clone(), reason));
//    }
}