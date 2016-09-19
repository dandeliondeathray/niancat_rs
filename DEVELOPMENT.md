Development notes
=================

Missing features
----------------

- Store unsolutions
- Integration tests for the event handler

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

Initialization
--------------
Initialization is done in a couple of steps. The arguments given on the command line are:

1. The Slack token for the given team
2. The _name_ of the channel in which the bot will write notifications
3. The dictionary file.

Initialization should result in a `NiancatHandler` being created, and logged in.
The handler requires a dictionary, and the _channel id_ of the notification channel. Therefore,
initialization must look up the channel id, given the channel name.

The initialization function will result in both a dictionary, and a channel id. The initialization
function must take a trait that will do a channel listing, so we can test the initialization
properly.

Serializing responses
---------------------
Somewhat difficult to test exactly. There are a number of tests in Julia which we can take.

Responses are always associated with a channel. There may be more than one response to a single
command. For instance, solving the puzzle leads to two responses: one to the user that the word is
correct, and one notification to the main channel.

We'll let the event handler actually send the messages. This means that the `response` module will
return a list of channels and strings to send.

Slack handler
-------------
This will be a relatively simple part to implement, but the testing would really need some mock
objects. Unfortunately there are no mature mock libraries for Rust that I'm aware.

2016-09-19
----------
The bot has now been in production for a few days. I messed up error handling in the main function,
causing it to panic more often than necessary.

The next issue should be validation of the puzzle. This involves both ensuring that there is a
solution to the puzzle, as well as counting the number of solutions. Counting the number of
solutions is already supported by the response module, so it simply needs to be added to the logic
module.

Furthermore we need to report the solutions, and who solved it, when the next puzzle is set. This
requires a bit more work, but nothing really massive.

1. We need to store the solutions somewhere, when the puzzle is set. This needs to map against the
   users that solved them. So that requires a map `Word -> User list`.
2. On each valid solution we enter the user name into that map, for the given word.
3. When the next puzzle is set we send the map with the response that the puzzle is set. Actually,
   it will need to be in its own response. If the puzzle is set in a private channel, the all
   solutions should still be sent to the main channel.
4. The map should be reset.

Note that on the first puzzle set on startup there will not be a map set. Hence, we should store it
in an `Option<SolutionMap>`.

2016-09-16
----------
Implemented responses.

2016-09-14
----------
Broke the bot into a main module and a library module, so integration testing can be done.

Implemented the initialization function. Still missing is the actual use of it in the main module,
as well as an adaptor struct for adapting the initialization to the Slack crate.

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
