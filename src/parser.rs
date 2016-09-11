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

    #[test]
    fn set_puzzle_test() {
        let test_channel = Channel("C0".into());
        let im_channel = Channel("D0".into());
        let test_user = Name("User 0".into());

        let tests = vec![
            CommandParserTest::new(
                "Set puzzle",
                "!setnian ABCDEFGHI", &test_channel,
                Some(Command::SetPuzzle(test_channel.clone(), Puzzle("ABCDEFGHI".into())))),

            CommandParserTest::new(
                "Get puzzle",
                "!nian", &test_channel,
                Some(Command::GetPuzzle(test_channel.clone()))),

            //CommandParserTest::new(
            //    "Help",
            //    "!helpnian", &test_channel,
            //    Some(HelpCommand(&test_channel, TEST_USER.clone()))),

            CommandParserTest::new(
                "Ignore non-commands in public channel",
                "ABCDEFGHI", &test_channel,
                None),

            CommandParserTest::new(
                "Check solution",
                "ABCDEFGHI", &im_channel,
                Some(Command::CheckSolution(im_channel.clone(), test_user.clone(), Word("ABCDEFGHI".into())))),

            CommandParserTest::new(
                "Check solution, with spaces",
                "ABC DEF GHI", &im_channel,
                Some(Command::CheckSolution(im_channel.clone(), test_user.clone(), Word("ABCDEFGHI".into())))),

            CommandParserTest::new(
                "No command",
                "  ", &test_channel,
                None),

            CommandParserTest::new(
                "Unknown command in public channel",
                "!nosuchcommand", &test_channel,
                None),

            CommandParserTest::new(
                "Unknown command in private channel",
                "!nosuchcommand", &im_channel,
                Some(Command::Invalid(im_channel.clone(), "!nosuchcommand".into(),
                                      InvalidReason::UnknownCommand))),

            CommandParserTest::new(
                "Set puzzle with too many parameters",
                "!setnian ABCDEFGHI more parameters", &test_channel,
                Some(Command::Invalid(test_channel.clone(), "!setnian ABCDEFGHI more parameters".into(),
                                      InvalidReason::WrongNoOfParameters))),

            CommandParserTest::new(
                "Get puzzle with too many parameters",
                "!nian yoyoyo", &test_channel,
                Some(Command::Invalid(test_channel.clone(), "!nian yoyoyo".into(),
                                      InvalidReason::WrongNoOfParameters))),

            CommandParserTest::new(
                "Help with too many parameters",
                "!helpnian yoyoyo", &test_channel,
                Some(Command::Invalid(test_channel.clone(), "!helpnian yoyoyo".into(),
                                      InvalidReason::WrongNoOfParameters))),

        ];

        for test in tests {
            let actual = parse_command(&test.text.into());
            assert_eq!(actual, test.expected);
        }
    }

            //CommandParserTest::new(
            //    "Set unsolution command",
            //    "!unsolution FOO BAR BAZ qux", &IM_CHANNEL,
            //    cmd(SetUnsolutionCommand(&IM_CHANNEL, TEST_USER.clone(), "FOO BAR BAZ qux"))),
            //
            //CommandParserTest::new(
            //    "Set unsolution command with no params",
            //    "!unsolution   ", &IM_CHANNEL,
            //    cmd(InvalidCommand(&IM_CHANNEL, TEST_USER.clone(), "!unsolution   ", :wrong_no_of_parameters))),
            //
            //CommandParserTest::new(
            //    "Set unsolution command ignored in public channel",
            //    "!unsolution FOO BAR BAZ qux", &TEST_CHANNEL,
            //    None),
            //
            //CommandParserTest::new(
            //    "Get unsolutions",
            //    "!unsolutions", &IM_CHANNEL,
            //    cmd(GetUnsolutionsCommand(&IM_CHANNEL, TEST_USER.clone()))),
            //
            //CommandParserTest::new(
            //    "Get unsolutions is also accepted in public (but response is in private)",
            //    "!unsolutions", &TEST_CHANNEL,
            //    cmd(GetUnsolutionsCommand(&TEST_CHANNEL, TEST_USER.clone()))),
            //
            //CommandParserTest::new(
            //    "Get unsolutions, too many params",
            //    "!unsolutions COOL COOL COOL", &IM_CHANNEL,
            //    cmd(InvalidCommand(&IM_CHANNEL, TEST_USER.clone(), "!unsolutions COOL COOL COOL",
            //                   :wrong_no_of_parameters))),

}