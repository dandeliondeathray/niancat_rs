use types::*;
use std::collections::HashMap;
use std::fmt;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct SlackResponse(pub Channel, pub String);

pub type TooMany = String;
pub type TooFew = String;
pub type WordHash = String;

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct SolutionsMap(pub HashMap<Word, Vec<String>>);

#[derive(Eq, PartialEq, Debug)]
pub enum Reason {
    NotInDictionary,
    NotNineCharacters,
    NonMatchingWord(Puzzle, TooMany, TooFew),
}

#[derive(Eq, PartialEq, Debug)]
pub enum InvalidPuzzleReason {
    NotInDictionary,
    NotNineCharacters,
}

#[derive(Eq, PartialEq, Debug)]
pub enum InvalidCommandReason {
    UnknownCommand,
    WrongNoOfParameters,
}

#[derive(Eq, PartialEq, Debug)]
pub enum Response {
    GetPuzzle(Channel, Puzzle, usize),
    NoPuzzleSet(Channel),
    SetPuzzle(Channel, Puzzle, usize),
    InvalidPuzzle(Channel, Puzzle, InvalidPuzzleReason),
    CorrectSolution(Channel, Word),
    Notification(Name, WordHash),
    SolutionsNotification(SolutionsMap),
    IncorrectSolution(Channel, Word, Reason),
    Help(Channel),
    Dual(Box<Response>, Box<Response>),
    //Triple(Box<Response>, Box<Response>, Box<Response>),
}

#[derive(PartialEq, Eq, Debug)]
pub struct InvalidCommand(pub Channel, pub String, pub InvalidCommandReason);

pub trait Respond {
    fn serialize(&self, r: &Response) -> Vec<SlackResponse>;
    fn serialize_invalid_command(&self, r: &InvalidCommand) -> Vec<SlackResponse>;
}

struct SlackResponder {
    main_channel: Channel,
}

pub fn break_puzzle(&Puzzle(ref p): &Puzzle) -> String {
    if !is_right_length(&p) {
        panic!("Can't break apart puzzle, because {} is not the right length!", p);
    }

    let a = p.chars().take(3).collect::<String>();
    let b = p.chars().skip(3).take(3).collect::<String>();
    let c = p.chars().skip(6).take(3).collect::<String>();
    let puzzle = vec![a, b, c].join(" ");
    puzzle
}

const HELP_TEXT: &'static str = r#"
"Dagens nia" är ett ordpussel från Svenska Dagbladet. Varje dag får man nio bokstäver, och ska hitta
vilket svenskt ord man kan konstruera med hjälp av dessa bokstäver.
Boten 'niancat' hjälper dig att lösa nian genom att kontrollera om ord finns med i SAOL eller inte,
och att bokstäverna matchar dagens nia. Om du skriver in ett lösningsförslag i ett privat-meddelande
till boten så kommer den säga till om ordet är korrekt, och i sådana fall automatiskt notifiera
kanalen om att du hittat en lösning.

Innan du har löst dagens nia är det bra om du inte skriver in lösningsförslag i kanalen, då det är
möjligt att du är nära utan att veta om det, och därmed i praktiken löser den åt andra. När du löst
den kan du skriva lösningsförslag i kanalen, men håll dig gärna till ord som inte är nära den
riktiga lösningen.

Kommandon:
    !setnian <pussel>   Sätt nian.
    !nian               Visa nian.
    !unsolution <text>  Sätt en olösning, att visas när nästa nian sätts.
    !unsolutions        Visa alla mina olösning. Svar, om det finns, visas i en privat kanal.
    !helpnian           Visa denna hjälptext.

Alla dessa kommandon kan man köra både i kanalen och i privat-meddelande till tiancat.
"#;

impl fmt::Display for SolutionsMap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "*Gårdagens lösningar:*\n"));

        for (&Word(ref word), solvers) in &self.0 {
            try!(write!(f, "*{}*: {}\n", word, solvers.join(", ")));
        }
        Ok(())
    }
}

impl Respond for SlackResponder {
    fn serialize(&self, r: &Response) -> Vec<SlackResponse> {
        match *r {
            Response::NoPuzzleSet(ref channel) => vec![
                SlackResponse(channel.clone(),
                    format!("Nian är inte satt!"))
            ],

            Response::GetPuzzle(ref channel, ref puzzle, 1) => vec![
                    SlackResponse(channel.clone(), format!("{}",
                                  break_puzzle(puzzle)))
            ],

            Response::GetPuzzle(ref channel, ref puzzle, n) => vec![
                    SlackResponse(channel.clone(),
                        format!("{}.\nDet finns {} lösningar.",
                                break_puzzle(puzzle), n))
            ],

            Response::SetPuzzle(ref channel, ref puzzle, 1) => vec![
                    SlackResponse(channel.clone(), format!("Dagens nia är satt till {}.",
                                  break_puzzle(puzzle)))
            ],

            Response::SetPuzzle(ref channel, ref puzzle, n) => vec![
                    SlackResponse(channel.clone(),
                        format!("Dagens nia är satt till {}.\nDet finns {} lösningar.",
                                break_puzzle(puzzle), n))
            ],

            Response::InvalidPuzzle(ref channel, Puzzle(ref puzzle), InvalidPuzzleReason::NotNineCharacters) => vec![
                SlackResponse(channel.clone(),
                    format!("Ogiltig nian! {} är inte nio tecken långt.", puzzle))
            ],

            Response::InvalidPuzzle(ref channel, Puzzle(ref puzzle), InvalidPuzzleReason::NotInDictionary) => vec![
                SlackResponse(channel.clone(),
                    format!("Ogiltig nian! {} finns inte med i SAOL.", puzzle))
            ],

            Response::CorrectSolution(ref channel, Word(ref word)) => vec![
                SlackResponse(channel.clone(),
                    format!("Ordet {} är korrekt!", word))
            ],

            Response::Notification(Name(ref name), ref hash) => vec![
                SlackResponse(self.main_channel.clone(),
                    format!("{} löste nian: {}", name, hash))
            ],

            Response::IncorrectSolution(ref channel, Word(ref w), Reason::NotInDictionary) => vec![
                SlackResponse(channel.clone(),
                    format!("Ordet {} finns inte med i SAOL.", w))
            ],

            Response::IncorrectSolution(ref channel, Word(ref w), Reason::NotNineCharacters) => vec![
                SlackResponse(channel.clone(),
                    format!("Ordet {} är inte nio tecken långt.", w))
            ],

            Response::IncorrectSolution(ref channel, Word(ref w),
                                         Reason::NonMatchingWord(Puzzle(ref puzzle), ref too_many, ref too_few)) => vec![
                SlackResponse(channel.clone(),
                    format!("Ordet {} matchar inte dagens nia {}. För många {}, för få {}.", w, puzzle, too_many, too_few))
            ],

            Response::SolutionsNotification(ref solutions) => vec![
                SlackResponse(self.main_channel.clone(), format!("{}", solutions)),
            ],

            Response::Help(ref channel) => vec![
                SlackResponse(channel.clone(), format!("{}", HELP_TEXT))
            ],

            Response::Dual(ref first, ref second) => {
                let mut f = self.serialize(&first);
                let mut s = self.serialize(&second);

                f.append(&mut s);
                f
            },

            //Response::Triple(ref first, ref second, ref third) => {
            //    let mut f = self.serialize(&first);
            //    let mut s = self.serialize(&second);
            //    let mut t = self.serialize(&third);
            //
            //    f.append(&mut s);
            //    f.append(&mut t);
            //    f
            //},
        }
    }

    fn serialize_invalid_command(&self, &InvalidCommand(ref channel,
                                                        ref text,
                                                        ref reason): &InvalidCommand) -> Vec<SlackResponse> {
        let reason_string = match *reason {
            InvalidCommandReason::UnknownCommand => "okänt kommando!",
            InvalidCommandReason::WrongNoOfParameters => "fel antal parametrar!"
        };
        vec![
            SlackResponse(channel.clone(),
                          format!("Ogiltigt kommando '{}'. Orsak: {}", text, reason_string))
        ]
    }
}

pub fn new_responder(main_channel: &Channel) -> Box<Respond> {
    Box::new(SlackResponder { main_channel: main_channel.clone() })
}

#[cfg(test)]
mod tests {
    use super::*;
    use types::*;
    use std::collections::HashMap;
    use std::iter::FromIterator;

    #[test]
    #[should_panic]
    fn test_break_puzzle_panics() {
        break_puzzle(&Puzzle("ABC".into()));
    }

    #[test]
    fn test_break_puzzle() {
        assert_eq!(break_puzzle(&Puzzle("ABCDEFGHI".into())), "ABC DEF GHI".to_string());
        assert_eq!(break_puzzle(&Puzzle("ABÅÄÖCDEF".into())), "ABÅ ÄÖC DEF".to_string());
    }

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

            ResponderTest {
                description: "Incorrect solution response to user",
                response: Response::IncorrectSolution(Channel("D0".into()), Word("FOO".into()),
                                                      Reason::NotInDictionary),
                expected: vec![
                    TestEvent {
                        channel: Channel("D0".into()),
                        has_texts: vec!["FOO", "inte"],
                        has_not_texts: vec![],
                    }
                ],
            },

            ResponderTest {
                description: "Invalid puzzle",
                response: Response::InvalidPuzzle(Channel("C0".into()), Puzzle("PUZZLE".into()),
                                                  InvalidPuzzleReason::NotNineCharacters),
                expected: vec![
                    TestEvent {
                        channel: Channel("C0".into()),
                        has_texts: vec!["PUZZLE"],
                        has_not_texts: vec![],
                    }
                ],
            },

            ResponderTest {
                description: "No puzzle set",
                response: Response::NoPuzzleSet(Channel("C0".into())),
                expected: vec![
                    TestEvent {
                        channel: Channel("C0".into()),
                        has_texts: vec!["inte", "satt"],
                        has_not_texts: vec![]
                    }
                ]
            },

            ResponderTest {
                description: "Set puzzle response, many solutions",
                response: Response::SetPuzzle(Channel("C0".into()), Puzzle("PUZZLEABC".into()), 17),
                expected: vec![
                    TestEvent {
                        channel: Channel("C0".into()),
                        has_texts: vec!["PUZ ZLE ABC", "17"],
                        has_not_texts: vec![],
                    }
                ]
            },

            ResponderTest {
                description: "Set puzzle response, one solution",
                response: Response::SetPuzzle(Channel("C0".into()), Puzzle("PUZZLEABC".into()), 1),
                expected: vec![
                    TestEvent {
                        channel: Channel("C0".into()),
                        has_texts: vec!["PUZ ZLE ABC"],
                        has_not_texts: vec!["1"]
                    }
                ]
            },

            ResponderTest {
                description: "Correct solution response to user",
                response: Response::CorrectSolution(Channel("D0".into()), Word("FOO".into())),
                expected: vec![
                    TestEvent {
                        channel: Channel("D0".into()),
                        has_texts: vec!["FOO", "korrekt"],
                        has_not_texts: vec![],
                    }
                ]
            },

           ResponderTest {
                description: "Get puzzle, many solutions",
                response: Response::GetPuzzle(Channel("C0".into()), Puzzle("PUZZLEABC".into()), 17),
                expected: vec![
                    TestEvent {
                        channel: Channel("C0".into()),
                        has_texts: vec!["PUZ ZLE ABC", "17"],
                        has_not_texts: vec![],
                    }
                ]
            },

            ResponderTest {
                description: "Get puzzle, one solution",
                response: Response::GetPuzzle(Channel("C0".into()), Puzzle("PUZZLEABC".into()), 1),
                expected: vec![
                    TestEvent {
                        channel: Channel("C0".into()),
                        has_texts: vec!["PUZ ZLE ABC"],
                        has_not_texts: vec!["1"],
                    }
                ]
            },

            ResponderTest {
                description: "Composite responses",
                response: Response::Dual(
                    Box::new(Response::CorrectSolution(Channel("D0".into()), Word("FOO".into()))),
                    Box::new(Response::Notification(Name("erike".into()), "abcdef".into()))),
                expected: vec![
                    TestEvent {
                        channel: Channel("D0".into()),
                        has_texts: vec!["FOO"],
                        has_not_texts: vec![],
                    },
                    TestEvent {
                        channel: Channel("C0123".into()),
                        has_texts: vec!["erike", "abcdef"],
                        has_not_texts: vec![],
                    },
                ]
            },

            //ResponderTest {
            //    description: "Triple responses",
            //    response: Response::Triple(
            //        Box::new(Response::CorrectSolution(Channel("D0".into()), Word("FOO".into()))),
            //        Box::new(Response::Notification(Name("erike".into()), "abcdef".into())),
            //        Box::new(Response::GetPuzzle(Channel("C0".into()), Puzzle("PUZZLEABC".into()), 1))),
            //    expected: vec![
            //        TestEvent {
            //            channel: Channel("D0".into()),
            //            has_texts: vec!["FOO"],
            //            has_not_texts: vec![],
            //        },
            //        TestEvent {
            //            channel: Channel("C0123".into()),
            //            has_texts: vec!["erike", "abcdef"],
            //            has_not_texts: vec![],
            //        },
            //        TestEvent {
            //            channel: Channel("C0".into()),
            //            has_texts: vec!["PUZ ZLE ABC"],
            //            has_not_texts: vec!["1"],
            //        },
            //    ]
            //},

            ResponderTest {
                description: "Help command",
                response: Response::Help(Channel("C0".into())),
                expected: vec![
                    TestEvent {
                        channel: Channel("C0".into()),
                        has_texts: vec!["!setnian", "!nian", "!helpnian"],
                        has_not_texts: vec![],
                    }
                ]
            },

            ResponderTest {
                description: "Incorrect solution, because it's not nine characters",
                response: Response::IncorrectSolution(Channel("D0".into()), Word("FOO".into()), Reason::NotNineCharacters),
                expected: vec![
                    TestEvent {
                        channel: Channel("D0".into()),
                        has_texts: vec!["FOO", "inte nio tecken"],
                        has_not_texts: vec![],
                    }
                ]
            },

            ResponderTest {
                description: "Incorrect solution, because it doesn't match todays puzzle",
                response: Response::IncorrectSolution(
                    Channel("D0".into()),
                    Word("FOO".into()),
                    Reason::NonMatchingWord(Puzzle("PUSSELDEF".into()), "ABC".into(), "DEF".into())),
                expected: vec![
                    TestEvent {
                        channel: Channel("D0".into()),
                        has_texts: vec!["FOO", "PUSSELDEF", "matchar inte", "många ABC", "få DEF"],
                        has_not_texts: vec![],
                    }
                ]
            },

            ResponderTest {
                description: "Notify main channel with solutions",
                response: Response::SolutionsNotification(
                    SolutionsMap(HashMap::from_iter(vec![
                        (Word("DATORSPEL".into()), vec!["foo".to_string(), "bar".to_string()]),
                        (Word("SPELDATOR".into()), vec![]),
                        ].into_iter()))),
                expected: vec![
                    TestEvent {
                        channel: main_channel_id.clone(),
                        has_texts: vec!["foo", "bar", "DATORSPEL", "SPELDATOR"],
                        has_not_texts: vec![],
                    }
                ]
            },
        ];

        for t in tests {
            let responder = new_responder(&main_channel_id);
            let slack_responses = responder.serialize(&t.response);

            assert_eq!(slack_responses.len(), t.expected.len(), "{}", t.description);

            for (expected, actual) in t.expected.iter().zip(slack_responses) {
                assert_eq!(expected.channel, actual.0, "{}, wrong channel", t.description);

                for s in &expected.has_texts {
                    assert!(actual.1.contains(s), "Expected substring {}, in actual string {}", s, actual.1);
                }

                for s in &expected.has_not_texts {
                    assert!(!actual.1.contains(s), "Did not expect substring {}, in actual string {}", s, actual.1);
                }
            }
        }
    }

    #[test]
    fn invalid_command_test() {
        let expected = vec![
            TestEvent {
                channel: Channel("C0".into()),
                has_texts: vec!["känt"],
                has_not_texts: vec![],
            }
        ];

        let main_channel_id = Channel("C0123".into());
        let responder = new_responder(&main_channel_id);

        let r = InvalidCommand(Channel("C0".into()), "!nosuchcommand".into(), InvalidCommandReason::UnknownCommand);
        let slack_responses = responder.serialize_invalid_command(&r);

        assert_eq!(slack_responses.len(), expected.len(), "{}", "Unexpected response length for invalid command");

        for (expected, actual) in expected.iter().zip(slack_responses) {
            assert_eq!(expected.channel, actual.0, "{}, wrong channel", "Invalid command");

            for s in &expected.has_texts {
                assert!(actual.1.contains(s), "Expected substring {}, in actual string {}", s, actual.1);
            }

            for s in &expected.has_not_texts {
                assert!(!actual.1.contains(s), "Did not expect substring {}, in actual string {}", s, actual.1);
            }
        }
    }



//
//    ResponderTest {
//        "Set an unsolution",
//        SetUnsolutionResponse(Channel("D0"), utf8("Hello")),
//        expected: vec![
//            TestEvent {
//                channel: Channel("D0"), "Hello", "Olösning")]),
//
//    ResponderTest {
//        "Get an unsolution",
//        GetUnsolutionsResponse(Channel("D0"), [utf8("Hello"), utf8("world")]),
//        expected: vec![
//            TestEvent {
//                channel: Channel("D0"), "Hello", "world")]),
//
//    ResponderTest {
//        "Notification response",
//        UnsolutionNotificationResponse(Dict{UserId, UnsolutionList}(
//            UserId("U0") => [utf8("FOO")],
//            UserId("U1") => [utf8("BAR"), utf8("BAZ")])),
//        expected: vec![
//            TestEvent {
//                channel: main_channel_id, "<@U0>", "FOO", "<@U1>", "BAR", "BAZ")]),
//
//    ResponderTest {
//        "Previous solutions response",
//        PreviousSolutionsResponse(Dict{Word, Vector{UserId}}(
//            Word("FOO") => [UserId("U0"), UserId("U1")],
//            Word("BAR") => [])),
//        expected: vec![
//            TestEvent {
//                channel: main_channel_id, "<@U0>", "FOO", "<@U1>", "BAR", "Gårdagens lösningar")]),
//
//    ResponderTest {
//        "Previous solutions response, only one solution",
//        PreviousSolutionsResponse(Dict{Word, Vector{UserId}}(
//            Word("FOO") => [UserId("U0"), UserId("U1")])),
//        expected: vec![
//           TestEvent {
//               channel: main_channel_id, "<@U0>", "FOO", "<@U1>", "Gårdagens lösning";
//            has_not=["lösningar"])])
//]
}