Development notes
=================

Missing modules
---------------

- Serializing responses
- Initialization from dictionary
- Parsing commands
- Slack handler (which ties together the above)

Missing features
----------------

- Validate todays puzzle
- Check the number of solutions to todays puzzle
- Store unsolutions

Parse module
------------
The parse module is responsible for creating a command from a Slack message. It is also responsible
for determining the context of the message. For instance, a `CheckSolutionCommand` should not be
created if the message originated in a public channel. Only private channels should be used when
creating `CheckSolutionCommand`.

Because of context dependent commands, it is possible that the parser does not create a command,
even though no error occurred. Therefore, the returned command from the parser should be
`Option<Box<Command>>`. It must be a `Box` because there are many different types of commands, so
the size it not known.
However, the parser may also result in an error, if the command is malformed. Therefore, it must
return a `Result<CommandResult, CommandParserError>`. Here I assume that `CommandResult` is an
alias for `Option<Box<Command>>`.

Module notes: Commands and Responses should be moved to the `types` module, or a module of their
own. They are now located in `logic`, and the parser should not depend on the logic.

We should be able to take a bunch of tests from the Julia version of Niancat.

Initialization from dictionary
------------------------------
This should be fairly easy, as the `Dictionary` struct does its own filtering of words. It will be
a function that reads a file and produces an iterator over the lines of the file. More or less.

NOTE: Check for whitespace trimming issues when integrating with `Dictionary`.

Serializing responses
---------------------
Somewhat difficult to test exactly. There are a number of tests in Julia which we can take.

Slack handler
-------------
This will be a relatively simple part to implement, but the testing would really need some mock
objects. Unfortunately there are no mature mock libraries for Rust that I'm aware.

2016-09-06
----------
The logic is minimally functioning. It does not validate todays puzzle, check how many solutions
there are for it, or store unsolutions.
