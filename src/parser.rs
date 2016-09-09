use logic::*;

pub fn parse_command(text: &String) -> Option<Command> {
    None
}

mod tests {
    use super::*;
    use types::*;
    use logic::*;

    struct CommandParserTest<'a> {
        description:    &'static str,
        text:           &'static str,
        channel:        &'a Channel,
        expected:       Option<Command>,
    }

    impl<'a> CommandParserTest<'a> {
        fn new(desc: &'static str,
               text: &'static str,
               chan: &'a Channel,
               expected: Option<Command>) -> CommandParserTest<'a> {
            CommandParserTest { description: desc,
                                text: text,
                                channel: &chan,
                                expected: expected }
        }
    }

    //const TEST_USER: Name       = Name("User 0".into());
    //const TEST_CHANNEL: Channel = Channel("C0".into());
    //const IM_CHANNEL: Channel   = Channel("D0".into());

    #[test]
    fn set_puzzle_test() {
        let test_user = Channel("C0".into());
        let test = CommandParserTest::new(
                    "Set puzzle",
                    "!setnian ABCDEFGHI", &test_user,
                    Some(Command::SetPuzzle(test_user.clone(), Puzzle("ABCDEFGHI".into()))));

        let actual = parse_command(&test.text.into());

        assert_eq!(actual, test.expected);
    }


    //        CommandParserTest::new(
    //            "Get puzzle",
    //            "!nian", &TEST_CHANNEL,
    //            cmd(GetCommand::new(&TEST_CHANNEL))),

//    #[test]
//    fn command_tests() {
//        let command_parser_tests: Vec<CommandParserTest> = vec![
//            CommandParserTest::new(
//                "Set puzzle",
//                "!setnian ABCDEFGHI", &TEST_CHANNEL,
//                cmd(SetPuzzleCommand::new(&TEST_CHANNEL, Puzzle("ABCDEFGHI".into())))),
//
//            CommandParserTest::new(
//                "Get puzzle",
//                "!nian", &TEST_CHANNEL,
//                cmd(GetCommand::new(&TEST_CHANNEL))),
//
//            //CommandParserTest::new(
//            //    "Help",
//            //    "!helpnian", &TEST_CHANNEL,
//            //    cmd(HelpCommand(&TEST_CHANNEL, TEST_USER.clone()))),
//
//            CommandParserTest::new(
//                "Ignore non-commands in public channel",
//                "ABCDEFGHI", &TEST_CHANNEL,
//                None),
//
//            CommandParserTest::new(
//                "Check solution",
//                "ABCDEFGHI", &IM_CHANNEL,
//                cmd(CheckSolutionCommand::new(&IM_CHANNEL, TEST_USER.clone(), Word("ABCDEFGHI".into())))),
//
//            CommandParserTest::new(
//                "Check solution, with spaces",
//                "ABC DEF GHI", &IM_CHANNEL,
//                cmd(CheckSolutionCommand::new(&IM_CHANNEL, TEST_USER.clone(), Word("ABCDEFGHI".into())))),
//
//            CommandParserTest::new(
//                "No command",
//                "  ", &TEST_CHANNEL,
//                None),
//
//            CommandParserTest::new(
//                "Unknown command in public channel",
//                "!nosuchcommand", &TEST_CHANNEL,
//                None),
//
//            //CommandParserTest::new(
//            //    "Unknown command in private channel",
//            //    "!nosuchcommand", &IM_CHANNEL,
//            //    cmd(InvalidCommand(&IM_CHANNEL, TEST_USER.clone(), "!nosuchcommand", :unknown))),
//            //
//            //CommandParserTest::new(
//            //    "Set puzzle with too many parameters",
//            //    "!setnian ABCDEFGHI more parameters", &TEST_CHANNEL,
//            //    cmd(InvalidCommand(&TEST_CHANNEL, TEST_USER.clone(),
//            //                   "!setnian ABCDEFGHI more parameters", :wrong_no_of_parameters))),
//            //
//            //CommandParserTest::new(
//            //    "Get puzzle with too many parameters",
//            //    "!nian yoyoyo", &TEST_CHANNEL,
//            //    cmd(InvalidCommand(&TEST_CHANNEL, TEST_USER.clone(), "!nian yoyoyo", :wrong_no_of_parameters))),
//            //
//            //CommandParserTest::new(
//            //    "Help with too many parameters",
//            //    "!helpnian yoyoyo", &TEST_CHANNEL,
//            //    cmd(InvalidCommand(&TEST_CHANNEL, TEST_USER.clone(), "!helpnian yoyoyo", :wrong_no_of_parameters))),
//            //
//            //CommandParserTest::new(
//            //    "Set unsolution command",
//            //    "!unsolution FOO BAR BAZ qux", &IM_CHANNEL,
//            //    cmd(SetUnsolutionCommand(&IM_CHANNEL, TEST_USER.clone(), "FOO BAR BAZ qux"))),
//            //
//            //CommandParserTest::new(
//            //    "Set unsolution command with no params",
//            //    "!unsolution   ", &IM_CHANNEL,
//            //    cmd(InvalidCommand(&IM_CHANNEL, TEST_USER.clone(), "!unsolution   ", :wrong_no_of_parameters))),
//            //
//            //CommandParserTest::new(
//            //    "Set unsolution command ignored in public channel",
//            //    "!unsolution FOO BAR BAZ qux", &TEST_CHANNEL,
//            //    None),
//            //
//            //CommandParserTest::new(
//            //    "Get unsolutions",
//            //    "!unsolutions", &IM_CHANNEL,
//            //    cmd(GetUnsolutionsCommand(&IM_CHANNEL, TEST_USER.clone()))),
//            //
//            //CommandParserTest::new(
//            //    "Get unsolutions is also accepted in public (but response is in private)",
//            //    "!unsolutions", &TEST_CHANNEL,
//            //    cmd(GetUnsolutionsCommand(&TEST_CHANNEL, TEST_USER.clone()))),
//            //
//            //CommandParserTest::new(
//            //    "Get unsolutions, too many params",
//            //    "!unsolutions COOL COOL COOL", &IM_CHANNEL,
//            //    cmd(InvalidCommand(&IM_CHANNEL, TEST_USER.clone(), "!unsolutions COOL COOL COOL",
//            //                   :wrong_no_of_parameters))),
//        ];
//
//        for test in command_parser_tests {
//            let result = parse_command(&test.text.into());
//
//            assert!(result == test.expected, test.description);
//        }
//    }

// facts("CommandParser") do
//     for test in command_parser_tests
//         context(test.description) do
//             event = MessageEvent(test.text, test.channel, TEST_USER.clone(), EventTimestamp("123"))
//             actual = parse_command(event)
//             @fact actual --> test.expected
//         end
//     end
// end
}