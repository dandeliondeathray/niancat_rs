use types::*;

use std::fmt;
use std::io;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct SlackResponse(pub Channel, pub String);

pub type TooMany = String;
pub type TooFew = String;
pub type WordHash = String;

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

#[derive(PartialEq, Eq, Debug)]
pub struct InvalidCommand(pub Channel, pub String, pub InvalidReason);

impl fmt::Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "")
    }
}

pub trait Respond {
    fn serialize(&self, r: &Response) -> Vec<SlackResponse>;
}

struct SlackResponder {
    main_channel: Channel,
}

impl Respond for SlackResponder {
    fn serialize(&self, r: &Response) -> Vec<SlackResponse> {
        vec![SlackResponse(Channel("C0".into()), "".into())]
    }
}

pub fn new_responder(main_channel: &Channel) -> Box<Respond> {
    Box::new(SlackResponder { main_channel: main_channel.clone() })
}

mod tests {
    use super::*;
    use types::*;

    struct TestEvent {
        channel:       Channel,
        has_texts:     Vec<&'static str>,
        has_not_texts: Vec<&'static str>,
    }

    struct ResponderTest {
        description: &'static str,
        response:    Response,
        expected:    Vec<TestEvent>,
    }

    #[test]
    fn responder_test() {
        let main_channel_id = Channel("C0123".into());
        let tests = vec![
            ResponderTest {
                description: "Solution notification response to main channel",
                response: Response::Notification(Name("erike".into()), "abcdef".into()),
                expected: vec![
                    TestEvent {
                        channel: main_channel_id.clone(),
                        has_texts: vec!["erike", "abcdef"],
                        has_not_texts: vec![],
                    },
                ],
            },
        ];

        for t in tests {
            let responder = new_responder(&main_channel_id);
            let slack_responses = responder.serialize(&t.response);

            assert_eq!(slack_responses.len(), t.expected.len(), "{}", t.description);

            for (expected, actual) in t.expected.iter().zip(slack_responses) {
                assert_eq!(expected.channel, actual.0);

                for s in &expected.has_texts {
                    assert!(actual.1.contains(s), "Expected substring {}, in actual string {}", s, actual.1);
                }

                for s in &expected.has_not_texts {
                    assert!(!actual.1.contains(s), "Did not expect substring {}, in actual string {}", s, actual.1);
                }
            }
        }
    }

//responder_tests = [
//    ResponderTest(
//        "Solution notification response to main channel",
//        Response::Notification(SlackName("erike"), utf8("abcdef")),
//        [TestEvent(main_channel_id, "erike", "abcdef")]),
//
//    ResponderTest(
//        "Incorrect solution response to user",
//        IncorrectSolutionResponse(ChannelId("D0"), Word("FOO"), :not_in_dictionary),
//        [TestEvent(ChannelId("D0"), "FOO", "inte")]),
//
//    ResponderTest(
//        "Correct solution response to user",
//        CorrectSolutionResponse(ChannelId("D0"), Word("FOO")),
//        [TestEvent(ChannelId("D0"), "FOO", "korrekt")]),
//
//    ResponderTest(
//        "Unknown user",
//        UnknownUserSolutionResponse(UserId("U0")),
//        [TestEvent(main_channel_id, "<@U0>", "känd")]),
//
//    ResponderTest(
//        "Get puzzle, many solutions",
//        GetPuzzleResponse(ChannelId("C0"), Puzzle("PUZZLEABC"), 17),
//        [TestEvent(ChannelId("C0"), "PUZ ZLE ABC", "17")]),
//
//    ResponderTest(
//        "Get puzzle, one solution",
//        GetPuzzleResponse(ChannelId("C0"), Puzzle("PUZZLEABC"), 1),
//        [TestEvent(ChannelId("C0"), "PUZ ZLE ABC"; has_not=["1"])]),
//
//    ResponderTest(
//        "No puzzle set",
//        NoPuzzleSetResponse(ChannelId("C0")),
//        [TestEvent(ChannelId("C0"), "inte", "satt")]),
//
//    ResponderTest(
//        "Set puzzle response, many solutions",
//        SetPuzzleResponse(ChannelId("C0"), Puzzle("PUZZLEABC"), 17),
//        [TestEvent(ChannelId("C0"), "PUZ ZLE ABC", "17")]),
//
//    ResponderTest(
//        "Set puzzle response, one solution",
//        SetPuzzleResponse(ChannelId("C0"), Puzzle("PUZZLEABC"), 1),
//        [TestEvent(ChannelId("C0"), "PUZ ZLE ABC"; has_not=["1"])]),
//
//    ResponderTest(
//        "Invalid puzzle",
//        InvalidPuzzleResponse(ChannelId("C0"), Puzzle("PUZZLE")),
//        [TestEvent(ChannelId("C0"), "PUZZLE")]),
//
//    ResponderTest(
//        "Ignore events",
//        IgnoredEventResponse(ChannelId("C0"), utf8("Some text")),
//        []),
//
//    ResponderTest(
//        "Composite responses",
//        CompositeResponse(
//            CorrectSolutionResponse(ChannelId("D0"), Word("FOO")),
//            Response::Notification(SlackName("erike"), utf8("abcdef"))),
//        [TestEvent(ChannelId("D0"), "FOO"),
//         TestEvent(ChannelId("C0123"), "erike", "abcdef")]),
//
//    ResponderTest(
//        "Invalid command",
//        InvalidCommandResponse(ChannelId("C0"), :unknown),
//        [TestEvent(ChannelId("C0"), "känt")]),
//
//    ResponderTest(
//        "Help command",
//        HelpResponse(ChannelId("C0")),
//        [TestEvent(ChannelId("C0"), "!setnian", "!nian", "!helpnian")]),
//
//    ResponderTest(
//        "Incorrect solution, because it's not nine characters",
//        IncorrectSolutionResponse(ChannelId("D0"), Word("FOO"), :not_nine_characters),
//        [TestEvent(ChannelId("D0"), "FOO", "inte nio tecken")]),
//
//    ResponderTest(
//        "Incorrect solution, because it doesn't match todays puzzle",
//        NonMatchingWordResponse(
//            ChannelId("D0"), Word("FOO"), Puzzle("BAR"), utf8("ABC"), utf8("DEF")),
//        [TestEvent(ChannelId("D0"), "FOO", "BAR", "matchar inte", "många ABC", "få DEF")]),
//
//    ResponderTest(
//        "Incorrect solution, because it doesn't match todays puzzle",
//        NonMatchingWordResponse(
//            ChannelId("D0"), Word("FOO"), Puzzle("BAR"), utf8("ABC"), utf8("")),
//        [TestEvent(ChannelId("D0"), "FOO", "matchar inte", "många ABC";
//            has_not=["få"])]),
//
//    ResponderTest(
//        "Incorrect solution, because it doesn't match todays puzzle",
//        NonMatchingWordResponse(
//            ChannelId("D0"), Word("FOO"), Puzzle("BAR"), utf8(""), utf8("DEF")),
//        [TestEvent(ChannelId("D0"), "FOO", "matchar inte", "få DEF";
//            has_not=["många"])]),
//
//    ResponderTest(
//        "Incorrect solution, for unknown reason",
//        IncorrectSolutionResponse(ChannelId("D0"), Word("FOO"), :other_reason),
//        [TestEvent(ChannelId("D0"), "FOO", "oklara skäl")]),
//
//    ResponderTest(
//        "Set an unsolution",
//        SetUnsolutionResponse(ChannelId("D0"), utf8("Hello")),
//        [TestEvent(ChannelId("D0"), "Hello", "Olösning")]),
//
//    ResponderTest(
//        "Get an unsolution",
//        GetUnsolutionsResponse(ChannelId("D0"), [utf8("Hello"), utf8("world")]),
//        [TestEvent(ChannelId("D0"), "Hello", "world")]),
//
//    ResponderTest(
//        "Notification response",
//        UnsolutionNotificationResponse(Dict{UserId, UnsolutionList}(
//            UserId("U0") => [utf8("FOO")],
//            UserId("U1") => [utf8("BAR"), utf8("BAZ")])),
//        [TestEvent(main_channel_id, "<@U0>", "FOO", "<@U1>", "BAR", "BAZ")]),
//
//    ResponderTest(
//        "Previous solutions response",
//        PreviousSolutionsResponse(Dict{Word, Vector{UserId}}(
//            Word("FOO") => [UserId("U0"), UserId("U1")],
//            Word("BAR") => [])),
//        [TestEvent(main_channel_id, "<@U0>", "FOO", "<@U1>", "BAR", "Gårdagens lösningar")]),
//
//    ResponderTest(
//        "Previous solutions response, only one solution",
//        PreviousSolutionsResponse(Dict{Word, Vector{UserId}}(
//            Word("FOO") => [UserId("U0"), UserId("U1")])),
//        [TestEvent(main_channel_id, "<@U0>", "FOO", "<@U1>", "Gårdagens lösning";
//            has_not=["lösningar"])])
//]
}