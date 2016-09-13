Development notes
=================

Missing modules
---------------

- Serializing responses
- Initialization from dictionary
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
similar to `Option<Command>`. However, the parser may result in a known but invalid command, so the
return value should be `Option<Result<Command, InvalidCommand>>`.

This means that the logic module will never receive invalid commands, which makes sense.

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

2016-09-13
----------
Finished the command parser, aside from unsolution commands. They will be simple to add however.
Impllementing the parser required a redesign of the commands, but it appears to be much simpler now.
My previous idea was that each command was a separate struct, implementing a trait `Command`. This
would work just fine, except that it's hard to test. The parser would have had to return a
`Box<&Command>`, and it became very difficult to test that the correct struct was returned inside
the box, as that type information had been lost. This led to a redesign where each command is part
of an enum `Command`. This means that the logic could no longer just call a function on the command
trait, but instead has to match each possible command. Given how few commands there are, and more
importantly it's a fixed number of commands, this was very easy. Also, we get a compiler error if
the match is non-exhaustive.

2016-09-06
----------
The logic is minimally functioning. It does not validate todays puzzle, check how many solutions
there are for it, or store unsolutions.
